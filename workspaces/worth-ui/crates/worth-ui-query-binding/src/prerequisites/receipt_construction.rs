use worth_query::facade::{
    ProjectionContractSourcePosture, ProjectionFactKind, WorthQueryConsumedProjectionAuthority,
};

use super::{
    WorthUiQueryMeasurementFactFamily, WorthUiQueryMeasurementFactObservation,
    WorthUiQueryMeasurementFactReceiptError, WorthUiQueryPrerequisiteEvidence,
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
            ProjectionFactKind::DisplayField | ProjectionFactKind::DerivedScalarField => {
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
    pub(crate) prerequisites: WorthUiQueryPrerequisiteEvidence,
    pub(crate) projection_contract_digest: String,
    pub(crate) projection_consumption_receipt_digest: String,
    pub(crate) projection_source_identity: String,
    pub(crate) consumed_families: Vec<WorthUiQueryMeasurementFactFamily>,
    pub(crate) observations: Vec<WorthUiQueryMeasurementFactObservation>,
}

pub(crate) fn collect_verified_receipt_parts(
    prerequisites: WorthUiQueryPrerequisiteEvidence,
    authority: &WorthQueryConsumedProjectionAuthority,
) -> Result<VerifiedMeasurementFactReceiptParts, WorthUiQueryMeasurementFactReceiptError> {
    verify_projection_contract(&prerequisites, authority)?;
    let prerequisites =
        prerequisites.bound_to_projection_contract(authority.contract().contract_digest());
    let consumed_families = classify_consumed_fact_families(authority);
    let observations = WorthUiQueryMeasurementFactObservation::from_query_authority(
        prerequisites.clone(),
        authority,
    )
    .map_err(WorthUiQueryMeasurementFactReceiptError::Observation)?
    .into_vec();
    Ok(VerifiedMeasurementFactReceiptParts {
        prerequisites,
        projection_contract_digest: authority.contract().contract_digest().to_string(),
        projection_consumption_receipt_digest: authority.receipt().receipt_digest().to_string(),
        projection_source_identity: authority.receipt().source_identity().to_string(),
        consumed_families,
        observations,
    })
}

pub(crate) fn collect_verified_partial_receipt_parts(
    prerequisites: WorthUiQueryPrerequisiteEvidence,
    authority: &WorthQueryConsumedProjectionAuthority,
) -> Result<VerifiedMeasurementFactReceiptParts, WorthUiQueryMeasurementFactReceiptError> {
    verify_projection_contract(&prerequisites, authority)?;
    Ok(VerifiedMeasurementFactReceiptParts {
        prerequisites: prerequisites
            .bound_to_projection_contract(authority.contract().contract_digest()),
        projection_contract_digest: authority.contract().contract_digest().to_string(),
        projection_consumption_receipt_digest: authority.receipt().receipt_digest().to_string(),
        projection_source_identity: authority.receipt().source_identity().to_string(),
        consumed_families: classify_consumed_fact_families(authority),
        observations: Vec::new(),
    })
}
