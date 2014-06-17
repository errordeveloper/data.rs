use std::num::Bitwise;

use encode;
use bytecode;

/// An encodable type is one that takes a Rust type and
/// produces the correct bytecode for it.
pub trait Encodable {
    fn write(&self) -> Vec<u8>;
}

impl Encodable for int {

    fn write(&self) -> Vec<u8> {
        let mut packets: Vec<u8> = Vec::new();

        match self.leading_zeros() {
            1..14 => {
                packets.push(bytecode::INT as u8);
                packets.push_all_move(encode::int64(*self as i64));
            },
            _ => {}
        }

        packets
    }

}

