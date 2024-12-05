//! Implemented by types for which we can retrieve the number of bytes.
//!
//! See also: [`chars` on `str`](https://doc.rust-lang.org/std/primitive.str.html#method.chars).

use crate::error::Error;

pub fn apply<T: Bytes>(v: &T, (min, max): (usize, usize)) -> Result<(), Error> {
    v.validate_num_bytes(min, max)
}

pub trait Bytes {
    fn validate_num_bytes(&self, min: usize, max: usize) -> Result<(), Error>;
}

impl<T: HasBytes> Bytes for T {
    fn validate_num_bytes(&self, min: usize, max: usize) -> Result<(), Error> {
        super::check_len(self.num_bytes(), min, max)
    }
}

impl<T: Bytes> Bytes for Option<T> {
    fn validate_num_bytes(&self, min: usize, max: usize) -> Result<(), Error> {
        match self {
            Some(v) => v.validate_num_bytes(min, max),
            None => Ok(()),
        }
    }
}

pub trait HasBytes {
    fn num_bytes(&self) -> usize;
}

macro_rules! impl_via_len {
    ($(in<$lifetime:lifetime>)? $T:ty) => {
        impl<$($lifetime)?> HasBytes for $T {
            fn num_bytes(&self) -> usize {
                self.len()
            }
        }
    };
}

impl_via_len!(alloc::string::String);
impl_via_len!(in<'a> &'a alloc::string::String);
impl_via_len!(in<'a> &'a str);
impl_via_len!(in<'a> alloc::borrow::Cow<'a, str>);
impl_via_len!(alloc::rc::Rc<str>);
impl_via_len!(alloc::sync::Arc<str>);
impl_via_len!(alloc::boxed::Box<str>);
impl_via_len!(in<'a> &'a [u8]);
impl_via_len!(alloc::rc::Rc<[u8]>);
impl_via_len!(alloc::sync::Arc<[u8]>);
impl_via_len!(alloc::boxed::Box<[u8]>);
impl_via_len!(alloc::vec::Vec<u8>);

impl<const N: usize> HasBytes for [u8; N] {
    fn num_bytes(&self) -> usize {
        self.len()
    }
}
