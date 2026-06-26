use forge_query::facade::{
    ForgeQueryGraphReadAccessAdmissionPosture, ForgeQueryGraphReadAccessDenialKind,
};
use std::collections::HashSet;

use super::super::cap_ledger::{
    WorthGraphReadAccessPostureCapLedger, WorthGraphReadAccessPostureCapReport,
    WorthGraphReadAccessPostureCapRow,
};
use super::super::errors::WorthGraphReadAccessPostureMatrixErrorKind;
use super::super::posture_resolution::{
    WorthGraphReadAccessResolvedPosture, WorthGraphReadRequirementPostureMap,
};
use super::{production_phase_three_closeout, production_phase_two_closeout};

#[test]
fn posture_cap_growth_requires_ledger_update() {
    let posture_map = WorthGraphReadRequirementPostureMap::from_rows_for_tests(vec![
        WorthGraphReadAccessResolvedPosture::for_tests(
            "requirement-a",
            crate::graph_read_access_plan_adoption::WorthGraphReadAccessPlanAdoptionPostureKind::MissingQueryReadFamilyArtifact,
        ),
    ])
    .expect("test posture map should build");
    let ledger = WorthGraphReadAccessPostureCapLedger::from_rows_for_tests(vec![
        WorthGraphReadAccessPostureCapRow::for_tests("unrelated_family", 1),
    ]);

    let error = WorthGraphReadAccessPostureCapReport::from_posture_map_and_ledger_for_tests(
        &posture_map,
        ledger,
    )
    .expect_err("uncapped posture families must fail");
    assert_eq!(
        WorthGraphReadAccessPostureMatrixErrorKind::UncappedPostureFamily,
        error.kind()
    );
    assert_eq!(
        Some("missing_query_read_family_artifact"),
        error.posture_family()
    );
    assert_eq!(Some(1), error.observed_count());
    assert_eq!(None, error.cap_count());
}

#[test]
fn cap_excess_requires_explicit_cap_change() {
    let posture_map = WorthGraphReadRequirementPostureMap::from_rows_for_tests(vec![
        WorthGraphReadAccessResolvedPosture::for_tests(
            "requirement-a",
            crate::graph_read_access_plan_adoption::WorthGraphReadAccessPlanAdoptionPostureKind::MissingQueryReadFamilyArtifact,
        ),
    ])
    .expect("test posture map should build");
    let ledger = WorthGraphReadAccessPostureCapLedger::from_rows_for_tests(vec![
        WorthGraphReadAccessPostureCapRow::for_tests("missing_query_read_family_artifact", 0),
    ]);

    let error = WorthGraphReadAccessPostureCapReport::from_posture_map_and_ledger_for_tests(
        &posture_map,
        ledger,
    )
    .expect_err("exceeding a cap must fail");
    assert_eq!(
        WorthGraphReadAccessPostureMatrixErrorKind::PostureFamilyCapExceeded,
        error.kind()
    );
    assert_eq!(
        Some("missing_query_read_family_artifact"),
        error.posture_family()
    );
    assert_eq!(Some(1), error.observed_count());
    assert_eq!(Some(0), error.cap_count());
}

#[test]
fn query_posture_and_denial_vocabularies_are_ledger_covered() {
    let ledger = WorthGraphReadAccessPostureCapLedger::current();

    for posture in ForgeQueryGraphReadAccessAdmissionPosture::ALL {
        assert!(
            ledger.covers_query_posture(posture.as_str()),
            "missing cap row for query posture {}",
            posture.as_str()
        );
    }
    for denial in ForgeQueryGraphReadAccessDenialKind::ALL {
        assert!(
            ledger.covers_query_denial_kind(denial.as_str()),
            "missing cap row for query denial {}",
            denial.as_str()
        );
    }
}

#[test]
fn production_cap_rows_are_complete_and_unique() {
    let ledger = WorthGraphReadAccessPostureCapLedger::current();
    let mut families = HashSet::new();
    let mut digests = HashSet::new();

    for row in ledger.rows() {
        assert!(
            families.insert(row.family()),
            "duplicate cap family {}",
            row.family()
        );
        assert!(
            digests.insert(row.row_digest()),
            "duplicate cap digest {}",
            row.row_digest()
        );
        assert!(
            row.max_count() > 0,
            "cap row {} must have positive cap",
            row.family()
        );
        assert!(
            !row.owner().is_empty(),
            "cap row {} must name owner",
            row.family()
        );
        assert!(
            !row.expected_denial().is_empty(),
            "cap row {} must name expected denial",
            row.family()
        );
        assert!(
            !row.suggested_posture().is_empty(),
            "cap row {} must name suggested posture",
            row.family()
        );
        assert!(
            !row.blocker().is_empty(),
            "cap row {} must name blocker",
            row.family()
        );
        assert!(
            !row.removal_trigger().is_empty(),
            "cap row {} must name removal trigger",
            row.family()
        );
    }
}

#[test]
fn matching_posture_and_denial_labels_count_once_per_row() {
    let phase_two = production_phase_two_closeout();
    let phase_three = production_phase_three_closeout();
    let missing_artifact_count = phase_three
        .cap_report()
        .observed_family_counts()
        .iter()
        .find(|row| row.family() == "missing_query_read_family_artifact")
        .expect("missing-artifact family should be observed")
        .observed_count();

    assert_eq!(
        phase_two
            .posture_report()
            .missing_query_read_family_artifact_count(),
        missing_artifact_count
    );
}

#[test]
fn observed_production_families_are_backed_by_cap_rows() {
    let phase_three = production_phase_three_closeout();

    for family_count in phase_three.cap_report().observed_family_counts() {
        let cap_row = phase_three
            .cap_report()
            .ledger()
            .row_for_family(family_count.family())
            .expect("observed family must have a cap row");
        assert_eq!(cap_row.max_count(), family_count.cap_count());
        assert!(family_count.observed_count() <= family_count.cap_count());
    }
}
