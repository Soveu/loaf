#![no_std]
#![cfg_attr(feature = "nightly", feature(const_generics))]
#![cfg_attr(feature = "nightly", allow(incomplete_features))]

//! Why have a slice when you can have a loaf?
//!
//! ## What this is
//! Sometimes you know that a slice _must_ have at least one element in it,
//! but Rust forces you to do "last minute decision" by `unwrap()`ing `Option`
//! from for example `first()` for `split_first()` methods.
//!
//! [Loaf] guarantees to have at least one element by its definition.
//! 
//! ## Safety
//! Currently unsafe code is only used to cast between `[T]` and `Loaf<T>` pointers.
//!
//! Rust's pointers to unsized types consist of two elements: pointer to data and 
//! length of the unsized part \
//! So a fat pointer to `[T]` has a pointer to the beginning of buffer and its length \
//! `Loaf` has an one element array of `T` _and_ `[T]` and because of it when casting
//! from slice pointer, the pointer is kept, but the length is decremented
//! (remember, fat pointer keeps length of the __unsized part__)
//!
//! The one element array is guaranteed to be first, because unsized types must
//! be at the end of struct.

#[cfg(feature = "alloc")]
#[doc(hidden)]
pub extern crate alloc;

#[cfg_attr(all(feature = "nightly", not(doc)), path = "loaf_nightly.rs")]
mod loaf;

pub use crate::loaf::*;

#[cfg(doc)]
mod loaf_nightly;

#[cfg(doc)]
pub use loaf_nightly::*;

