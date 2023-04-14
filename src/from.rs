pub trait FromWithContext<U, C> {
	fn from_with(value: U, context: &C) -> Self;
}

pub trait IntoWithContext<U, C> {
	fn into_with(self, context: &C) -> U;
}
