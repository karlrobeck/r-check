pub trait Arbitrary {
    type Output;
    fn generate(&self) -> Self::Output;
}
