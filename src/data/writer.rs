use stream::Stream;
use result::DataResult;
use std::num::Bitwise;
use bytecode;

/// A writer is responsible for encoding the data. The output is simply a stream of
/// u8s (a stream of bytes).
///
/// If a chunked encoding is used, the stream is not closed immediately. More items can be pushed until
/// the writer is finished.
pub trait Writer {
    fn write<T: Encodable>(&mut self, value: T) -> DataResult<()>;
}

pub fn write_raw_int64(i: i64) -> Vec<u8> {
    let u = i as u64;
    let mut buf: Vec<u8> = Vec::with_capacity(8);

    buf.push((u >> 56) as u8);
    buf.push((u >> 48) as u8);
    buf.push((u >> 40) as u8);
    buf.push((u >> 32) as u8);
    buf.push((u >> 24) as u8);
    buf.push((u >> 16) as u8);
    buf.push((u >> 8) as u8);
    buf.push((u >> 0) as u8);

    buf
}

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
                packets.push(bytecode::INT);
                packets.push_all_move(write_raw_int64(*self as i64));
            },
            _ => {}
        }
        Vec::new()
    }
}

/// DataWriter is the default implementation for the writer.
pub struct DataWriter {
    stream: Stream
}

impl DataWriter {
    pub fn new() -> DataWriter {
        DataWriter {
            stream: Stream::new(true)
        }
    }
}

impl Writer for DataWriter {
    fn write<T: Encodable>(&mut self, value: T) -> DataResult<()> {
        value.write();
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encode_int() {
        let mut writer = DataWriter::new();

        writer.write(5);
    }
}
