use pathfinder_gl::{GLDevice, GLVersion};
use pathfinder_svg::SVGScene;
use usvg::{Tree, Options};
use pathfinder_renderer::{
    concurrent::{
        rayon::RayonExecutor,
    },
    gpu::{
        options::{DestFramebuffer, RendererOptions, RendererMode, RendererLevel},
        renderer::{Renderer},
    },
    options::{BuildOptions, RenderTransform}
};
use pathfinder_gpu::{Device, TextureData, RenderTarget, TextureFormat};
use pathfinder_geometry::{
    vector::{Vector2F, Vector2I},
    rect::{RectI},
};
use pathfinder_color::ColorF;
use pathfinder_resources::embedded::EmbeddedResourceLoader;
use khronos_egl as egl;
use egl::{DynamicInstance};

pub struct Rasterizer {
    egl: DynamicInstance::<egl::EGL1_4>,
    display: egl::Display,
    renderer: Option<(Renderer<GLDevice>, Vector2I)>,
}

// unsafe impl Send for Rasterizer {}

impl Rasterizer {

    pub fn new() -> Self {
        let egl = unsafe {
            egl::DynamicInstance::<egl::EGL1_4>::load_required().expect("unable to load libEGL.so.1")
        };

        let display = egl.get_display(egl::DEFAULT_DISPLAY).expect("display");
        let (_major, _minor) = egl.initialize(display).expect("init");
    
        let attrib_list = [
            egl::SURFACE_TYPE, egl::PBUFFER_BIT,
            egl::BLUE_SIZE, 8,
            egl::GREEN_SIZE, 8,
            egl::RED_SIZE, 8,
            egl::DEPTH_SIZE, 8,
            egl::RENDERABLE_TYPE, egl::OPENGL_BIT,
            egl::NONE
        ];
        
        let config = egl.choose_first_config(display, &attrib_list).unwrap().unwrap();
    
        let pbuffer_attrib_list = [
            egl::NONE
        ];
        let surface = egl.create_pbuffer_surface(display, config, &pbuffer_attrib_list).unwrap();
    
        egl.bind_api(egl::OPENGL_API).expect("unable to select OpenGL API");
    
        let context = egl.create_context(display, config, None, &[egl::NONE]).unwrap();
        egl.make_current(display, Some(surface), Some(surface), Some(context)).unwrap();
    
        // Setup Open GL.
        gl::load_with(|name| egl.get_proc_address(name).unwrap() as *const std::ffi::c_void);
    

        Rasterizer {
            egl, display,
            renderer: None,
        }
    }

    fn renderer_for_size(&mut self, size: Vector2I) -> &mut Renderer<GLDevice> {
        let (ref mut renderer, ref mut current_size) = *self.renderer.get_or_insert_with(|| {
            let render_level = RendererLevel::D3D9;
            let background = ColorF::new(0.0, 0.0, 0.0, 0.0);
            let resource_loader = EmbeddedResourceLoader::new();

            let renderer_gl_version = match render_level {
                RendererLevel::D3D9 => GLVersion::GLES3,
                RendererLevel::D3D11 => GLVersion::GL4,
            };
            let render_mode = RendererMode { level: render_level };
    
            let device = GLDevice::new(renderer_gl_version, 0);

            let tex = device.create_texture(TextureFormat::RGBA8, size);
            let fb = device.create_framebuffer(tex);
            let dest = DestFramebuffer::Other(fb);
            let render_options = RendererOptions {
                dest,
                background_color: Some(background),
                show_debug_ui: false,
            };
            let renderer = Renderer::new(device,
                &resource_loader,
                render_mode,
                render_options,
            );
            (renderer, size)
        });

        if size != *current_size {
            let tex = renderer.device().create_texture(TextureFormat::RGBA8, size);
            let fb = renderer.device().create_framebuffer(tex);
            let dest = DestFramebuffer::Other(fb);
            renderer.options_mut().dest = dest;
            *current_size = size;
        }

        renderer
    }

    pub fn rasterize(&mut self, data: Vec<u8>) -> Vec<u8> {
        let tree = Tree::from_data(&data, &Options::default()).unwrap();
        let mut scene = SVGScene::from_tree(&tree).scene;
        let size = scene.view_box().size().ceil().to_i32();

        let renderer = self.renderer_for_size(size);
        
        let options = BuildOptions {
            transform: RenderTransform::default(),
            dilation: Vector2F::default(),
            subpixel_aa_enabled: false
        };

        scene.build_and_render(renderer, options, RayonExecutor);

        let render_target = match renderer.options().dest {
            DestFramebuffer::Other(ref fb) => RenderTarget::Framebuffer(fb),
            _=> panic!()
        };
        let texture_data_receiver = renderer.device().read_pixels(&render_target, RectI::new(Vector2I::zero(), size));
        let pixels = match renderer.device().recv_texture_data(&texture_data_receiver) {
            TextureData::U8(pixels) => pixels,
            _ => panic!("Unexpected pixel format for default framebuffer!"),
        };
        return pixels;
    }
}

impl Drop for Rasterizer {
    fn drop(&mut self) {
        self.egl.terminate(self.display).unwrap();
    }
}

