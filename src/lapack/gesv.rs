use crate::ComplexField;
use lapacke::{Layout, sgesv, dgesv, cgesv, zgesv};
use num_complex::Complex32 as c32;
use num_complex::Complex64 as c64;

pub trait Tgesv : ComplexField{
    unsafe fn gesv(layout: Layout,
            n: i32,
            nrhs: i32,
            a: &mut [Self],
            lda: i32,
            ipiv: &mut [i32],
            b: &mut [Self],
            ldb: i32,) -> i32;
}

macro_rules! impl_tgesv(
    ($N: ty, $tgesv: path) => (
        impl Tgesv for $N{
            #[inline]
            unsafe fn gesv(
                layout: Layout,
                n: i32,
                nrhs: i32,
                a: &mut [Self],
                lda: i32,
                ipiv: &mut [i32],
                b: &mut [Self],
                ldb: i32,) -> i32
            {
                $tgesv(layout, n, nrhs, a, lda, ipiv, b, ldb)
            }
        }
    )
);

impl_tgesv!(f32, sgesv);
impl_tgesv!(f64, dgesv);
impl_tgesv!(c32, cgesv);
impl_tgesv!(c64, zgesv);

