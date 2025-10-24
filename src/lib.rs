//! Const array operations: map, zip, unzip, scan, fold.
//!
//! This crate provides macros for performing various operations on arrays in const contexts.
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
