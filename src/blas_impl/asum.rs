use crate::Scalar;
use num_complex::Complex32 as c32;
use num_complex::Complex64 as c64;

pub trait RTasum: Scalar {
    unsafe fn asum(n: i32, x: &[Self], incx: i32) -> Self::Real;
}

macro_rules! impl_rtasum (
    ($N: ty, $rtasum: path) => (
        impl RTasum for $N{
            unsafe fn asum(n: i32, x: &[Self], incx: i32) -> Self::Real{
                $rtasum(n, x, incx)
            }
        }
    )
);

impl_rtasum!(f32, blas::sasum);
impl_rtasum!(f64, blas::dasum);
impl_rtasum!(c32, blas::scasum);
impl_rtasum!(c64, blas::dzasum);