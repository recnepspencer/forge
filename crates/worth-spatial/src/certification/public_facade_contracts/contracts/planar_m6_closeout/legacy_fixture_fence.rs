use std::collections::BTreeSet;

use worth_spatial::facade::planar_m6_closeout::{
    M6LegacyFixtureFencePosture, M6PlanarCloseoutCertification, M6PlanarCloseoutDenialKind,
    M6PlanarCloseoutQueryCertification, M6PremetabossEvidencePosture, M6PremetabossEvidenceRow,
    M6PremetabossFamily, M6QueryBoundaryEvidenceRow,
};
use worth_spatial::facade::workload_inventory::SeedInventoryReport;
use worth_spatial::facade::workload_vocabulary::{
    WorkloadEvidenceLedger, WorkloadEvidenceRow, WorkloadEvidenceStage,
};

use crate::public_api_workload_vocabulary::evidence_ledger_receipts::counter_backed_rows;

use super::fixture::{
    all_legacy_deletion_rows, all_premetaboss_rows, closeout_contracts, legacy_fixture_fence,
    readiness_receipt,
};

#[test]
fn legacy_synthetic_fixture_paths_cannot_register_as_metaboss_closeout() {
    let readiness = readiness_receipt("m6-closeout-synthetic-claim");
    let synthetic_rows = vec![M6PremetabossEvidenceRow::synthetic_end_to_end_claim(
        M6PremetabossFamily::BooleanReadinessFinalBoss,
        "static-coordinate-fixture-claim",
    )];

    let denial = match M6PlanarCloseoutQueryCertification::from_certification(
        M6PlanarCloseoutCertification::from_m7_readiness(readiness.clone())
            .with_premetaboss_evidence(synthetic_rows)
            .with_legacy_deletion_evidence(all_legacy_deletion_rows())
            .with_legacy_fixture_fence(legacy_fixture_fence())
            .with_query_boundary_evidence(M6QueryBoundaryEvidenceRow::from_m7_readiness(
                &readiness,
            )),
    )
    .compile(&closeout_contracts("m6-closeout-synthetic-claim"))
    {
        Ok(_) => panic!("synthetic MB claim must not compile as closeout evidence"),
        Err(denial) => denial,
    };

    assert_eq!(
        denial.kind(),
        M6PlanarCloseoutDenialKind::SyntheticEndToEndBlocked
    );
    assert!(denial
        .reason()
        .contains("cannot register synthetic MB closeout evidence"));
}

#[test]
fn all_metaboss_tests_have_platform_evidence_targets() {
    let rows = all_premetaboss_rows();
    assert_eq!(rows.len(), M6PremetabossFamily::ALL.len());
    let mut digests = BTreeSet::new();

    for family in M6PremetabossFamily::ALL {
        let row = rows
            .iter()
            .find(|row| row.family() == family)
            .unwrap_or_else(|| panic!("missing platform target for {}", family.as_str()));
        assert_ne!(row.evidence_digest(), "");
        assert!(
            digests.insert(row.evidence_digest()),
            "{} reused another MB target digest",
            family.as_str()
        );
        assert_eq!(
            row.posture(),
            M6PremetabossEvidencePosture::WorkloadPlatform,
            "{} must register through the workload-platform posture",
            family.as_str()
        );
        assert!(
            row.source_rows() >= minimum_platform_source_rows(family),
            "{} registered only {} source rows",
            family.as_str(),
            row.source_rows()
        );
        assert!(
            row.human_reason().contains("workload-platform evidence"),
            "{} must explain the platform evidence basis",
            family.as_str()
        );
        assert!(
            row.human_reason().contains(expected_receipt_phrase(family)),
            "{} must be backed by a concrete MB workload receipt, not a generic ledger fixture",
            family.as_str()
        );
    }
}

fn minimum_platform_source_rows(family: M6PremetabossFamily) -> usize {
    match family {
        M6PremetabossFamily::CoplanarOverlapStorm => 10,
        M6PremetabossFamily::HighValencePlanarSingularity => 14,
        M6PremetabossFamily::ThinFeatureScaleSeparation => 11,
        M6PremetabossFamily::RetainedHistoryCancellationChain => 8,
        M6PremetabossFamily::DirtyPlanarInputCleanFail => 6,
        M6PremetabossFamily::UnboundedHalfSpacePosture => 7,
        M6PremetabossFamily::ProjectionConsumedPlanarFactParity => 18,
        M6PremetabossFamily::BooleanReadinessFinalBoss => 30,
    }
}

fn expected_receipt_phrase(family: M6PremetabossFamily) -> &'static str {
    match family {
        M6PremetabossFamily::CoplanarOverlapStorm => "real coplanar overlap storm workload receipt",
        M6PremetabossFamily::HighValencePlanarSingularity => {
            "real high-valence singularity workload receipt"
        }
        M6PremetabossFamily::ThinFeatureScaleSeparation => {
            "real thin-feature scale-separation workload receipt"
        }
        M6PremetabossFamily::RetainedHistoryCancellationChain => {
            "real retained cancellation-chain workload receipt"
        }
        M6PremetabossFamily::DirtyPlanarInputCleanFail => {
            "real dirty planar clean-fail workload receipt"
        }
        M6PremetabossFamily::UnboundedHalfSpacePosture => {
            "real open planar posture workload receipt"
        }
        M6PremetabossFamily::ProjectionConsumedPlanarFactParity => {
            "real projection fact-parity workload receipt"
        }
        M6PremetabossFamily::BooleanReadinessFinalBoss => {
            "real boolean-readiness final-boss workload receipt"
        }
    }
}

#[test]
fn legacy_fixture_fence_classifies_every_inventory_surface() {
    let report = SeedInventoryReport::certify_existing_surfaces()
        .expect("existing inventory should certify");
    let fence = legacy_fixture_fence();

    assert_eq!(fence.rows().len(), report.rows().len());
    assert_eq!(fence.workload_platform_recipes(), 3);
    assert_eq!(fence.unit_only_fixtures(), 10);
    assert_eq!(fence.blocked_synthetic_claims(), 11);

    for row in fence.rows() {
        assert_ne!(row.fence_digest(), "");
        assert_ne!(row.human_reason(), "");
    }
}

#[test]
fn complete_workload_ledger_blocks_manual_label_and_reextraction_claims() {
    for (stage, label) in [
        (
            WorkloadEvidenceStage::Topology,
            "static coordinate topology fixture",
        ),
        (
            WorkloadEvidenceStage::Transform,
            "label-only transform helper",
        ),
        (
            WorkloadEvidenceStage::RetainedReplay,
            "re-extraction replay helper",
        ),
    ] {
        let mut rows = counter_backed_rows("m6-closeout-guard-hostility");
        let stage_index = rows
            .iter()
            .position(|row| row.stage() == stage)
            .expect("stage should exist in platform rows");
        rows[stage_index] = WorkloadEvidenceRow::new(stage, label);

        let denial = WorkloadEvidenceLedger::from_rows(rows)
            .expect("hostile rows should remain inspectable")
            .certify_complete()
            .expect_err("manual hostile stage must not certify as complete");

        assert!(
            denial.human_reason().contains("hand-filled"),
            "{label} should report a human-readable hand-filled evidence reason"
        );
    }
}

#[test]
fn legacy_fixture_fence_blocks_metaboss_and_replay_migration_surfaces() {
    let fence = legacy_fixture_fence();
    for surface in [
        "planar_overlap::metaboss::scenario",
        "planar_overlap::metaboss::certify_storm_with_retained_replay",
        "planar_m6_closeout::fixture",
    ] {
        let row = fence
            .rows()
            .iter()
            .find(|row| row.classification().surface_id().as_str() == surface)
            .unwrap_or_else(|| panic!("missing fence row for {surface}"));
        assert_eq!(
            row.posture(),
            M6LegacyFixtureFencePosture::SyntheticEndToEndBlocked
        );
        assert!(
            row.human_reason()
                .contains("cannot claim MB closeout authority"),
            "{surface} should explain why the fence blocks it"
        );
    }
}
