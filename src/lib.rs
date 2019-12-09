//! # bitwrap
//!
//! [![docs](https://docs.rs/bitwrap/badge.svg)](https://docs.rs/bitwrap)
//!
//! ## Intro
//!
//! bitwrap is a derive macro and interface to declare a struct data member
//! with explicit size, in bits.


#![cfg_attr(not(feature = "std"), no_std)]


pub use bitwrap_derive::*;


pub trait BitWrap {
    /// Build byte array
    fn pack(&self, dst: &mut [u8]) -> usize;

    /// Extract object field values from byte array
    fn unpack(&mut self, src: &[u8]) -> usize;
}


#[cfg(feature = "std")]
impl BitWrap for std::net::Ipv4Addr {
    #[inline]
    fn pack(&self, dst: &mut [u8]) -> usize {
        assert!(dst.len() >= 4, "failed to BitWrap::pack() for std::net::Ipv4Addr");
        (&mut dst[.. 4]).clone_from_slice(&self.octets());
        4
    }

    #[inline]
    fn unpack(&mut self, src: &[u8]) -> usize {
        assert!(src.len() >= 4, "failed to BitWrap::unpack() for std::net::Ipv4Addr");
        *self = std::net::Ipv4Addr::from(unsafe { *(src.as_ptr() as *const [u8; 4]) });
        4
    }
}


#[cfg(feature = "std")]
impl BitWrap for std::net::Ipv6Addr {
    #[inline]
    fn pack(&self, dst: &mut [u8]) -> usize {
        assert!(dst.len() >= 16, "failed to BitWrap::pack() for std::net::Ipv4Addr");
        (&mut dst[.. 16]).clone_from_slice(&self.octets());
        16
    }

    #[inline]
    fn unpack(&mut self, src: &[u8]) -> usize {
        assert!(src.len() >= 16, "failed to BitWrap::unpack() for std::net::Ipv6Addr");
        *self = std::net::Ipv6Addr::from(unsafe { *(src.as_ptr() as *const [u8; 16]) });
        16
    }
}
