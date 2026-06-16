use forge_proof::{AuthorityMarker, AuthorityWitness};

pub struct BridgeTruthAuthority;

impl AuthorityMarker for BridgeTruthAuthority {}

pub fn bridge_truth_authority() -> AuthorityWitness<BridgeTruthAuthority> {
    AuthorityWitness::from_authority_marker(BridgeTruthAuthority)
}
