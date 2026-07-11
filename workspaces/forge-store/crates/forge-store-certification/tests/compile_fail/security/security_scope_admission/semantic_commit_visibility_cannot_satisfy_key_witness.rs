use forge_store_authority::StoreLowerAuthoritySource;
use forge_store_security::StoreCurrentKeyScopeWitness;

fn require_key_scope_witness(_: StoreCurrentKeyScopeWitness) {}

fn main() {
    require_key_scope_witness(StoreLowerAuthoritySource::SemanticCommitVisibility);
}
