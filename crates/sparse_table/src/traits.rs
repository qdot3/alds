pub trait Semigroup {
    fn binary_operation(&self, rhs: &Self) -> Self;
}

pub trait Idempotent {}
