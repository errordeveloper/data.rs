use std::str::SendStr;

pub type DataResult<T> = Result<T, DataError>;

pub struct DataError {
    kind: DataErrorKind,
    message: SendStr
}

pub enum DataErrorKind {
    Default
}
