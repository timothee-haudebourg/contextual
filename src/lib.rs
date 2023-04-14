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
mod as_ref;
mod display;
mod from;
mod into_ref;
mod try_from;

pub use as_ref::*;
pub use display::*;
pub use from::*;
pub use into_ref::*;
pub use try_from::*;

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
