#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiIdentitySeedContribution {
    contributor: String,
    identity_basis: String,
}

impl WorthUiIdentitySeedContribution {
    pub fn contributor(&self) -> &str {
        &self.contributor
    }

    pub fn identity_basis(&self) -> &str {
        &self.identity_basis
    }
}
