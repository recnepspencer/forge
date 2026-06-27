/// Machine-readable snapshot validation violation kind.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SnapshotReferenceViolationKind {
    MissingCrossFamilyReference,
    DeferredEntryUsedAsAdmitted,
}

/// Reference violation discovered after registries freeze into a snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SnapshotReferenceViolation {
    kind: SnapshotReferenceViolationKind,
    source_family_name: &'static str,
    source_identity_text: String,
    target_family_name: &'static str,
    target_identity_text: String,
}

impl SnapshotReferenceViolation {
    pub(crate) fn new(
        kind: SnapshotReferenceViolationKind,
        source_family_name: &'static str,
        source_identity_text: impl Into<String>,
        target_family_name: &'static str,
        target_identity_text: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            source_family_name,
            source_identity_text: source_identity_text.into(),
            target_family_name,
            target_identity_text: target_identity_text.into(),
        }
    }

    pub fn kind(&self) -> SnapshotReferenceViolationKind {
        self.kind
    }

    pub fn source_family_name(&self) -> &'static str {
        self.source_family_name
    }

    pub fn source_identity_text(&self) -> &str {
        &self.source_identity_text
    }

    pub fn target_family_name(&self) -> &'static str {
        self.target_family_name
    }

    pub fn target_identity_text(&self) -> &str {
        &self.target_identity_text
    }

    pub(crate) fn ordering_key(
        &self,
    ) -> (
        SnapshotReferenceViolationKind,
        &'static str,
        &str,
        &'static str,
        &str,
    ) {
        (
            self.kind,
            self.source_family_name,
            &self.source_identity_text,
            self.target_family_name,
            &self.target_identity_text,
        )
    }
}
