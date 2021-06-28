use crate::Scalar;
use lapack::{ sgeqrf, dgeqrf, cgeqrf, zgeqrf,
              sorgqr, dorgqr, cungqr, zungqr};
use num_complex::Complex32 as c32;
use num_complex::Complex64 as c64;

pub trait Tgeqrf: Scalar {
    unsafe fn geqrf(
             m: i32,
             n: i32,
             a: &mut [Self],
             lda: i32,
             tau: &mut [Self],
            work: &mut [Self],
            lwork: i32,
            info: &mut i32);

    unsafe fn ungqr(
             m: i32,
             n: i32,
             k: i32,
             a: &mut [Self],
             lda: i32,
             tau: &[Self],
             work: &mut [Self],
             lwork: i32,
             info: &mut i32);
}

macro_rules! impl_tgeqrf(
($N: ty, $tgeqrf: path, $torungqr: path) => (
        impl Tgeqrf for $N{
            #[inline]
            unsafe fn geqrf(
                m: i32,
                n: i32,
                a: &mut [Self],
                lda: i32,
                tau: &mut [Self],
                work: &mut [Self],
                lwork: i32,
                info: &mut i32
            ){
                    $tgeqrf(m, n, a, lda, tau, work, lwork, info)
            }

            unsafe fn ungqr(
                m: i32,
                n: i32,
                k: i32,
                a: &mut [Self],
                lda: i32,
                tau: &[Self],
                work: &mut [Self],
                lwork: i32,
                info: &mut i32
            ){
                    $torungqr(m, n, k, a, lda, tau, work, lwork, info)
            }
        }
    )
);

impl_tgeqrf!(f32, sgeqrf, sorgqr);
impl_tgeqrf!(f64, dgeqrf, dorgqr);
impl_tgeqrf!(c32, cgeqrf, cungqr);
impl_tgeqrf!(c64, zgeqrf, zungqr);

