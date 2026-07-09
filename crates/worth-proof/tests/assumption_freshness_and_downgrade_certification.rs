mod support;

use support::compile_fail::run_compile_fail_bundle;
use support::compile_pass::run_compile_pass_bundle;
use support::milestone3;

#[test]
fn assumption_freshness_readmission_and_downgrade_certification() {
    let compile_fail_bundle = milestone3::compile_fail_bundle();
    let compile_pass_bundle = milestone3::compile_pass_bundle();
    let basis_digest = milestone3::basis_digest();
    let failure_digest = milestone3::failure_digest();
    let transition_digest = milestone3::transition_digest();
    let residual_debt_report = milestone3::residual_debt_report();

    run_compile_fail_bundle(&compile_fail_bundle);
    run_compile_pass_bundle(&compile_pass_bundle);

    assert_eq!(
        compile_fail_bundle.suite(),
        "assumption_freshness_readmission_and_downgrade"
    );
    assert_eq!(
        compile_fail_bundle.families(),
        vec![
            "unresolved_misuse",
            "stale_misuse",
            "rebind_misuse",
            "pre_readmission_misuse",
            "basis_drift_misuse",
        ]
    );
    assert_eq!(
        compile_pass_bundle.suite(),
        "assumption_freshness_readmission_and_downgrade"
    );
    assert_eq!(
        compile_pass_bundle.families(),
        vec![
            "control_progression",
            "same_basis_readmission_progression",
            "shifted_basis_readmission_progression",
        ]
    );
    assert_eq!(
        basis_digest.suite(),
        "assumption_freshness_readmission_and_downgrade"
    );
    assert_eq!(
        basis_digest.entries(),
        [
            "worth_proof::assumption::freshness::FreshnessScopedBasis<worth_proof::assumption::freshness::CurrentValidity, worth_proof::assumption::basis::AssumptionBasis<u8>>",
            "worth_proof::assumption::freshness::FreshnessScopedBasis<worth_proof::assumption::freshness::CurrentValidity, worth_proof::assumption::basis::AssumptionBasis<u16>>",
            "worth_proof::assumption::freshness::FreshnessScopedBasis<worth_proof::assumption::freshness::StaleReadable, worth_proof::assumption::basis::AssumptionBasis<u8>>",
            "worth_proof::assumption::freshness::FreshnessScopedBasis<worth_proof::assumption::freshness::RebindRequired, worth_proof::assumption::basis::AssumptionBasis<u8>>",
            "worth_proof::assumption::freshness::FreshnessScopedBasis<worth_proof::assumption::freshness::AuthorityRevalidationRequired, worth_proof::assumption::basis::AssumptionBasis<u8>>",
            "worth_proof::assumption::readmission::BoundaryBridged<worth_proof::assumption::freshness::FreshnessScopedBasis<worth_proof::assumption::freshness::AuthorityRevalidationRequired, worth_proof::assumption::basis::AssumptionBasis<u8>>>",
        ]
    );
    assert_eq!(
        failure_digest.entries(),
        [
            "unresolved_misuse::tests/ui/milestone3/unresolved_recipe_cannot_bridge_trust_boundary.rs",
            "stale_misuse::tests/ui/milestone3/stale_readable_recipe_rejects_strong_basis_api.rs",
            "rebind_misuse::tests/ui/milestone3/rebind_required_recipe_cannot_lower.rs",
            "pre_readmission_misuse::tests/ui/milestone3/boundary_bridged_recipe_rejects_strong_basis_api.rs",
            "basis_drift_misuse::tests/ui/milestone3/shifted_basis_readmission_cannot_be_treated_as_original_basis.rs",
        ]
    );
    assert_eq!(
        transition_digest.suite(),
        "assumption_freshness_readmission_and_downgrade"
    );
    assert_eq!(
        transition_digest.entries(),
        [
            "worth_proof::recipe::stages::Recipe<worth_proof::recipe::stages::Resolved, u64, worth_proof::assumption::freshness::FreshnessScopedBasis<worth_proof::assumption::freshness::CurrentValidity, worth_proof::assumption::basis::AssumptionBasis<u8>>>",
            "worth_proof::recipe::stages::Recipe<worth_proof::recipe::stages::Resolved, u64, worth_proof::assumption::freshness::FreshnessScopedBasis<worth_proof::assumption::freshness::RebindRequired, worth_proof::assumption::basis::AssumptionBasis<u8>>>",
            "worth_proof::recipe::stages::Recipe<worth_proof::recipe::stages::Resolved, u64, worth_proof::assumption::readmission::BoundaryBridged<worth_proof::assumption::freshness::FreshnessScopedBasis<worth_proof::assumption::freshness::RebindRequired, worth_proof::assumption::basis::AssumptionBasis<u8>>>>",
            "worth_proof::recipe::stages::Recipe<worth_proof::recipe::stages::Lowered, u64, worth_proof::assumption::freshness::FreshnessScopedBasis<worth_proof::assumption::freshness::CurrentValidity, worth_proof::assumption::basis::AssumptionBasis<u8>>>",
            "worth_proof::recipe::stages::Recipe<worth_proof::recipe::stages::Lowered, u64, worth_proof::assumption::freshness::FreshnessScopedBasis<worth_proof::assumption::freshness::StaleReadable, worth_proof::assumption::basis::AssumptionBasis<u8>>>",
            "worth_proof::recipe::stages::Recipe<worth_proof::recipe::stages::Lowered, u64, worth_proof::assumption::readmission::BoundaryBridged<worth_proof::assumption::freshness::FreshnessScopedBasis<worth_proof::assumption::freshness::StaleReadable, worth_proof::assumption::basis::AssumptionBasis<u8>>>>",
            "worth_proof::recipe::stages::Recipe<worth_proof::recipe::stages::Admitted, u64, worth_proof::assumption::freshness::FreshnessScopedBasis<worth_proof::assumption::freshness::CurrentValidity, worth_proof::assumption::basis::AssumptionBasis<u8>>>",
            "worth_proof::recipe::stages::Recipe<worth_proof::recipe::stages::Admitted, u64, worth_proof::assumption::freshness::FreshnessScopedBasis<worth_proof::assumption::freshness::CurrentValidity, worth_proof::assumption::basis::AssumptionBasis<u16>>>",
            "worth_proof::recipe::stages::Recipe<worth_proof::recipe::stages::Admitted, u64, worth_proof::assumption::freshness::FreshnessScopedBasis<worth_proof::assumption::freshness::AuthorityRevalidationRequired, worth_proof::assumption::basis::AssumptionBasis<u8>>>",
            "worth_proof::recipe::stages::Recipe<worth_proof::recipe::stages::Admitted, u64, worth_proof::assumption::readmission::BoundaryBridged<worth_proof::assumption::freshness::FreshnessScopedBasis<worth_proof::assumption::freshness::AuthorityRevalidationRequired, worth_proof::assumption::basis::AssumptionBasis<u8>>>>",
        ]
    );
    assert_eq!(
        residual_debt_report
            .items()
            .iter()
            .map(|item| item.category())
            .collect::<Vec<_>>(),
        vec!["representative_domain_basis_catalog"]
    );
}
