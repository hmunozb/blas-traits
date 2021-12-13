use crate::Scalar;
use num_complex::Complex32 as c32;
use num_complex::Complex64 as c64;

pub trait Tnrm2: Scalar{
    unsafe fn nrm2(n: i32, x: &[Self], incx: i32) -> Self::Real;
}

macro_rules! impl_tnrm2(
    ($N: ty, $tnrm2: path) => (
        impl Tnrm2 for $N{
            unsafe fn nrm2(n: i32, x: &[Self], incx: i32) -> Self::Real{
                $tnrm2(n, x, incx)
            }
        }
    )
);

impl_tnrm2!(f32, cblas::snrm2);
impl_tnrm2!(f64, cblas::dnrm2);
impl_tnrm2!(c32, cblas::scnrm2);
impl_tnrm2!(c64, cblas::dznrm2);