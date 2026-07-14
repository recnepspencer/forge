use worth_proof::{AuthorityMarker, AuthorityWitness};

pub struct RelationalSourceTruthAuthority;

impl AuthorityMarker for RelationalSourceTruthAuthority {}

pub fn relational_source_truth_authority() -> AuthorityWitness<RelationalSourceTruthAuthority> {
    AuthorityWitness::from_authority_marker(RelationalSourceTruthAuthority)
}
