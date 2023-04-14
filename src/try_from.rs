use crate::IntoWithContext;

pub trait TryFromWithContext<U, C>: Sized {
	type Error;

	fn try_from_with(value: U, context: &C) -> Result<Self, Self::Error>;
}

impl<T, U: IntoWithContext<T, C>, C> TryFromWithContext<U, C> for T {
	type Error = std::convert::Infallible;

	fn try_from_with(value: U, context: &C) -> Result<Self, Self::Error> {
		Ok(value.into_with(context))
	}
}

pub trait TryIntoWithContext<U, C> {
	type Error;

	fn try_into_with(self, context: &C) -> Result<U, Self::Error>;
}

impl<T, U: TryFromWithContext<T, C>, C> TryIntoWithContext<U, C> for T {
	type Error = U::Error;

	fn try_into_with(self, context: &C) -> Result<U, Self::Error> {
		U::try_from_with(self, context)
	}
}
