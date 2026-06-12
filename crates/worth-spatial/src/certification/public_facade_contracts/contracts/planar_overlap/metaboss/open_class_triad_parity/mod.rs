pub(crate) mod subject;

use worth_spatial::facade::open_class_triad_parity::{
    OpenClassTriadOutcomeKind, OpenTopologyClass,
};
use worth_spatial::facade::projection_fact_parity::ProjectionFactParityLane;
use worth_spatial::facade::user_response::{
    WorthUserOutcomeCauseKind, WorthUserOutcomeKind, WorthUserResponseSource,
    WorthUserResponseWorkload,
};

use subject::{
    closed_storm_digest, cross_class_projection_denial, denied_upgrade_denial, missing_lane_denial,
    open_class_triad_subject, storm_extraction_denial, topology_parity_mismatch_denial,
};

#[test]
fn mb_m6_nmt_3_open_class_triad_parity_compares_nine_receipt_backed_lanes() {
    let subject = open_class_triad_subject("mb-m6-nmt-3-honest");

    assert_eq!(subject.receipt.counters().open_classes_compared(), 3);
    assert_eq!(subject.receipt.counters().lanes_per_class(), 9);
    assert_eq!(subject.receipt.counters().receipt_backed_lanes(), 27);
    assert_eq!(subject.receipt.counters().bounded_conversion_guards(), 3);

    for class in OpenTopologyClass::REQUIRED {
        let lane_set = subject
            .receipt
            .lane_set_for(class)
            .expect("triad class must be present");
        assert_eq!(lane_set.lane_count(), 9);
        assert_eq!(lane_set.receipt_backed_lane_count(), 9);
        assert!(!lane_set.topology_identity().is_empty());
        assert!(!lane_set.parity().parity_digest().is_empty());
    }

    assert_ne!(
        subject
            .receipt
            .lane_set_for(OpenTopologyClass::Wire)
            .unwrap()
            .topology_identity(),
        subject
            .receipt
            .lane_set_for(OpenTopologyClass::Sheet)
            .unwrap()
            .topology_identity()
    );
    assert_eq!(
        subject.outcome_matrix.rows()[0].kind(),
        OpenClassTriadOutcomeKind::Admitted
    );
    for required in OpenClassTriadOutcomeKind::REQUIRED {
        assert!(
            subject
                .outcome_matrix
                .rows()
                .iter()
                .any(|row| row.kind() == required),
            "outcome matrix must branch {required:?}"
        );
    }
    let unique_reasons = subject
        .outcome_matrix
        .rows()
        .iter()
        .map(|row| row.human_reason())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        unique_reasons.len(),
        subject.outcome_matrix.rows().len(),
        "each outcome row must carry a distinct human-readable reason"
    );
    assert_eq!(subject.user_outcome.kind(), WorthUserOutcomeKind::Admitted);
}

#[test]
fn mb_m6_nmt_3_cross_class_checkpoint_and_projection_forgery_denies() {
    let subject = open_class_triad_subject("mb-m6-nmt-3-cross-class");
    let checkpoint_denial = subject
        .receipt
        .attempt_cross_class_checkpoint_replay(OpenTopologyClass::Wire, OpenTopologyClass::Sheet)
        .expect_err("wire retained checkpoint cannot satisfy sheet parity");
    subject
        .receipt
        .attempt_cross_class_checkpoint_replay(OpenTopologyClass::Wire, OpenTopologyClass::Wire)
        .expect("same open class checkpoint is not a cross-class forgery");
    assert_eq!(
        checkpoint_denial.source_class(),
        Some(OpenTopologyClass::Wire)
    );
    assert_eq!(
        checkpoint_denial.target_class(),
        Some(OpenTopologyClass::Sheet)
    );
    assert!(checkpoint_denial.human_reason().contains("open wire"));
    assert!(checkpoint_denial.human_reason().contains("open sheet"));

    let projection_denial = cross_class_projection_denial(&subject);
    assert_eq!(
        projection_denial.lane(),
        Some(ProjectionFactParityLane::ProjectionConsumed)
    );
    assert!(projection_denial
        .human_reason()
        .contains("projection-consumption boundary"));

    let storm_denial = storm_extraction_denial(&subject, &closed_storm_digest());
    assert!(storm_denial
        .human_reason()
        .contains("Closed storm extraction bundle"));
    assert!(storm_denial.human_reason().contains("open sheet"));

    let topology_mismatch = topology_parity_mismatch_denial();
    assert!(topology_mismatch.human_reason().contains("open wire"));
    assert!(topology_mismatch
        .human_reason()
        .contains("topology construction receipt"));
}

#[test]
fn mb_m6_nmt_3_denied_upgrade_and_bounded_conversion_traps_hold() {
    let subject = open_class_triad_subject("mb-m6-nmt-3-upgrade-traps");
    let upgrade = denied_upgrade_denial(&subject, ProjectionFactParityLane::Recovered);
    assert!(upgrade.human_reason().contains("open NMT radial fan"));
    assert!(upgrade.human_reason().contains("recovery lane"));

    let missing = missing_lane_denial(&subject, ProjectionFactParityLane::Replayed);
    assert!(missing.human_reason().contains("no options"));
    assert!(missing
        .human_reason()
        .contains("replayed retained fact lane"));

    let response = WorthUserResponseWorkload::from_source(
        WorthUserResponseSource::from_open_class_triad_parity_denial(&missing),
    )
    .declared("mb-m6-nmt-3 missing lane response")
    .respond()
    .expect("missing lane response");
    assert_eq!(
        response
            .outcome()
            .cause()
            .expect("missing lane response must explain cause")
            .kind(),
        WorthUserOutcomeCauseKind::MissingEvidence
    );
}
