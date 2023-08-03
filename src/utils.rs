use std::ops::Mul;

pub struct Percentage(u16);

macro_rules! mul_impl {
    // macth like arm for macro
    ($t:ty) => {
        // macro expand to this code
        // $a and $b will be templated using the value/variable provided to macro
        impl Mul<$t> for Percentage {
            type Output = $t;

            fn mul(self, rhs: $t) -> Self::Output {
                let perc = self.0 as $t;
                (rhs * perc + 50) / 100
            }
        }
    };
}

mul_impl!(u16);
mul_impl!(u32);
mul_impl!(u64);
mul_impl!(i16);
mul_impl!(i32);
mul_impl!(i64);
