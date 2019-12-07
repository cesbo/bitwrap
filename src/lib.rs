//! # bitwrap
//!
//! [![docs](https://docs.rs/bitwrap/badge.svg)](https://docs.rs/bitwrap)
//!
//! ## Intro
//!
//! bitwrap is a derive macro and interface to declare a struct data member
//! with explicit size, in bits.


pub use bitwrap_derive::*;


pub trait BitWrap {
    /// Build byte array
    fn pack(&self, dst: &mut Vec<u8>) -> usize;

    /// Extract object field values from byte array
    fn unpack<R: AsRef<[u8]>>(&mut self, src: R) -> usize;
}
