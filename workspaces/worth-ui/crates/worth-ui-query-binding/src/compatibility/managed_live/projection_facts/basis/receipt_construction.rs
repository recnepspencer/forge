use worth_query::facade::foundation::{
    ProjectionContractSourcePosture, ProjectionFactKind, WorthQueryConsumedProjectionAuthority,
};

use super::{
    WorthUiQueryMeasurementFactFamily, WorthUiQueryMeasurementFactObservation,
    WorthUiQueryMeasurementFactReceiptError, WorthUiQueryMeasurementRefinementCounters,
    WorthUiQueryPrerequisiteEvidence,
};

pub(crate) fn verify_projection_contract(
    prerequisites: &WorthUiQueryPrerequisiteEvidence,
    authority: &WorthQueryConsumedProjectionAuthority,
) -> Result<(), WorthUiQueryMeasurementFactReceiptError> {
    if authority.contract().source_posture()
        != ProjectionContractSourcePosture::QueryOwnedReceiptSource
    {
        return Err(WorthUiQueryMeasurementFactReceiptError::NonQueryOwnedProjectionSource);
    }
    if !authority.binds_resolved_basis(prerequisites.basis()) {
        return Err(WorthUiQueryMeasurementFactReceiptError::BasisDigestMismatch);
    }
    if !prerequisites.accepts_projection_contract(
        super::WorthUiQueryProjectionContractIdentity::from_authority(authority),
    ) {
        return Err(WorthUiQueryMeasurementFactReceiptError::ProjectionContractMismatch);
    }
    Ok(())
}

pub(crate) fn classify_consumed_fact_families(
    authority: &WorthQueryConsumedProjectionAuthority,
) -> Vec<WorthUiQueryMeasurementFactFamily> {
    let mut consumed_families = authority
        .contract()
        .fact_families()
        .iter()
        .filter_map(|fact_family| match fact_family.kind() {
            ProjectionFactKind::DisplayField | ProjectionFactKind::DerivedField => {
                Some(WorthUiQueryMeasurementFactFamily::ScrollContentExtent)
            }
            ProjectionFactKind::EntityIdentity
            | ProjectionFactKind::ViewLocalIdentity
            | ProjectionFactKind::TargetIdentity
            | ProjectionFactKind::SourceReference
            | ProjectionFactKind::EffectContinuity
            | ProjectionFactKind::Membership
            | ProjectionFactKind::RelationEndpoint => None,
        })
        .collect::<Vec<_>>();
    consumed_families.sort_unstable();
    consumed_families.dedup();
    consumed_families
}

pub(crate) struct VerifiedMeasurementFactReceiptParts {
    pub(crate) consumed_families: Vec<WorthUiQueryMeasurementFactFamily>,
    pub(crate) observations: Vec<WorthUiQueryMeasurementFactObservation>,
    pub(crate) refinement_counters: WorthUiQueryMeasurementRefinementCounters,
}

pub(crate) fn collect_verified_receipt_parts(
    prerequisites: WorthUiQueryPrerequisiteEvidence,
    authority: &WorthQueryConsumedProjectionAuthority,
) -> Result<VerifiedMeasurementFactReceiptParts, WorthUiQueryMeasurementFactReceiptError> {
    verify_projection_contract(&prerequisites, authority)?;
    collect_authority_receipt_parts(authority)
}

pub(crate) fn collect_authority_receipt_parts(
    authority: &WorthQueryConsumedProjectionAuthority,
) -> Result<VerifiedMeasurementFactReceiptParts, WorthUiQueryMeasurementFactReceiptError> {
    if authority.contract().source_posture()
        != ProjectionContractSourcePosture::QueryOwnedReceiptSource
    {
        return Err(WorthUiQueryMeasurementFactReceiptError::NonQueryOwnedProjectionSource);
    }
    let consumed_families = classify_consumed_fact_families(authority);
    let (observations, refinement_counters) =
        WorthUiQueryMeasurementFactObservation::from_query_authority(authority)
            .map_err(WorthUiQueryMeasurementFactReceiptError::Observation)?;
    Ok(VerifiedMeasurementFactReceiptParts {
        consumed_families,
        observations: observations.into_vec(),
        refinement_counters,
    })
}

pub(crate) fn collect_verified_partial_receipt_parts(
    prerequisites: WorthUiQueryPrerequisiteEvidence,
    authority: &WorthQueryConsumedProjectionAuthority,
) -> Result<VerifiedMeasurementFactReceiptParts, WorthUiQueryMeasurementFactReceiptError> {
    verify_projection_contract(&prerequisites, authority)?;
    collect_authority_partial_receipt_parts(authority)
}

pub(crate) fn collect_authority_partial_receipt_parts(
    authority: &WorthQueryConsumedProjectionAuthority,
) -> Result<VerifiedMeasurementFactReceiptParts, WorthUiQueryMeasurementFactReceiptError> {
    if authority.contract().source_posture()
        != ProjectionContractSourcePosture::QueryOwnedReceiptSource
    {
        return Err(WorthUiQueryMeasurementFactReceiptError::NonQueryOwnedProjectionSource);
    }
    Ok(VerifiedMeasurementFactReceiptParts {
        consumed_families: classify_consumed_fact_families(authority),
        observations: Vec::new(),
        refinement_counters: WorthUiQueryMeasurementRefinementCounters::default(),
    })
}
