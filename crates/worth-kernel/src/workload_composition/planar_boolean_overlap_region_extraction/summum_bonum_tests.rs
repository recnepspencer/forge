use crate::workload_composition::{
    PlanarBooleanOverlapRegionMetabossSubcase, PlanarBooleanOverlapRegionSummumBonumCloseoutInput,
    WorkloadCompositionError,
};
use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapRegionSummumBonumCloseoutDenialKind as DenialKind,
    PlanarBooleanOverlapRegionSummumBonumSubcaseKind as SubcaseKind,
};

use super::test_support::{completed_overlap_owner_seam_fixture, foreign_readiness_binding};

#[test]
fn planar_boolean_overlap_region_metaboss_is_canonical_replayable_area_honest_and_unforgeable() {
    let fixture = completed_overlap_owner_seam_fixture("phase7.5 overlap region summum bonum");
    let closeout = fixture
        .completed
        .certify_planar_boolean_overlap_region_summum_bonum(
            PlanarBooleanOverlapRegionSummumBonumCloseoutInput::new(
                &fixture.readiness,
                &fixture.readiness_consumer,
                fixture.request.readiness_loop_ledger_binding(),
            ),
        )
        .expect("phase-16 closeout should certify from the real owner seam");
    let foreign_binding =
        foreign_readiness_binding("phase7.5 overlap region summum bonum foreign binding");
    let foreign_fixture =
        completed_overlap_owner_seam_fixture("phase7.5 overlap region summum bonum foreign ledger");

    for subcase in PlanarBooleanOverlapRegionMetabossSubcase::all() {
        match subcase {
            PlanarBooleanOverlapRegionMetabossSubcase::BoundaryOnlyCoincidentEdgesDoNotAdmitArea => {
                assert!(closeout.is_canonical(), "{}", subcase.spec_name());
                assert_has_subcase(&closeout, SubcaseKind::BoundaryOnlyCoincidentEdgesDoNotAdmitArea);
            }
            PlanarBooleanOverlapRegionMetabossSubcase::OppositeSenseSameAreaOverlapHasStableWinding => {
                assert_has_subcase(&closeout, SubcaseKind::OppositeSenseSameAreaOverlapHasStableWinding);
            }
            PlanarBooleanOverlapRegionMetabossSubcase::NestedOverlapIslandsPreserveRegionIdentity => {
                assert_has_subcase(&closeout, SubcaseKind::NestedOverlapIslandsPreserveRegionIdentity);
            }
            PlanarBooleanOverlapRegionMetabossSubcase::MixedBoundaryAndAreaContactDoesNotCollapse => {
                assert_has_subcase(&closeout, SubcaseKind::MixedBoundaryAndAreaContactDoesNotCollapse);
            }
            PlanarBooleanOverlapRegionMetabossSubcase::BenignLoopOrderVariationPreservesLedgerDigest => {
                assert_has_subcase(&closeout, SubcaseKind::BenignLoopOrderVariationPreservesLedgerDigest);
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
                assert_has_subcase(&closeout, SubcaseKind::CheckpointReplayPreservesRegionIdentityAndNames);
            }
            PlanarBooleanOverlapRegionMetabossSubcase::OverlapStormUsesIndexNotPairwiseRediscovery => {
                assert_has_subcase(&closeout, SubcaseKind::OverlapStormUsesIndexNotPairwiseRediscovery);
                assert_eq!(
                    closeout.counters().pairwise_rediscovery_attempts(),
                    0,
                    "{}",
                    subcase.spec_name()
                );
            }
        }
    }
}

#[test]
fn summum_bonum_closeout_rejects_foreign_readiness_binding() {
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
}

fn assert_has_subcase(
    closeout: &worth_spatial::facade::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapRegionSummumBonumCloseout,
    kind: SubcaseKind,
) {
    let row = closeout.subcase(kind).expect(kind.spec_name());
    assert!(!row.detail().is_empty(), "{}", kind.spec_name());
}

fn assert_overlap_closeout_denial(
    denial: WorkloadCompositionError,
    kind: DenialKind,
    label: &str,
) {
    let denial = denial
        .overlap_region_summum_bonum_closeout_denial()
        .expect(label);
    assert_eq!(denial.kind(), kind, "{label}");
    assert!(!denial.subcase_name().is_empty(), "{label}");
}
