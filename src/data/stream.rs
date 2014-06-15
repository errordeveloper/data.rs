use result::DataResult;

/// Each writer and reader have their own streams. A stream can either be ended or
/// not. If a stream is not ended, then it's expected that more data will
/// be flowing through it.
///
/// Only chunked encoding will be using a non-ended stream.
pub struct Stream {
    open: bool,
    data: Vec<u8>
}

impl Stream {

    pub fn new(open: bool) -> Stream {
        Stream {
            open: open,
            data: Vec::new()
        }
    }

    pub fn with_capacity(open: bool, n: uint) -> Stream {
        Stream {
            open: open,
            data: Vec::with_capacity(n)
        }
    }

    // Push a new byte onto the stream. If the stream is closed
    pub fn push(byte: u8) -> DataResult<()> {
        Ok(())
    }

}
