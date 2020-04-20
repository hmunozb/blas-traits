use crate::{ComplexField};
use lapacke::{Layout, ssyevr, dsyevr, cheevr, zheevr};
use num_complex::Complex32 as c32;
use num_complex::Complex64 as c64;

pub trait Theevr: ComplexField{
    unsafe fn heevr(layout: Layout,
        jobz: u8,
        range: u8,
        uplo: u8,
        n: i32,
        a: &mut [Self],
        lda: i32,
        vl: Self::RealField,
        vu: Self::RealField,
        il: i32,
        iu: i32,
        abstol: Self::RealField,
        m: &mut i32,
        w: &mut [Self::RealField],
        z: &mut [Self],
        ldz: i32,
        isuppz: &mut [i32],
    ) -> i32;
}

macro_rules! impl_heevr (
    ($N: ty, $heevr: path) => (
        impl Theevr for $N{
            #[inline]
            unsafe fn heevr(layout: Layout,
                jobz: u8,
                range: u8,
                uplo: u8,
                n: i32,
                a: &mut [Self],
                lda: i32,
                vl: Self::RealField,
                vu: Self::RealField,
                il: i32,
                iu: i32,
                abstol: Self::RealField,
                m: &mut i32,
                w: &mut [Self::RealField],
                z: &mut [Self],
                ldz: i32,
                isuppz: &mut [i32],
            ) -> i32{
                $heevr(layout, jobz, range, uplo, n, a, lda, vl, vu, il, iu, abstol,
                       m, w, z, ldz, isuppz)
            }
        }
    )
);

impl_heevr!(f32, ssyevr);
impl_heevr!(f64, dsyevr);
impl_heevr!(c32, cheevr);
impl_heevr!(c64, zheevr);