const FEATURE_DOC: &str = include_str!("../docs/capabilities/projection-consumption.md");
const AI_ORIENTATION: &str = include_str!("../docs/AI_README.md");
const RECIPE: &str = include_str!(
    "../docs/domain-capabilities/recipes/carry-query-facts-into-a-downstream-runtime.md"
);
const FACADE_EXPORTS: &str = include_str!("../src/facade/exports_foundation.rs");
const FLUENT_GOLDEN: &str =
    include_str!("ui/projection_consumption/golden/projection_authority_fluent_path_compiles.rs");
const CLOSEOUT: &str = include_str!("../../../_docs/WORTH-query/milestone-9.11-closeout.md");

#[test]
fn docs_lead_from_ordinary_authority_to_execution_in_required_order() {
    let headings = [
        "### Ordinary fluent path",
        "### Contract reference",
        "### Denial and inspection",
        "### Advanced lifecycle",
        "## Core Mental Model",
        "## How It Executes",
    ];
    let positions =
        headings.map(|heading| FEATURE_DOC.find(heading).expect("required docs section"));
    assert!(positions.windows(2).all(|pair| pair[0] < pair[1]));
}

#[test]
fn docs_ai_orientation_and_compile_golden_teach_the_same_facade_path() {
    for source in [FEATURE_DOC, AI_ORIENTATION, RECIPE, FLUENT_GOLDEN] {
        assert!(source.contains("ProjectionAuthorityContract"));
        assert!(source.contains("consume_projection_authority"));
    }
    assert!(FEATURE_DOC.contains("WorthQueryConsumedProjectionAuthority"));
    assert!(FEATURE_DOC.contains("load_projection_authority_contract_document"));
    assert!(FEATURE_DOC.contains("ProjectionAuthorityOutcome::into_admitted()"));
    assert!(AI_ORIENTATION.contains("WorthQueryConsumedProjectionAuthority"));
    assert!(AI_ORIENTATION.contains("to_terminal_json_document()"));
    assert!(FLUENT_GOLDEN.contains("use worth_query::facade::"));
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
        "Worth UI",
    ] {
        assert!(
            CLOSEOUT.contains(required),
            "missing closeout proof: {required}"
        );
    }
}

#[test]
fn curated_facade_and_docs_exclude_independently_pairable_legacy_types() {
    for legacy in [
        "CompletedProjectionFactConsumption",
        "ProjectionFactConsumptionAttempt",
    ] {
        assert!(
            !FACADE_EXPORTS.contains(legacy),
            "legacy facade export: {legacy}"
        );
        assert!(
            !FEATURE_DOC.contains(legacy),
            "legacy type taught in docs: {legacy}"
        );
        assert!(
            !AI_ORIENTATION.contains(legacy),
            "legacy type taught to AI: {legacy}"
        );
    }
}
