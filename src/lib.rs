//! # bitwrap
//!
//! [![docs](https://docs.rs/bitwrap/badge.svg)](https://docs.rs/bitwrap)
//!
//! ## Intro
//!
//! bitwrap is a derive macro and interface to declare a struct data member
//! with explicit size, in bits.

#![cfg_attr(not(feature = "std"), no_std)]

use core::fmt;

pub use bitwrap_derive::*;


pub struct BitWrapError;


impl fmt::Debug for BitWrapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BitWrapError").finish()
    }
}


impl fmt::Display for BitWrapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "index out of bounds")
    }
}


pub trait BitWrap {
    /// Build byte array
    fn pack(&self, dst: &mut [u8]) -> Result<usize, BitWrapError>;

    /// Extract object field values from byte array
    fn unpack(&mut self, src: &[u8]) -> Result<usize, BitWrapError>;
}


#[cfg(feature = "std")]
impl BitWrap for std::net::Ipv4Addr {
    #[inline]
    fn pack(&self, dst: &mut [u8]) -> Result<usize, BitWrapError> {
        if dst.len() >= 4 {
            (&mut dst[.. 4]).clone_from_slice(&self.octets());
            Ok(4)
        } else {
            Err(BitWrapError)
        }
    }

    #[inline]
    fn unpack(&mut self, src: &[u8]) -> Result<usize, BitWrapError> {
        if src.len() >= 4 {
            *self = std::net::Ipv4Addr::from(unsafe { *(src.as_ptr() as *const [u8; 4]) });
            Ok(4)
        } else {
            Err(BitWrapError)
        }
    }
}


#[cfg(feature = "std")]
impl BitWrap for std::net::Ipv6Addr {
    #[inline]
    fn pack(&self, dst: &mut [u8]) -> Result<usize, BitWrapError> {
        if dst.len() >= 16 {
            (&mut dst[.. 16]).clone_from_slice(&self.octets());
            Ok(16)
        } else {
            Err(BitWrapError)
        }
    }

    #[inline]
    fn unpack(&mut self, src: &[u8]) -> Result<usize, BitWrapError> {
        if src.len() >= 16 {
            *self = std::net::Ipv6Addr::from(unsafe { *(src.as_ptr() as *const [u8; 16]) });
            Ok(16)
        } else {
            Err(BitWrapError)
        }
    }
}
