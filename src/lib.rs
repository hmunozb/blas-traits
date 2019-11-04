mod amax;
mod asum;
mod gemm;
mod syrk;

pub use amax::ITamax;
pub use asum::RTasum;
pub use gemm::Tgemm;
pub use syrk::Tsyrk;

pub trait BlasScalar: ITamax + RTasum + Tgemm + Tsyrk{ }

impl<T> BlasScalar for T
where T: ITamax + RTasum + Tgemm + Tsyrk { }
