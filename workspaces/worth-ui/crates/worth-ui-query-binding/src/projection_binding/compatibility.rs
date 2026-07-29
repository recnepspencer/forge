use worth_query::facade::runtime::WorthQueryEvidenceIdentity;

#[derive(Debug, Eq, PartialEq)]
pub struct UiProjectionBindingCompatibilityProof {
    query_compatibility_identity: WorthQueryEvidenceIdentity,
    predecessor_binding_identity: WorthQueryEvidenceIdentity,
    successor_binding_identity: WorthQueryEvidenceIdentity,
}

impl UiProjectionBindingCompatibilityProof {
    pub fn query_compatibility_identity_for_reporting(&self) -> &str {
        self.query_compatibility_identity
            .terminal_projection_for_reporting()
    }

    pub fn predecessor_binding_identity_for_reporting(&self) -> &str {
        self.predecessor_binding_identity
            .terminal_projection_for_reporting()
    }

    pub fn successor_binding_identity_for_reporting(&self) -> &str {
        self.successor_binding_identity
            .terminal_projection_for_reporting()
    }
}
