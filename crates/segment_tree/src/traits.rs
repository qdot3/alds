/// Defines a set of elements which forms a monoid
pub trait Monoid {
    const IS_COMMUTATIVE: bool;

    /// Returns the identity element.
    fn identity() -> Self;

    /// Performs the associative binary operation on the monoid.
    fn binary_operation(&self, rhs: &Self) -> Self;
}

pub trait MonoidAct {
    type Arg: Monoid;

    /// If acts are commutative, then it should be set `true`. Otherwise `false`.
    const IS_COMMUTATIVE: bool;

    fn identity() -> Self;
    fn composite(&self, rhs: &Self) -> Self;
    fn apply(&self, arg: &Self::Arg) -> Self::Arg;
}
