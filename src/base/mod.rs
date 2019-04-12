//! [Reexported at the root of this crate.] Data structures for vector and matrix computations.

pub mod allocator;
mod blas;
pub mod constraint;
pub mod coordinates;
pub mod default_allocator;
pub mod dimension;
pub mod iter;
mod ops;
pub mod storage;

mod alias;
mod alias_slice;
mod cg;
mod componentwise;
mod construction;
mod construction_slice;
mod conversion;
mod edition;
pub mod indexing;
mod matrix;
mod matrix_alga;
mod array_storage;
mod matrix_slice;
#[cfg(any(feature = "std", feature = "alloc"))]
mod vec_storage;
mod properties;
mod scalar;
mod swizzle;
mod unit;
mod statistics;
mod norm;

#[doc(hidden)]
pub mod helper;

pub use self::{
    matrix::*,
    scalar::*,
    unit::*,
    norm::*,

    default_allocator::*,
    dimension::*,

    alias::*,
    alias_slice::*,
    array_storage::*,
    matrix_slice::*
};

#[cfg(any(feature = "std", feature = "alloc"))]
pub use self::vec_storage::*;
