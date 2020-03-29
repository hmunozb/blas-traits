BLAS/LAPACK Traits for Rust
==========================

Wraps real and complex scalar routines in BLAS/LAPACK
as traits for use in scalar-generic mathematical/scientific
Rust crates. 

Currently implements a small subset of BLAS and LAPACK
routines, but more are being added over time. 
Contributions for any missing routines are welcome.

## Naming and behavior conventions

Almost all simple routines with uniform patterns over all types
behave as defined in BLAS/LAPACK. 
For a routine `xgemm`, where `x` is one
of the types `s`, `d`, `c`, or `z`, the trait name is written as `Tgemm`,
and the method is `gemm`.
For some routines that mix real and complex scalars, such as
`yasum`, where `y = s, d, sc, dz`, the traits are name `RTasum`,
while the methods are simply `asum`.
Routines with integer types such as `ixamax` correspond to
`ITamax` and `amax`.

Additional naming and behavioral complexities arise when 
real and complex matrices are mixed together. 
These are explicitly documented and wrapped over all 
data types as uniformly as possible. In general, `he`
means "hermitian" for complex types and "symmetric" for real types, 
while `sy` means "symmetric" for all types.
Likewise, `un` means unitary for complex types
and orthogonal for real types.

Related routines may be grouped under 
a single trait.