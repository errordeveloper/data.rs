use result::DataResult;
use std::io::MemWriter;
use serialize;
use serialize::Encodable;
use std::io::{IoError, IoResult};

use bytecode;


/// A writer is responsible for encoding the data. The output is simply a stream of
/// u8s (a stream of bytes).
///
/// If a chunked encoding is used, the stream is not closed immediately. More items can be pushed until
/// the writer is finished.
pub trait Writer {
    fn write<T: Encodable<Encoder, IoError>>(&mut self, value: T) -> DataResult<()>;
}

/// DataWriter is the default implementation for the writer.
pub struct Encoder {
    stream: MemWriter
}

impl serialize::Encoder<IoError> for Encoder {
    fn emit_nil(&mut self) -> IoResult<()> { Ok(()) }

    fn emit_uint(&mut self, v: uint) -> IoResult<()> { Ok(()) }
    fn emit_u64(&mut self, v: u64) -> IoResult<()> { Ok(()) }
    fn emit_u32(&mut self, v: u32) -> IoResult<()> { Ok(()) }
    fn emit_u16(&mut self, v: u16) -> IoResult<()> { Ok(()) }
    fn emit_u8(&mut self, v: u8) -> IoResult<()> { Ok(()) }

    fn emit_int(&mut self, v: int) -> IoResult<()> { Ok(()) }
    fn emit_i64(&mut self, v: i64) -> IoResult<()> { Ok(()) }
    fn emit_i32(&mut self, v: i32) -> IoResult<()> { Ok(()) }
    fn emit_i16(&mut self, v: i16) -> IoResult<()> { Ok(()) }
    fn emit_i8(&mut self, v: i8) -> IoResult<()> { Ok(()) }

    fn emit_bool(&mut self, v: bool) -> IoResult<()> { Ok(()) }

    fn emit_f64(&mut self, v: f64) -> IoResult<()> { Ok(()) }
    fn emit_f32(&mut self, v: f32) -> IoResult<()> { Ok(()) }

    fn emit_char(&mut self, v: char) -> IoResult<()> { Ok(()) }
    fn emit_str(&mut self, v: &str) -> IoResult<()> { Ok(()) }

    fn emit_enum(&mut self,
                 name: &str,
                 f: |&mut Encoder| -> IoResult<()>) -> IoResult<()> {
        Ok(())
    }
    fn emit_enum_variant(&mut self,
                        _: &str,
                        _: uint,
                        len: uint,
                        f: |&mut Encoder| -> IoResult<()>) -> IoResult<()> {
        Ok(())
    }

    fn emit_enum_variant_arg(&mut self,
                             id: uint,
                             f: |&mut Encoder| -> IoResult<()>) -> IoResult<()> {
        Ok(())
    }

    fn emit_enum_struct_variant(&mut self,
                                _: &str,
                                _: uint,
                                len: uint,
                                f: |&mut Encoder| -> IoResult<()>) -> IoResult<()> {
        Ok(())
    }

    fn emit_enum_struct_variant_field(&mut self,
                                      _: &str,
                                      _: uint,
                                      f: |&mut Encoder| -> IoResult<()>) -> IoResult<()> {
        Ok(())
    }

    fn emit_struct(&mut self,
                   name: &str,
                   len: uint,
                   f: |&mut Encoder| -> IoResult<()>) -> IoResult<()> {
        Ok(())
    }

    fn emit_struct_field(&mut self,
                         name: &str,
                         id: uint,
                         f: |&mut Encoder| -> IoResult<()>) -> IoResult<()> {
        Ok(())
    }

    fn emit_tuple(&mut self,
                  id: uint,
                  f: |&mut Encoder| -> IoResult<()>) -> IoResult<()> {
        Ok(())
    }

    fn emit_tuple_arg(&mut self,
                      len: uint,
                      f: |&mut Encoder| -> IoResult<()>) -> IoResult<()> {
        Ok(())
    }

    fn emit_tuple_struct(&mut self,
                         name: &str,
                         len: uint,
                         f: |&mut Encoder| -> IoResult<()>) -> IoResult<()> {
        Ok(())
    }


    fn emit_tuple_struct_arg(&mut self,
                             len: uint,
                             f: |&mut Encoder| -> IoResult<()>) -> IoResult<()> {
        Ok(())
    }

    fn emit_option(&mut self, f: |&mut Encoder| -> IoResult<()>) -> IoResult<()> {
        Ok(())
    }

    fn emit_option_none(&mut self) -> IoResult<()> {
        Ok(())
    }

    fn emit_option_some(&mut self, f: |&mut Encoder| -> IoResult<()>) -> IoResult<()> {
        Ok(())
    }

    fn emit_seq(&mut self,
                len: uint,
                f: |&mut Encoder| -> IoResult<()>) -> IoResult<()> {
        Ok(())
    }

    fn emit_seq_elt(&mut self,
                    id: uint,
                    f: |&mut Encoder| -> IoResult<()>) -> IoResult<()> {
        Ok(())
    }

    fn emit_map(&mut self,
                len: uint,
                f: |&mut Encoder| -> IoResult<()>) -> IoResult<()> {
        Ok(())
    }

    fn emit_map_elt_key(&mut self,
                        id: uint,
                        f: |&mut Encoder| -> IoResult<()>) -> IoResult<()> {
        Ok(())
    }

    fn emit_map_elt_val(&mut self,
                        id: uint,
                        f: |&mut Encoder| -> IoResult<()>) -> IoResult<()> {
        Ok(())
    }
}

pub struct DataWriter {
    encoder: Encoder
}

impl Encoder {

    /// Create a new data writer with an open stream to begin with.
    pub fn new() -> Encoder {
        Encoder {
            stream: MemWriter::new()
        }
    }
}

impl Writer for DataWriter {
    fn write<T: Encodable<Encoder, IoError>>(&mut self, value: T) -> DataResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
