//! Generic real/complex scalar trait wrappers for BLAS and LAPACK routines

pub use alga::general::ComplexField as ComplexField;
pub use cblas::Layout as Layout;
pub use cblas::Transpose as Transpose;

pub use num_complex::Complex32 as c32;
pub use num_complex::Complex64 as c64;


pub mod blas;
pub mod lapack;

pub use blas::*;
pub use lapack::*;

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use openblas_src;

    #[test]
    fn link() {
        ()
    }
}