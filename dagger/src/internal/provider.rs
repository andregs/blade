pub trait Provider<T> {
    fn get() -> T;
}
