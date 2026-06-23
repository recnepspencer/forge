use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanSplitIdentityEvolutionOutcomeKind, PlanarBooleanSplitIdentityEvolutionRow,
};

fn main() {
    let _ = PlanarBooleanSplitIdentityEvolutionRow {
        source_edge_identity: "source".to_string(),
        query_digest: "query".to_string(),
        basis_digest: "basis".to_string(),
        lineage_digest: "lineage".to_string(),
        result_digest: "result".to_string(),
        outcome_kind: PlanarBooleanSplitIdentityEvolutionOutcomeKind::PluralSplitSuccessors,
        successor_identities: Vec::new(),
    };
}
