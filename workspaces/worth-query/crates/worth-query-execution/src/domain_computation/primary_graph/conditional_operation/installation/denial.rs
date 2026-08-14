#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryConditionalRuntimeInstallationDenialKind {
    PrimaryGraphPublication,
    ForeignBinding,
    DuplicateBinding,
    IncompleteBindingInventory,
    BridgeRejected,
    ReconstructionPrincipal,
    ReconstructionScope,
    ReconstructionQuery,
    ReconstructionProjection,
    ReconstructionIntent,
    RebindRequired,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConditionalRuntimeInstallationDenial {
    kind: WorthQueryConditionalRuntimeInstallationDenialKind,
    subject: String,
}

impl WorthQueryConditionalRuntimeInstallationDenial {
    pub(in crate::domain_computation::primary_graph::conditional_operation) fn new(
        kind: WorthQueryConditionalRuntimeInstallationDenialKind,
        subject: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            subject: subject.into(),
        }
    }

    pub fn kind(&self) -> WorthQueryConditionalRuntimeInstallationDenialKind {
        self.kind
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }
}
