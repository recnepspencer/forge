#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupFamilyCatalogErrorKind {
    EmptyCatalog,
    MissingFamilyIdentity,
    MissingSpatialTouchAuthority,
    MissingTopologyInputPosture,
    MissingStageApplicability,
    EmptyStageApplicability,
    DuplicateStageApplicability,
    MissingStageReceiptFamilyIdentity,
    MissingEvidenceClass,
    EmptyEvidenceClassSet,
    DuplicateEvidenceClass,
    MissingLookupProductPosture,
    MissingIndexPosture,
    MissingQueryPosture,
    MissingDiagnosticWitness,
    MissingSourceInventoryPressure,
    DuplicateFamilyIdentity,
    MissingPhaseTwoInventoryPressure,
    MissingRequiredTopologyReceipt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupFamilyCatalogError {
    kind: EvidenceLookupFamilyCatalogErrorKind,
    message: Option<String>,
}

impl EvidenceLookupFamilyCatalogError {
    pub(crate) const fn new(kind: EvidenceLookupFamilyCatalogErrorKind) -> Self {
        Self {
            kind,
            message: None,
        }
    }

    pub(crate) fn with_message(
        kind: EvidenceLookupFamilyCatalogErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: Some(message.into()),
        }
    }

    pub const fn kind(&self) -> EvidenceLookupFamilyCatalogErrorKind {
        self.kind
    }

    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
