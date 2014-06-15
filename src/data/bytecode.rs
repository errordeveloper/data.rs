/// A single bytecode traditionally represented as hexadecimal. The specification has a number of
/// default bytecodes that are defined.
pub struct Bytecode(int);

pub static FOO: Bytecode = Bytecode(5);
