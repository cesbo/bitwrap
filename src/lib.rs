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
    fn pack<R: AsMut<[u8]>>(&self, dst: &mut R) -> usize;

    /// Extract object field values from byte array
    fn unpack<R: AsRef<[u8]>>(&mut self, src: R) -> usize;
}


// #[cfg(feature="std")]
// impl BitWrap for std::net::Ipv4Addr {
//     #[inline]
//     fn pack(&self, dst: &mut Vec<u8>) -> usize {
//         dst.extend_from_slice(&self.octets());
//         4
//     }

//     #[inline]
//     fn unpack<R: AsRef<[u8]>>(&mut self, src: R) -> usize {
//         let src = src.as_ref();
//         if src.len() >= 4 {
//             *self = std::net::Ipv4Addr::from(unsafe { *(src.as_ptr() as *const [u8; 4]) });
//             4
//         } else {
//             src.len()
//         }
//     }
// }


// #[cfg(feature="std")]
// impl BitWrap for std::net::Ipv6Addr {
//     #[inline]
//     fn pack(&self, dst: &mut Vec<u8>) -> usize {
//         dst.extend_from_slice(&self.octets());
//         16
//     }

//     #[inline]
//     fn unpack<R: AsRef<[u8]>>(&mut self, src: R) -> usize {
//         let src = src.as_ref();
//         if src.len() >= 16 {
//             *self = std::net::Ipv6Addr::from(unsafe { *(src.as_ptr() as *const [u8; 16]) });
//             16
//         } else {
//             src.len()
//         }
//     }
// }
