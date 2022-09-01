//! A small crate to deal with data in context.
//!
//! ```
//! use contextual::{WithContext, AsRefWithContext, DisplayWithContext};
//! use std::fmt;
//!
//! /// Index of an element in some array.
//! pub struct Index(usize);
//!
//! impl<T, C: AsRef<[T]>> AsRefWithContext<T, C> for Index {
//!   fn as_ref_with<'a>(&'a self, context: &'a C) -> &'a T {
//!     &context.as_ref()[self.0]
//!   }
//! }
//!
//! impl<'a, C: AsRef<[&'a str]>> DisplayWithContext<C> for Index {
//!   fn fmt_with(&self, context: &C, f: &mut fmt::Formatter) -> fmt::Result {
//!     use fmt::Display;
//!     context.as_ref()[self.0].fmt(f)
//!   }
//! }
//!
//! let i = Index(1);
//! let context = ["a", "b", "c"];
//!
//! print!("index: {}", i.with(&context));
//! assert_eq!(*i.with(&context).as_ref(), "b")
//! ```
use std::fmt;

pub struct Contextual<T, C>(pub T, pub C);

pub trait WithContext {
	fn with<C>(&self, context: C) -> Contextual<&Self, C>;

	fn into_with<C>(self, context: C) -> Contextual<Self, C>
	where
		Self: Sized;
}

impl<T: ?Sized> WithContext for T {
	fn with<C>(&self, context: C) -> Contextual<&Self, C> {
		Contextual(self, context)
	}

	fn into_with<C>(self, context: C) -> Contextual<Self, C>
	where
		Self: Sized,
	{
		Contextual(self, context)
	}
}

impl<T, C> std::ops::Deref for Contextual<T, C> {
	type Target = T;

	fn deref(&self) -> &T {
		&self.0
	}
}

impl<T, C> std::ops::DerefMut for Contextual<T, C> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}

impl<'c, T: DisplayWithContext<C>, C> fmt::Display for Contextual<T, &'c C> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt_with(self.1, f)
	}
}

impl<'a, T: AsRefWithContext<str, C> + ?Sized, C: ?Sized> Contextual<&'a T, &'a C> {
	pub fn as_str(&self) -> &'a str {
		self.0.as_ref_with(self.1)
	}
}

impl<'a, T: IntoRefWithContext<'a, str, C>, C: ?Sized> Contextual<T, &'a C> {
	pub fn into_str(self) -> &'a str {
		self.0.into_ref_with(self.1)
	}
}

impl<'t, 'c, T: AsRefWithContext<U, C> + ?Sized, U: ?Sized, C> AsRef<U>
	for Contextual<&'t T, &'c C>
{
	fn as_ref(&self) -> &U {
		self.0.as_ref_with(self.1)
	}
}

pub trait DisplayWithContext<C: ?Sized> {
	fn fmt_with(&self, context: &C, f: &mut fmt::Formatter) -> fmt::Result;
}

impl<'a, T: DisplayWithContext<C> + ?Sized, C> DisplayWithContext<C> for &'a T {
	fn fmt_with(&self, context: &C, f: &mut fmt::Formatter) -> fmt::Result {
		T::fmt_with(*self, context, f)
	}
}

pub trait AsRefWithContext<U: ?Sized, C: ?Sized> {
	fn as_ref_with<'a>(&'a self, context: &'a C) -> &'a U;
}

pub trait IntoRefWithContext<'a, U: ?Sized, C: ?Sized> {
	fn into_ref_with(self, context: &'a C) -> &'a U;
}
