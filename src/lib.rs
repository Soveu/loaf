#![no_std]
#![cfg_attr(any(feature = "nightly", doc), feature(const_generics))]
#![cfg_attr(any(feature = "nightly", doc), allow(incomplete_features))]

//! Why have a slice when you can have a loaf?
//!
//! ## What this is
//! Sometimes you know that a slice _must_ have at least one element in it,
//! but Rust forces you to do "last minute decision" by `unwrap()`ing `Option`
//! from for example `first()` for `split_first()` methods.
//!
//! [Loaf] guarantees to have at least one element by its definition.
//!
//! ## How it works
//! First, lets consider a simple slice
//!
//! ```
//! let x: &[u8] = &[10, 42, 0, 7, 91];
//! ```
//!
//! `&[u8]` underneath is really just a pair of a pointer to buffer and its length
//! (a fat pointer)
//!
//! ```text
//! [ ptr: *const u8 | len: usize = 5 ]
//!    |                |
//!    |                v       
//!    |    | <-       [u8]       -> |
//!    |    +----+----+----+----+----+
//!    +--->| 10 | 42 | 00 | 07 | 91 |
//!         +----+----+----+----+----+
//! ```
//!
//! Thats because size of `[u8]` can be known only at runtime.
//!
//! Rust also allows to define a structure that has exactly one dynamically-sized
//! type at the end of it.
//!
//! ```text
//! struct LoafT<u8, 2> {
//!     loaf: [u8; 2],
//!     rest: [u8],
//! }
//! ```
//!
//! ```compile_fail
//! let x: &[u8] = &[10, 42, 0, 7, 91];
//! let loaf: &LoafN<u8, 2> = abracadabra!(slice);
//! ```
//!
//! In this case the `len` also contains the length of `[u8]`
//!
//! ```text
//! [ ptr: *const ?? | len: usize = 3 ]
//!    |                     |
//!    |                     v       
//!    |    | [u8; 2] | <-  [u8]  -> |
//!    |    +----+----+----+----+----+
//!    +--->| 10 | 42 | 00 | 07 | 91 |
//!         +----+----+----+----+----+
//! ```
//!
//! `ptr` doesn't have here a clear type, because `*const LoafN<u8, 2>` is
//! itself a fat pointer (because of the `[u8]` field).
//!
//! ## The Hack
//! Rust does have a way to fiddle with fat pointer internals, but it
//! requires untagged unions, which are only avaliable on nightly.\
//! The hack here to create an `*mut [T]` as it was `*mut Loaf<T>` and then cast it
//!
//! See [Loaf::from_slice] source code for more details
//!
//! ## Safety
//! As long as two arrays could be interpreted as one bigger and vice versa,
//! everything should be alright

#[cfg(feature = "alloc")]
#[doc(hidden)]
pub(crate) extern crate alloc;

mod loaf;
pub use crate::loaf::*;

#[cfg(feature = "nightly")]
mod loaf_nightly;
#[cfg(feature = "nightly")]
pub use crate::loaf_nightly::*;

#[cfg(feature = "alloc")]
pub mod loaf_vec;
