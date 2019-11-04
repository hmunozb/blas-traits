mod amax;
mod asum;
mod gemm;

pub use amax::ITamax;
pub use asum::RTasum;
pub use gemm::Tgemm;

pub trait BlasScalar: RTasum + Tgemm + ITamax { }

impl<T> BlasScalar for T where T: RTasum + Tgemm + ITamax { }
