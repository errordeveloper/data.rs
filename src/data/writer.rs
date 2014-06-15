/// A writer is responsible for encoding the data. The output is simply a stream of
/// u8s (a stream of bytes).
///
/// If a chunked encoding is used, the stream is not closed immediately. More items can be pushed until
/// the writer is finished.
pub trait Writer {
    fn write<T>(value: T);
}
