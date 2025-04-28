macro_rules! impl_memset {
    ($LEN:expr) => {
        #[inline]
        pub const fn memset(val: T) -> Self {
            Self([val; $LEN])
        }
    };
}

pub(crate) use impl_memset;
