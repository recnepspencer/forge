use forge_query::facade::{ForgeQueryAdmittedWorldBasis, ForgeQueryCommitIdentity};

fn main() {
    let evidence = ForgeQueryCommitIdentity::from_relational_commit_id(1).evidence_identity();

    let _ = ForgeQueryAdmittedWorldBasis {
        domain_key: "example.geometry.world-basis",
        display_name: "GeometryWorldBasisDomain",
        operating_context_identity_digest: String::new(),
        handle_identity: evidence.clone(),
        support_snapshot_digest: String::new(),
        basis_lifecycle_support_identity: evidence,
    };
}
