mod geqrf;
mod gesv;
mod heevx;

pub use gesv::Tgesv;
pub use heevx::Theevx;
pub use geqrf::Tgeqrf;
use crate::blas::BlasScalar;

pub trait LapackScalar: BlasScalar
  + Tgeqrf +Tgesv + Theevx { }

impl LapackScalar for f32{}
impl LapackScalar for f64{}
impl LapackScalar for num_complex::Complex<f32>{}
impl LapackScalar for num_complex::Complex<f64>{}