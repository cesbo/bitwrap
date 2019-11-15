#[macro_use]
extern crate bitwrap_derive;
pub use bitwrap_derive::*;


pub trait BitWrap {
    fn unpack(&mut self, src: &[u8]);
}
