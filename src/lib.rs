//! # bitwrap
//!
//! [![docs](https://docs.rs/bitwrap/badge.svg)](https://docs.rs/bitwrap)
//!
//! ## Intro
//!
//! bitwrap is a derive macro and interface to declare a struct data member
//! with explicit size, in bits.


#![no_std]


pub use bitwrap_derive::*;


pub trait BitWrap {
    /// Build byte array
    fn pack(&self, dst: &mut [u8]) -> usize;

    /// Extract object field values from byte array
    fn unpack(&mut self, src: &[u8]) -> usize;
}
