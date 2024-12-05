//! Implemented by string-like types for which we can retrieve the number of [Unicode Scalar Values](https://www.unicode.org/glossary/#unicode_scalar_value).
//!
//! See also: [`chars` on `str`](https://doc.rust-lang.org/std/primitive.str.html#method.chars).

use crate::error::Error;

pub fn apply<T: Chars>(v: &T, (min, max): (usize, usize)) -> Result<(), Error> {
    v.validate_num_chars(min, max)
}

pub trait Chars {
    fn validate_num_chars(&self, min: usize, max: usize) -> Result<(), Error>;
}

impl<T: HasChars> Chars for T {
    fn validate_num_chars(&self, min: usize, max: usize) -> Result<(), Error> {
        super::check_len(self.num_chars(), min, max)
    }
}

impl<T: Chars> Chars for Option<T> {
    fn validate_num_chars(&self, min: usize, max: usize) -> Result<(), Error> {
        match self {
            Some(v) => v.validate_num_chars(min, max),
            None => Ok(()),
        }
    }
}

pub trait HasChars {
    fn num_chars(&self) -> usize;
}

macro_rules! impl_via_chars {
    ($(in <$lifetime:lifetime>)? $T:ty) => {
        impl<$($lifetime)?> HasChars for $T {
            fn num_chars(&self) -> usize {
                self.chars().count()
            }
        }
    };
}

impl_via_chars!(alloc::string::String);
impl_via_chars!(in<'a> &'a alloc::string::String);
impl_via_chars!(in<'a> &'a str);
impl_via_chars!(in<'a> alloc::borrow::Cow<'a, str>);
impl_via_chars!(alloc::rc::Rc<str>);
impl_via_chars!(alloc::sync::Arc<str>);
impl_via_chars!(alloc::boxed::Box<str>);

macro_rules! impl_via_len {
    ($(in<$lifetime:lifetime>)? $T:ty) => {
        impl<$($lifetime)?> HasChars for $T {
            fn num_chars(&self) -> usize {
                self.len()
            }
        }
    };
}

impl_via_len!(in<'a> &'a [char]);
impl_via_len!(alloc::sync::Arc<[char]>);
impl_via_len!(alloc::rc::Rc<[char]>);
impl_via_len!(alloc::boxed::Box<[char]>);
impl_via_len!(alloc::vec::Vec<char>);
