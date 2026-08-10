use crate::{
    failure::{StoreError, StoreErrorKind},
    SubscriptionResumeClassification, SubscriptionSupportCatalog, SubscriptionSupportFamilyKind,
    SubscriptionSupportMissingSupportRecoveryRequest,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct MissingSubscriptionSupportClassificationEvidence {
    family_is_admitted: bool,
    rebuild_evidence_present: bool,
    family_supports_rebuild: bool,
    retained_basis_matches: bool,
}

pub(super) fn collect_missing_support_classification_evidence(
    request: &SubscriptionSupportMissingSupportRecoveryRequest,
    artifact_is_durable: bool,
) -> Result<MissingSubscriptionSupportClassificationEvidence, StoreError> {
    if artifact_is_durable {
        return Err(StoreError::new(
            StoreErrorKind::SubscriptionSupportClassificationViolation,
            "subscription-support missing recovery received an artifact that is still durable",
        ));
    }
    Ok(MissingSubscriptionSupportClassificationEvidence {
        family_is_admitted: SubscriptionSupportCatalog::first_ship()
            .density_for(request.family_kind())
            .is_some(),
        rebuild_evidence_present: !request.basis_digest().trim().is_empty()
            && !request.cursor_digest().trim().is_empty()
            && !request.checkpoint_digest().trim().is_empty(),
        family_supports_rebuild: request.family_kind()
            == SubscriptionSupportFamilyKind::MaterializedNarrowingSupport,
        retained_basis_matches: request.retained_rebuild_basis_digest()
            == Some(request.basis_digest()),
    })
}

pub(super) fn classify_missing_support(
    evidence: MissingSubscriptionSupportClassificationEvidence,
) -> SubscriptionResumeClassification {
    if evidence.family_is_admitted
        && evidence.rebuild_evidence_present
        && evidence.family_supports_rebuild
        && evidence.retained_basis_matches
    {
        SubscriptionResumeClassification::RebuildRequired
    } else {
        SubscriptionResumeClassification::NotResumable
    }
}
