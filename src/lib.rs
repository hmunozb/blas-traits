extern crate num_complex;
pub use num_complex::Complex32 as c32;
pub use num_complex::Complex64 as c64;

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

impl BlasScalar for f32{}
impl BlasScalar for f64{}
impl BlasScalar for num_complex::Complex<f32>{}
impl BlasScalar for num_complex::Complex<f64>{}


//impl<T> BlasScalar for T
//where T: ITamax + RTasum + Therk + Tgemm + Tsyrk
//        + Tgeqrf + Tgesv + Tsyheevx { }

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use openblas_src;

    #[test]
    fn link() {
        ()
    }
}