mod support;

use std::any::type_name;

use worth_proof::{
    Admitted, AssumptionBasis, AuthorityWitness, CanonicalOrder, CanonicalVec, CapabilityWitness,
    DisjointPair, Lowered, Proof, Recipe, Resolved, StructuralProofAuthority, UniqueVec,
};
use support::compile_fail::run_compile_fail_bundle;
use support::milestone2;

#[test]
fn sealed_minting_and_witness_authority_certification() {
    let compile_fail_bundle = milestone2::compile_fail_bundle();
    let proof_shape_digest = milestone2::proof_shape_digest();
    let failure_digest = milestone2::failure_digest();
    let codegen_honesty_report = milestone2::codegen_honesty_report();
    let residual_debt_report = milestone2::residual_debt_report();

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
        compile_fail_bundle
            .cases()
            .iter()
            .map(|case| (case.family(), case.path()))
            .collect::<Vec<_>>(),
        vec![
            (
                "sealed_minting",
                "tests/ui/milestone1/stronger_proof_bearing_constructors_are_not_public.rs",
            ),
            (
                "sealed_minting",
                "tests/ui/milestone1/observed_proofs_cannot_be_duplicated.rs",
            ),
            (
                "proof_authority",
                "tests/ui/milestone1/proof_authority_scope_cannot_be_substituted.rs",
            ),
            (
                "proof_authority",
                "tests/ui/milestone1/authority_cannot_mint_unproven_proof_kind.rs",
            ),
            (
                "proof_authority",
                "tests/ui/milestone1/current_basis_rejects_mixed_authority_proof_set.rs",
            ),
            (
                "witness_minting",
                "tests/ui/milestone2/witnesses_are_not_publicly_mintable.rs",
            ),
            (
                "witness_boundaries",
                "tests/ui/milestone2/witness_required_apis_reject_callers_without_witness.rs",
            ),
            (
                "recipe_boundaries",
                "tests/ui/milestone2/recipe_stages_are_not_publicly_skippable.rs",
            ),
        ]
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
            type_name::<AuthorityWitness<milestone2::RepresentativeAuthority>>(),
            type_name::<CapabilityWitness<milestone2::RepresentativeCapability>>(),
            type_name::<Recipe<Resolved, u64, AssumptionBasis<u8>>>(),
            type_name::<Recipe<Lowered, u64, AssumptionBasis<u8>>>(),
            type_name::<Recipe<Admitted, u64, AssumptionBasis<u8>>>(),
        ]
    );

    assert_eq!(
        failure_digest.suite(),
        "sealed_minting_and_witness_authority"
    );
    assert_eq!(
        failure_digest.entries(),
        [
            "sealed_minting::tests/ui/milestone1/stronger_proof_bearing_constructors_are_not_public.rs",
            "sealed_minting::tests/ui/milestone1/observed_proofs_cannot_be_duplicated.rs",
            "proof_authority::tests/ui/milestone1/proof_authority_scope_cannot_be_substituted.rs",
            "proof_authority::tests/ui/milestone1/authority_cannot_mint_unproven_proof_kind.rs",
            "proof_authority::tests/ui/milestone1/current_basis_rejects_mixed_authority_proof_set.rs",
            "witness_minting::tests/ui/milestone2/witnesses_are_not_publicly_mintable.rs",
            "witness_boundaries::tests/ui/milestone2/witness_required_apis_reject_callers_without_witness.rs",
            "recipe_boundaries::tests/ui/milestone2/recipe_stages_are_not_publicly_skippable.rs",
        ]
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
