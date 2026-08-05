use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityReadAdmissionOutcome {
    family_id: ArtifactFamilyId,
    manifest_digest: CompatibilityManifestDigest,
    relation: Option<CompatibilityRelation>,
    rejection_kind: Option<CompatibilityRejectionKind>,
    counters: CompatibilityAdmissionCounters,
}
impl CompatibilityReadAdmissionOutcome {
    pub(crate) fn accepted(
        receipt: &ReadCompatibilityReceipt,
        counters: &CompatibilityAdmissionCounters,
    ) -> Self {
        Self {
            family_id: receipt.receipt().family_id().clone(),
            manifest_digest: receipt.receipt().manifest_digest().clone(),
            relation: Some(receipt.receipt().relation()),
            rejection_kind: None,
            counters: counters.clone(),
        }
    }

    pub(crate) fn rejected(
        artifact: &QuarantinedDecodedArtifact,
        rejection: &CompatibilityRejection,
        counters: &CompatibilityAdmissionCounters,
    ) -> Self {
        Self {
            family_id: artifact.family_id().clone(),
            manifest_digest: artifact.manifest_digest().clone(),
            relation: None,
            rejection_kind: Some(rejection.kind()),
            counters: counters.clone(),
        }
    }

    pub fn is_accepted(&self) -> bool {
        self.rejection_kind.is_none()
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn manifest_digest(&self) -> &CompatibilityManifestDigest {
        &self.manifest_digest
    }

    pub fn relation(&self) -> Option<CompatibilityRelation> {
        self.relation
    }

    pub fn rejection_kind(&self) -> Option<CompatibilityRejectionKind> {
        self.rejection_kind
    }

    pub fn counters(&self) -> &CompatibilityAdmissionCounters {
        &self.counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityWriteAdmissionOutcome {
    family_id: ArtifactFamilyId,
    manifest_digest: CompatibilityManifestDigest,
    relation: Option<CompatibilityRelation>,
    rejection_kind: Option<CompatibilityRejectionKind>,
    counters: CompatibilityAdmissionCounters,
}

impl CompatibilityWriteAdmissionOutcome {
    pub(crate) fn accepted(
        receipt: &WriteCompatibilityReceipt,
        counters: &CompatibilityAdmissionCounters,
    ) -> Self {
        Self {
            family_id: receipt.receipt().family_id().clone(),
            manifest_digest: receipt.receipt().manifest_digest().clone(),
            relation: Some(receipt.receipt().relation()),
            rejection_kind: None,
            counters: counters.clone(),
        }
    }

    pub(crate) fn rejected(
        artifact: &QuarantinedDecodedArtifact,
        rejection: &CompatibilityRejection,
        counters: &CompatibilityAdmissionCounters,
    ) -> Self {
        Self {
            family_id: artifact.family_id().clone(),
            manifest_digest: artifact.manifest_digest().clone(),
            relation: None,
            rejection_kind: Some(rejection.kind()),
            counters: counters.clone(),
        }
    }

    pub fn is_accepted(&self) -> bool {
        self.rejection_kind.is_none()
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn manifest_digest(&self) -> &CompatibilityManifestDigest {
        &self.manifest_digest
    }

    pub fn relation(&self) -> Option<CompatibilityRelation> {
        self.relation
    }

    pub fn rejection_kind(&self) -> Option<CompatibilityRejectionKind> {
        self.rejection_kind
    }

    pub fn counters(&self) -> &CompatibilityAdmissionCounters {
        &self.counters
    }
}
