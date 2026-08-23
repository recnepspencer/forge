#[path = "../../worth-query-host/tests/temporal_conditional_operation/adapters.rs"]
mod adapters;
#[path = "../../worth-query-host/tests/temporal_conditional_operation/contract.rs"]
mod contract;
#[path = "../../worth-query-host/tests/temporal_conditional_operation/world.rs"]
mod host_world;
#[path = "../../worth-query-host/tests/temporal_conditional_operation/schema.rs"]
mod schema;

#[path = "granular_invalidation/delivery_convergence.rs"]
mod delivery_convergence;
#[path = "granular_invalidation/financial_runtime_world.rs"]
mod financial_runtime_world;
#[path = "granular_invalidation/lifecycle_certification.rs"]
mod lifecycle_certification;
#[path = "granular_invalidation/necessity_manifest.rs"]
mod necessity_manifest;
#[path = "granular_invalidation/performed_identity_observer.rs"]
mod performed_identity_observer;
#[path = "granular_invalidation/production_evidence.rs"]
mod production_evidence;
#[path = "granular_invalidation/production_scenarios.rs"]
mod production_scenarios;
#[path = "granular_invalidation/query_runtime_world.rs"]
mod query_runtime_world;
#[path = "granular_invalidation/runtime_composition.rs"]
mod runtime_composition;
#[path = "granular_invalidation/scenario_execution.rs"]
mod scenario_execution;
#[path = "granular_invalidation/sealed_run.rs"]
mod sealed_run;
#[path = "granular_invalidation/sealed_run_adversarial.rs"]
mod sealed_run_adversarial;
#[path = "granular_invalidation/shared_lifecycle.rs"]
mod shared_lifecycle;
#[path = "granular_invalidation/structural_slopes.rs"]
mod structural_slopes;
#[path = "granular_invalidation/world.rs"]
mod world;

use sealed_run::GranularInvalidationCertificationRun;
use world::GranularInvalidationScenario;

#[test]
fn six_production_worlds_seal_against_the_independent_manifest() {
    let run =
        GranularInvalidationCertificationRun::seal(sealed_run::production_claims(17)).unwrap();
    assert_eq!(run.case_count(), 6);
    assert_ne!(run.report_digest(), &[0; 32]);
}

#[test]
fn curve_claim_rejects_the_opaque_query_signal_installation() {
    let evidence = financial_runtime_world::run_curve_with_opaque_query_substitution(17);
    let denial = match sealed_run::verify_evidence(
        GranularInvalidationScenario::CurveDetailToLiveRisk,
        17,
        evidence,
    ) {
        Ok(_) => panic!("the curve claim must reject an opaque Query Signal installation"),
        Err(denial) => denial,
    };
    assert_eq!(
        denial,
        "performed Signal identities differ from the independent manifest"
    );
}

#[test]
fn portfolio_claim_rejects_a_forged_relational_record_identity() {
    let (world, evidence) =
        financial_runtime_world::run_portfolio_with_relational_record_substitution(17);
    let expected = necessity_manifest::CrossRuntimeInvalidationNecessityManifest::derive(&world);
    let denial = match sealed_run::verify_production_scenario(world, expected, evidence) {
        Ok(_) => panic!("the portfolio claim must reject a forged Relational record identity"),
        Err(denial) => denial,
    };
    assert_eq!(
        denial,
        "performed relational identities differ from the independent manifest"
    );
}

#[test]
fn implemented_structural_slopes_run_through_the_real_composition_root() {
    structural_slopes::assert_measured_bridge_and_result_slopes();
}

#[test]
fn primary_runtime_carries_real_direct_truth_and_performed_signal_evidence() {
    runtime_composition::assert_primary_runtime_composition();
}

#[test]
fn duplicate_and_reordered_deliveries_converge_before_query_maintenance() {
    delivery_convergence::assert_duplicate_and_reordered_convergence();
}

#[test]
fn primary_runtime_rejects_a_foreign_primary_source_adapter() {
    query_runtime_world::assert_foreign_primary_source_is_denied_at_build();
}

#[test]
fn primary_runtime_rejects_a_read_after_the_admitted_head_advances() {
    runtime_composition::assert_head_advance_denies_stale_granular_read();
}

#[test]
fn primary_runtime_stamps_granular_receipts_from_the_execution_basis() {
    runtime_composition::assert_granular_receipt_uses_execution_snapshot_basis();
}

#[test]
fn financial_curve_host_emits_real_granular_signal_delivery() {
    financial_runtime_world::assert_financial_host_curve_delivery();
}

#[test]
fn financial_curve_detail_performs_query_owned_risk_patch() {
    financial_runtime_world::assert_financial_curve_query_patch();
}

#[test]
fn financial_curve_detail_does_no_work_for_sibling_record_consumer() {
    financial_runtime_world::assert_sibling_curve_record_does_no_query_work();
}

#[test]
fn financial_quote_tolerance_suppresses_then_publishes_query_patch() {
    financial_runtime_world::assert_suppressed_quote_has_no_query_patch();
}

#[test]
fn production_curve_and_quote_evidence_bind_real_owner_work() {
    let curve = financial_runtime_world::run_curve_certification(17);
    let quote = financial_runtime_world::run_quote_certification(17);
    let portfolio = financial_runtime_world::run_portfolio_certification(17);
    let shared = financial_runtime_world::run_shared_certification(17);
    let restored = lifecycle_certification::run_correspondence_certification(17);
    let opaque = financial_runtime_world::run_opaque_certification(17);
    assert_eq!(
        curve.scenario(),
        GranularInvalidationScenario::CurveDetailToLiveRisk
    );
    assert_eq!(
        quote.scenario(),
        GranularInvalidationScenario::SuppressedQuoteNoQueryPatch
    );
    assert_eq!(
        portfolio.scenario(),
        GranularInvalidationScenario::OrderedPortfolioMembership
    );
    assert_eq!(
        shared.scenario(),
        GranularInvalidationScenario::SharedLeaseDisclosureNoninterference
    );
    assert_eq!(
        restored.scenario(),
        GranularInvalidationScenario::CorrespondenceRebindRestore
    );
    assert_eq!(
        opaque.scenario(),
        GranularInvalidationScenario::OpaqueRegionPlatformTwin
    );
}

#[test]
fn ordered_portfolio_preserves_all_granular_query_consequences() {
    financial_runtime_world::assert_ordered_portfolio_membership();
}

#[test]
fn shared_primary_consumers_execute_once_and_publish_per_lease() {
    financial_runtime_world::assert_shared_financial_execution_and_publication();
}

#[test]
fn shared_primary_disclosure_is_revalidated_after_selection() {
    financial_runtime_world::assert_shared_financial_disclosure_revalidation();
}

#[test]
fn correspondence_restore_requires_current_rebinding_at_every_owner() {
    shared_lifecycle::assert_correspondence_rebind_restore();
}

#[test]
fn seal_rejects_missing_duplicate_and_wrong_scenario_evidence() {
    financial_runtime_world::assert_mixed_runtime_evidence_denied();
    sealed_run_adversarial::assert_adversarial_sealing_denials();
}
