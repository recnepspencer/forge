#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAdmittedWorldBasis {
    domain_key: &'static str,
    display_name: &'static str,
    operating_context_identity_digest: String,
    handle_identity_digest: String,
    support_snapshot_digest: String,
}

impl ForgeQueryAdmittedWorldBasis {
    pub(crate) fn new(
        domain_key: &'static str,
        display_name: &'static str,
        operating_context_identity_digest: String,
        handle_identity_digest: String,
        support_snapshot_digest: String,
    ) -> Self {
        Self {
            domain_key,
            display_name,
            operating_context_identity_digest,
            handle_identity_digest,
            support_snapshot_digest,
        }
    }

    pub fn domain_key(&self) -> &'static str {
        self.domain_key
    }

    pub fn display_name(&self) -> &'static str {
        self.display_name
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        &self.operating_context_identity_digest
    }

    pub fn handle_identity_digest(&self) -> &str {
        &self.handle_identity_digest
    }

    pub fn support_snapshot_digest(&self) -> &str {
        &self.support_snapshot_digest
    }
}
