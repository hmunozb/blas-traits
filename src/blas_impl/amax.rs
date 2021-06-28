use crate::Scalar;
use num_complex::Complex32 as c32;
use num_complex::Complex64 as c64;

pub trait ITamax: Scalar {
    unsafe fn amax(n: i32, x: &[Self], incx: i32) -> usize;
}

macro_rules! impl_itamax(
    ($N: ty, $itamax: path) => (
        impl ITamax for $N{
            unsafe fn amax(n: i32, x: &[Self], incx: i32) -> usize{
                $itamax(n, x, incx)
            }
        }
    )
);

impl_itamax!(f32, blas::isamax);
impl_itamax!(f64, blas::idamax);
impl_itamax!(c32, blas::icamax);
impl_itamax!(c64, blas::izamax);