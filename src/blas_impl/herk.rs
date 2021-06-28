use crate::Scalar;
use blas::{cherk, zherk};
use num_complex::Complex32 as c32;
use num_complex::Complex64 as c64;
use crate::blas_impl::syrk::Tsyrk;

pub trait Therk : Scalar + Tsyrk{
    /// Hermitian rank k update
    /// For real scalars, herk casts Transpose::Conjugate into Transpose::Ordinary and is
    /// completely equivalent to syrk
    unsafe fn herk(
                   uplo: u8,
                   trans: u8,
                   n: i32,
                   k: i32,
                   alpha: Self::Real,
                   a: &[Self],
                   lda: i32,
                   beta: Self::Real,
                   c: &mut [Self],
                   ldc: i32,
    );
}

macro_rules! impl_therk_real{
    ($N: ty) => (
        impl Therk for $N{
            #[inline]
            unsafe fn herk(
                uplo: u8,
                trans: u8,
                n: i32,
                k: i32,
                alpha: Self::Real,
                a: &[Self],
                lda: i32,
                beta: Self::Real,
                c: &mut [Self],
                ldc: i32
            )
            {
                let real_trans = if trans == ('C' as u8) || trans == ('c' as u8) { 'T' as u8 }
                                 else { trans };
                <$N>::syrk(uplo, real_trans, n, k, alpha, a, lda, beta, c, ldc)
            }
        }
    )
}

macro_rules! impl_therk_complex{
    ($N: ty, $therk: path) => (
        impl Therk for $N{
            #[inline]
            unsafe fn herk(
                uplo: u8,
                trans: u8,
                n: i32,
                k: i32,
                alpha: Self::Real,
                a: &[Self],
                lda: i32,
                beta: Self::Real,
                c: &mut [Self],
                ldc: i32
            )
            {
                $therk(uplo, trans, n, k, alpha, a, lda, beta, c, ldc)
            }
        }
    )
}

impl_therk_real!(f32);
impl_therk_real!(f64);
impl_therk_complex!(c32, cherk);
impl_therk_complex!(c64, zherk);