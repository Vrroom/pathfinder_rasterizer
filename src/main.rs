mod rasterizer;
use rasterizer::*;
use image::RgbaImage;

fn main() {
    let mut args = std::env::args();
    args.next().unwrap();

    let input = args.next().expect("input");
    let output = args.next().expect("output");

    let input_data = std::fs::read(&input).expect("read input");
    let mut rasterizer = Rasterizer::new();
    let (pixels, h, w) = rasterizer.rasterize(input_data);
    let image = RgbaImage::from_raw(h, w, pixels).unwrap();
    image.save(&output).unwrap();
}
