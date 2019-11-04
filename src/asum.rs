use alga::general::ComplexField;
use cblas::{Layout, Transpose, sasum, dasum, scasum, dzasum};
use num_complex::Complex32 as c32;
use num_complex::Complex64 as c64;

pub trait RTasum: ComplexField{
    fn asum(n: i32, x: &[Self], incx: i32) -> Self::RealField;
}

macro_rules! impl_xasum(
    ($N: ty, $xasum: path) =>(
        impl Xasum for $N{
            fn asum(n: i32, x: &[Self], incx: i32) -> Self::RealField{
                $xasum(n, x, incx)
            }
        }
    )
);

impl_xasum!(f32, sasum);
impl_xasum!(f64, dasum);
impl_xasum!(c32, scasum);
impl_xasum!(c64, dzasum);