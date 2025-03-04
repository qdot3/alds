pub trait Monoid {
    fn identity() -> Self;
    fn binary_operation(&self, rhs: &Self) -> Self;
}
