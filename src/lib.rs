pub use bitwrap_derive::*;


pub trait BitWrap {
    fn unpack(&mut self, src: &[u8]) -> usize;
}
