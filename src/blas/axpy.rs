use crate::ComplexField;
use num_complex::Complex32 as c32;
use num_complex::Complex64 as c64;

pub trait Taxpy: ComplexField{
    unsafe fn axpy(n: i32, alpha: Self, x: &[Self], incx: i32, y: &mut [Self], incy: i32);
}

macro_rules! impl_taxpy(
    ($N: ty, $taxpy: path) => (
        impl Taxpy for $N{
            unsafe fn axpy(n: i32, alpha: Self, x: &[Self], incx: i32, y: &mut [Self], incy: i32) {
                $taxpy(n, alpha, x, incx, y, incy)
            }
        }
    )
);

impl_taxpy!(f32, cblas::saxpy);
impl_taxpy!(f64, cblas::daxpy);
impl_taxpy!(c32, cblas::caxpy);
impl_taxpy!(c64, cblas::zaxpy);