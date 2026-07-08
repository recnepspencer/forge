use forge_query::facade::{
    CompletedProjectionFactConsumption, ProjectionContractSourcePosture, ProjectionFactKind,
};

use super::{
    WorthUiQueryMeasurementFactFamily, WorthUiQueryMeasurementFactObservation,
    WorthUiQueryMeasurementFactReceiptError, WorthUiQueryPrerequisiteEvidence,
};

pub(crate) fn verify_projection_contract(
    prerequisites: &WorthUiQueryPrerequisiteEvidence,
    completed: &CompletedProjectionFactConsumption,
) -> Result<(), WorthUiQueryMeasurementFactReceiptError> {
    if completed.contract().source_posture()
        != ProjectionContractSourcePosture::QueryOwnedReceiptSource
    {
        return Err(WorthUiQueryMeasurementFactReceiptError::NonQueryOwnedProjectionSource);
    }
    if completed.contract().basis_digest()
        != Some(prerequisites.resolution_report().basis_digest().as_str())
    {
        return Err(WorthUiQueryMeasurementFactReceiptError::BasisDigestMismatch);
    }
    Ok(())
}

pub(crate) fn classify_consumed_fact_families(
    completed: &CompletedProjectionFactConsumption,
) -> Vec<WorthUiQueryMeasurementFactFamily> {
    let mut consumed_families = completed
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
    pub(crate) projection_consumption_declaration_digest: String,
    pub(crate) projection_consumption_receipt_digest: String,
    pub(crate) projection_fact_set_digest: String,
    pub(crate) projection_source_identity: String,
    pub(crate) consumed_families: Vec<WorthUiQueryMeasurementFactFamily>,
    pub(crate) observations: Vec<WorthUiQueryMeasurementFactObservation>,
}

pub(crate) fn collect_verified_receipt_parts(
    prerequisites: WorthUiQueryPrerequisiteEvidence,
    completed: &CompletedProjectionFactConsumption,
) -> Result<VerifiedMeasurementFactReceiptParts, WorthUiQueryMeasurementFactReceiptError> {
    verify_projection_contract(&prerequisites, completed)?;
    let prerequisites =
        prerequisites.bound_to_projection_contract(completed.contract().contract_digest());
    let consumed_families = classify_consumed_fact_families(completed);
    let observations = WorthUiQueryMeasurementFactObservation::from_completed_projection_consumption(
        prerequisites.clone(),
        completed,
    )
    .map_err(WorthUiQueryMeasurementFactReceiptError::Observation)?
    .into_vec();
    Ok(VerifiedMeasurementFactReceiptParts {
        prerequisites,
        projection_contract_digest: completed.contract().contract_digest().to_string(),
        projection_consumption_declaration_digest: completed
            .receipt()
            .declaration_digest()
            .to_string(),
        projection_consumption_receipt_digest: completed.receipt().receipt_digest().to_string(),
        projection_fact_set_digest: completed.receipt().fact_set_digest().to_string(),
        projection_source_identity: completed.receipt().source_identity().to_string(),
        consumed_families,
        observations,
    })
}