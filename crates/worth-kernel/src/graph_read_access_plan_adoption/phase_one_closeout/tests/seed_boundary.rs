use super::super::current_worth_graph_read_access_plan_adoption_phase_one_closeout;
use crate::graph_read_access_plan_adoption::test_fixtures::production_milestone_eight_seed;
use crate::graph_read_access_plan_adoption::WorthGraphReadAccessPlanAdoptionPhaseOneErrorKind;

#[test]
fn milestone_eight_seed_is_the_only_access_plan_adoption_start() {
    let seed = production_milestone_eight_seed();
    let closeout = current_worth_graph_read_access_plan_adoption_phase_one_closeout(&seed)
        .expect("Milestone 8 seed should admit into plan-adoption Phase 1");

    assert_eq!(
        closeout.milestone_seven_closeout_digest(),
        seed.milestone_seven_closeout_digest()
    );
    assert_eq!(
        closeout.declaration_catalog_digest(),
        seed.declaration_catalog_digest()
    );
    assert!(!closeout.claims_access_plan_admission());
    assert!(!closeout.claims_access_plan_consumption());
    assert!(!closeout.claims_graph_read_execution());
    assert!(!closeout.claims_graph_read_receipts());
    assert!(!closeout.claims_validator_selection());
}

#[test]
fn phase_one_public_entrypoint_accepts_only_structured_milestone_eight_seed() {
    let closeout_source = include_str!("../closeout.rs");
    let seed_admission_source = include_str!("../../seed_admission/admitted_seed.rs");
    let public_sources = [closeout_source, seed_admission_source];

    assert!(closeout_source.contains("seed: &WorthGraphReadAccessDeclarationMilestoneEightSeed"));
    for forbidden in [
        "seed_digest: &str",
        "closeout_digest: &str",
        "read_family_digest: &str",
        "requirement_row_digest: &str",
        "ForgeQueryGraphReadAccessPlanConsumption",
        "ForgeQueryGraphReadReceipt",
        "ForgeQueryGraphReadStreamingReceipt",
    ] {
        for source in public_sources {
            assert!(
                !source.contains(forbidden),
                "Phase 1 must not expose lower-authority adoption starts: {forbidden}"
            );
        }
    }
}

#[test]
fn seed_admission_rejects_execution_shaped_or_incomplete_handoff() {
    let seed = production_milestone_eight_seed();

    for (invalid_seed, expected_kind) in [
        (
            seed.with_graph_read_execution_claim_for_tests(),
            WorthGraphReadAccessPlanAdoptionPhaseOneErrorKind::SeedClaimedGraphReadExecution,
        ),
        (
            seed.with_access_plan_consumption_claim_for_tests(),
            WorthGraphReadAccessPlanAdoptionPhaseOneErrorKind::SeedClaimedAccessPlanConsumption,
        ),
        (
            seed.without_read_family_identities_for_tests(),
            WorthGraphReadAccessPlanAdoptionPhaseOneErrorKind::MissingReadFamilyIdentity,
        ),
        (
            seed.without_requirement_row_evidence_for_tests(),
            WorthGraphReadAccessPlanAdoptionPhaseOneErrorKind::MissingRequirementRowEvidence,
        ),
    ] {
        let error = current_worth_graph_read_access_plan_adoption_phase_one_closeout(&invalid_seed)
            .expect_err("invalid Milestone 8 seed should be rejected before inventory");
        assert_eq!(error.kind(), expected_kind);
    }
}

#[test]
fn phase_one_preserves_structured_milestone_seven_handoff() {
    let seed = production_milestone_eight_seed();
    let closeout = current_worth_graph_read_access_plan_adoption_phase_one_closeout(&seed)
        .expect("Milestone 8 seed should admit");

    assert_eq!(
        closeout.read_family_identities(),
        seed.read_family_identities()
    );
    assert_eq!(
        closeout.requirement_row_evidence(),
        seed.requirement_row_evidence()
    );
    assert_eq!(
        closeout.admission_capability_gaps(),
        seed.admission_capability_gaps()
    );
    assert_eq!(
        closeout.carried_requirement_derivation_gaps(),
        seed.carried_requirement_derivation_gaps()
    );
    assert_eq!(
        closeout.deletion_ledger_report(),
        seed.deletion_ledger_report()
    );
    assert_eq!(
        closeout.capped_residue_report(),
        seed.capped_residue_report()
    );
    assert_eq!(
        closeout.source_firewall_report(),
        seed.source_firewall_report()
    );
}
