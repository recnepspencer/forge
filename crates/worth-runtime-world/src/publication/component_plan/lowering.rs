use worth_relational::facade::branch::{
    AdmittedRelationalBranchBasis, AdmittedRelationalForkSourceBasis,
};
use worth_relational::facade::mvcc::PreparedRelationalCommitCandidate;
use worth_signal::facade::branch::{AdmittedSignalBranchBasis, ValidatedSignalBranchName};

use super::{LoweredOwnerComponentPlan, RelationalComponentPlan, SignalComponentPlan};
use crate::branch::{ProductBranchComponentPosture, ProductBranchObservation};
use crate::publication::{
    CompositeComponentIntent, NoEffectCause, NoEffectCompositePublication,
    ResolvedExpectedProductHead,
};

pub(crate) fn lower_component_plans(
    mut expected: ResolvedExpectedProductHead,
    intent: CompositeComponentIntent,
) -> Result<LoweredOwnerComponentPlan, NoEffectCompositePublication> {
    if expected.intent().component_intent() != intent {
        return Err(lowering_denied(expected.expected()));
    }

    let (prepared_candidate, fork_source, signal_fork_name) = expected.take_plan_inputs();
    let expected_head = expected.expected().clone();
    let basis = expected_head.basis();
    let postures = expected.intent().component_postures();
    let relational = lower_relational_plan(
        basis.relational_basis().clone(),
        RelationalPlanInput {
            posture: postures.relational(),
            changes: intent.changes_relational(),
            prepared_candidate,
            fork_source,
        },
        &expected_head,
    )?;
    let signal = lower_signal_plan(
        basis.signal_basis().clone(),
        SignalPlanInput {
            posture: postures.signal(),
            changes: intent.changes_signal(),
            branch_name: signal_fork_name,
        },
        &expected_head,
    )?;
    Ok(LoweredOwnerComponentPlan::new(
        expected, intent, relational, signal,
    ))
}

struct RelationalPlanInput {
    posture: ProductBranchComponentPosture,
    changes: bool,
    prepared_candidate: Option<PreparedRelationalCommitCandidate>,
    fork_source: Option<AdmittedRelationalForkSourceBasis>,
}

fn lower_relational_plan(
    expected: AdmittedRelationalBranchBasis,
    input: RelationalPlanInput,
    expected_head: &ProductBranchObservation,
) -> Result<RelationalComponentPlan, NoEffectCompositePublication> {
    match input.posture {
        ProductBranchComponentPosture::ReuseExact => {
            lower_reuse_relational_plan(expected, input, expected_head)
        }
        ProductBranchComponentPosture::ForkExact
        | ProductBranchComponentPosture::ForkAndAdvance => {
            lower_unsupported_relational_fork_plan(expected, input, expected_head)
        }
    }
}

fn lower_reuse_relational_plan(
    expected: AdmittedRelationalBranchBasis,
    input: RelationalPlanInput,
    expected_head: &ProductBranchObservation,
) -> Result<RelationalComponentPlan, NoEffectCompositePublication> {
    if !input.changes {
        return match (input.prepared_candidate, input.fork_source) {
            (None, None) => Ok(RelationalComponentPlan::retain_exact(expected)),
            _ => Err(lowering_denied(expected_head)),
        };
    }
    if input.fork_source.is_some() {
        return Err(lowering_denied(expected_head));
    }
    match input.prepared_candidate {
        Some(_) | None => Err(lowering_denied(expected_head)),
    }
}

fn lower_unsupported_relational_fork_plan(
    expected: AdmittedRelationalBranchBasis,
    input: RelationalPlanInput,
    expected_head: &ProductBranchObservation,
) -> Result<RelationalComponentPlan, NoEffectCompositePublication> {
    if input.prepared_candidate.is_some() {
        return Err(lowering_denied(expected_head));
    }
    let Some(source) = input.fork_source else {
        return Err(lowering_denied(expected_head));
    };
    if !fork_source_matches(&source, &expected) {
        return Err(lowering_denied(expected_head));
    }
    // The frozen Runtime World plan has no distinct ForkExact or post-fork
    // mutation-evidence phase. Do not relabel either posture as a publishable
    // Relational plan until that owner contract exists.
    let _ = (expected, source);
    Err(lowering_denied(expected_head))
}

struct SignalPlanInput {
    posture: ProductBranchComponentPosture,
    changes: bool,
    branch_name: Option<ValidatedSignalBranchName>,
}

fn lower_signal_plan(
    expected: AdmittedSignalBranchBasis,
    input: SignalPlanInput,
    expected_head: &ProductBranchObservation,
) -> Result<SignalComponentPlan, NoEffectCompositePublication> {
    match input.posture {
        ProductBranchComponentPosture::ReuseExact
            if input.changes && input.branch_name.is_none() =>
        {
            Ok(SignalComponentPlan::advance_exact(expected))
        }
        ProductBranchComponentPosture::ReuseExact if input.branch_name.is_none() => {
            Ok(SignalComponentPlan::retain_exact(expected))
        }
        ProductBranchComponentPosture::ReuseExact => Err(lowering_denied(expected_head)),
        ProductBranchComponentPosture::ForkExact
        | ProductBranchComponentPosture::ForkAndAdvance => {
            lower_fork_signal_plan(expected, input, expected_head)
        }
    }
}

fn lower_fork_signal_plan(
    expected: AdmittedSignalBranchBasis,
    input: SignalPlanInput,
    expected_head: &ProductBranchObservation,
) -> Result<SignalComponentPlan, NoEffectCompositePublication> {
    if !valid_fork_route(input.posture, input.changes) {
        return Err(lowering_denied(expected_head));
    }
    let Some(name) = input.branch_name else {
        return Err(lowering_denied(expected_head));
    };
    Ok(SignalComponentPlan::fork_then_advance(expected, name))
}

fn valid_fork_route(posture: ProductBranchComponentPosture, changes: bool) -> bool {
    matches!(
        (posture, changes),
        (ProductBranchComponentPosture::ForkExact, false)
            | (ProductBranchComponentPosture::ForkAndAdvance, true)
    )
}

fn fork_source_matches(
    source: &AdmittedRelationalForkSourceBasis,
    expected: &AdmittedRelationalBranchBasis,
) -> bool {
    let source = source.descriptor();
    let expected = expected.descriptor();
    source.runtime_instance_id() == expected.runtime_instance_id()
        && source.source_branch() == expected.branch_id()
        && source.observation() == expected.reference()
        && source.truth_version() == expected.truth_version()
}

fn lowering_denied(expected: &ProductBranchObservation) -> NoEffectCompositePublication {
    NoEffectCompositePublication::new(NoEffectCause::PreEffectFailure, Some(expected.clone()))
}
