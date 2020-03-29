mod amax;
mod asum;
mod axpy;
mod gemm;
mod herk;
mod syrk;

pub use amax::ITamax;
pub use asum::RTasum;
pub use axpy::Taxpy;
pub use gemm::Tgemm;
pub use herk::Therk;
pub use syrk::Tsyrk;

pub trait BlasScalar:
    ITamax +
    RTasum +
    Taxpy +
    Tgemm +
    Therk +
    Tsyrk
 { }

impl BlasScalar for f32{}
impl BlasScalar for f64{}
impl BlasScalar for num_complex::Complex<f32>{}
impl BlasScalar for num_complex::Complex<f64>{}