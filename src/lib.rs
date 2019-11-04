mod amax;
mod asum;
mod gemm;
mod syrk;

mod lapack_traits;

pub use amax::ITamax;
pub use asum::RTasum;
pub use gemm::Tgemm;
pub use syrk::Tsyrk;
pub use lapack_traits::Tgesv;
pub use lapack_traits::Tsyheevx;

pub trait BlasScalar: ITamax + RTasum + Tgemm + Tsyrk + Tgesv + Tsyheevx { }

impl<T> BlasScalar for T
where T: ITamax + RTasum + Tgemm + Tsyrk + Tgesv + Tsyheevx { }
