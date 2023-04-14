use crate::Contextual;

pub trait AsRefWithContext<U: ?Sized, C: ?Sized> {
	fn as_ref_with<'a>(&'a self, context: &'a C) -> &'a U;
}

impl<'t, 'c, T: AsRefWithContext<U, C> + ?Sized, U: ?Sized, C> AsRef<U>
	for Contextual<&'t T, &'c C>
{
	fn as_ref(&self) -> &U {
		self.0.as_ref_with(self.1)
	}
}

impl<'t, 'c, T: AsRefWithContext<U, C> + ?Sized, U: ?Sized, C> AsRef<U>
	for Contextual<&'t T, &'c mut C>
{
	fn as_ref(&self) -> &U {
		self.0.as_ref_with(self.1)
	}
}

impl<'a, T: AsRefWithContext<str, C> + ?Sized, C: ?Sized> Contextual<&'a T, &'a C> {
	pub fn as_str(&self) -> &'a str {
		self.0.as_ref_with(self.1)
	}
}

impl<'a, T: AsRefWithContext<str, C> + ?Sized, C: ?Sized> Contextual<&'a T, &'a mut C> {
	pub fn as_str(&self) -> &str {
		self.0.as_ref_with(self.1)
	}
}
