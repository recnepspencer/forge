mod support;

use std::any::type_name;

use support::compile_fail::run_compile_fail_bundle;
use support::sealed_minting;
use worth_proof::{
    Admitted, AssumptionBasis, AuthorityWitness, CanonicalOrder, CanonicalVec, CapabilityWitness,
    DisjointPair, Lowered, Proof, Recipe, Resolved, StructuralProofAuthority, UniqueVec,
};

#[test]
fn sealed_minting_and_witness_authority_certification() {
    sealed_minting::assert_fixture_completeness();
    let compile_fail_bundle = sealed_minting::compile_fail_bundle();
    let proof_shape_digest = sealed_minting::proof_shape_digest();
    let failure_digest = sealed_minting::failure_digest();
    let codegen_honesty_report = sealed_minting::codegen_honesty_report();
    let residual_debt_report = sealed_minting::residual_debt_report();

    run_compile_fail_bundle(&compile_fail_bundle);

    assert_eq!(
        compile_fail_bundle.suite(),
        "sealed_minting_and_witness_authority"
    );
    assert_eq!(
        compile_fail_bundle.families(),
        vec![
            "sealed_minting",
            "proof_authority",
            "witness_minting",
            "witness_boundaries",
            "recipe_boundaries",
        ]
    );
    assert_eq!(
        compile_fail_bundle.cases().len(),
        sealed_minting::CASES.len()
    );

    assert_eq!(
        proof_shape_digest.suite(),
        "sealed_minting_and_witness_authority"
    );
    assert_eq!(
        proof_shape_digest.entries(),
        [
            type_name::<Proof<CanonicalOrder, StructuralProofAuthority>>(),
            type_name::<CanonicalVec<u64>>(),
            type_name::<UniqueVec<u64>>(),
            type_name::<DisjointPair<u64>>(),
            type_name::<AuthorityWitness<sealed_minting::RepresentativeAuthority>>(),
            type_name::<CapabilityWitness<sealed_minting::RepresentativeCapability>>(),
            type_name::<Recipe<Resolved, u64, AssumptionBasis<u8>>>(),
            type_name::<Recipe<Lowered, u64, AssumptionBasis<u8>>>(),
            type_name::<Recipe<Admitted, u64, AssumptionBasis<u8>>>(),
        ]
    );

    assert_eq!(
        failure_digest.suite(),
        "sealed_minting_and_witness_authority"
    );
    assert_eq!(failure_digest.entries().len(), sealed_minting::CASES.len());
    assert_eq!(
        failure_digest.entries(),
        sealed_minting::CASES
            .iter()
            .map(|case| format!("{}::{}", case.family(), case.path()))
            .collect::<Vec<_>>()
    );

    assert_eq!(
        codegen_honesty_report.suite(),
        "sealed_minting_and_witness_authority"
    );
    assert_eq!(
        codegen_honesty_report.verified_scope(),
        "size_layout_and_drop_only"
    );
    assert_eq!(
        codegen_honesty_report
            .checks()
            .iter()
            .map(|check| check.lane())
            .collect::<Vec<_>>(),
        vec![
            "proof",
            "authority_witness",
            "capability_witness",
            "canonical_vec",
            "unique_vec",
            "disjoint_pair",
            "resolved_recipe",
            "lowered_recipe",
            "admitted_recipe",
        ]
    );
    assert!(codegen_honesty_report
        .checks()
        .iter()
        .all(|check| check.matches()));
    assert!(!codegen_honesty_report.hidden_dynamic_lookup());
    assert!(!codegen_honesty_report.hidden_virtual_dispatch());
    assert!(!codegen_honesty_report.mandatory_allocation_introduced());

    assert_eq!(
        residual_debt_report.suite(),
        "sealed_minting_and_witness_authority"
    );
    assert_eq!(
        residual_debt_report
            .items()
            .iter()
            .map(|item| item.category())
            .collect::<Vec<_>>(),
        vec!["trusted_witness_issuers", "codegen_honesty"]
    );
}
