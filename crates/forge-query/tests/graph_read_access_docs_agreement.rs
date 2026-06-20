use std::path::Path;

use forge_query::facade::runtime::{
    ForgeQueryGraphReadAccessAdmissionPosture, ForgeQueryGraphReadAccessDenialKind,
    ForgeQueryGraphReadAccessRequirementKind, ForgeQueryGraphReadRequiredCapabilityOwner,
};

#[path = "graph_obligation_docs_agreement/markdown_links.rs"]
mod markdown_links;

use markdown_links::assert_markdown_links_resolve;

const AI_README: &str = include_str!("../docs/AI_README.md");
const DOCS_README: &str = include_str!("../docs/README.md");
const READ_COMPOSITION_DOC: &str = include_str!("../docs/authoring/read-composition.md");
const GRAPH_READ_ACCESS_PLANNING_DOC: &str =
    include_str!("../docs/authoring/graph-read-access-planning.md");
const SUPPORT_MATRIX_DOC: &str =
    include_str!("../docs/foundations/support-matrix-and-admission.md");

#[test]
fn graph_read_access_docs_publish_distinct_ai_navigation() {
    assert_contains_all(
        AI_README,
        &[
            "## Graph Read Access Planning",
            "./authoring/graph-read-access-planning.md",
            "The declaration is the authoring surface",
            "The access plan is the accountability surface",
        ],
    );
    assert_contains_all(
        DOCS_README,
        &[
            "./authoring/graph-read-access-planning.md",
            "admitted access postures",
            "no-N+1 receipt proof",
        ],
    );
}

#[test]
fn graph_read_access_docs_share_runtime_vocabulary() {
    let posture_labels = ForgeQueryGraphReadAccessAdmissionPosture::ALL
        .iter()
        .map(ForgeQueryGraphReadAccessAdmissionPosture::as_str)
        .collect::<Vec<_>>();
    let denial_labels = ForgeQueryGraphReadAccessDenialKind::ALL
        .iter()
        .map(ForgeQueryGraphReadAccessDenialKind::as_str)
        .collect::<Vec<_>>();
    let requirement_labels = ForgeQueryGraphReadAccessRequirementKind::all()
        .iter()
        .map(ForgeQueryGraphReadAccessRequirementKind::as_str)
        .collect::<Vec<_>>();
    let owner_labels = ForgeQueryGraphReadRequiredCapabilityOwner::ALL
        .iter()
        .map(ForgeQueryGraphReadRequiredCapabilityOwner::as_str)
        .collect::<Vec<_>>();

    for doc in [
        AI_README,
        GRAPH_READ_ACCESS_PLANNING_DOC,
        SUPPORT_MATRIX_DOC,
    ] {
        assert_contains_all(doc, &posture_labels);
        assert_contains_all(doc, &denial_labels);
        assert_contains_all(doc, &requirement_labels);
        assert_contains_all(doc, &owner_labels);
    }
}

#[test]
fn graph_read_access_docs_lock_authoring_and_accountability_surfaces() {
    for doc in [
        AI_README,
        READ_COMPOSITION_DOC,
        GRAPH_READ_ACCESS_PLANNING_DOC,
        SUPPORT_MATRIX_DOC,
    ] {
        assert_contains_all(
            doc,
            &[
                "The declaration is the authoring surface",
                "The access plan is the accountability surface",
            ],
        );
    }
    assert_contains_all(
        GRAPH_READ_ACCESS_PLANNING_DOC,
        &[
            "authored read declaration",
            "access requirement set",
            "admitted access plan",
            "read receipt plus access-plan consumption counters",
            "per_result_neighbor_lookup_count",
        ],
    );
}

#[test]
fn graph_read_access_docs_show_denial_and_no_magic_requirement_rows() {
    assert_contains_all(
        GRAPH_READ_ACCESS_PLANNING_DOC,
        &[
            "broad boolean predicate",
            "budget_exceeded",
            "async_materialization_required",
            "not a prompt to increase an arbitrary limit and retry",
            "directional_adjacency",
            "predicate_support",
            "traversal_workset",
            "visited_set",
            "dedup_set",
            "result_buffer",
        ],
    );
}

#[test]
fn graph_read_access_docs_do_not_teach_unwrap_or_local_graph_fallbacks() {
    for (doc_name, doc) in [
        ("AI_README", AI_README),
        ("read-composition", READ_COMPOSITION_DOC),
        ("graph-read-access-planning", GRAPH_READ_ACCESS_PLANNING_DOC),
        ("support-matrix", SUPPORT_MATRIX_DOC),
    ] {
        assert!(
            !doc.contains(".unwrap()"),
            "{doc_name} must not teach unwrap-based examples in Phase 18 docs"
        );
        assert!(
            !doc.contains("automatic indexing"),
            "{doc_name} must not describe graph read access as automatic indexing"
        );
        assert!(
            !doc.contains("increase limit and retry"),
            "{doc_name} must not teach broad-read denial as limit bumping"
        );
    }
}

#[test]
fn graph_read_access_markdown_links_resolve() {
    let docs_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("docs");
    assert_markdown_links_resolve(
        &docs_root,
        "authoring/graph-read-access-planning.md",
        GRAPH_READ_ACCESS_PLANNING_DOC,
    );
}

fn assert_contains_all(haystack: &str, needles: &[&str]) {
    let normalized_haystack = normalize_markdown_text(haystack);
    for needle in needles {
        assert!(
            normalized_haystack.contains(&normalize_markdown_text(needle)),
            "expected docs to contain `{needle}`"
        );
    }
}

fn normalize_markdown_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}
