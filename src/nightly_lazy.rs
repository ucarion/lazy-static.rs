extern crate std;

use self::std::prelude::v1::*;
use self::std::sync::{Once, ONCE_INIT};

pub struct Lazy<T: Sync>(Option<T>, Once);

impl<T: Sync> Lazy<T> {
    #[inline(always)]
    pub const fn new() -> Self {
        Lazy(None, ONCE_INIT)
    }

    #[inline(always)]
    pub fn get<F>(&'static mut self, f: F) -> &T
        where F: FnOnce() -> T
    {
        {
            let r = &mut self.0;
            self.1.call_once(|| {
                *r = Some(f());
            });
        }
        unsafe {
            match self.0 {
                Some(ref x) => x,
                None => std::intrinsics::unreachable(),
            }
        }
    }
}

unsafe impl<T: Sync> Sync for Lazy<T> {}

#[macro_export]
#[allow_internal_unstable]
macro_rules! __lazy_static_create {
    ($NAME:ident, $T:ty) => {
        static mut $NAME: $crate::lazy::Lazy<$T> = $crate::lazy::Lazy::new();
    }
}
