use crate::Contextual;

pub trait IntoRefWithContext<'a, U: ?Sized, C: ?Sized> {
	fn into_ref_with(self, context: &'a C) -> &'a U;
}

impl<'a, T: IntoRefWithContext<'a, str, C>, C: ?Sized> Contextual<T, &'a C> {
	pub fn into_str(self) -> &'a str {
		self.0.into_ref_with(self.1)
	}
}

impl<'a, T: IntoRefWithContext<'a, str, C>, C: ?Sized> Contextual<T, &'a mut C> {
	pub fn into_str(self) -> &'a str {
		self.0.into_ref_with(self.1)
	}
}
