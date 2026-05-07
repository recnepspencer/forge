use forge_proof::{
    Artifact, AssumptionBasis, CanonicalOrder, CanonicalVec, DisjointPair, Disjointness,
    PhaseMarker, Proof, UniqueVec, Uniqueness,
};

struct RawPhase;
impl PhaseMarker for RawPhase {}

fn main() {
    let proof = Proof::<CanonicalOrder>::mint();

    let _artifact = Artifact::<RawPhase, _, _, _>::with_state(
        vec![1_u8, 2, 3],
        proof,
        AssumptionBasis::new(7_u8),
    );

    let _canonical = CanonicalVec::new(vec![1_u8, 2, 3], Proof::<CanonicalOrder>::mint());
    let _unique = UniqueVec::new(vec![1_u8, 2, 3], Proof::<Uniqueness>::mint());
    let _disjoint = DisjointPair::new(1_u8, 2_u8, Proof::<Disjointness>::mint());
}
