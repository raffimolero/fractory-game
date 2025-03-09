use crate::prelude::*;

pub mod prelude {
    pub use super::OptionExt;
}

pub trait OptionExt {
    type Inner;
    fn is_none_or(self, f: impl FnOnce(Self::Inner) -> bool) -> bool;
}
impl<T> OptionExt for Option<T> {
    type Inner = T;

    #[inline]
    fn is_none_or(self, f: impl FnOnce(Self::Inner) -> bool) -> bool {
        match self {
            None => true,
            Some(x) => f(x),
        }
    }
}
