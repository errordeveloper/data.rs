use result::DataResult;
use std::io::MemWriter;
use bytecode;
use encodable::Encodable;

/// A writer is responsible for encoding the data. The output is simply a stream of
/// u8s (a stream of bytes).
///
/// If a chunked encoding is used, the stream is not closed immediately. More items can be pushed until
/// the writer is finished.
pub trait Writer {
    fn write<T: Encodable>(&mut self, value: T) -> DataResult<()>;
}

/// DataWriter is the default implementation for the writer.
pub struct DataWriter {
    stream: MemWriter
}

impl DataWriter {

    /// Create a new data writer with an open stream to begin with.
    pub fn new() -> DataWriter {
        DataWriter {
            stream: MemWriter::new()
        }
    }
}

impl Writer for DataWriter {

    /// Encode a given value T that has to implement the local `Encodable` type
    /// which has all the logic for the actual encoding. This simply appends
    /// the raw representation to the current stream.
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
