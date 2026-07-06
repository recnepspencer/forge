use crate::workload_composition::{
    PlanarBooleanOverlapRegionMetabossSubcase, PlanarBooleanOverlapRegionSummumBonumCloseoutInput,
    WorkloadCompositionError,
};
use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapRegionSummumBonumCloseoutDenialKind as DenialKind,
    PlanarBooleanOverlapRegionSummumBonumSubcaseKind as SubcaseKind,
};

use super::test_support::{
    completed_overlap_owner_seam_fixture, foreign_readiness_binding,
    run_stack_heavy_overlap_region_test,
};

#[test]
fn planar_boolean_overlap_region_metaboss_is_canonical_replayable_area_honest_and_unforgeable() {
    run_stack_heavy_overlap_region_test(|| {
        let fixture = completed_overlap_owner_seam_fixture("phase7.5 overlap region summum bonum");
        let primary_closeout = certify_closeout(fixture);
        let foreign_binding =
            foreign_readiness_binding("phase7.5 overlap region summum bonum foreign binding");
        let foreign_fixture = completed_overlap_owner_seam_fixture(
            "phase7.5 overlap region summum bonum foreign ledger",
        );

        for subcase in PlanarBooleanOverlapRegionMetabossSubcase::all() {
            match subcase {
            PlanarBooleanOverlapRegionMetabossSubcase::BoundaryOnlyCoincidentEdgesDoNotAdmitArea => {
                let boundary_only_closeout = &primary_closeout;
                assert!(boundary_only_closeout.is_canonical(), "{}", subcase.spec_name());
                assert_has_subcase(
                    boundary_only_closeout,
                    SubcaseKind::BoundaryOnlyCoincidentEdgesDoNotAdmitArea,
                );
                assert!(
                    boundary_only_closeout.boundary_only_outcome().row_count() > 0,
                    "{}",
                    subcase.spec_name()
                );
            }
            PlanarBooleanOverlapRegionMetabossSubcase::OppositeSenseSameAreaOverlapHasStableWinding => {
                assert_has_subcase(
                    &primary_closeout,
                    SubcaseKind::OppositeSenseSameAreaOverlapHasStableWinding,
                );
                assert!(
                    primary_closeout.canonical_winding_outcome().stable_region_count() > 0,
                    "{}",
                    subcase.spec_name()
                );
            }
            PlanarBooleanOverlapRegionMetabossSubcase::NestedOverlapIslandsPreserveRegionIdentity => {
                assert_has_subcase(
                    &primary_closeout,
                    SubcaseKind::NestedOverlapIslandsPreserveRegionIdentity,
                );
                assert!(
                    primary_closeout.nested_identity_outcome().nested_region_count() > 0,
                    "{}",
                    subcase.spec_name()
                );
            }
            PlanarBooleanOverlapRegionMetabossSubcase::MixedBoundaryAndAreaContactDoesNotCollapse => {
                let mixed_boundary_area_closeout = &primary_closeout;
                assert_has_subcase(
                    mixed_boundary_area_closeout,
                    SubcaseKind::MixedBoundaryAndAreaContactDoesNotCollapse,
                );
                assert!(
                    mixed_boundary_area_closeout
                        .mixed_boundary_area_outcome()
                        .boundary_only_rows()
                        > 0,
                    "{}",
                    subcase.spec_name()
                );
                assert!(
                    mixed_boundary_area_closeout
                        .mixed_boundary_area_outcome()
                        .area_rows()
                        > 0,
                    "{}",
                    subcase.spec_name()
                );
            }
            PlanarBooleanOverlapRegionMetabossSubcase::BenignLoopOrderVariationPreservesLedgerDigest => {
                assert_has_subcase(
                    &primary_closeout,
                    SubcaseKind::BenignLoopOrderVariationPreservesLedgerDigest,
                );
                assert_eq!(
                    primary_closeout.ordering_parity().canonical_digest(),
                    primary_closeout.ordering_parity().order_invariant_digest(),
                    "{}",
                    subcase.spec_name()
                );
            }
            PlanarBooleanOverlapRegionMetabossSubcase::SyntheticOverlapLedgerIsRejected => {
                let denial = foreign_fixture
                    .completed
                    .certify_planar_boolean_overlap_region_summum_bonum(
                        PlanarBooleanOverlapRegionSummumBonumCloseoutInput::new(
                            &fixture.readiness,
                            &fixture.readiness_consumer,
                            fixture.request.readiness_loop_ledger_binding(),
                        ),
                    )
                    .expect_err("foreign overlap ledger must not pass summum-bonum closeout");
                assert_overlap_closeout_denial(
                    denial,
                    DenialKind::LoopLedgerMismatch,
                    subcase.spec_name(),
                );
            }
            PlanarBooleanOverlapRegionMetabossSubcase::SyntheticReadinessOrMismatchedLoopLedgerIsRejected => {
                let denial = fixture
                    .completed
                    .certify_planar_boolean_overlap_region_summum_bonum(
                        PlanarBooleanOverlapRegionSummumBonumCloseoutInput::new(
                            &fixture.readiness,
                            &fixture.readiness_consumer,
                            &foreign_binding,
                        ),
                    )
                    .expect_err("foreign readiness binding must not pass summum-bonum closeout");
                assert_overlap_closeout_denial(
                    denial,
                    DenialKind::ReadinessBindingMismatch,
                    subcase.spec_name(),
                );
            }
            PlanarBooleanOverlapRegionMetabossSubcase::CheckpointReplayPreservesRegionIdentityAndNames => {
                assert_has_subcase(
                    &primary_closeout,
                    SubcaseKind::CheckpointReplayPreservesRegionIdentityAndNames,
                );
                assert_eq!(
                    primary_closeout.replay_parity().original_outcome_digest(),
                    primary_closeout.replay_parity().replayed_outcome_digest(),
                    "{}",
                    subcase.spec_name()
                );
                assert_eq!(
                    primary_closeout.checkpoint_parity().certified_outcome_digest(),
                    primary_closeout.replay_parity().original_outcome_digest(),
                    "{}",
                    subcase.spec_name()
                );
            }
            PlanarBooleanOverlapRegionMetabossSubcase::OverlapStormUsesIndexNotPairwiseRediscovery => {
                let coplanar_closeout = &primary_closeout;
                assert_has_subcase(
                    coplanar_closeout,
                    SubcaseKind::OverlapStormUsesIndexNotPairwiseRediscovery,
                );
                assert_eq!(
                    coplanar_closeout.storm_witness().pairwise_rediscovery_attempts(),
                    0,
                    "{}",
                    subcase.spec_name()
                );
            }
        }
        }
    });
}

#[test]
fn summum_bonum_closeout_rejects_foreign_readiness_binding() {
    run_stack_heavy_overlap_region_test(|| {
        let fixture =
            completed_overlap_owner_seam_fixture("phase7.5 overlap region summum bonum denial");
        let foreign_binding =
            foreign_readiness_binding("phase7.5 overlap region summum bonum denial foreign");

        let denial = fixture
            .completed
            .certify_planar_boolean_overlap_region_summum_bonum(
                PlanarBooleanOverlapRegionSummumBonumCloseoutInput::new(
                    &fixture.readiness,
                    &fixture.readiness_consumer,
                    &foreign_binding,
                ),
            )
            .expect_err("foreign binding must not certify");

        assert_overlap_closeout_denial(
            denial,
            DenialKind::ReadinessBindingMismatch,
            "foreign_readiness_binding",
        );
    });
}

fn assert_has_subcase(
    closeout: &worth_spatial::facade::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapRegionSummumBonumCloseout,
    kind: SubcaseKind,
) {
    let row = closeout.subcase(kind).expect(kind.spec_name());
    assert!(!row.detail().is_empty(), "{}", kind.spec_name());
}

fn assert_overlap_closeout_denial(denial: WorkloadCompositionError, kind: DenialKind, label: &str) {
    let denial = denial
        .overlap_region_summum_bonum_closeout_denial()
        .expect(label);
    assert_eq!(denial.kind(), kind, "{label}");
    assert!(!denial.subcase_name().is_empty(), "{label}");
}

fn certify_closeout(
    fixture: &super::test_support::RealOverlapOwnerSeamFixture,
) -> worth_spatial::facade::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapRegionSummumBonumCloseout{
    fixture
        .completed
        .certify_planar_boolean_overlap_region_summum_bonum(
            PlanarBooleanOverlapRegionSummumBonumCloseoutInput::new(
                &fixture.readiness,
                &fixture.readiness_consumer,
                fixture.request.readiness_loop_ledger_binding(),
            ),
        )
        .unwrap_or_else(|error| {
            panic!(
                "phase-16 closeout should certify from the real owner seam; stage_counts={:?}; error={:?}",
                fixture.stage_counts, error
            )
        })
}
