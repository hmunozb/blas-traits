#![cfg(feature = "simba")]

use crate::LapackScalar;

use simba::scalar::ComplexField;

pub trait LComplexField: ComplexField + LapackScalar<Real=Self::RealField>
{

}

impl<T> LComplexField for T where
T: ComplexField + LapackScalar<Real=Self::RealField>{

}