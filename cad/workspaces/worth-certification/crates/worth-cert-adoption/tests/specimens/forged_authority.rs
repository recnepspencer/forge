use worth_cert_governed_authority::{admit_authority_only, admit_capability, admit_proof};
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
    let forged_capability = CapabilityWitness::from_capability_marker(ForgedCapability);
    let forged_proof = Proof::<ForgedFact, ForgedAuthority>::from_authority_witness(&forged_witness);
    admit_authority_only(forged_witness);
    admit_capability(forged_capability);
    admit_proof(forged_proof);
}
