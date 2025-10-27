//! This crate provides macros for destructuring ([`crate::destructure`]) and for performing various operations on arrays in const contexts ([`crate::map`], [`crate::unzip`], ...).
//!
//! ## Fused Operations
//!
//! Many operations are fused together in order to generate less code.
//! For example, [`map!`] can operate directly on [`zip!`]'ed arrays without requiring a separate import:
//!
//! ```
//! use const_tools::map;  // `zip!` is built into `map!`
//!
//! const fn combine<A, B, const N: usize>(a: [A; N], b: [B; N]) -> [(A, B); N] {
//!     map!(zip!(a, b), |(x, y)| (x, y))
//! }
//! ```
//!
//! Similarly, [`unzip!`] can operate directly on the output of [`map!`].
//! You typically only need to import the outermost operation you're performing.
#![no_std]

#[doc(hidden)]
pub mod __call;

#[doc(hidden)]
pub mod __manually_drop_inner_ref;
pub use __manually_drop_inner_ref::*;

#[doc(hidden)]
pub mod __maybe_uninit_array_assume_init;
pub use __maybe_uninit_array_assume_init::*;

#[doc(hidden)]
pub mod __maybe_uninit_array_uninit;
pub use __maybe_uninit_array_uninit::*;

#[doc(hidden)]
pub mod __same_len;
pub use __same_len::*;

#[doc(hidden)]
pub mod __zip_left;

#[doc(hidden)]
pub mod destructure;
pub use destructure::*;

#[doc(hidden)]
mod fold;

#[doc(hidden)]
mod map;

#[doc(hidden)]
mod scan;

#[doc(hidden)]
mod unzip;

#[doc(hidden)]
mod zip;

// The array-related macros implemented by this library use the following (meta)variable naming scheme:
//
// $(i|o)(i|a)(|e|p)
// - (i|o) -> input or output
// - (i|a) -> item or array
// - (|e|p) -> ident, expression, or pattern
//
// As an example, `$iae:expr` would be an expression for the input array, or multiple in case of a repetition.
