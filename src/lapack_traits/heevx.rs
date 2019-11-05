use alga::general::{ComplexField};
use lapacke::{Layout, ssyevx_work, dsyevx_work, zheevx_work, cheevx_work};
use num_complex::Complex32 as c32;
use num_complex::Complex64 as c64;

pub trait Tsyheevx : ComplexField
{
    /// Symmetric/Hermitian eigenvalue problem
    /// Binds to syevx for real scalars and to heevx for complex scalars
    fn syheevx( jobz: u8,
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
                work: &mut [Self],
                lwork: i32,
                rwork: &mut [Self::RealField],
                iwork: &mut [i32],
                ifail: &mut [i32]) -> i32;

    fn rwork_const() -> isize;
}

macro_rules! impl_he_evx (
    ($N: ty, $heevx: path) => (
        impl Tsyheevx for $N {
            #[inline]
            fn syheevx(jobz: u8, range: u8, uplo: u8, n: i32, a: &mut [Self], lda: i32,
                 vl: Self::RealField, vu: Self::RealField, il: i32, iu: i32,  abstol: Self::RealField,
                 m: &mut i32, w: &mut [Self::RealField], z: &mut [Self], ldz: i32,
                 work: &mut [Self], lwork: i32, rwork: &mut [Self::RealField],  //Not used for real-symmetric routines
                 iwork: &mut [i32], ifail: &mut [i32]) -> i32 {
                    let info: i32 =
                        unsafe {
                                $heevx( Layout::RowMajor,
                                        jobz, range, uplo, n, a, lda,
                                        vl, vu, il, iu, abstol,
                                        m, w, z, ldz,
                                        work, lwork, rwork, iwork, ifail)
                        };
                    info
            }

            fn rwork_const() -> isize {
                7
            }
        }
    )
);

macro_rules! impl_sy_evx (
    ($N: ty, $syevx: path) => (
        impl Tsyheevx for $N {
            #[inline]
            fn syheevx(jobz: u8, range: u8, uplo: u8, n: i32, a: &mut [Self], lda: i32,
                 vl: Self::RealField, vu: Self::RealField, il: i32, iu: i32,  abstol: Self::RealField,
                 m: &mut i32, w: &mut [Self::RealField], z: &mut [Self], ldz: i32,
                 work: &mut [Self], lwork: i32, _rwork: &mut [Self::RealField],  //Not used for real-symmetric routines
                 iwork: &mut [i32], ifail: &mut [i32]) -> i32 {
                    let info: i32 =
                        unsafe {
                                $syevx( Layout::RowMajor,
                                        jobz, range, uplo, n, a, lda,
                                        vl, vu, il, iu, abstol,
                                        m, w, z, ldz,
                                        work, lwork, iwork, ifail)
                        };
                    info
            }
            fn rwork_const() -> isize {
                -1
            }
        }
    )
);

impl_sy_evx!(f32, ssyevx_work);
impl_sy_evx!(f64, dsyevx_work);
impl_he_evx!(c32, cheevx_work);
impl_he_evx!(c64, zheevx_work);