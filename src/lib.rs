//! # bitwrap
//!
//! [![docs](https://docs.rs/bitwrap/badge.svg)](https://docs.rs/bitwrap)
//!
//! ## Intro
//!
//! bitwrap is a derive macro and interface to declare a struct data member
//! with explicit size, in bits.

#![cfg_attr(not(feature = "std"), no_std)]


use {
    core::{
        fmt,
        convert::Infallible,
    },
};


pub use {
    bitwrap_derive::BitWrap,
};


#[derive(Debug, PartialEq)]
pub struct BitWrapError;


impl fmt::Display for BitWrapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "index out of bounds")
    }
}


#[cfg(feature = "std")]
impl std::error::Error for BitWrapError {}


impl From<Infallible> for BitWrapError {
    fn from(x: Infallible) -> BitWrapError {
        match x {}
    }
}


pub trait BitWrapExt {
    /// Build byte array
    fn pack(&self, dst: &mut [u8]) -> Result<usize, BitWrapError>;

    /// Extract object field values from byte array
    fn unpack(&mut self, src: &[u8]) -> Result<usize, BitWrapError>;
}


#[cfg(feature = "std")]
impl BitWrapExt for Vec<u8> {
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
impl<T: BitWrapExt + Default> BitWrapExt for Vec<T> {
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
