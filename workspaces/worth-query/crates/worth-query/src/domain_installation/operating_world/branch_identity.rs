#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBranchHeadIdentity(Box<str>);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryBranchHeadIdentityError {
    Empty,
}

impl WorthQueryBranchHeadIdentity {
    pub fn new(value: impl Into<String>) -> Result<Self, WorthQueryBranchHeadIdentityError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(WorthQueryBranchHeadIdentityError::Empty);
        }
        Ok(Self(value.into_boxed_str()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
