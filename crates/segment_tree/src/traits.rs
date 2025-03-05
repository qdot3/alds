/// Defines a set of elements which forms a monoid
pub trait Monoid {
    /// Returns the identity element.
    fn identity() -> Self;

    /// Performs the associative binary operation on the monoid.
    fn binary_operation(&self, rhs: &Self) -> Self;
}

/// Defines a set of maps which forms a monoid
pub trait MapMonoid<Arg>: Clone {
    /// Returns the identity map.
    fn identity() -> Self;

    /// Composites two maps.
    fn composite(&self, rhs: &Self) -> Self;

    /// Applies the map to the element.
    fn apply(&self, arg: &Arg) -> Arg;
}