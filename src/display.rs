use crate::Contextual;
use std::fmt;

pub trait DisplayWithContext<C: ?Sized> {
	fn fmt_with(&self, context: &C, f: &mut fmt::Formatter) -> fmt::Result;
}

impl<T: DisplayWithContext<C::Target>, C: std::ops::Deref> fmt::Display for Contextual<T, C> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt_with(self.1.deref(), f)
	}
}

impl<'a, T: DisplayWithContext<C> + ?Sized, C> DisplayWithContext<C> for &'a T {
	fn fmt_with(&self, context: &C, f: &mut fmt::Formatter) -> fmt::Result {
		T::fmt_with(*self, context, f)
	}
}

impl<'a, T: DisplayWithContext<C> + std::borrow::ToOwned + ?Sized, C> DisplayWithContext<C>
	for std::borrow::Cow<'a, T>
{
	fn fmt_with(&self, context: &C, f: &mut fmt::Formatter) -> fmt::Result {
		T::fmt_with(self, context, f)
	}
}
