use std::any::type_name;
use std::mem::{align_of, needs_drop};

use forge_proof::{
    Admitted, AssumptionBasis, AuthorityMarker, AuthorityWitness, CanonicalOrder, CanonicalVec,
    CapabilityMarker, CapabilityWitness, DisjointPair, Lowered, Proof, Recipe, Resolved,
    StructuralProofAuthority, UniqueVec,
};

use super::compile_fail::{CompileFailBundle, CompileFailCase};
use super::proof_shapes::{FailureDigest, ProofShapeDigest};
use super::type_shapes::{CodegenHonestyReport, CodegenShapeCheck, DebtItem, ResidualDebtReport};

pub(crate) struct RepresentativeAuthority;
impl AuthorityMarker for RepresentativeAuthority {}

pub(crate) struct RepresentativeCapability;
impl CapabilityMarker for RepresentativeCapability {}

pub fn compile_fail_bundle() -> CompileFailBundle {
    CompileFailBundle::new(
        "sealed_minting_and_witness_authority",
        vec![
            CompileFailCase::new(
                "sealed_minting",
                "tests/ui/milestone1/stronger_proof_bearing_constructors_are_not_public.rs",
            ),
            CompileFailCase::new(
                "sealed_minting",
                "tests/ui/milestone1/observed_proofs_cannot_be_duplicated.rs",
            ),
            CompileFailCase::new(
                "proof_authority",
                "tests/ui/milestone1/proof_authority_scope_cannot_be_substituted.rs",
            ),
            CompileFailCase::new(
                "proof_authority",
                "tests/ui/milestone1/authority_cannot_mint_unproven_proof_kind.rs",
            ),
            CompileFailCase::new(
                "proof_authority",
                "tests/ui/milestone1/current_basis_rejects_mixed_authority_proof_set.rs",
            ),
            CompileFailCase::new(
                "witness_minting",
                "tests/ui/milestone2/witnesses_are_not_publicly_mintable.rs",
            ),
            CompileFailCase::new(
                "witness_boundaries",
                "tests/ui/milestone2/witness_required_apis_reject_callers_without_witness.rs",
            ),
            CompileFailCase::new(
                "recipe_boundaries",
                "tests/ui/milestone2/recipe_stages_are_not_publicly_skippable.rs",
            ),
        ],
    )
}

pub fn proof_shape_digest() -> ProofShapeDigest {
    ProofShapeDigest::new(
        "sealed_minting_and_witness_authority",
        vec![
            type_name::<Proof<CanonicalOrder, StructuralProofAuthority>>(),
            type_name::<CanonicalVec<u64>>(),
            type_name::<UniqueVec<u64>>(),
            type_name::<DisjointPair<u64>>(),
            type_name::<AuthorityWitness<RepresentativeAuthority>>(),
            type_name::<CapabilityWitness<RepresentativeCapability>>(),
            type_name::<Recipe<Resolved, u64, AssumptionBasis<u8>>>(),
            type_name::<Recipe<Lowered, u64, AssumptionBasis<u8>>>(),
            type_name::<Recipe<Admitted, u64, AssumptionBasis<u8>>>(),
        ],
    )
}

pub fn failure_digest() -> FailureDigest {
    FailureDigest::new(
        "sealed_minting_and_witness_authority",
        vec![
            "sealed_minting::tests/ui/milestone1/stronger_proof_bearing_constructors_are_not_public.rs",
            "sealed_minting::tests/ui/milestone1/observed_proofs_cannot_be_duplicated.rs",
            "proof_authority::tests/ui/milestone1/proof_authority_scope_cannot_be_substituted.rs",
            "proof_authority::tests/ui/milestone1/authority_cannot_mint_unproven_proof_kind.rs",
            "proof_authority::tests/ui/milestone1/current_basis_rejects_mixed_authority_proof_set.rs",
            "witness_minting::tests/ui/milestone2/witnesses_are_not_publicly_mintable.rs",
            "witness_boundaries::tests/ui/milestone2/witness_required_apis_reject_callers_without_witness.rs",
            "recipe_boundaries::tests/ui/milestone2/recipe_stages_are_not_publicly_skippable.rs",
        ],
    )
}

pub fn codegen_honesty_report() -> CodegenHonestyReport {
    CodegenHonestyReport::size_layout_and_drop_certified(
        "sealed_minting_and_witness_authority",
        vec![
            CodegenShapeCheck::new(
                "proof",
                align_of::<Proof<CanonicalOrder, StructuralProofAuthority>>(),
                align_of::<()>(),
                needs_drop::<Proof<CanonicalOrder, StructuralProofAuthority>>(),
                needs_drop::<()>(),
            ),
            CodegenShapeCheck::new(
                "authority_witness",
                align_of::<AuthorityWitness<RepresentativeAuthority>>(),
                align_of::<()>(),
                needs_drop::<AuthorityWitness<RepresentativeAuthority>>(),
                needs_drop::<()>(),
            ),
            CodegenShapeCheck::new(
                "capability_witness",
                align_of::<CapabilityWitness<RepresentativeCapability>>(),
                align_of::<()>(),
                needs_drop::<CapabilityWitness<RepresentativeCapability>>(),
                needs_drop::<()>(),
            ),
            CodegenShapeCheck::new(
                "canonical_vec",
                align_of::<CanonicalVec<u64>>(),
                align_of::<Vec<u64>>(),
                needs_drop::<CanonicalVec<u64>>(),
                needs_drop::<Vec<u64>>(),
            ),
            CodegenShapeCheck::new(
                "unique_vec",
                align_of::<UniqueVec<u64>>(),
                align_of::<Vec<u64>>(),
                needs_drop::<UniqueVec<u64>>(),
                needs_drop::<Vec<u64>>(),
            ),
            CodegenShapeCheck::new(
                "disjoint_pair",
                align_of::<DisjointPair<u64>>(),
                align_of::<(u64, u64)>(),
                needs_drop::<DisjointPair<u64>>(),
                needs_drop::<(u64, u64)>(),
            ),
            CodegenShapeCheck::new(
                "resolved_recipe",
                align_of::<Recipe<Resolved, u64, AssumptionBasis<u8>>>(),
                align_of::<(u64, AssumptionBasis<u8>)>(),
                needs_drop::<Recipe<Resolved, u64, AssumptionBasis<u8>>>(),
                needs_drop::<(u64, AssumptionBasis<u8>)>(),
            ),
            CodegenShapeCheck::new(
                "lowered_recipe",
                align_of::<Recipe<Lowered, u64, AssumptionBasis<u8>>>(),
                align_of::<(u64, AssumptionBasis<u8>)>(),
                needs_drop::<Recipe<Lowered, u64, AssumptionBasis<u8>>>(),
                needs_drop::<(u64, AssumptionBasis<u8>)>(),
            ),
            CodegenShapeCheck::new(
                "admitted_recipe",
                align_of::<Recipe<Admitted, u64, AssumptionBasis<u8>>>(),
                align_of::<(u64, AssumptionBasis<u8>)>(),
                needs_drop::<Recipe<Admitted, u64, AssumptionBasis<u8>>>(),
                needs_drop::<(u64, AssumptionBasis<u8>)>(),
            ),
        ],
        "Milestone 2 certifies representative size/layout/drop honesty for sealed and witness-bearing forms; no handwritten MIR or ASM diff is shipped yet.",
    )
}

pub fn residual_debt_report() -> ResidualDebtReport {
    ResidualDebtReport::new(
        "sealed_minting_and_witness_authority",
        vec![
            DebtItem::new(
                "trusted_witness_issuers",
                "Milestone 2 hardens witness minting inside crate-owned boundaries, but does not yet ship public cross-crate issuer ergonomics; later milestones may add domain-facing proving facades without weakening the sealed witness posture.",
            ),
            DebtItem::new(
                "codegen_honesty",
                "Representative size/layout/drop lanes are certified, but no handwritten baseline diff against bespoke domain code is shipped yet.",
            ),
        ],
    )
}
