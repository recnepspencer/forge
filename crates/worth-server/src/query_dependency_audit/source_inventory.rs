use worth_query::facade::consumer_kit::WorthQueryBoundaryAuditSourceSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerQueryBoundarySourceInventory {
    required_roots: Vec<String>,
    source_paths: Vec<String>,
    inventory_digest: String,
    boundary_sources: WorthQueryBoundaryAuditSourceSet,
}

pub(crate) fn worth_server_query_boundary_source_inventory(
) -> WorthServerQueryBoundarySourceInventory {
    let required_roots = vec![
        "crates/worth-server/src/declaration_intake".to_string(),
        "crates/worth-server/src/query_handoff".to_string(),
        "crates/worth-server/src/surfaces/compat_http".to_string(),
    ];
    let source_paths = vec![
        "crates/worth-server/src/declaration_intake/progression.rs".to_string(),
        "crates/worth-server/src/query_handoff/progression.rs".to_string(),
        "crates/worth-server/src/surfaces/compat_http/read_execution/query_execution.rs"
            .to_string(),
        "crates/worth-server/src/surfaces/compat_http/mutation_execution/query_execution.rs"
            .to_string(),
    ];
    let inventory_digest = source_paths.join("|");
    let boundary_sources = WorthQueryBoundaryAuditSourceSet::new("worth-server")
        .source_file(
            "declaration-intake-progression",
            "crates/worth-server/src/declaration_intake/progression.rs",
            include_str!("../declaration_intake/progression.rs"),
        )
        .source_file(
            "query-handoff-progression",
            "crates/worth-server/src/query_handoff/progression.rs",
            include_str!("../query_handoff/progression.rs"),
        )
        .source_file(
            "compat-http-read-execution",
            "crates/worth-server/src/surfaces/compat_http/read_execution/query_execution.rs",
            include_str!("../surfaces/compat_http/read_execution/query_execution.rs"),
        )
        .source_file(
            "compat-http-mutation-execution",
            "crates/worth-server/src/surfaces/compat_http/mutation_execution/query_execution.rs",
            include_str!("../surfaces/compat_http/mutation_execution/query_execution.rs"),
        );

    WorthServerQueryBoundarySourceInventory {
        required_roots,
        source_paths,
        inventory_digest,
        boundary_sources,
    }
}

impl WorthServerQueryBoundarySourceInventory {
    pub fn required_roots(&self) -> &[String] {
        &self.required_roots
    }

    pub fn source_paths(&self) -> &[String] {
        &self.source_paths
    }

    pub fn inventory_digest(&self) -> &str {
        &self.inventory_digest
    }

    pub fn boundary_sources(&self) -> WorthQueryBoundaryAuditSourceSet {
        self.boundary_sources.clone()
    }
}
