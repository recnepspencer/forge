use worth_store_authority::StoreLowerAuthoritySource;
use worth_store_security::StoreCurrentKeyScopeWitness;

fn require_key_scope_witness(_: StoreCurrentKeyScopeWitness) {}

fn main() {
    require_key_scope_witness(StoreLowerAuthoritySource::SemanticCommitVisibility);
}
