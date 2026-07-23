use std::marker::PhantomData;

use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::operation_identity_basis::aftermath_material;
use crate::domain_installation::{
    WorthQueryAftermathPostcondition, WorthQueryBoundDomainOperation,
    WorthQueryCompletedWorkflowTrace, WorthQueryOperationReversalContract,
};
use crate::identity::hash_parts;

use super::super::{
    mint_aftermath_authority, WorthQueryAftermathAdmissionDenial,
    WorthQueryAftermathAuthorityBasis, WorthQueryAftermathCounters, WorthQueryAftermathKind,
    WorthQueryAftermathOriginalEvidence,
};
use super::WorthQueryAdmittedAftermath;

pub(super) struct ValidatedAftermath {
    pub(super) kind: WorthQueryAftermathKind,
    postcondition: WorthQueryAftermathPostcondition,
    effect_receipt_identities: Vec<String>,
}

pub(super) fn validate_aftermath_candidate<D, OO, OF, OL, CO, CF, CL>(
    trace: &WorthQueryCompletedWorkflowTrace<D, OO, OF, OL>,
    candidate: &WorthQueryBoundDomainOperation<D, CO, CF, CL>,
    declared: WorthQueryOperationReversalContract,
    counters: &mut WorthQueryAftermathCounters,
) -> Result<ValidatedAftermath, WorthQueryAftermathAdmissionDenial>
where
    OL: BasisOperationLane,
    CL: BasisOperationLane,
{
    require_current_authority_pair(trace, candidate)?;
    let (kind, expected_operation, expected_lowering, postcondition) =
        classify_reversal_contract(declared)?;
    let effect_receipt_identities = require_realized_effect_receipts(trace)?;
    counters.effect_receipt_checks = effect_receipt_identities.len();
    if candidate.definition().identity() != &expected_operation {
        return Err(WorthQueryAftermathAdmissionDenial::CandidateOperationMismatch);
    }
    counters.candidate_lowering_checks = usize::from(expected_lowering.is_some());
    if expected_lowering
        .is_some_and(|expected| candidate.definition().semantics().lowering.family != expected)
    {
        return Err(WorthQueryAftermathAdmissionDenial::CandidateLoweringMismatch);
    }
    counters.postcondition_checks = 1;
    if !valid_postcondition(kind, &postcondition) {
        return Err(WorthQueryAftermathAdmissionDenial::InvalidPostcondition);
    }
    Ok(ValidatedAftermath {
        kind,
        postcondition,
        effect_receipt_identities,
    })
}

fn require_current_authority_pair<D, OO, OF, OL, CO, CF, CL>(
    trace: &WorthQueryCompletedWorkflowTrace<D, OO, OF, OL>,
    candidate: &WorthQueryBoundDomainOperation<D, CO, CF, CL>,
) -> Result<(), WorthQueryAftermathAdmissionDenial>
where
    OL: BasisOperationLane,
    CL: BasisOperationLane,
{
    if !trace.bound().installation_is_current() {
        return Err(WorthQueryAftermathAdmissionDenial::OriginalInstallationStale);
    }
    if !candidate.installation_is_current() {
        return Err(WorthQueryAftermathAdmissionDenial::CandidateInstallationStale);
    }
    let original_authority = trace.bound().operation().domain_authority();
    let candidate_authority = candidate.operation().domain_authority();
    if original_authority.runtime_authority() != candidate_authority.runtime_authority() {
        return Err(WorthQueryAftermathAdmissionDenial::ForeignRuntime);
    }
    if trace.bound().operation().installation_generation()
        != candidate.operation().installation_generation()
    {
        return Err(WorthQueryAftermathAdmissionDenial::InstallationGenerationMismatch);
    }
    if trace.bound().basis().capability_digest() != candidate.basis().capability_digest() {
        return Err(WorthQueryAftermathAdmissionDenial::BasisMismatch);
    }
    Ok(())
}

fn classify_reversal_contract(
    declared: WorthQueryOperationReversalContract,
) -> Result<
    (
        WorthQueryAftermathKind,
        worth_query_installation::facade::WorthQueryDomainOperationIdentity,
        Option<String>,
        WorthQueryAftermathPostcondition,
    ),
    WorthQueryAftermathAdmissionDenial,
> {
    match declared {
        WorthQueryOperationReversalContract::ExactInverseWithPostcondition {
            operation,
            lowering_family,
            postcondition,
        } => Ok((
            WorthQueryAftermathKind::ExactInverse,
            operation,
            Some(lowering_family),
            postcondition,
        )),
        WorthQueryOperationReversalContract::CompensationWithPostcondition {
            operation,
            postcondition,
        } => Ok((
            WorthQueryAftermathKind::Compensation,
            operation,
            None,
            postcondition,
        )),
        WorthQueryOperationReversalContract::ExactInverse { .. }
        | WorthQueryOperationReversalContract::Compensation { .. } => {
            Err(WorthQueryAftermathAdmissionDenial::DeclarationIncomplete)
        }
        WorthQueryOperationReversalContract::Irreversible => {
            Err(WorthQueryAftermathAdmissionDenial::Irreversible)
        }
        WorthQueryOperationReversalContract::ProvisionalDiscard => {
            Err(WorthQueryAftermathAdmissionDenial::ProvisionalDiscardOnly)
        }
        WorthQueryOperationReversalContract::RebuildRequired { .. } => {
            Err(WorthQueryAftermathAdmissionDenial::RebuildRequired)
        }
    }
}

fn require_realized_effect_receipts<D, O, F, L: BasisOperationLane>(
    trace: &WorthQueryCompletedWorkflowTrace<D, O, F, L>,
) -> Result<Vec<String>, WorthQueryAftermathAdmissionDenial> {
    let identities = trace
        .stage_receipts()
        .iter()
        .flat_map(|stage| stage.effect_evidence())
        .map(|effect| effect.receipt_identity().to_owned())
        .collect::<Vec<_>>();
    if identities.is_empty() {
        return Err(WorthQueryAftermathAdmissionDenial::NoExecutedEffects);
    }
    let closure_matches = identities.iter().all(|identity| {
        trace
            .semantic_aspect_dependency_closure()
            .is_some_and(|closure| closure.contains_workflow_effect_receipt(identity))
    });
    if !closure_matches {
        return Err(WorthQueryAftermathAdmissionDenial::DependencyClosureMismatch);
    }
    Ok(identities)
}

fn valid_postcondition(
    kind: WorthQueryAftermathKind,
    postcondition: &WorthQueryAftermathPostcondition,
) -> bool {
    matches!(
        (kind, postcondition),
        (
            WorthQueryAftermathKind::ExactInverse,
            WorthQueryAftermathPostcondition::ExactPriorTruth
        ) | (
            WorthQueryAftermathKind::Compensation,
            WorthQueryAftermathPostcondition::InvariantRestored { .. }
                | WorthQueryAftermathPostcondition::BusinessPostcondition { .. }
        )
    )
}

pub(super) fn mint_validated_aftermath<D, OO, OF, OL, CO, CF, CL>(
    trace: &WorthQueryCompletedWorkflowTrace<D, OO, OF, OL>,
    candidate: WorthQueryBoundDomainOperation<D, CO, CF, CL>,
    validated: ValidatedAftermath,
    counters: WorthQueryAftermathCounters,
) -> WorthQueryAdmittedAftermath<D, OO, OF, OL, CO, CF, CL>
where
    OL: BasisOperationLane,
    CL: BasisOperationLane,
{
    let identity = admitted_aftermath_identity(trace, &candidate, &validated);
    let basis = aftermath_authority_basis(trace, &candidate, &validated);
    let proof = mint_aftermath_authority(
        identity,
        trace.phase_proof().payload().identity().to_owned(),
        basis,
    );
    debug_assert_eq!(proof.payload().predecessor_identity(), trace.identity());
    WorthQueryAdmittedAftermath {
        candidate,
        kind: validated.kind,
        postcondition: validated.postcondition.clone(),
        original_trace_identity: trace.identity().to_owned(),
        counters,
        proof,
        original_evidence: original_aftermath_evidence(trace, &validated),
        _original: PhantomData,
    }
}

fn admitted_aftermath_identity<D, OO, OF, OL, CO, CF, CL>(
    trace: &WorthQueryCompletedWorkflowTrace<D, OO, OF, OL>,
    candidate: &WorthQueryBoundDomainOperation<D, CO, CF, CL>,
    validated: &ValidatedAftermath,
) -> String
where
    OL: BasisOperationLane,
    CL: BasisOperationLane,
{
    hash_parts(&[
        "worth_query_admitted_aftermath_v1".into(),
        format!("original:{}", trace.identity()),
        format!("candidate:{}", candidate.binding_identity()),
        format!("candidate-capability:{}", candidate.capability_identity()),
        format!("effects:{}", validated.effect_receipt_identities.join(",")),
        format!(
            "lineage:{}",
            trace.lineage_report().map_or(
                "none",
                crate::domain_installation::WorthQueryTraceLineageReport::identity
            )
        ),
        aftermath_material(validated.kind, &validated.postcondition),
    ])
}

fn aftermath_authority_basis<D, OO, OF, OL, CO, CF, CL>(
    trace: &WorthQueryCompletedWorkflowTrace<D, OO, OF, OL>,
    candidate: &WorthQueryBoundDomainOperation<D, CO, CF, CL>,
    validated: &ValidatedAftermath,
) -> WorthQueryAftermathAuthorityBasis
where
    OL: BasisOperationLane,
    CL: BasisOperationLane,
{
    WorthQueryAftermathAuthorityBasis {
        runtime_authority: trace
            .bound()
            .operation()
            .domain_authority()
            .runtime_authority()
            .as_u64(),
        installation_generation: trace
            .bound()
            .operation()
            .installation_generation()
            .ordinal(),
        original_operation_identity: trace.bound().definition().canonical_identity().to_owned(),
        original_binding_identity: trace.bound().binding_identity().to_owned(),
        original_capability_identity: trace.bound().capability_identity(),
        original_trace_identity: trace.identity().to_owned(),
        candidate_operation_identity: candidate.definition().canonical_identity().to_owned(),
        candidate_binding_identity: candidate.binding_identity().to_owned(),
        candidate_capability_identity: candidate.capability_identity(),
        basis_identity: candidate.basis().capability_digest().to_owned(),
        effect_receipt_identities: validated.effect_receipt_identities.clone(),
        original_lineage_report_identity: trace
            .lineage_report()
            .map(|report| report.identity().to_owned()),
    }
}

fn original_aftermath_evidence<D, O, F, L: BasisOperationLane>(
    trace: &WorthQueryCompletedWorkflowTrace<D, O, F, L>,
    validated: &ValidatedAftermath,
) -> WorthQueryAftermathOriginalEvidence {
    WorthQueryAftermathOriginalEvidence::new(
        trace.identity().to_owned(),
        validated.kind,
        validated.postcondition.clone(),
        trace
            .stage_receipts()
            .iter()
            .flat_map(|stage| stage.effect_evidence().iter().cloned())
            .collect(),
        trace
            .lineage_report()
            .map(|report| report.identity().to_owned()),
    )
}
