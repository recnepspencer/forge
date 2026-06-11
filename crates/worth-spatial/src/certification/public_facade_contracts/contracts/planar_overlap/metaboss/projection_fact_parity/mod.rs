pub(crate) mod catalog;
pub(crate) mod runtime_handles;
pub(crate) mod subject;

use worth_spatial::facade::projection_fact_parity::{
    ProjectionFactParityCase, ProjectionFactParityLane,
};
use worth_spatial::facade::user_response::{
    WorthUserOutcome, WorthUserOutcomeCauseKind, WorthUserOutcomeKind,
};

use self::catalog::ProjectionParityCatalog;
use self::subject::{
    certify_projection_fact_parity, certify_projection_fact_parity_for_catalog,
    denied_parity_outcome, denied_upgrade_outcome, mismatch_outcome,
};

#[test]
fn mb_m6_7_projection_consumed_planar_fact_parity() {
    run_with_real_workload_stack(|| {
        let subject = certify_projection_fact_parity("mb-m6-7-parity");

        assert_eq!(
            subject.receipt.case(),
            ProjectionFactParityCase::AdmittedAcrossAllLanes
        );
        assert_eq!(subject.receipt.counters().lanes_compared(), 9);
        assert_eq!(subject.receipt.counters().receipt_backed_lanes(), 9);
        assert!(!subject.receipt.parity_digest().is_empty());
        assert_outcome(&subject.user_outcome, WorthUserOutcomeKind::Admitted, None);
        assert_human_readable(subject.user_outcome.human_response().summary());

        for (catalog, label) in [
            (ProjectionParityCatalog::Cube, "clean cube planar face set"),
            (
                ProjectionParityCatalog::CoplanarOverlapStorm,
                "coplanar overlap storm subset",
            ),
            (
                ProjectionParityCatalog::ThinFeatureWall,
                "thin-feature wall",
            ),
            (
                ProjectionParityCatalog::RetainedCancellationChain,
                "retained cancellation chain",
            ),
        ] {
            let representative = certify_projection_fact_parity_for_catalog(label, catalog);
            assert_eq!(
                representative.receipt.case(),
                ProjectionFactParityCase::AdmittedAcrossAllLanes
            );
            assert_eq!(representative.receipt.counters().lanes_compared(), 9);
            assert_outcome(
                &representative.user_outcome,
                WorthUserOutcomeKind::Admitted,
                None,
            );
        }
    });
}

#[test]
fn mb_m6_7_denied_paths_remain_denied_across_all_views() {
    run_with_real_workload_stack(|| {
        let denied = denied_parity_outcome("mb-m6-7-denied");
        assert_outcome(&denied, WorthUserOutcomeKind::Admitted, None);

        let upgraded = denied_upgrade_outcome("mb-m6-7-upgraded");
        assert_outcome(
            &upgraded,
            WorthUserOutcomeKind::Denied,
            Some(WorthUserOutcomeCauseKind::DeniedMovementOrRotation),
        );
        assert!(
            upgraded
                .human_response()
                .summary()
                .contains("recovery lane"),
            "denied upgrade must name the upgraded lane: {}",
            upgraded.human_response().summary()
        );
    });
}

#[test]
fn mb_m6_7_parity_outcome_matrix_localizes_each_mismatch_surface() {
    run_with_real_workload_stack(|| {
        let matrix = [
            (
                ProjectionFactParityLane::Projected,
                "projected geometry lane",
            ),
            (
                ProjectionFactParityLane::ProjectionConsumed,
                "projection-consumed fact lane",
            ),
            (ProjectionFactParityLane::Retained, "retained fact lane"),
            (
                ProjectionFactParityLane::Replayed,
                "replayed retained fact lane",
            ),
            (ProjectionFactParityLane::Recovered, "recovery lane"),
            (
                ProjectionFactParityLane::Transformed,
                "transformed geometry lane",
            ),
            (ProjectionFactParityLane::LocalRebuild, "local rebuild lane"),
            (ProjectionFactParityLane::Diagnostics, "diagnostic lane"),
        ];

        for (lane, human_lane) in matrix {
            let outcome = mismatch_outcome("mb-m6-7-matrix", lane);
            assert_outcome(
                &outcome,
                WorthUserOutcomeKind::IntegrityMismatch,
                Some(WorthUserOutcomeCauseKind::IntegrityMismatch),
            );
            assert!(
                outcome.human_response().summary().contains(human_lane),
                "mismatch response must localize {human_lane}: {}",
                outcome.human_response().summary()
            );
            assert_human_readable(outcome.human_response().summary());
        }
    });
}

fn run_with_real_workload_stack(test: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name("projection-fact-parity-mb".to_string())
        .stack_size(16 * 1024 * 1024)
        .spawn(test)
        .expect("projection fact parity test thread")
        .join()
        .expect("projection fact parity test passed");
}

fn assert_outcome(
    outcome: &WorthUserOutcome,
    kind: WorthUserOutcomeKind,
    cause: Option<WorthUserOutcomeCauseKind>,
) {
    assert_eq!(outcome.kind(), kind);
    assert_eq!(outcome.cause().map(|cause| cause.kind()), cause);
}

fn assert_human_readable(message: &str) {
    assert!(
        message.contains(' ') && !message.contains("-parity-") && !message.contains("::"),
        "message must be human-readable, got {message}"
    );
}
