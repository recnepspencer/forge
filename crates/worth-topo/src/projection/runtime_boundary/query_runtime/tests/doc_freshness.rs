use super::super::topology_query_runtime_doc_contract;

const DOMAIN_READS_DOC: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/features/domain-reads.md"
));
const RUNTIME_SUPPORT_DOC: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/features/runtime-support.md"
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
const DECLARED_QUERY_SURFACES_MOD_RS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/projection/runtime_boundary/declared_query_surfaces/mod.rs"
));
const WRITE_AUTHORITY_RS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/projection/runtime_boundary/query_runtime/adapters/write_authority.rs"
));
const WRITE_AUTHORITY_COMMAND_LOWERING_RS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/projection/runtime_boundary/query_runtime/adapters/write_authority/command_lowering.rs"
));
const DECLARED_MUTATION_ARTIFACT_RS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/topology_operators/application/declared_mutation_artifact.rs"
));

#[test]
fn topology_query_docs_do_not_regress_snapshot_reads_to_deferred_fallback() {
    let contract = topology_query_runtime_doc_contract();
    for doc in [DOMAIN_READS_DOC, RUNTIME_SUPPORT_DOC, SUBSTRATE_DOC] {
        let lower = doc.to_ascii_lowercase();
        for forbidden in contract.forbidden_legacy_doc_phrases() {
            assert!(!lower.contains(forbidden));
        }
        assert!(!doc.contains("WorthTopologyRead"));
        assert!(!doc.contains("worth_topology_read_"));
        assert!(!doc.contains("crates/-topo"));
    }

    assert!(RUNTIME_SUPPORT_DOC.contains(contract.runtime_support_type_name()));
    assert!(RUNTIME_SUPPORT_DOC.contains(contract.runtime_support_read_family_surface_name()));
    assert!(RUNTIME_SUPPORT_DOC.contains(contract.snapshot_support_phrase()));
    assert!(RUNTIME_SUPPORT_DOC.contains("public topology-domain read"));
    assert!(DOMAIN_READS_DOC.contains(&format!(
        "execution engine: `{}`",
        contract.historical_read_execution_engine()
    )));
    assert!(SUBSTRATE_DOC.contains("including snapshot"));
    assert!(SUBSTRATE_DOC.contains(contract.historical_basis_phrase()));
}

#[test]
fn topology_query_source_comments_do_not_keep_naming_purge_artifacts() {
    let contract = topology_query_runtime_doc_contract();
    for source in [
        LIB_RS,
        FACADE_RS,
        PROJECTION_MOD_RS,
        DECLARED_QUERY_SURFACES_MOD_RS,
    ] {
        for forbidden in contract.forbidden_comment_artifacts() {
            assert!(!source.contains(forbidden));
        }
    }
}

#[test]
fn runtime_mutation_lowering_sources_do_not_regress_to_batch_first_vocabulary() {
    let contract = topology_query_runtime_doc_contract();
    for source in [
        WRITE_AUTHORITY_RS,
        WRITE_AUTHORITY_COMMAND_LOWERING_RS,
        DECLARED_MUTATION_ARTIFACT_RS,
    ] {
        for forbidden in contract.forbidden_batch_first_tokens() {
            assert!(!source.contains(forbidden));
        }
    }
}
