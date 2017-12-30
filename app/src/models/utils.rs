pub trait Deletable<T=Self> {
    fn delete(&self) -> Result<T, String>;
}
