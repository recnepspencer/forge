use worth_proof::{AuthorityMarker, AuthorityWitness};

pub struct BridgeTruthAuthority {
    _owner_seal: (),
}

impl AuthorityMarker for BridgeTruthAuthority {}

pub(crate) fn bridge_truth_authority() -> AuthorityWitness<BridgeTruthAuthority> {
    AuthorityWitness::from_authority_marker(BridgeTruthAuthority { _owner_seal: () })
}
