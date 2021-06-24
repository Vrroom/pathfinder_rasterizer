use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::{
    class::PyObjectProtocol,
};
use image::RgbaImage;

macro_rules! wrap {
    ($name:ident, $inner:ty) => {
        #[pyclass(unsendable)]
        pub struct $name {
            inner: $inner
        }
        impl std::convert::From<$inner> for $name {
            fn from(inner: $inner) -> Self {
                $name { inner }
            }
        }
        impl std::convert::Into<$inner> for $name {
            fn into(self) -> $inner {
                self.inner
            }
        }
        impl std::ops::Deref for $name {
            type Target = $inner;
            fn deref(&self) -> &$inner {
                &self.inner
            }
        }
        impl $name {
            pub fn into_inner(self) -> $inner {
                self.inner
            }
        }
    };
}

pub mod rasterizer;
use rasterizer::*;

wrap!(PyRasterizer, Rasterizer);

pub struct Data(Vec<u8>);
impl IntoPy<PyObject> for Data {
    fn into_py(self, py: Python<'_>) -> PyObject {
        PyBytes::new(py, &self.0).into()
    }
}

#[pymethods]
impl PyRasterizer {
    #[new]
    pub fn new() -> PyRasterizer {
        PyRasterizer::from(Rasterizer::new())
    }

    #[text_signature = "($self, data: Vector)"]
    pub fn rasterize(&mut self, data: Vec<u8>, minx: i32, miny: i32) -> PyResult<(Data, u32, u32)>{
        let (data, width, height) = self.inner.rasterize(data, minx, miny);
        Ok((Data(data), width, height))
    }
}

#[pyproto]
impl PyObjectProtocol for PyRasterizer {
    fn __str__(&self) -> PyResult<String> {
        Ok("Rasterizer".to_string())
    }
}

#[pymodule]
fn pathfinder_rasterizer(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyRasterizer>()?;
    Ok(())    
}

