#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthServerAuthenticatedPrincipal {
    principal_id: String,
}

impl WorthServerAuthenticatedPrincipal {
    pub(crate) fn new(principal_id: String) -> Self {
        Self { principal_id }
    }

    pub fn principal_id(&self) -> &str {
        &self.principal_id
    }
}
