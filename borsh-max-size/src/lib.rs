pub use borsh_max_size_derive::MaxSize;

pub trait MaxSize {
    fn max_size() -> usize;
}

impl<T> MaxSize for Option<T>
where
    T: MaxSize,
{
    fn max_size() -> usize {
        1 + T::max_size()
    }
}

impl MaxSize for bool {
    fn max_size() -> usize {
        1
    }
}

impl MaxSize for u8 {
    fn max_size() -> usize {
        1
    }
}

impl MaxSize for u16 {
    fn max_size() -> usize {
        2
    }
}

impl MaxSize for u32 {
    fn max_size() -> usize {
        4
    }
}

impl MaxSize for u64 {
    fn max_size() -> usize {
        8
    }
}

impl MaxSize for u128 {
    fn max_size() -> usize {
        16
    }
}

impl MaxSize for i8 {
    fn max_size() -> usize {
        1
    }
}

impl MaxSize for i16 {
    fn max_size() -> usize {
        2
    }
}

impl MaxSize for i32 {
    fn max_size() -> usize {
        4
    }
}

impl MaxSize for i64 {
    fn max_size() -> usize {
        8
    }
}

impl MaxSize for i128 {
    fn max_size() -> usize {
        16
    }
}

impl MaxSize for f32 {
    fn max_size() -> usize {
        4
    }
}

impl MaxSize for f64 {
    fn max_size() -> usize {
        8
    }
}

impl MaxSize for () {
    fn max_size() -> usize {
        0
    }
}

impl MaxSize for char {
    fn max_size() -> usize {
        4
    }
}

impl<T, const N: usize> MaxSize for [T; N]
where
    T: MaxSize,
{
    fn max_size() -> usize {
        N * T::max_size()
    }
}

#[cfg(feature = "arrayvec")]
impl<T, const N: usize> MaxSize for ArrayVec<T, N>
where
    T: MaxSize,
{
    fn max_size() -> usize {
        4 + N * T::max_size()
    }
}
