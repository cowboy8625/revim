pub trait Unsigned {}

impl Unsigned for u8 {}
impl Unsigned for u16 {}
impl Unsigned for u32 {}
impl Unsigned for u64 {}
impl Unsigned for u128 {}
impl Unsigned for usize {}

pub fn usubtraction<T>(x: T, y: T) -> T
where
    T: std::ops::Sub<Output = T> + std::cmp::PartialOrd + From<u8> + Unsigned,
{
    if y > x {
        T::from(0)
    } else {
        x - y
    }
}
