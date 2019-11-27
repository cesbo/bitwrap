pub use bitwrap_derive::*;


pub trait BitWrap {
    fn pack(&self, dst: &mut Vec<u8>) -> usize;
    fn unpack(&mut self, src: &[u8]) -> usize;
}
