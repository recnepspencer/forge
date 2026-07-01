use super::{
    ReplayParityErrorKind, ReplayParityKind, ReplayParityReport, ReplayParitySpatialAdmissionCause,
};
use crate::facade::spatial_compiled_product_family::SpatialCompiledProductConsumer;
use crate::public_api_planar_projection_consumption::contract_subject::projection_consumed_planar_parts;
use crate::public_api_planar_projection_consumption::runtime_handles::projection_consumption_handle;
use crate::workload_platform::compiled_product_admission::{
    admit_spatial_compiled_product_input, SpatialCompiledProductAdmissionRequest,
};
use crate::workload_platform::spatial_compiled_product_consumer_cutover::build_retained_replay_parity_report;
use worth_spatial::facade::planar_projection_consumption::{
    ProjectionConsumedPlanarFacts, ProjectionConsumedPlanarFactsContracts,
};

#[test]
fn replay_parity_is_rerun_stable_for_same_real_receipts() {
    run_with_real_workload_stack(|| {
        let (retained, projected) = retained_and_projected_receipts("phase-2-replay-parity-stable");

        let left = build_retained_replay_parity_report(
            &retained,
            &retained
                .historical_replay(&retained.replay_subject())
                .expect("historical replay"),
            &projected,
        )
        .expect("left replay parity");
        let right = build_retained_replay_parity_report(
            &retained,
            &retained
                .historical_replay(&retained.replay_subject())
                .expect("historical replay"),
            &projected,
        )
        .expect("right replay parity");

        assert_eq!(left.row_count(), 1);
        assert_eq!(
            left.rows()[0].kind(),
            ReplayParityKind::LiveRetainedReplayedProjectionMatch
        );
        assert_eq!(
            left.rows()[0].parity_identity(),
            right.rows()[0].parity_identity()
        );
        assert_eq!(left.admission_provenance(), right.admission_provenance());
        assert_eq!(left.admission_witness(), right.admission_witness());
    });
}

#[test]
fn replay_parity_identity_changes_with_retained_authority_or_projection_locality() {
    run_with_real_workload_stack(|| {
        let (retained, projected) =
            retained_and_projected_receipts("phase-2-replay-parity-baseline");
        let (foreign_retained, _) =
            retained_and_projected_receipts("phase-2-replay-parity-foreign-retained");
        let (_, projection_changed_receipt) = retained_and_projected_receipts_with_projection_world(
            "phase-2-replay-parity-baseline",
            "phase-2-replay-parity-foreign-projection",
        );

        let baseline_report = build_retained_replay_parity_report(
            &retained,
            &retained
                .historical_replay(&retained.replay_subject())
                .expect("historical replay"),
            &projected,
        )
        .expect("baseline replay parity");
        assert_eq!(baseline_report.row_count(), 1);

        let retained_changed = build_retained_replay_parity_report(
            &foreign_retained,
            &foreign_retained
                .historical_replay(&foreign_retained.replay_subject())
                .expect("foreign historical replay"),
            &projected,
        )
        .expect_err("foreign retained authority must deny at replay parity consumer boundary");
        assert_eq!(
            retained_changed.kind(),
            ReplayParityErrorKind::SpatialAdmission
        );
        assert_eq!(
            retained_changed.spatial_admission_cause(),
            Some(ReplayParitySpatialAdmissionCause::WrongAuthorityBasis)
        );

        let projection_changed = build_retained_replay_parity_report(
            &retained,
            &retained
                .historical_replay(&retained.replay_subject())
                .expect("historical replay"),
            &projection_changed_receipt,
        )
        .expect("projection-changed replay parity");
        assert_ne!(
            baseline_report.rows()[0].parity_identity(),
            projection_changed.rows()[0].parity_identity()
        );
        assert_eq!(
            baseline_report
                .admission_provenance()
                .source_authority_digest(),
            projection_changed
                .admission_provenance()
                .source_authority_digest()
        );
        assert_ne!(
            baseline_report
                .admission_provenance()
                .locality_footprint_digest(),
            projection_changed
                .admission_provenance()
                .locality_footprint_digest()
        );
    });
}

#[test]
fn replay_parity_report_carries_admission_derived_provenance() {
    run_with_real_workload_stack(|| {
        let (retained, projected) =
            retained_and_projected_receipts("phase-7-replay-parity-provenance");
        let historical = retained
            .historical_replay(&retained.replay_subject())
            .expect("historical replay");
        let expected = admit_spatial_compiled_product_input(
            &crate::spatial_compiled_product_family::current_spatial_compiled_product_family_catalog(),
            SpatialCompiledProductAdmissionRequest::for_retained_replay(&historical, &retained, &projected),
        )
        .expect("retained replay admission");

        let report = build_retained_replay_parity_report(&retained, &historical, &projected)
            .expect("retained replay parity report");

        assert_eq!(report.admission_witness(), expected.witness());
        assert_eq!(
            report.admission_provenance().source_authority_digest(),
            historical.historical_digest()
        );
        assert_eq!(
            report.admission_witness().consumer(),
            SpatialCompiledProductConsumer::RetainedReplayParity
        );
        assert_eq!(
            report.admission_witness().family_identity(),
            report.selected_family_identity()
        );
        assert_eq!(
            report.admission_provenance().locality_footprint_digest(),
            projected.projection_consumption_digest()
        );
        assert_eq!(
            report
                .admission_provenance()
                .compiled_product_identity_digest(),
            report.rows()[0].parity_identity()
        );
        assert!(!report
            .admission_provenance()
            .evidence_support_digest()
            .trim()
            .is_empty());
    });
}

fn run_with_real_workload_stack(test: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name("retained-replay-parity-tests".to_string())
        .stack_size(16 * 1024 * 1024)
        .spawn(test)
        .expect("retained replay parity test thread")
        .join()
        .expect("retained replay parity test passed");
}

fn retained_and_projected_receipts(
    world: &'static str,
) -> (
    crate::facade::planar_retained_facts::RetainedPlanarFactsReceipt,
    crate::facade::planar_projection_consumption::ProjectionConsumedPlanarFactsReceipt,
) {
    retained_and_projected_receipts_with_projection_world(world, world)
}

fn retained_and_projected_receipts_with_projection_world(
    retained_world: &'static str,
    projection_world: &'static str,
) -> (
    crate::facade::planar_retained_facts::RetainedPlanarFactsReceipt,
    crate::facade::planar_projection_consumption::ProjectionConsumedPlanarFactsReceipt,
) {
    let parts = projection_consumed_planar_parts(retained_world);
    let retained = parts.retained;
    let projected = ProjectionConsumedPlanarFacts::from_retained_planar_facts(retained.clone())
        .consume_bundle_projection_receipts(parts.projections)
        .compile(&ProjectionConsumedPlanarFactsContracts::new(
            projection_consumption_handle(projection_world),
        ))
        .expect("projection-consumed plan")
        .consume()
        .expect("projection-consumed receipt");
    (retained, projected)
}
