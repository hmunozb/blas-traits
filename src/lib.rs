mod asum;
mod gemm;
pub use asum::RTasum;
pub use gemm::Tgemm;

pub trait BlasScalar: RTasum + Tgemm { }

impl<T> BlasScalar for T where T: RTasum + Tgemm { }
