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


#[derive(Debug, PartialEq)]
pub struct BitWrapError;


impl fmt::Display for BitWrapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "index out of bounds")
    }
}


#[cfg(feature = "std")]
impl std::error::Error for BitWrapError {}


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
            dst[.. 4].clone_from_slice(&self.octets());
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
            dst[.. 16].clone_from_slice(&self.octets());
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


#[cfg(feature = "std")]
impl BitWrap for Vec<u8> {
    #[inline]
    fn pack(&self, dst: &mut [u8]) -> Result<usize, BitWrapError> {
        let len = self.len();
        if dst.len() >= len {
            dst[.. len].clone_from_slice(self.as_slice());
            Ok(len)
        } else {
            Err(BitWrapError)
        }
    }

    #[inline]
    fn unpack(&mut self, src: &[u8]) -> Result<usize, BitWrapError> {
        self.extend_from_slice(src);
        Ok(src.len())
    }
}


#[cfg(feature = "std")]
impl<T: BitWrap + Default> BitWrap for Vec<T> {
    #[inline]
    fn pack(&self, dst: &mut [u8]) -> Result<usize, BitWrapError> {
        let mut skip = 0;
        for item in self {
            skip += item.pack(&mut dst[skip ..])?;
        }
        Ok(skip)
    }

    #[inline]
    fn unpack(&mut self, src: &[u8]) -> Result<usize, BitWrapError> {
        let mut skip = 0;
        while skip < src.len() {
            let mut item = T::default();
            skip += item.unpack(&src[skip ..])?;
            self.push(item);
        }
        Ok(skip)
    }
}


impl<const N: usize> BitWrap for [u8; N] {
    #[inline]
    fn pack(&self, dst: &mut [u8]) -> Result<usize, BitWrapError> {
        let len = self.len();
        if dst.len() >= len {
            dst[.. len].clone_from_slice(self);
            Ok(len)
        } else {
            Err(BitWrapError)
        }
    }

    #[inline]
    fn unpack(&mut self, src: &[u8]) -> Result<usize, BitWrapError> {
        let len = self.len();
        if src.len() >= len {
            self.clone_from_slice(&src[.. len]);
            Ok(len)
        } else {
            Err(BitWrapError)
        }
    }
}
