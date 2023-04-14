pub trait FromWithContext<U, C> {
	fn from_with(value: U, context: &C) -> Self;
}

impl<T, C> FromWithContext<T, C> for T {
	fn from_with(value: Self, _context: &C) -> Self {
		value
	}
}

pub trait IntoWithContext<U, C> {
	fn into_with(self, context: &C) -> U;
}

impl<T, U: FromWithContext<T, C>, C> IntoWithContext<U, C> for T {
	fn into_with(self, context: &C) -> U {
		U::from_with(self, context)
	}
}
