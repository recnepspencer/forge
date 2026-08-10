use super::super::{
    classification_error, stable_digest, SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind,
};
use super::version_window::SupportFamilyVersionWindow;
use crate::failure::StoreError;
use crate::{
    ArtifactFamilyId, CompatibilityReadAdmissionOutcome, CompatibilityRejectionKind,
    CompatibilityRelation, ReadCompatibilityReceipt,
};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportCompatibilityReceiptWitness {
    support_family_id: SubscriptionSupportFamilyId,
    support_family_kind: SubscriptionSupportFamilyKind,
    milestone12_family_id: ArtifactFamilyId,
    version_window: SupportFamilyVersionWindow,
    manifest_digest: String,
    registry_snapshot_identity: Option<String>,
    manifest_frontier_identity: Option<String>,
    relation: Option<CompatibilityRelation>,
    rejection_kind: Option<CompatibilityRejectionKind>,
    receipt_digest: String,
}

#[allow(dead_code)]
impl SupportCompatibilityReceiptWitness {
    pub(crate) fn from_read_receipt(
        support_family_id: SubscriptionSupportFamilyId,
        support_family_kind: SubscriptionSupportFamilyKind,
        receipt: &ReadCompatibilityReceipt,
    ) -> Result<Self, StoreError> {
        let inner = receipt.receipt();
        if inner.family_id().as_str() != support_family_id.as_str() {
            return Err(classification_error(
                "subscription-support compatibility receipt witness must match support family id",
            ));
        }
        let observed = inner.observed_semantic_version().value();
        let target = inner.target_semantic_version().value();
        let minimum_reader_version = u16::try_from(observed.min(target)).map_err(|_| {
            classification_error(
                "subscription-support compatibility receipt semantic versions exceed support window range",
            )
        })?;
        let maximum_payload_version = u16::try_from(observed.max(target)).map_err(|_| {
            classification_error(
                "subscription-support compatibility receipt semantic versions exceed support window range",
            )
        })?;
        let version_window = SupportFamilyVersionWindow::new(
            support_family_id.clone(),
            support_family_kind,
            minimum_reader_version,
            maximum_payload_version,
        )?;
        let receipt_digest = stable_digest(inner)?;
        Ok(Self {
            support_family_id,
            support_family_kind,
            milestone12_family_id: inner.family_id().clone(),
            version_window,
            manifest_digest: inner.manifest_digest().as_str().to_string(),
            registry_snapshot_identity: Some(inner.registry_snapshot_identity().to_string()),
            manifest_frontier_identity: Some(inner.manifest_frontier_identity().to_string()),
            relation: Some(inner.relation()),
            rejection_kind: None,
            receipt_digest,
        })
    }

    pub(crate) fn from_read_admission_outcome(
        support_family_id: SubscriptionSupportFamilyId,
        support_family_kind: SubscriptionSupportFamilyKind,
        version_window: SupportFamilyVersionWindow,
        outcome: &CompatibilityReadAdmissionOutcome,
    ) -> Result<Self, StoreError> {
        if outcome.family_id().as_str() != support_family_id.as_str()
            || version_window.family_id() != &support_family_id
            || version_window.family_kind() != support_family_kind
        {
            return Err(classification_error(
                "subscription-support compatibility read outcome witness must match support family",
            ));
        }
        if outcome.is_accepted() && outcome.relation().is_none() {
            return Err(classification_error(
                "accepted subscription-support compatibility read outcomes require a relation",
            ));
        }
        if !outcome.is_accepted() && outcome.rejection_kind().is_none() {
            return Err(classification_error(
                "rejected subscription-support compatibility read outcomes require a typed rejection",
            ));
        }
        let receipt_digest = stable_digest(outcome)?;
        Ok(Self {
            support_family_id,
            support_family_kind,
            milestone12_family_id: outcome.family_id().clone(),
            version_window,
            manifest_digest: outcome.manifest_digest().as_str().to_string(),
            registry_snapshot_identity: None,
            manifest_frontier_identity: None,
            relation: outcome.relation(),
            rejection_kind: outcome.rejection_kind(),
            receipt_digest,
        })
    }

    pub(super) fn unbound_legacy(
        version_window: &SupportFamilyVersionWindow,
        manifest_digest: &str,
    ) -> Self {
        Self {
            support_family_id: version_window.family_id().clone(),
            support_family_kind: version_window.family_kind(),
            milestone12_family_id: ArtifactFamilyId::new(version_window.family_id().as_str()),
            version_window: version_window.clone(),
            manifest_digest: manifest_digest.to_string(),
            registry_snapshot_identity: None,
            manifest_frontier_identity: None,
            relation: None,
            rejection_kind: None,
            receipt_digest: "unbound-legacy-support-compatibility-receipt".into(),
        }
    }

    pub fn support_family_id(&self) -> &SubscriptionSupportFamilyId {
        &self.support_family_id
    }

    pub fn support_family_kind(&self) -> SubscriptionSupportFamilyKind {
        self.support_family_kind
    }

    pub fn milestone12_family_id(&self) -> &ArtifactFamilyId {
        &self.milestone12_family_id
    }

    pub fn version_window(&self) -> &SupportFamilyVersionWindow {
        &self.version_window
    }

    pub fn manifest_digest(&self) -> &str {
        &self.manifest_digest
    }

    pub fn relation(&self) -> Option<CompatibilityRelation> {
        self.relation
    }

    pub fn rejection_kind(&self) -> Option<CompatibilityRejectionKind> {
        self.rejection_kind
    }

    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }
}
