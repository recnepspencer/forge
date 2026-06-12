#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeOutcomeProjectionKey {
    runtime_identity_basis: String,
    projection_basis: String,
}

impl RuntimeOutcomeProjectionKey {
    pub(crate) fn new(
        runtime_identity_basis: impl Into<String>,
        projection_basis: impl Into<String>,
    ) -> Self {
        Self {
            runtime_identity_basis: runtime_identity_basis.into(),
            projection_basis: projection_basis.into(),
        }
    }

    pub fn runtime_identity_basis(&self) -> &str {
        &self.runtime_identity_basis
    }

    pub fn projection_basis(&self) -> &str {
        &self.projection_basis
    }
}
