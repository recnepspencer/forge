use forge_store_authority::StoreLowerAuthoritySource;
use forge_store_security::StoreKeyScope;

fn require_key_scope(_: StoreKeyScope) {}

fn main() {
    require_key_scope(StoreLowerAuthoritySource::SemanticCommitVisibility);
}
