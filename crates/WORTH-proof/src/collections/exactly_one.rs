#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExactlyOne<T>(T);

impl<T> ExactlyOne<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }

    pub fn get(&self) -> &T {
        &self.0
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use super::ExactlyOne;

    #[test]
    fn exactly_one_preserves_access_and_ownership() {
        let value = ExactlyOne::new("only");

        assert_eq!(value.get(), &"only");
        assert_eq!(value.into_inner(), "only");
    }

    #[test]
    fn exactly_one_is_size_honest() {
        assert_eq!(size_of::<ExactlyOne<u64>>(), size_of::<u64>());
    }
}
