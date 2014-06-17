/// Given a normal signed 64-bit integer, produce a raw version of that
/// in an 8 byte vector of u8s. This is the lowest-level of identity for
/// the integer.
pub fn int64(i: i64) -> Vec<u8> {
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
