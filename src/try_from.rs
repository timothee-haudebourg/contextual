use crate::{FromWithContext, IntoWithContext};

pub trait TryFromWithContext<U, C>: Sized {
	type Error;

	fn try_from_with(value: U, context: &C) -> Result<Self, Self::Error>;
}

impl<T: FromWithContext<U, C>, U, C> TryFromWithContext<U, C> for T {
	type Error = std::convert::Infallible;

	fn try_from_with(value: U, context: &C) -> Result<Self, Self::Error> {
		Ok(T::from_with(value, context))
	}
}

pub trait TryIntoWithContext<U, C> {
	type Error;

	fn try_into_with(self, context: &C) -> Result<U, Self::Error>;
}

impl<T: IntoWithContext<U, C>, U, C> TryIntoWithContext<U, C> for T {
	type Error = std::convert::Infallible;

	fn try_into_with(self, context: &C) -> Result<U, Self::Error> {
		Ok(self.into_with(context))
	}
}
