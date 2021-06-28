//! Generic real/complex scalar trait wrappers for BLAS and LAPACK routines

pub trait Scalar : Copy{
    type Real;
}

impl Scalar for f32{
    type Real = f32;
}

impl Scalar for f64{
    type Real = f64;
}

impl Scalar for c32{
    type Real = f32;
}

impl Scalar for c64{
    type Real = f64;
}


//#[cfg(not(feature = "simba"))]
//pub use alga::general::{RealField, ComplexField};
//#[cfg(not(feature = "simba"))]
//pub use alga::general::{SubsetOf, SupersetOf};

pub mod simba_feat;

#[cfg(feature = "simba")]
pub use simba_feat::LComplexField;


//pub use cblas::Layout as Layout;
//pub use cblas::Transpose as Transpose;

pub use num_complex::Complex32 as c32;
pub use num_complex::Complex64 as c64;


pub mod blas_impl;
pub mod lapack_impl;

pub use blas_impl::*;
pub use lapack_impl::*;

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use lapack_sys;
    use blas_sys;

    #[test]
    fn link() {
        ()
    }
}