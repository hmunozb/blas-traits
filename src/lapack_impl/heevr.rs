#![allow(unused_variables)]

use crate::Scalar;
use lapack::{ssyevr, dsyevr, cheevr, zheevr};
use num_complex::Complex32 as c32;
use num_complex::Complex64 as c64;

pub trait Theevr: Scalar {
    /// rwork and lrwork are not utilized for f32 and f64
    unsafe fn heevr(
                    jobz: u8,
                    range: u8,
                    uplo: u8,
                    n: i32,
                    a: &mut [Self],
                    lda: i32,
                    vl: Self::Real,
                    vu: Self::Real,
                    il: i32,
                    iu: i32,
                    abstol: Self::Real,
                    m: &mut i32,
                    w: &mut [Self::Real],
                    z: &mut [Self],
                    ldz: i32,
                    isuppz: &mut [i32],
                    work: &mut [Self],
                    lwork: i32,
                    rwork: &mut [Self::Real],
                    lrwork: i32,
                    iwork: &mut [i32],
                    liwork: i32,
                    info: &mut i32
    );
}


macro_rules! impl_heevr_real (
    ($N: ty, $heevr: path) => (
        impl Theevr for $N{
            #[inline]
            unsafe fn heevr(
                jobz: u8,
                range: u8,
                uplo: u8,
                n: i32,
                a: &mut [Self],
                lda: i32,
                vl: Self::Real,
                vu: Self::Real,
                il: i32,
                iu: i32,
                abstol: Self::Real,
                m: &mut i32,
                w: &mut [Self::Real],
                z: &mut [Self],
                ldz: i32,
                isuppz: &mut [i32],
                work: &mut [Self],
                lwork: i32,
                rwork: &mut [Self::Real],
                lrwork: i32,
                iwork: &mut [i32],
                liwork: i32,
                info: &mut i32
            ){
                $heevr(jobz, range, uplo, n, a, lda, vl, vu, il, iu, abstol,
                       m, w, z, ldz, isuppz,work, lwork, iwork, liwork, info)
            }
        }
    )
);

macro_rules! impl_heevr (
    ($N: ty, $heevr: path) => (
        impl Theevr for $N{
            #[inline]
            unsafe fn heevr(
                jobz: u8,
                range: u8,
                uplo: u8,
                n: i32,
                a: &mut [Self],
                lda: i32,
                vl: Self::Real,
                vu: Self::Real,
                il: i32,
                iu: i32,
                abstol: Self::Real,
                m: &mut i32,
                w: &mut [Self::Real],
                z: &mut [Self],
                ldz: i32,
                isuppz: &mut [i32],
                work: &mut [Self],
                lwork: i32,
                rwork: &mut [Self::Real],
                lrwork: i32,
                iwork: &mut [i32],
                liwork: i32,
                info: &mut i32
            ){
                $heevr(jobz, range, uplo, n, a, lda, vl, vu, il, iu, abstol,
                       m, w, z, ldz, isuppz, work, lwork, rwork, lrwork, iwork, liwork,
                       info)
            }
        }
    )
);

impl_heevr_real!(f32, ssyevr);
impl_heevr_real!(f64, dsyevr);
impl_heevr!(c32, cheevr);
impl_heevr!(c64, zheevr);