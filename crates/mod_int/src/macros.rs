macro_rules! forward_ref_mint_binop {
    // dynamic mint
    ( impl<$lt:lifetime> $trait:ident, $method:ident for $t:ty ) => {
        impl<$lt> $trait<&$t> for $t {
            type Output = $t;

            #[inline]
            fn $method(self, rhs: &$t) -> Self::Output {
                self.$method(*rhs)
            }
        }

        impl<$lt> $trait<$t> for &$t {
            type Output = $t;

            #[inline]
            fn $method(self, rhs: $t) -> Self::Output {
                (*self).$method(rhs)
            }
        }

        impl<$lt> $trait<&$t> for &$t {
            type Output = $t;

            #[inline]
            fn $method(self, rhs: &$t) -> Self::Output {
                (*self).$method(rhs)
            }
        }
    };
    // static mint
    ( impl<const $const_generics:ident : $const_ty:ty> $trait:ident, $method:ident for $t:ty ) => {
        impl<const $const_generics: $const_ty> $trait<&$t> for $t {
            type Output = $t;

            #[inline]
            fn $method(self, rhs: &$t) -> Self::Output {
                self.$method(*rhs)
            }
        }

        impl<const $const_generics: $const_ty> $trait<$t> for &$t {
            type Output = $t;

            #[inline]
            fn $method(self, rhs: $t) -> Self::Output {
                (*self).$method(rhs)
            }
        }

        impl<const $const_generics: $const_ty> $trait<&$t> for &$t {
            type Output = $t;

            #[inline]
            fn $method(self, rhs: &$t) -> Self::Output {
                self.$method(*rhs)
            }
        }
    };
}

pub(crate) use forward_ref_mint_binop;

macro_rules! forward_ref_mint_op_assign {
    // dynamic mint
    ( impl<$lt:lifetime> $trait:ident, $method:ident for $t:ty ) => {
        impl<$lt> $trait<&$t> for $t {
            #[inline]
            fn $method(&mut self, rhs: &$t) {
                self.$method(*rhs)
            }
        }
    };
    // static mint
    ( impl<const $const_generics:ident : $const_ty:ty> $trait:ident, $method:ident for $t:ty ) => {
        impl<const $const_generics: $const_ty> $trait<&$t> for $t {
            #[inline]
            fn $method(&mut self, rhs: &$t) {
                self.$method(*rhs)
            }
        }
    };
}

pub(crate) use forward_ref_mint_op_assign;

macro_rules! forward_ref_mint_unop {
    // dynamic mint
    ( impl<$lt:lifetime> $trait:ident, $method:ident for $t:ty ) => {
        impl<$lt> $trait for &$t {
            type Output = $t;

            #[inline]
            fn $method(self) -> Self::Output {
                (*self).$method()
            }
        }
    };
    // static mint
    ( impl<const $const_generics:ident: $const_ty:ty> $trait:ident, $method:ident for $t:ty ) => {
        impl<const $const_generics: $const_ty> $trait for &$t {
            type Output = $t;

            #[inline]
            fn $method(self) -> Self::Output {
                (*self).$method()
            }
        }
    };
}

pub(crate) use forward_ref_mint_unop;
