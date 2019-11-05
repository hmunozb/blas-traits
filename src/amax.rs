use alga::general::ComplexField;
use cblas::{isamax, idamax, icamax, izamax};
use num_complex::Complex32 as c32;
use num_complex::Complex64 as c64;

pub trait ITamax: ComplexField{
    fn amax(n: i32, x: &[Self], incx: i32) -> i32;
}

macro_rules! impl_itamax(
    ($N: ty, $itamax: path) => (
        impl ITamax for $N{
            fn amax(n: i32, x: &[Self], incx: i32) -> i32{
                unsafe{ $itamax(n, x, incx) }
            }
        }
    )
);

impl_itamax!(f32, isamax);
impl_itamax!(f64, idamax);
impl_itamax!(c32, icamax);
impl_itamax!(c64, izamax);