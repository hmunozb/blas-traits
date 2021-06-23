use crate::Scalar;
use cblas::{Layout, Transpose, sgemm, dgemm, cgemm, zgemm};
use num_complex::Complex32 as c32;
use num_complex::Complex64 as c64;

pub trait Tgemm : Scalar {
    unsafe fn gemm(
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

macro_rules! impl_tgemm {
    ($N: ty, $tgemm: path) => (
        impl Tgemm for $N{
            #[inline]
            unsafe fn gemm(
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
                $tgemm(layout, transa, transb, m, n, k, alpha,
                        a, lda, b, ldb, beta, c ,ldc);
            }
        }
    )
}

impl_tgemm!(f32, sgemm);
impl_tgemm!(f64, dgemm);
impl_tgemm!(c32, cgemm);
impl_tgemm!(c64, zgemm);