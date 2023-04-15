use core::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use core::num::Wrapping;
use core::convert::From;
use core::cmp::PartialEq;
use core::fmt;


/// An arbitrary sized unsigned integer.
/// The first integer holds the 
/// 
#[derive(Copy, Clone, PartialEq)]
pub struct Uint<const BITS: u8, T>(Wrapping<T>);


impl<const BITS: u8, T: fmt::Debug> fmt::Debug for Uint<BITS, T> { 
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "u{}({:?})", BITS, self.0)
    }
}

impl<const BITS: u8, T: fmt::Display> fmt::Display for Uint<BITS, T> { 
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "u{}({})", BITS, self.0)
    }
}

macro_rules! impl_bit_uint {
    ($t:ty) => {
        impl<const BITS: u8> Uint<BITS, $t> {
            const MASK: Wrapping<$t> = Wrapping(((1 << BITS) - 1) as $t);

            #[inline]
            pub fn new(value: $t) -> Self {
                Self(Wrapping(value) & Self::MASK)
            }
        }

        impl<const BITS: u8> Add for Uint<BITS, $t> {
            type Output = Self;

            #[inline]
            fn add(self, rhs: Self) -> Self::Output {
                Self::from(self.0 + rhs.0)
            }
        }

        impl<const BITS: u8> AddAssign for Uint<BITS, $t> {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                self.0 = (self.0 + rhs.0) & Self::MASK;
            }
        }

        impl<const BITS: u8> Sub for Uint<BITS, $t> {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: Self) -> Self::Output {
                Self::from(self.0 - rhs.0)
            }
        }

        impl<const BITS: u8> SubAssign for Uint<BITS, $t> {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                self.0 = (self.0 - rhs.0) & Self::MASK;
            }
        }

        impl<const BITS: u8> Mul for Uint<BITS, $t> {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: Self) -> Self::Output {
                Self::from(self.0 * rhs.0)
            }
        }

        impl<const BITS: u8> MulAssign for Uint<BITS, $t> {
            #[inline]
            fn mul_assign(&mut self, rhs: Self) {
                self.0 = (self.0 * rhs.0) & Self::MASK;
            }
        }

        impl<const BITS: u8> Div for Uint<BITS, $t> {
            type Output = Self;
            
            #[inline]
            fn div(self, rhs: Self) -> Self::Output {
                assert_ne!(rhs.0, Wrapping(0));
                Self::from(self.0 / rhs.0)
            }
        }

        impl<const BITS: u8> DivAssign for Uint<BITS, $t> {
            #[inline]
            fn div_assign(&mut self, rhs: Self) {
                assert_ne!(rhs.0.0, 0);
                self.0 /= rhs.0;
            }
        }

        impl<const BITS: u8> From<Wrapping<$t>> for Uint<BITS, $t> {
            #[inline]
            fn from(value: Wrapping<$t>) -> Self {
                Self::new(value.0)
            }
        }

        impl<const BITS: u8> PartialEq<$t> for Uint<BITS, $t> {
            #[inline]
            fn eq(&self, other: &$t) -> bool {
                self.0.0 == *other
            }
        }

        impl<const BITS: u8> PartialEq<Uint<BITS, $t>> for $t {
            #[inline]
            fn eq(&self, other: &Uint<BITS, $t>) -> bool {
                *self == other.0.0
            }
        }
    };
}

impl_bit_uint!(u8);
impl_bit_uint!(u16);
impl_bit_uint!(u32);
impl_bit_uint!(u64);
impl_bit_uint!(u128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut a = Uint::<5, u8>::new(23);
        let b = Uint::<5, u8>::new(27);
        println!("{}", a + b);
        assert_eq!(28u8, a - b);
        a -= b;
        println!("{}", a);
    }
}
