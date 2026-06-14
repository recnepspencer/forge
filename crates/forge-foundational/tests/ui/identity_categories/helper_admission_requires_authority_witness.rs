use forge_foundational::{FoundationalIdentityKind, admit_foundational_authority_identity};
use forge_proof::AuthorityMarker;

struct QueryIdentityAuthority(());
impl AuthorityMarker for QueryIdentityAuthority {}

struct QueryCommitIdentityKind;
impl FoundationalIdentityKind for QueryCommitIdentityKind {}

fn main() {
    let _identity = admit_foundational_authority_identity::<
        u64,
        QueryIdentityAuthority,
        QueryCommitIdentityKind,
    >(42, ());
}
