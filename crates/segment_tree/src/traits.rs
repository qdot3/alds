pub trait Monoid {
    /// Returns the identity element of the monoid.
    fn identity() -> Self;
    /// Performs the associative binary operation on the monoid.
    fn binary_operation(&self, rhs: &Self) -> Self;
}

/// Map on monoid.
pub trait Map<Arg: Monoid>: Clone {
    /// Returns the identity map.
    fn identity() -> Self;
    /// Applies map on the monoid.
    fn apply(&self, x: &Arg, size: usize) -> Arg;
    fn compose(&self, rhs: &Self) -> Self;
}
