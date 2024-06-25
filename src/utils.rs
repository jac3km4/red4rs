#[macro_export]
macro_rules! wrap {
    ($newty:ty,$binding:ty) => {
        impl AsRef<$newty> for *const $binding {
            fn as_ref(&self) -> &$newty {
                unsafe { &*self.cast::<$newty>() }
            }
        }

        #[allow(clippy::not_unsafe_ptr_arg_deref)]
        impl From<*const $binding> for $newty {
            fn from(value: *const $binding) -> Self {
                Self(unsafe { &*value }.clone())
            }
        }
    };
}
