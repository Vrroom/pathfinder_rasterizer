use pyo3::prelude::*;
use pyo3::{
    class::PyObjectProtocol,
};

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

#[pymethods]
impl PyRasterizer {
    #[new]
    pub fn new() -> PyRasterizer {
        PyRasterizer::from(Rasterizer::new())
    }

    #[text_signature = "($self, data: Vector)"]
    pub fn rasterize(&mut self, data: Vec<u8>) -> PyResult<(Vec<u8>, u32, u32)>{
        Ok(self.inner.rasterize(data))
    }
}

#[pyproto]
impl PyObjectProtocol for PyRasterizer {
    fn __str__(&self) -> PyResult<String> {
        Ok("Rasterizer".to_string())
    }
}

#[pymodule]
fn rasterize(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyRasterizer>()?;
    Ok(())    
}

