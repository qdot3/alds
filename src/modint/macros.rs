macro_rules! forward_ref_dyn_mint_binop {
    ( impl<$lt:lifetime> $trait:ident, $method:ident for $t:ty ) => {
        impl<$lt> $trait<&$t> for $t {
            type Output = $t;

            fn $method(self, rhs: &$t) -> Self::Output {
                self.$method(*rhs)
            }
        }

        impl<$lt> $trait<$t> for &$t {
            type Output = $t;

            fn $method(self, rhs: $t) -> Self::Output {
                (*self).$method(rhs)
            }
        }

        impl<$lt> $trait<&$t> for &$t {
            type Output = $t;

            fn $method(self, rhs: &$t) -> Self::Output {
                (*self).$method(rhs)
            }
        }
    };
}

pub(super) use forward_ref_dyn_mint_binop;

macro_rules! forward_ref_dyn_mint_op_assign {
    ( impl<$lt:lifetime> $trait:ident, $method:ident for $t:ty ) => {
        impl<$lt> $trait<&$t> for $t {
            fn $method(&mut self, rhs: &$t) {
                self.$method(*rhs)
            }
        }
    };
}

pub(super) use forward_ref_dyn_mint_op_assign;

macro_rules! forward_ref_dyn_mint_unop {
    ( impl<$lt:lifetime> $trait:ident, $method:ident for $t:ty ) => {
        impl<$lt> $trait for &$t {
            type Output = $t;

            fn $method(self) -> Self::Output {
                (*self).$method()
            }
        }
    };
}

pub(super) use forward_ref_dyn_mint_unop;
