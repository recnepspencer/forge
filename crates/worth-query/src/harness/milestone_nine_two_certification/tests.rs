use super::{
    MilestoneNineTwoCertificationAdapter, MILESTONE_NINE_TWO_REQUIRED_COMPILE_FAIL_TARGETS,
};
use crate::harness::certification::{
    contains_row, milestone_nine_two_requirements, unmet_required_assertion_classes,
    unmet_required_rows, HostileExpectation, ParityAnchor, RequiredAssertionClass,
};

#[test]
fn milestone_nine_two_certification_adapter_emits_named_matrix() {
    let artifact = MilestoneNineTwoCertificationAdapter::
        subscription_lifecycle_sharing_and_preview_certification_artifact();

    assert_eq!(
        artifact.suite_name,
        "Subscription Lifecycle Sharing And Preview Parity Test"
    );
    assert!(!artifact.certification_bundle_digest.is_empty());
    assert!(!artifact.coverage_matrix_digest.is_empty());
}

#[test]
fn milestone_nine_two_certification_matrix_meets_required_rows() {
    let matrix = MilestoneNineTwoCertificationAdapter::
        subscription_lifecycle_sharing_and_preview_parity_test();
    let requirements = milestone_nine_two_requirements();
    let missing = unmet_required_rows(
        &matrix,
        requirements.required_canonical_rows,
        requirements.required_rejection_rows,
    );

    assert!(
        missing.is_empty(),
        "missing milestone 9.2 certification rows: {missing:?}"
    );
}

#[test]
fn milestone_nine_two_rows_have_required_outputs() {
    let matrix = MilestoneNineTwoCertificationAdapter::
        subscription_lifecycle_sharing_and_preview_parity_test();

    for row in &matrix.rows {
        assert!(row.control_lane.has_required_outputs());
        assert!(row.hostile_lane.has_required_outputs());
        assert!(row.parity_lane.has_required_outputs());
    }
    for row in &matrix.rejection_rows {
        assert!(row.control_lane.has_required_outputs());
        assert!(row.parity_lane.has_required_outputs());
        assert!(!row.hostile_lane.failure_kind.is_empty());
        assert!(!row.hostile_lane.failure_digest.is_empty());
        assert!(!row.hostile_lane.lifecycle_denial_digest.is_empty());
        assert!(!row.hostile_lane.counter_snapshot.is_empty());
    }
}

#[test]
fn milestone_nine_two_rows_enforce_lifecycle_semantics() {
    let matrix = MilestoneNineTwoCertificationAdapter::
        subscription_lifecycle_sharing_and_preview_parity_test();
    let mut covered = Vec::new();

    for row in &matrix.rows {
        let control = row.control_lane.lifecycle_signature();
        let hostile = row.hostile_lane.lifecycle_signature();
        let parity = row.parity_lane.lifecycle_signature();
        match row.hostile_expectation {
            HostileExpectation::EquivalentToControl => {
                assert_eq!(control, hostile, "row '{}'", row.row_name);
                covered.push(RequiredAssertionClass::Equality);
            }
            HostileExpectation::DistinctFromControl => {
                assert_ne!(control, hostile, "row '{}'", row.row_name);
                covered.push(RequiredAssertionClass::Inequality);
            }
        }
        match row.parity_anchor {
            ParityAnchor::Control => assert_eq!(parity, control, "row '{}'", row.row_name),
            ParityAnchor::Hostile => assert_eq!(parity, hostile, "row '{}'", row.row_name),
        }
    }

    for row in &matrix.rejection_rows {
        covered.push(RequiredAssertionClass::TypedFailure);
        if row.hostile_lane.counter_snapshot != row.control_lane.counter_snapshot {
            covered.push(RequiredAssertionClass::ZeroResidue);
        }
    }
    covered.sort();
    covered.dedup();
    let missing = unmet_required_assertion_classes(
        &covered,
        milestone_nine_two_requirements().required_assertion_classes,
    );
    assert!(missing.is_empty(), "missing assertion classes: {missing:?}");
}

#[test]
fn milestone_nine_two_covers_required_named_rows() {
    let matrix = MilestoneNineTwoCertificationAdapter::
        subscription_lifecycle_sharing_and_preview_parity_test();

    for row_name in milestone_nine_two_requirements().required_canonical_rows {
        assert!(contains_row(&matrix, row_name), "missing {row_name}");
    }
    for row_name in milestone_nine_two_requirements().required_rejection_rows {
        assert!(contains_row(&matrix, row_name), "missing {row_name}");
    }
}

#[test]
fn milestone_nine_two_preview_rows_emit_real_preview_evidence() {
    let matrix = MilestoneNineTwoCertificationAdapter::
        subscription_lifecycle_sharing_and_preview_parity_test();

    let discard = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "preview-discard-zero-authoritative-residue")
        .unwrap();
    assert_ne!(discard.control_lane.preview_isolation_digest, "none");
    assert_ne!(discard.control_lane.preview_residue_digest, "none");

    let promotion = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "preview-promotion-boundary-handoff")
        .unwrap();
    assert_ne!(
        promotion.hostile_lane.preview_residue_digest,
        discard.control_lane.preview_residue_digest
    );
}

#[test]
fn milestone_nine_two_sharing_row_proves_shared_lane_and_consumer_local_delivery() {
    let matrix = MilestoneNineTwoCertificationAdapter::
        subscription_lifecycle_sharing_and_preview_parity_test();
    let row = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "equivalent-subscription-sharing-fanout")
        .unwrap();

    for lane in [&row.control_lane, &row.hostile_lane, &row.parity_lane] {
        assert!(
            lane.counter_evidence
                .iter()
                .any(|part| part == "same_lane:true"),
            "sharing row did not prove both consumers joined one lane"
        );
        assert!(
            lane.counter_evidence
                .iter()
                .any(|part| part == "consumer_local_delivery_count:2"),
            "sharing row did not prove both consumers received local delivery evidence"
        );
        assert_ne!(lane.acknowledgement_frontier_digest, "none");
        assert_ne!(lane.delivery_receipt_digest, "none");
    }
}

#[test]
fn milestone_nine_two_performance_receipt_row_is_posture_sensitive() {
    let matrix = MilestoneNineTwoCertificationAdapter::
        subscription_lifecycle_sharing_and_preview_parity_test();
    let row = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "performance-receipt-posture-sensitive")
        .unwrap();

    assert_eq!(
        row.control_lane.query_digest, row.hostile_lane.query_digest,
        "the row must hold functional query meaning constant"
    );
    assert_ne!(
        row.control_lane.subscription_performance_receipt_digest,
        row.hostile_lane.subscription_performance_receipt_digest,
        "performance receipt must change when density/allocation posture changes"
    );
    assert_ne!(
        row.control_lane.active_delivery_density_posture_digest,
        row.hostile_lane.active_delivery_density_posture_digest
    );
    assert_ne!(
        row.control_lane.allocation_posture_digest,
        row.hostile_lane.allocation_posture_digest
    );
}

#[test]
fn milestone_nine_two_scale_row_binds_widths_not_only_row_count() {
    let matrix = MilestoneNineTwoCertificationAdapter::
        subscription_lifecycle_sharing_and_preview_parity_test();
    let row = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "scale-slope-width-bounded-lifecycle")
        .unwrap();

    assert_ne!(
        row.control_lane.subscription_lifecycle_scale_slope_digest,
        row.hostile_lane.subscription_lifecycle_scale_slope_digest
    );
    assert_ne!(
        row.control_lane.patch_group_digest,
        row.hostile_lane.patch_group_digest
    );
    for required_axis in [
        "scale_axis:unrelated_row_count",
        "scale_axis:active_lane_count",
        "scale_axis:consumers_per_lane",
        "scale_axis:group_count",
    ] {
        assert!(
            row.hostile_lane
                .counter_evidence
                .iter()
                .any(|part| part == required_axis),
            "scale row missing axis evidence {required_axis}"
        );
    }
}

#[test]
fn milestone_nine_two_required_compile_fail_targets_are_present() {
    let ui_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/ui");

    for target in MILESTONE_NINE_TWO_REQUIRED_COMPILE_FAIL_TARGETS {
        assert!(
            ui_dir.join(target).exists(),
            "missing compile-fail fixture {target}"
        );
        assert!(
            ui_dir
                .join(target.trim_end_matches(".rs").to_string() + ".stderr")
                .exists(),
            "missing compile-fail stderr for {target}"
        );
    }
}
