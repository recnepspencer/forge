#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryNativeProjectionRequestDenialKind {
    WholeAspectNotProjected,
    FieldRequiresStruct,
    UnknownField,
    FieldNotProjected,
    UnsupportedAspectShape,
    ConflictingDeclaration,
    NoNativeFacts,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryNativeProjectionRequestDenial {
    kind: WorthQueryNativeProjectionRequestDenialKind,
    contract_key: worth_foundational::facade::AspectKey,
    contract_revision: worth_foundational::facade::AspectContractRevision,
    requested_field: Option<worth_foundational::facade::FieldKey>,
}

impl WorthQueryNativeProjectionRequestDenial {
    pub(crate) fn new(
        kind: WorthQueryNativeProjectionRequestDenialKind,
        contract: &worth_foundational::facade::AspectContract,
        requested_field: Option<worth_foundational::facade::FieldKey>,
    ) -> Self {
        Self {
            kind,
            contract_key: contract.key().clone(),
            contract_revision: contract.revision(),
            requested_field,
        }
    }

    pub fn kind(&self) -> WorthQueryNativeProjectionRequestDenialKind {
        self.kind
    }

    pub fn contract_key(&self) -> &worth_foundational::facade::AspectKey {
        &self.contract_key
    }

    pub fn contract_revision(&self) -> worth_foundational::facade::AspectContractRevision {
        self.contract_revision
    }

    pub fn requested_field(&self) -> Option<&worth_foundational::facade::FieldKey> {
        self.requested_field.as_ref()
    }
}
