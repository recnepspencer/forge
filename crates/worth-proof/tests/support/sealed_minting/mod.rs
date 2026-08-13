mod cases;

use std::any::type_name;
use std::mem::{align_of, needs_drop};

use worth_proof::{
    Admitted, AssumptionBasis, AuthorityMarker, AuthorityWitness, CanonicalOrder, CanonicalVec,
    CapabilityMarker, CapabilityWitness, DisjointPair, Lowered, Proof, Recipe, Resolved,
    StructuralProofAuthority, UniqueVec,
};

use super::proof_shapes::ProofShapeDigest;
use super::type_shapes::{CodegenHonestyReport, CodegenShapeCheck, DebtItem, ResidualDebtReport};

pub const CASES: &[cases::SealedMintingCase] = cases::CASES;

pub fn assert_fixture_completeness() {
    cases::assert_fixture_completeness();
}

pub fn compile_fail_bundle() -> super::compile_fail::CompileFailBundle {
    cases::compile_fail_bundle()
}

pub fn failure_digest() -> super::proof_shapes::FailureDigest {
    cases::failure_digest()
}

pub(crate) struct RepresentativeAuthority;
impl AuthorityMarker for RepresentativeAuthority {}

pub(crate) struct RepresentativeCapability;
impl CapabilityMarker for RepresentativeCapability {}

pub fn proof_shape_digest() -> ProofShapeDigest {
    ProofShapeDigest::new(
        cases::SUITE,
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

pub fn codegen_honesty_report() -> CodegenHonestyReport {
    CodegenHonestyReport::size_layout_and_drop_certified(
        cases::SUITE,
        vec![
            shape::<Proof<CanonicalOrder, StructuralProofAuthority>, ()>("proof"),
            shape::<AuthorityWitness<RepresentativeAuthority>, ()>("authority_witness"),
            shape::<CapabilityWitness<RepresentativeCapability>, ()>("capability_witness"),
            shape::<CanonicalVec<u64>, Vec<u64>>("canonical_vec"),
            shape::<UniqueVec<u64>, Vec<u64>>("unique_vec"),
            shape::<DisjointPair<u64>, (u64, u64)>("disjoint_pair"),
            shape::<Recipe<Resolved, u64, AssumptionBasis<u8>>, (u64, AssumptionBasis<u8>)>(
                "resolved_recipe",
            ),
            shape::<Recipe<Lowered, u64, AssumptionBasis<u8>>, (u64, AssumptionBasis<u8>)>(
                "lowered_recipe",
            ),
            shape::<Recipe<Admitted, u64, AssumptionBasis<u8>>, (u64, AssumptionBasis<u8>)>(
                "admitted_recipe",
            ),
        ],
        "Representative size/layout/drop evidence is scoped to sealed and witness-bearing forms; no handwritten MIR or ASM diff is claimed.",
    )
}

fn shape<Actual, Expected>(lane: &'static str) -> CodegenShapeCheck {
    CodegenShapeCheck::new(
        lane,
        align_of::<Actual>(),
        align_of::<Expected>(),
        needs_drop::<Actual>(),
        needs_drop::<Expected>(),
    )
}

pub fn residual_debt_report() -> ResidualDebtReport {
    ResidualDebtReport::new(
        cases::SUITE,
        vec![
            DebtItem::new(
                "trusted_witness_issuers",
                "Sealed-minting certification hardens crate-owned witness issuance but does not claim cross-crate issuer ergonomics.",
            ),
            DebtItem::new(
                "codegen_honesty",
                "Representative size/layout/drop lanes are certified; handwritten implementation codegen comparison remains outside this suite.",
            ),
        ],
    )
}
