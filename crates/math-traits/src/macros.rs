macro_rules! forward_ref_binop {
    ( $( impl $trait:ident, $method:ident for $t:ty )* ) => {$(
        impl $trait<&$t> for $t {
            type Output = <$t as $trait>::Output;

            fn $method(self, other: &$t) -> Self::Output {
                self.$method(*other)
            }
        }

        impl $trait<$t> for &$t {
            type Output = <$t as $trait>::Output;

            fn $method(self, other: $t) -> Self::Output {
                (*self).$method(other)
            }
        }

        impl $trait<&$t> for &$t {
            type Output = <$t as $trait>::Output;

            fn $method(self, other: &$t) -> Self::Output {
                (*self).$method(*other)
            }
        }
    )*};
}

pub(crate) use forward_ref_binop;
