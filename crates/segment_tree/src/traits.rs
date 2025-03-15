/// Defines a set of elements which forms a monoid
pub trait Monoid {
    const IS_COMMUTATIVE: bool;

    /// Returns the identity element.
    fn identity() -> Self;

    /// Performs the associative binary operation on the monoid.
    fn binary_operation(&self, rhs: &Self) -> Self;
}

/// Defines a set of maps which forms a monoid
pub trait MonoidAction<Arg> {
    /// If maps are commutative, then it should be set `true`. Otherwise `false`.
    const IS_COMMUTATIVE: bool;

    /// Returns the identity map.
    fn identity() -> Self;

    /// Composites two maps.
    fn composite(&self, rhs: &Self) -> Self;

    /// Applies the map to the element.
    fn apply(&self, arg: &Arg) -> Arg;
}

pub trait MonoidAct {
    type Arg;

    /// If acts are commutative, then it should be set `true`. Otherwise `false`.
    const IS_COMMUTATIVE: bool;

    fn identity() -> Self;
    fn composite(&self, rhs: &Self) -> Self;
    fn apply(&self, arg: &Self::Arg) -> Self::Arg;
}
