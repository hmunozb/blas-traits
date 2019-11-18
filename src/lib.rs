mod amax;
mod asum;
mod gemm;
mod herk;
mod syrk;

mod lapack_traits;

pub use amax::ITamax;
pub use asum::RTasum;
pub use herk::Therk;
pub use gemm::Tgemm;
pub use syrk::Tsyrk;
pub use lapack_traits::Tgeqrf;
pub use lapack_traits::Tgesv;
pub use lapack_traits::Tsyheevx;

pub trait BlasScalar: ITamax + RTasum + Therk + Tgemm + Tsyrk
        + Tgeqrf +Tgesv + Tsyheevx { }

impl<T> BlasScalar for T
where T: ITamax + RTasum + Therk + Tgemm + Tsyrk
        + Tgeqrf + Tgesv + Tsyheevx { }

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use openblas_src;

    #[test]
    fn link() {
        ()
    }
}