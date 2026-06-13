use worth_spatial::facade::projection_fact_parity::{
    ProjectionFactParityLane, ProjectionFactParityLaneStatus, ProjectionFactParityReceipt,
    ProjectionFactParityWorkload,
};
use worth_spatial::facade::projection_policy_choice::{
    ProjectionPolicyChoiceMatrix, ProjectionPolicyChoiceReceipt,
};
use worth_spatial::facade::user_response::{
    WorthUserOutcome, WorthUserResponseSource, WorthUserResponseWorkload,
};

use super::catalog::ProjectionParityCatalog;
use super::subject::{admitted_basis, real_parity_parts};

pub(crate) fn policy_required_outcome(
    world: &'static str,
    lane: ProjectionFactParityLane,
) -> WorthUserOutcome {
    policy_required_subject(world, lane).user_outcome
}

pub(crate) struct PolicyRequiredSubject {
    pub(crate) parity: ProjectionFactParityReceipt,
    pub(crate) policy_choice: ProjectionPolicyChoiceReceipt,
    pub(crate) user_outcome: WorthUserOutcome,
}

pub(crate) fn policy_required_subject(
    world: &'static str,
    lane: ProjectionFactParityLane,
) -> PolicyRequiredSubject {
    let parts = real_parity_parts(world, ProjectionParityCatalog::CoplanarOverlapStorm);
    let admitted = ProjectionFactParityWorkload::from_evidence_basis(admitted_basis(&parts))
        .declared(format!("MB-M6-7 admitted parity basis {world}"))
        .compare_lanes()
        .certify()
        .expect("admitted parity basis should certify before policy matrix");
    let basis = admitted_basis(&parts)
        .with_lane_status(lane, ProjectionFactParityLaneStatus::PolicyRequired);
    let denial = ProjectionFactParityWorkload::from_evidence_basis(basis)
        .declared(format!(
            "MB-M6-7 policy-required {} {world}",
            lane.human_name()
        ))
        .compare_lanes()
        .certify()
        .expect_err("policy-required parity lane must stop before comparison");
    let user_outcome = WorthUserResponseWorkload::from_source(
        WorthUserResponseSource::from_projection_fact_parity_denial(&denial),
    )
    .declared("explain projection parity policy requirement")
    .respond()
    .expect("policy-required projection parity response")
    .outcome()
    .clone();
    let policy_choice = ProjectionPolicyChoiceMatrix::from_parity_receipt(&admitted)
        .with_policy_required_outcome(lane, &user_outcome)
        .compile()
        .expect("policy-required outcome should compile to choice receipt");
    PolicyRequiredSubject {
        parity: admitted,
        policy_choice,
        user_outcome,
    }
}

pub(crate) fn policy_required_matrix_subject(
    world: &'static str,
) -> (
    ProjectionFactParityReceipt,
    ProjectionPolicyChoiceReceipt,
    Vec<(ProjectionFactParityLane, WorthUserOutcome)>,
) {
    let parts = real_parity_parts(world, ProjectionParityCatalog::CoplanarOverlapStorm);
    let admitted = ProjectionFactParityWorkload::from_evidence_basis(admitted_basis(&parts))
        .declared(format!("MB-M6-7 admitted policy matrix basis {world}"))
        .compare_lanes()
        .certify()
        .expect("admitted parity basis should certify before full policy matrix");
    let mut matrix = ProjectionPolicyChoiceMatrix::from_parity_receipt(&admitted);
    let outcomes: Vec<_> = ProjectionFactParityLane::REQUIRED
        .into_iter()
        .map(|lane| (lane, policy_required_outcome(world, lane)))
        .collect();
    for (lane, outcome) in &outcomes {
        matrix = matrix.with_policy_required_outcome(*lane, outcome);
    }
    let receipt = matrix
        .compile()
        .expect("all policy-required lanes should compile into one choice matrix");
    (admitted, receipt, outcomes)
}
