use worth_query::facade::QueryCausalEvidenceReferenceArtifact;

fn main() {
    let _ = QueryCausalEvidenceReferenceArtifact {
        owner: "runtime_bridge".to_string(),
        family: "bridge_route".to_string(),
        reference_identity: "route:Worthd".to_string(),
        binding_digest: "binding:Worthd".to_string(),
        retained_record_digest: None,
        detail_redacted: false,
        reference_digest: "reference:Worthd".to_string(),
    };
}
