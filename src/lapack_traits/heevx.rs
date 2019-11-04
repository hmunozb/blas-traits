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

//
//macro_rules! rwork_arg (
//    (f32; $call:ident ; $($head:expr),* ; $($tail:expr),* )
//        => ( $call($($head,)*  $($tail,)*) );
//    (f64; $call:ident ; $($head:expr),* ; $($tail:expr),* )
//        => ( $call($($head,)*  $($tail,)*) );
//    (c32; $call:ident ; $($head:expr),* ; $($tail:expr),* )
//        => ( $call($($head,)* rwork,  $($tail,)*) );
//    (c64; $call:ident ; $($head:expr),* ; $($tail:expr),* )
//        => ( $call($($head,)* rwork,  $($tail,)*) );
//);

//macro_rules! rwork_const (
//    (f32; )
//        => ( -1 );
//    (f64; )
//        => ( -1 );
//    (c32; )
//        => ( 7 );
//    (c64; )
//        => ( 7 );
//);


//macro_rules! with_rwork {
//    ( $heevx: path; $($head:expr),* ; $($tail:expr),*;  0; $rwork:ident) => {
//                $heevx($($head,)*  $($tail,)*)  };
//    ( $heevx: path; $($head:expr),* ; $($tail:expr),*;  1; $rwork:ident) => {
//                $heevx($($head,)* $rwork, $($tail,)*)  };
//    ( $heevx: path; $($head:expr),* ; $($tail:expr),*; $r:literal; $rwork:ident) => {
//                $heevx($($head,)* $rwork, $($tail,)*) };
//
//}


//            fn syhe_evx_work_size(jobz: u8, uplo: u8, n: i32, a: &mut [Self], lda: i32, info: &mut i32,
//                                vl: Self::RealField, vu: Self::RealField, il: i32, iu: i32, abstol: Self::RealField) -> i32 {
//                let mut m: i32 = 0;
//                let mut w    = [ <Self::RealField as Zero>::zero() ];
//                let mut w    = [ Self::zero() ];
//                let mut work = [ Self::zero() ];
//                let mut rwork = [ <Self::RealField as Zero>::zero() ];
//                let mut iwork = [ Zero::zero() ];
//                let mut ifail = 0;
//                let lwork    = -1 as i32;
//                let info = Self::syhe_evx(jobz, range, uplo, n, a, lda,vl, vu, il, iu, m, w, z, 0,
//                                            &mut work, -1, &mut iwork, &mut ifail)
//                unsafe { $xsyev(jobz, uplo, n, a, lda, &mut w, &mut work, lwork, info); }
//                ComplexHelper::real_part(work[0]) as i32
//            }

macro_rules! impl_he_evx (
    ($N: ty, $heevx: path) => (
        impl SyHeEvx for $N {
            #[inline]
            fn syhe_evx(jobz: u8, range: u8, uplo: u8, n: i32, a: &mut [Self], lda: i32,
                 vl: Self::RealField, vu: Self::RealField, il: i32, iu: i32,  abstol: Self::RealField,
                 m: &mut i32, w: &mut [Self::RealField], z: &mut [Self], ldz: i32,
                 work: &mut [Self], lwork: i32, rwork: &mut [Self::RealField],  //Not used for real-symmetric routines
                 iwork: &mut [i32], ifail: &mut [i32]) -> i32 {
                    let mut info: i32 = 0;
                    unsafe {
                            $heevx( Layout::RowMajor,
                                    jobz, range, uplo, n, a, lda,
                                    vl, vu, il, iu, abstol,
                                    m, w, z, ldz,
                                    work, lwork, rwork, iwork, ifail, &mut info);
                    }
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
        impl SyHeEvx for $N {
            #[inline]
            fn syhe_evx(jobz: u8, range: u8, uplo: u8, n: i32, a: &mut [Self], lda: i32,
                 vl: Self::RealField, vu: Self::RealField, il: i32, iu: i32,  abstol: Self::RealField,
                 m: &mut i32, w: &mut [Self::RealField], z: &mut [Self], ldz: i32,
                 work: &mut [Self], lwork: i32, _rwork: &mut [Self::RealField],  //Not used for real-symmetric routines
                 iwork: &mut [i32], ifail: &mut [i32]) -> i32 {
                    let mut info: i32 = 0;
                    unsafe {
                            $syevx( Layout::RowMajor,
                                    jobz, range, uplo, n, a, lda,
                                    vl, vu, il, iu, abstol,
                                    m, w, z, ldz,
                                    work, lwork, iwork, ifail,
                                    &mut info)
                    }
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