use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanSplitNamedArtifactKind, PlanarBooleanSplitPersistentNameRow,
};

fn main() {
    let _ = PlanarBooleanSplitPersistentNameRow {
        row_identity: "forged".to_string(),
        source_edge_identity: "source".to_string(),
        artifact_kind: PlanarBooleanSplitNamedArtifactKind::SplitFragment,
        artifact_identity: "fragment".to_string(),
        persistent_name_identity: "name".to_string(),
        identity_evolution_query_digest: "query".to_string(),
        identity_evolution_result_digest: "result".to_string(),
        identity_evolution_lineage_digest: "lineage".to_string(),
        event_cause_identities: Vec::new(),
        subshape_signature_identity: "signature".to_string(),
    };
}
