#![doc = include_str!("../README.md")]
#![no_std]
#![deny(clippy::nursery, clippy::pedantic)]
#![allow(clippy::wildcard_imports, clippy::partialeq_ne_impl)]

use core::{
    borrow::{Borrow, BorrowMut}, cmp, fmt, hash::{BuildHasher, Hash, Hasher}, iter::{Product, Sum}, ops::{Deref, DerefMut}
};

mod ops_impl;

#[doc = include_str!("../README.md")]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Default, Clone)]
#[repr(transparent)]
pub struct NoCopy<T: ?Sized>(pub T);

impl<T: IntoIterator> IntoIterator for NoCopy<T> {
    type Item = T::Item;
    type IntoIter = T::IntoIter;

    #[track_caller]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T: Copy> NoCopy<T> {
    /// Only from can copy types.
    ///
    /// # Examples
    ///
    /// ```
    /// use no_copy::NoCopy;
    ///
    /// assert_eq!(NoCopy::from_copy(2), NoCopy(2));
    /// ```
    ///
    /// ```compile_fail
    /// use no_copy::NoCopy;
    ///
    /// let non_copy = String::new();
    /// let _ = NoCopy::from_copy(non_copy);
    /// ```
    pub const fn from_copy(value: T) -> Self {
        Self(value)
    }
}

impl<T: Copy> NoCopy<T> {
    pub const fn into_inner(this: Self) -> T {
        this.0
    }
}

impl<T: ?Sized + fmt::Write> fmt::Write for NoCopy<T> {
    #[track_caller]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write_str(s)
    }

    #[track_caller]
    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
        self.0.write_fmt(args)
    }

    #[track_caller]
    fn write_char(&mut self, c: char) -> fmt::Result {
        self.0.write_char(c)
    }
}

impl<T> From<T> for NoCopy<T> {
    #[track_caller]
    fn from(value: T) -> Self {
        Self(value)
    }
}
impl<T: ?Sized + PartialEq> PartialEq<T> for NoCopy<T> {
    #[track_caller]
    fn eq(&self, other: &T) -> bool {
        self.0.eq(other)
    }

    #[track_caller]
    fn ne(&self, other: &T) -> bool {
        self.0.ne(other)
    }
}
impl<T: ?Sized + PartialOrd> PartialOrd<T> for NoCopy<T> {
    #[track_caller]
    fn partial_cmp(&self, other: &T) -> Option<cmp::Ordering> {
        self.0.partial_cmp(other)
    }

    #[track_caller]
    fn lt(&self, other: &T) -> bool {
        self.0.lt(other)
    }

    #[track_caller]
    fn le(&self, other: &T) -> bool {
        self.0.le(other)
    }

    #[track_caller]
    fn gt(&self, other: &T) -> bool {
        self.0.gt(other)
    }

    #[track_caller]
    fn ge(&self, other: &T) -> bool {
        self.0.ge(other)
    }
}
impl<T: ?Sized> Borrow<T> for NoCopy<T> {
    #[track_caller]
    fn borrow(&self) -> &T {
        self
    }
}
impl<T: ?Sized> BorrowMut<T> for NoCopy<T> {
    #[track_caller]
    fn borrow_mut(&mut self) -> &mut T {
        self
    }
}
impl<U, T: AsRef<U>> AsRef<U> for NoCopy<T> {
    #[track_caller]
    fn as_ref(&self) -> &U {
        self.0.as_ref()
    }
}
impl<U, T: AsMut<U>> AsMut<U> for NoCopy<T> {
    #[track_caller]
    fn as_mut(&mut self) -> &mut U {
        self.0.as_mut()
    }
}
impl<T: ?Sized> Deref for NoCopy<T> {
    type Target = T;

    #[track_caller]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: ?Sized> DerefMut for NoCopy<T> {
    #[track_caller]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<H: ?Sized + Hasher> Hasher for NoCopy<H> {
    #[track_caller]
    fn finish(&self) -> u64 {
        (**self).finish()
    }

    #[track_caller]
    fn write(&mut self, bytes: &[u8]) {
        (**self).write(bytes);
    }

    #[track_caller]
    fn write_u8(&mut self, i: u8) {
        (**self).write_u8(i);
    }

    #[track_caller]
    fn write_u16(&mut self, i: u16) {
        (**self).write_u16(i);
    }

    #[track_caller]
    fn write_u32(&mut self, i: u32) {
        (**self).write_u32(i);
    }

    #[track_caller]
    fn write_u64(&mut self, i: u64) {
        (**self).write_u64(i);
    }

    #[track_caller]
    fn write_u128(&mut self, i: u128) {
        (**self).write_u128(i);
    }

    #[track_caller]
    fn write_usize(&mut self, i: usize) {
        (**self).write_usize(i);
    }

    #[track_caller]
    fn write_i8(&mut self, i: i8) {
        (**self).write_i8(i);
    }

    #[track_caller]
    fn write_i16(&mut self, i: i16) {
        (**self).write_i16(i);
    }

    #[track_caller]
    fn write_i32(&mut self, i: i32) {
        (**self).write_i32(i);
    }

    #[track_caller]
    fn write_i64(&mut self, i: i64) {
        (**self).write_i64(i);
    }

    #[track_caller]
    fn write_i128(&mut self, i: i128) {
        (**self).write_i128(i);
    }

    #[track_caller]
    fn write_isize(&mut self, i: isize) {
        (**self).write_isize(i);
    }
}
impl<H: BuildHasher> BuildHasher for NoCopy<H> {
    type Hasher = H::Hasher;

    #[track_caller]
    fn hash_one<T: Hash>(&self, x: T) -> u64
    where Self: Sized,
          Self::Hasher: Hasher,
    {
        self.0.hash_one(x)
    }

    #[track_caller]
    fn build_hasher(&self) -> Self::Hasher {
        self.0.build_hasher()
    }
}
impl<T: Product<A>, A> Product<A> for NoCopy<T> {
    #[track_caller]
    fn product<I: Iterator<Item = A>>(iter: I) -> Self {
        Self(iter.product())
    }
}
impl<T: Sum<A>, A> Sum<A> for NoCopy<T> {
    #[track_caller]
    fn sum<I: Iterator<Item = A>>(iter: I) -> Self {
        Self(iter.sum())
    }
}
impl<T: Extend<A>, A> Extend<A> for NoCopy<T> {
    #[track_caller]
    fn extend<T1: IntoIterator<Item = A>>(&mut self, iter: T1) {
        self.0.extend(iter);
    }
}

macro_rules! impl_fmts {
    ($trait:ident) => {
        impl<T: ?Sized + fmt::$trait> fmt::$trait for NoCopy<T> {
            #[track_caller]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

impl_fmts!(Binary);
impl_fmts!(Debug);
impl_fmts!(Display);
impl_fmts!(LowerExp);
impl_fmts!(LowerHex);
impl_fmts!(Octal);
impl_fmts!(Pointer);
impl_fmts!(UpperExp);
impl_fmts!(UpperHex);
