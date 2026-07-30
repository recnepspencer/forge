use worth_query::facade::runtime::WorthQueryEvidenceIdentity;

#[derive(Debug, Eq, PartialEq)]
pub struct UiProjectionFactReceipt {
    query_world_identity: WorthQueryEvidenceIdentity,
    binding_identity: WorthQueryEvidenceIdentity,
    source_generation_identity: WorthQueryEvidenceIdentity,
    result_generation_identity: WorthQueryEvidenceIdentity,
}

impl UiProjectionFactReceipt {
    pub(crate) fn admitted(
        query_world_identity: WorthQueryEvidenceIdentity,
        binding_identity: WorthQueryEvidenceIdentity,
        source_generation_identity: WorthQueryEvidenceIdentity,
        result_generation_identity: WorthQueryEvidenceIdentity,
    ) -> Self {
        Self {
            query_world_identity,
            binding_identity,
            source_generation_identity,
            result_generation_identity,
        }
    }

    pub(crate) fn binding_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.binding_identity
    }

    pub(crate) fn source_generation_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.source_generation_identity
    }

    pub(crate) fn result_generation_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.result_generation_identity
    }

    pub fn query_world_identity_for_reporting(&self) -> &str {
        self.query_world_identity
            .terminal_projection_for_reporting()
    }

    pub fn binding_identity_for_reporting(&self) -> &str {
        self.binding_identity.terminal_projection_for_reporting()
    }

    pub fn source_generation_for_reporting(&self) -> &str {
        self.source_generation_identity
            .terminal_projection_for_reporting()
    }

    pub fn result_generation_for_reporting(&self) -> &str {
        self.result_generation_identity
            .terminal_projection_for_reporting()
    }
}
