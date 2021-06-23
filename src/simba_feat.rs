#![cfg(feature = "simba")]

use crate::LapackScalar;

use simba::scalar::ComplexField as ComplexField;

pub trait LComplexField: ComplexField<RealField=Self::Real> + LapackScalar
{

}

impl<T> LComplexField for T where
T: LapackScalar + ComplexField<RealField=Self::Real>{

}