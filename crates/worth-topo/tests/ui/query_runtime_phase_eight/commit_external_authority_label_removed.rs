use forge_query::facade::ForgeQueryCommitIdentity;

fn main() {
    let _commit = ForgeQueryCommitIdentity::from_external_authority_label("commit:test");
}
