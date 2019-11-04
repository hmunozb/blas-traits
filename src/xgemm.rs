use alga::general::ComplexField;
use cblas::{Layout, Transpose, dgemm, sgemm, cgemm, zgemm};
use num_complex::Complex32 as c32;
use num_complex::Complex64 as c64;

pub trait Xgemm : ComplexField{
    fn xgemm(
        layout: Layout,
        transa: Transpose,
        transb: Transpose,
        m: i32,
        n: i32,
        k: i32,
        alpha: Self,
        a: &[Self],
        lda: i32,
        b: &[Self],
        ldb: i32,
        beta: Self,
        c: &mut [Self],
        ldc: i32,
    );
}

macro_rules! impl_xgemm {
    ($N: ty, $xgemm: path) => (
        impl Xgemm for $N{
            #[inline]
            fn xgemm(
                layout: Layout,
                transa: Transpose,
                transb: Transpose,
                m: i32,
                n: i32,
                k: i32,
                alpha: Self,
                a: &[Self],
                lda: i32,
                b: &[Self],
                ldb: i32,
                beta: Self,
                c: &mut [Self],
                ldc: i32,
            ){
                unsafe{
                    $xgemm(layout, transa, transb, m, n, k, alpha,
                            a, lda, b, ldb, b, ldb, beta, c ,ldc);
                }
            }
        }
    )
}

impl_xgemm!(f32, sgemm);
impl_xgemm!(f64, dgemm);
impl_xgemm!(c32, cgemm);
impl_xgemm!(c64, zgemm);