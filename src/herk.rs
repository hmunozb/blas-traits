use alga::general::ComplexField;
use cblas::{Layout, Part, Transpose, cherk, zherk};
use num_complex::Complex32 as c32;
use num_complex::Complex64 as c64;
use crate::Tsyrk;

pub trait Therk : ComplexField + Tsyrk{
    /// Hermitian rank k update
    /// For real scalars, herk casts Transpose::Conjugate into Transpose::Ordinary and is
    /// completely equivalent to syrk
    fn herk(layout: Layout,
            uplo: Part,
            trans: Transpose,
            n: i32,
            k: i32,
            alpha: Self::RealField,
            a: &[Self],
            lda: i32,
            beta: Self::RealField,
            c: &mut [Self],
            ldc: i32,
    );
}

macro_rules! impl_therk_real{
    ($N: ty) => (
        impl Therk for $N{
            #[inline]
            fn herk(
                layout: Layout,
                uplo: Part,
                trans: Transpose,
                n: i32,
                k: i32,
                alpha: Self::RealField,
                a: &[Self],
                lda: i32,
                beta: Self::RealField,
                c: &mut [Self],
                ldc: i32
            )
            {
                let real_trans = if let Transpose::Conjugate = trans { Transpose::Ordinary } else { trans };
                <$N>::syrk(layout, uplo, real_trans, n, k, alpha, a, lda, beta, c, ldc)
            }
        }
    )
}

macro_rules! impl_therk_complex{
    ($N: ty, $therk: path) => (
        impl Therk for $N{
            #[inline]
            fn herk(
                layout: Layout,
                uplo: Part,
                trans: Transpose,
                n: i32,
                k: i32,
                alpha: Self::RealField,
                a: &[Self],
                lda: i32,
                beta: Self::RealField,
                c: &mut [Self],
                ldc: i32
            )
            {
                unsafe{ $therk(layout, uplo, trans, n, k, alpha, a, lda, beta, c, ldc) }
            }
        }
    )
}

impl_therk_real!(f32);
impl_therk_real!(f64);
impl_therk_complex!(c32, cherk);
impl_therk_complex!(c64, zherk);