use worth_proof::{
    AuthorityMarker, AuthorityProves, AuthorityWitness, CapabilityMarker, CapabilityWitness, Proof,
    ProofMarker,
};

struct ForgedAuthority;
impl AuthorityMarker for ForgedAuthority {}

struct ForgedCapability;
impl CapabilityMarker for ForgedCapability {}

struct ForgedFact;
impl ProofMarker for ForgedFact {}
impl AuthorityProves<ForgedFact> for ForgedAuthority {}

fn main() {
    let forged_witness = AuthorityWitness::from_authority_marker(ForgedAuthority);
    let _forged_capability =
        CapabilityWitness::from_capability_marker(ForgedCapability);
    let _forged_proof =
        Proof::<ForgedFact, ForgedAuthority>::from_authority_witness(&forged_witness);
}
