const FEATURE_DOC: &str = include_str!("../docs/capabilities/projection-consumption.md");
const DECLARATIVE_DOC: &str =
    include_str!("../docs/capabilities/declarative-query-experience.md");
const AI_ORIENTATION: &str = include_str!("../docs/AI_README.md");
const READ_FACADE: &str = include_str!("../src/facade/exports_read.rs");
const ORDINARY_PROJECTION: &str = include_str!("../src/ordinary/read/projection.rs");
const COMPILED_EXAMPLE: &str = include_str!("declarative_facade_docs.rs");
const CLOSEOUT: &str = include_str!("../../../_docs/WORTH-query/milestone-9.11-closeout.md");

#[test]
fn projection_feature_doc_follows_the_current_usage_shape() {
    let headings = [
        "## What This Feature Is",
        "## Why You Use It",
        "## Stable Entry Points",
        "## Core Mental Model",
        "## How It Executes",
        "## Small Example",
        "## Real Example",
        "## How It Relates To Other Features",
        "## Inspection And Debugging",
        "## Anti-Patterns",
        "## Current Limits",
        "## Related Docs",
    ];
    let positions =
        headings.map(|heading| FEATURE_DOC.find(heading).expect("required docs section"));
    assert!(positions.windows(2).all(|pair| pair[0] < pair[1]));
}

#[test]
fn docs_examples_and_facade_teach_the_same_ordinary_projection_path() {
    for source in [FEATURE_DOC, DECLARATIVE_DOC, AI_ORIENTATION, COMPILED_EXAMPLE] {
        assert!(source.contains("project_facts"));
        assert!(source.contains("consume_projection"));
        assert!(source.contains("into_admitted"));
    }
    for source in [FEATURE_DOC, DECLARATIVE_DOC, AI_ORIENTATION] {
        assert!(source.contains("WorthQueryProjectionOutcome"));
    }
    for symbol in [
        "project_facts",
        "WorthQueryProjectionOutcome",
        "WorthQueryConsumedProjectionAuthority",
    ] {
        assert!(READ_FACADE.contains(symbol), "missing read facade symbol {symbol}");
        assert!(ORDINARY_PROJECTION.contains(symbol), "missing implementation symbol {symbol}");
    }
}

#[test]
fn discovery_docs_do_not_teach_the_displaced_projection_assembly_path() {
    for source in [FEATURE_DOC, DECLARATIVE_DOC, AI_ORIENTATION] {
        for displaced in [
            "ProjectionAuthorityContract::declare",
            "consume_projection_authority",
            "to_terminal_json_document",
            "load_projection_authority_contract_document",
        ] {
            assert!(!source.contains(displaced), "displaced docs path: {displaced}");
        }
    }
}

#[test]
fn closeout_records_authority_deletion_complexity_and_consumer_proof() {
    for required in [
        "WorthQueryConsumedProjectionAuthority",
        "WorthQueryDownstreamAuthorityDeletionReceipt",
        "requirement width",
        "fact width",
        "unrelated Query workspace growth",
        "historical basis growth",
        "downstream consumer graph growth",
    ] {
        assert!(CLOSEOUT.contains(required), "missing closeout proof: {required}");
    }
}

#[test]
fn ordinary_facade_and_docs_exclude_independently_pairable_legacy_types() {
    for legacy in [
        "CompletedProjectionFactConsumption",
        "ProjectionFactConsumptionAttempt",
    ] {
        assert!(!READ_FACADE.contains(legacy), "legacy facade export: {legacy}");
        assert!(!FEATURE_DOC.contains(legacy), "legacy type taught in docs: {legacy}");
        assert!(!AI_ORIENTATION.contains(legacy), "legacy type taught to AI: {legacy}");
    }
}
