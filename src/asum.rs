use alga::general::ComplexField;
use cblas::{sasum, dasum, scasum, dzasum};
use num_complex::Complex32 as c32;
use num_complex::Complex64 as c64;

pub trait RTasum: ComplexField{
    fn asum(n: i32, x: &[Self], incx: i32) -> Self::RealField;
}

macro_rules! impl_rtasum (
    ($N: ty, $rtasum: path) => (
        impl RTasum for $N{
            fn asum(n: i32, x: &[Self], incx: i32) -> Self::RealField{

                unsafe{ $rtasum(n, x, incx) }
            }
        }
    )
);

impl_rtasum!(f32, sasum);
impl_rtasum!(f64, dasum);
impl_rtasum!(c32, scasum);
impl_rtasum!(c64, dzasum);