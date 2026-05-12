const DOMAIN_READS_DOC: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/domain-reads.md"));
const RUNTIME_SUPPORT_DOC: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/runtime-support.md"
));
const SUBSTRATE_DOC: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../_docs/worth/worth-query-domain-substrate.md"
));
const LIB_RS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));
const FACADE_RS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/facade.rs"));
const PROJECTION_MOD_RS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/projection/mod.rs"
));
const QUERY_ASSEMBLY_MOD_RS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/projection/runtime_boundary/query_assembly/mod.rs"
));

#[test]
fn topology_query_docs_do_not_regress_snapshot_reads_to_deferred_fallback() {
    for doc in [DOMAIN_READS_DOC, RUNTIME_SUPPORT_DOC, SUBSTRATE_DOC] {
        let lower = doc.to_ascii_lowercase();
        assert!(!lower.contains("historical topology read families are deferred"));
        assert!(!lower.contains("historical topology reads are deferred"));
        assert!(!lower.contains("snapshot read-only runtime blocks"));
        assert!(!lower.contains("historical runtime families for now"));
        assert!(!lower.contains("not migrated yet and remains explicit"));
        assert!(!lower.contains("snapshot-index fallback"));
        assert!(!lower.contains("snapshot_index fallback"));
        assert!(!doc.contains("WorthTopologyDomainQuery"));
        assert!(!doc.contains("worth_topology_read_"));
        assert!(!doc.contains("crates/-topo"));
    }

    assert!(RUNTIME_SUPPORT_DOC
        .contains("The snapshot read-only runtime admits those same public topology-domain"));
    assert!(DOMAIN_READS_DOC.contains("execution engine: `query_runtime_historical`"));
    assert!(SUBSTRATE_DOC.contains("including snapshot"));
    assert!(SUBSTRATE_DOC.contains("read-only execution through the historical basis-aware path"));
}

#[test]
fn topology_query_source_comments_do_not_keep_naming_purge_artifacts() {
    for source in [LIB_RS, FACADE_RS, PROJECTION_MOD_RS, QUERY_ASSEMBLY_MOD_RS] {
        assert!(!source.contains("-topo"));
        assert!(!source.contains("for  without"));
        assert!(!source.contains("the -owned"));
    }
}
