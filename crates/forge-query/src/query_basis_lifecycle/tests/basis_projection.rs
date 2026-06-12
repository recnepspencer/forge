use super::binding_support::{observation_runtime, MAIN_BRANCH, OTHER_BRANCH};
use super::{
    admit_observation_basis, admit_replay_basis, evaluate_basis_eligibility, normalize_raw_basis,
    readmit_bridge_truth_view_evidence, BasisCapabilityAdmission, BasisIntentDenialKind,
    BasisOperationLaneRequest, DeniedBasisCapabilityKind, RawBasisIntent,
    RawFutureBasisNeighborFamily,
};
use crate::runtime::tests::support::stateful_bridge_task_runtime;
use crate::runtime::{
    ForgeQueryAuthorityLane, ForgeQueryRuntimeStateKind, ForgeQueryRuntimeStateTarget,
};
use forge_runtime_bridge::facade::{BridgeTruthViewEvaluationRequest, TruthBranchIdentity};

#[test]
fn family_specific_basis_capabilities_project_into_their_public_authority_lanes() {
    let runtime = stateful_bridge_task_runtime();
    let current = admit_observation_basis(
        evaluate_basis_eligibility(
            normalize_raw_basis(RawBasisIntent::current_head(
                BasisOperationLaneRequest::Observation,
            ))
            .expect("current-head observation should normalize"),
        )
        .expect("current-head observation should be eligible"),
    )
    .expect("current-head observation should admit");
    let branch = admit_observation_basis(
        evaluate_basis_eligibility(
            normalize_raw_basis(RawBasisIntent::branch_head(
                super::test_branch_identity(MAIN_BRANCH),
                BasisOperationLaneRequest::Observation,
            ))
            .expect("branch-head observation should normalize"),
        )
        .expect("branch-head observation should be eligible"),
    )
    .expect("branch-head observation should admit");
    let branch_snapshot = admit_observation_basis(
        evaluate_basis_eligibility(
            normalize_raw_basis(RawBasisIntent::branch_snapshot(
                super::test_branch_identity(MAIN_BRANCH),
                super::test_snapshot_identity("snapshot:1"),
                BasisOperationLaneRequest::Observation,
            ))
            .expect("branch-snapshot observation should normalize"),
        )
        .expect("branch-snapshot observation should be eligible"),
    )
    .expect("branch-snapshot observation should admit");
    let historical = admit_observation_basis(
        evaluate_basis_eligibility(
            normalize_raw_basis(RawBasisIntent::historical_snapshot(
                super::test_snapshot_identity("history:snapshot-1"),
                BasisOperationLaneRequest::Observation,
            ))
            .expect("historical observation should normalize"),
        )
        .expect("historical observation should be eligible"),
    )
    .expect("historical observation should admit");

    let current_state = (&current)
        .into_state_snapshot(&runtime)
        .expect("current-head state should project");
    let branch_state = (&branch)
        .into_state_snapshot(&runtime)
        .expect("branch-head state should project");
    let branch_snapshot_state = (&branch_snapshot)
        .into_state_snapshot(&runtime)
        .expect("branch-snapshot state should project");
    let historical_state = (&historical)
        .into_state_snapshot(&runtime)
        .expect("historical state should project");

    assert_eq!(current_state.kind(), ForgeQueryRuntimeStateKind::Ready);
    assert_eq!(
        current_state.authority_lane(),
        ForgeQueryAuthorityLane::AuthoritativeTruth
    );
    assert_eq!(branch_state.kind(), ForgeQueryRuntimeStateKind::Ready);
    assert_eq!(
        branch_state.authority_lane(),
        ForgeQueryAuthorityLane::BranchLocalTruth
    );
    assert_eq!(
        branch_snapshot_state.kind(),
        ForgeQueryRuntimeStateKind::Ready
    );
    assert_eq!(
        branch_snapshot_state.authority_lane(),
        ForgeQueryAuthorityLane::BranchLocalTruth
    );
    assert_eq!(historical_state.kind(), ForgeQueryRuntimeStateKind::Ready);
    assert_eq!(
        historical_state.authority_lane(),
        ForgeQueryAuthorityLane::BridgeExternalState
    );

    match current.admission() {
        BasisCapabilityAdmission::Admitted(admitted) => {
            assert_eq!(
                current_state.basis_digest(),
                admitted.normalized_basis_intent_digest()
            );
        }
        other => panic!("unexpected current admission: {other:?}"),
    }
    match branch.admission() {
        BasisCapabilityAdmission::Admitted(admitted) => {
            assert_eq!(
                branch_state.basis_digest(),
                admitted.normalized_basis_intent_digest()
            );
        }
        other => panic!("unexpected branch admission: {other:?}"),
    }
}

#[test]
fn preview_replay_and_restart_postures_project_without_raw_id_side_channels() {
    let runtime = stateful_bridge_task_runtime();
    let preview = admit_observation_basis(
        evaluate_basis_eligibility(
            normalize_raw_basis(RawBasisIntent::preview(
                super::test_preview_identity("preview:session-1"),
                BasisOperationLaneRequest::Observation,
            ))
            .expect("preview observation should normalize"),
        )
        .expect("preview observation should remain eligible"),
    )
    .expect("preview observation should admit as advisory");
    let replay = admit_replay_basis(
        evaluate_basis_eligibility(
            normalize_raw_basis(RawBasisIntent::historical_snapshot(
                super::test_snapshot_identity("history:snapshot-1"),
                BasisOperationLaneRequest::Replay,
            ))
            .expect("historical replay should normalize"),
        )
        .expect("historical replay should be eligible"),
    )
    .expect("historical replay should admit");
    let restart_denial = normalize_raw_basis(RawBasisIntent::future_neighbor(
        RawFutureBasisNeighborFamily::RestartStableEnvelope,
        BasisOperationLaneRequest::Replay,
    ))
    .expect_err("restart-stable envelope should deny during normalization");
    let async_denial = normalize_raw_basis(RawBasisIntent::future_neighbor(
        RawFutureBasisNeighborFamily::AsyncResource,
        BasisOperationLaneRequest::Observation,
    ))
    .expect_err("async-resource future neighbor should deny during normalization");

    let preview_state = (&preview)
        .into_state_snapshot(&runtime)
        .expect("preview state should project");
    let replay_state = (&replay)
        .into_state_snapshot(&runtime)
        .expect("replay state should project");
    let restart_state = (&restart_denial)
        .into_state_snapshot(&runtime)
        .expect("restart denial state should project");
    let async_state = (&async_denial)
        .into_state_snapshot(&runtime)
        .expect("async denial state should project");

    assert_eq!(preview_state.kind(), ForgeQueryRuntimeStateKind::Pending);
    assert_eq!(
        preview_state.authority_lane(),
        ForgeQueryAuthorityLane::PreviewTruth
    );
    assert_eq!(replay_state.kind(), ForgeQueryRuntimeStateKind::Ready);
    assert_eq!(
        replay_state.authority_lane(),
        ForgeQueryAuthorityLane::BridgeExternalState
    );
    assert_eq!(
        restart_state.kind(),
        ForgeQueryRuntimeStateKind::Unsupported
    );
    assert_eq!(
        restart_state.authority_lane(),
        ForgeQueryAuthorityLane::BridgeExternalState
    );
    assert_eq!(async_state.kind(), ForgeQueryRuntimeStateKind::Unsupported);
    assert_eq!(
        async_state.authority_lane(),
        ForgeQueryAuthorityLane::AsyncResourceState
    );

    match restart_denial.kind() {
        BasisIntentDenialKind::UnsupportedFutureNeighbor { family, owner } => {
            assert_eq!(family, &RawFutureBasisNeighborFamily::RestartStableEnvelope);
            assert_eq!(owner, &"forge_store");
        }
        other => panic!("unexpected restart denial kind: {other:?}"),
    }
    match async_denial.kind() {
        BasisIntentDenialKind::UnsupportedFutureNeighbor { family, owner } => {
            assert_eq!(family, &RawFutureBasisNeighborFamily::AsyncResource);
            assert_eq!(owner, &"forge_signal");
        }
        other => panic!("unexpected async denial kind: {other:?}"),
    }
}

#[test]
fn lower_runtime_bound_and_cross_branch_denied_basis_states_preserve_typed_posture() {
    let state_runtime = stateful_bridge_task_runtime();
    let bridge_runtime = observation_runtime();
    let evaluation = bridge_runtime
        .evaluate(BridgeTruthViewEvaluationRequest::for_branch_head(
            TruthBranchIdentity::from_bridge_harness_label(MAIN_BRANCH),
        ))
        .expect("branch-head truth view should evaluate");
    let matching = branch_head_observation(MAIN_BRANCH);
    let mismatched = branch_head_observation(OTHER_BRANCH);

    let bound = readmit_bridge_truth_view_evidence(matching, &evaluation)
        .expect("matching branch-head evidence should bind");
    let denial = readmit_bridge_truth_view_evidence(mismatched, &evaluation)
        .expect_err("cross-branch evidence should deny");

    let bound_state = (&bound)
        .into_state_snapshot(&state_runtime)
        .expect("bound observation state should project");
    let denial_state = (&denial)
        .into_state_snapshot(&state_runtime)
        .expect("denied observation state should project");

    assert_eq!(bound_state.kind(), ForgeQueryRuntimeStateKind::Ready);
    assert_eq!(
        bound_state.authority_lane(),
        ForgeQueryAuthorityLane::BranchLocalTruth
    );
    assert_eq!(bound_state.result_shape_digest(), bound.binding_digest());
    assert_eq!(denial_state.kind(), ForgeQueryRuntimeStateKind::Denied);
    assert_eq!(
        denial_state.authority_lane(),
        ForgeQueryAuthorityLane::BranchLocalTruth
    );

    match denial.kind() {
        DeniedBasisCapabilityKind::LowerRuntimeBindingMismatch {
            authority,
            expected,
            observed,
        } => {
            assert_eq!(authority, &"forge_runtime_bridge");
            assert_eq!(
                expected,
                &format!(
                    "branch_head:{}",
                    super::test_branch_identity(OTHER_BRANCH).as_str()
                )
            );
            assert_eq!(
                observed,
                &format!(
                    "branch_head:{}",
                    super::test_branch_identity(MAIN_BRANCH).as_str()
                )
            );
        }
        other => panic!("unexpected denial kind: {other:?}"),
    }
}

#[test]
fn historical_basis_denials_preserve_historical_authority_lane() {
    let state_runtime = stateful_bridge_task_runtime();
    let bridge_runtime = observation_runtime();
    let evaluation = bridge_runtime
        .evaluate(BridgeTruthViewEvaluationRequest::for_branch_head(
            TruthBranchIdentity::from_bridge_harness_label(MAIN_BRANCH),
        ))
        .expect("branch-head truth view should evaluate");
    let historical = admit_observation_basis(
        evaluate_basis_eligibility(
            normalize_raw_basis(RawBasisIntent::historical_snapshot(
                super::test_snapshot_identity("history:snapshot-1"),
                BasisOperationLaneRequest::Observation,
            ))
            .expect("historical observation should normalize"),
        )
        .expect("historical observation should be eligible"),
    )
    .expect("historical observation should admit");

    let denial = readmit_bridge_truth_view_evidence(historical, &evaluation)
        .expect_err("historical basis should deny against branch-head evidence");
    let denial_state = (&denial)
        .into_state_snapshot(&state_runtime)
        .expect("historical denial state should project");

    assert_eq!(denial_state.kind(), ForgeQueryRuntimeStateKind::Denied);
    assert_eq!(
        denial_state.authority_lane(),
        ForgeQueryAuthorityLane::BridgeExternalState
    );
}

fn branch_head_observation(branch_identity: &str) -> super::ObservationBasisCapability {
    admit_observation_basis(
        evaluate_basis_eligibility(
            normalize_raw_basis(RawBasisIntent::branch_head(
                super::test_branch_identity(branch_identity),
                BasisOperationLaneRequest::Observation,
            ))
            .expect("branch-head observation should normalize"),
        )
        .expect("branch-head observation should be eligible"),
    )
    .expect("branch-head observation should admit")
}
