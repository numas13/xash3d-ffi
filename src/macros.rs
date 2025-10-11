macro_rules! const_assert {
    ($expr:expr $(,)?) => {
        const _: () = {
            const ASSERT: [(); 1] = [()];
            const COND: usize = 1 - (($expr as bool) as usize);
            ASSERT[COND]
        };
    };
}

macro_rules! const_assert_eq {
    ($lhs:expr, $rhs:expr $(,)?) => {
        const_assert!($lhs == $rhs);
    };
}

macro_rules! const_assert_size {
    ($ty:ty, $size:expr) => {
        const_assert_size!($ty, $size, $size);
    };
    ($ty:ty, $size32:expr, $size64:expr) => {
        const_assert_eq!(core::mem::size_of::<$ty>(), {
            if cfg!(target_pointer_width = "32") {
                $size32
            } else {
                $size64
            }
        });
    };
}

macro_rules! impl_copy_clone {
    ($( $ty:ty ),* $(,)?) => {
        $(
            impl Copy for $ty {}

            impl Clone for $ty {
                fn clone(&self) -> Self {
                    *self
                }
            }
        )*
    };
}
