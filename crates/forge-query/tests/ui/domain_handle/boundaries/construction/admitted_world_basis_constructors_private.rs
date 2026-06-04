use forge_query::facade::ForgeQueryAdmittedWorldBasis;

fn main() {
    let _ = ForgeQueryAdmittedWorldBasis {
        domain_key: "example.geometry.world-basis",
        display_name: "GeometryWorldBasisDomain",
        operating_context_identity_digest: String::new(),
        handle_identity_digest: String::new(),
        support_snapshot_digest: String::new(),
    };
}
