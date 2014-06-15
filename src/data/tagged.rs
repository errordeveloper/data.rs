/// A tagged value.
pub trait Tagged {
    fn get_tag<T>();
    fn get_value<T>();
}
