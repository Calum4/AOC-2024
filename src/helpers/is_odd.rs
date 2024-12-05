pub trait IsOdd {
    fn is_odd(&self) -> bool;
}

macro_rules! is_odd {
    ($($t:ident),*) => {$(
        impl IsOdd for $t {
            #[inline]
            fn is_odd(&self) -> bool {
                (self & 1) != 0
            }
        }
    )*}
}



is_odd!(u8, u16, u32, u64, u128, usize);
is_odd!(i8, i16, i32, i64, i128, isize);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unsigned_odd() {
        assert!(1u8.is_odd());
        assert!(1u16.is_odd());
        assert!(1u32.is_odd());
        assert!(1u64.is_odd());
        assert!(1u128.is_odd());
        assert!(1usize.is_odd());
    }

    #[test]
    fn test_unsigned_even() {
        assert!(!2u8.is_odd());
        assert!(!2u16.is_odd());
        assert!(!2u32.is_odd());
        assert!(!2u64.is_odd());
        assert!(!2u128.is_odd());
        assert!(!2usize.is_odd());
    }

    #[test]
    fn test_signed_odd() {
        assert!(1i8.is_odd());
        assert!((-1i8).is_odd());
        assert!(1i16.is_odd());
        assert!((-1i16).is_odd());
        assert!(1i32.is_odd());
        assert!((-1i32).is_odd());
        assert!(1i64.is_odd());
        assert!((-1i32).is_odd());
        assert!(1i128.is_odd());
        assert!((-1i128).is_odd());
        assert!(1isize.is_odd());
        assert!((-1isize).is_odd());
    }

    #[test]
    fn test_signed_even() {
        assert!(!2i8.is_odd());
        assert!(!(-2i8).is_odd());
        assert!(!2i16.is_odd());
        assert!(!(-2i16).is_odd());
        assert!(!2i32.is_odd());
        assert!(!(-2i32).is_odd());
        assert!(!2i64.is_odd());
        assert!(!(-2i32).is_odd());
        assert!(!2i128.is_odd());
        assert!(!(-2i128).is_odd());
        assert!(!2isize.is_odd());
        assert!(!(-2isize).is_odd());
    }
}
