#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ForgeServerAuthenticatedPrincipal {
    principal_id: String,
}

impl ForgeServerAuthenticatedPrincipal {
    pub(crate) fn new(principal_id: String) -> Self {
        Self { principal_id }
    }

    pub fn principal_id(&self) -> &str {
        &self.principal_id
    }
}
