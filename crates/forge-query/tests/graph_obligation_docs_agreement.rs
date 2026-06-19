use std::path::Path;

use forge_query::facade::runtime::{
    ForgeQueryGraphObligationExecutionStatus, ForgeQueryGraphObligationKind,
    ForgeQueryGraphObligationSupportLane, ForgeQueryGraphObligationSupportMatrix,
    ForgeQueryGraphObligationSupportStatus,
};

#[path = "graph_obligation_docs_agreement/markdown_links.rs"]
mod markdown_links;

use markdown_links::assert_markdown_links_resolve;

const AI_README: &str = include_str!("../docs/AI_README.md");
const DOCS_README: &str = include_str!("../docs/README.md");
const AUTHORITY_DOC: &str = include_str!("../docs/authoring/graph-touch-obligation-authority.md");
const CONSUMER_KIT_DOC: &str = include_str!("../docs/authoring/graph-obligation-consumer-kit.md");
const GRAPH_COMPOSITION_DOC: &str =
    include_str!("../docs/authoring/graph-composition-authoring.md");
const READ_COMPOSITION_DOC: &str = include_str!("../docs/authoring/read-composition.md");
const SUPPORT_MATRIX_DOC: &str =
    include_str!("../docs/foundations/support-matrix-and-admission.md");
const LIVE_VIEWS_DOC: &str = include_str!("../docs/runtime-surfaces/live-views.md");
const READS_OBSERVE_MATERIALIZE_DOC: &str =
    include_str!("../docs/runtime-surfaces/reads-observe-materialize.md");

const CONSUMER_KIT_JOBS: &[&str] = &[
    "ordinary downstream adoption path",
    "registration",
    "selector coverage",
    "support pinning",
    "in-memory proof",
    "bypass audit",
    "adoption manifests",
    "residue manifests",
];

#[test]
fn graph_obligation_docs_publish_ai_navigation_and_consumer_kit_path() {
    assert_contains_all(
        AI_README,
        &[
            "## Graph Touch Obligation Authority",
            "./authoring/graph-touch-obligation-authority.md",
            "./authoring/graph-obligation-consumer-kit.md",
        ],
    );
    assert_contains_all(
        DOCS_README,
        &[
            "./authoring/graph-touch-obligation-authority.md",
            "./authoring/graph-obligation-consumer-kit.md",
        ],
    );
    assert_contains_all(CONSUMER_KIT_DOC, CONSUMER_KIT_JOBS);
    assert_contains_all(AI_README, CONSUMER_KIT_JOBS);
}

#[test]
fn graph_obligation_docs_share_production_support_and_budget_vocabulary() {
    let expected_kind_labels = ForgeQueryGraphObligationKind::ALL
        .into_iter()
        .map(ForgeQueryGraphObligationKind::as_str)
        .collect::<Vec<_>>();
    let expected_status_labels = ForgeQueryGraphObligationSupportStatus::ALL
        .into_iter()
        .map(ForgeQueryGraphObligationSupportStatus::as_str)
        .collect::<Vec<_>>();
    let authority_matrix =
        ForgeQueryGraphObligationSupportMatrix::milestone_9_9_authority_surface();
    let expected_lane_labels = ForgeQueryGraphObligationSupportLane::MILESTONE_9_9_COVERED
        .into_iter()
        .map(ForgeQueryGraphObligationSupportLane::as_str)
        .collect::<Vec<_>>();
    let expected_budget_terms = budget_terms_from_matrix(&authority_matrix);

    assert_eq!(
        authority_matrix.rows().len(),
        ForgeQueryGraphObligationKind::ALL.len()
            * ForgeQueryGraphObligationSupportLane::MILESTONE_9_9_COVERED.len()
    );
    assert_authority_matrix_covers_all_kinds_and_lanes(&authority_matrix);
    assert_authority_matrix_names_every_support_status(&authority_matrix);

    for doc in [
        AI_README,
        AUTHORITY_DOC,
        CONSUMER_KIT_DOC,
        SUPPORT_MATRIX_DOC,
    ] {
        assert_contains_all(doc, &expected_kind_labels);
        assert_contains_all(doc, &expected_status_labels);
    }
    for doc in [
        AI_README,
        AUTHORITY_DOC,
        CONSUMER_KIT_DOC,
        SUPPORT_MATRIX_DOC,
    ] {
        assert_contains_all(doc, &expected_budget_terms);
    }
    for doc in [
        AI_README,
        AUTHORITY_DOC,
        CONSUMER_KIT_DOC,
        SUPPORT_MATRIX_DOC,
    ] {
        assert_contains_all(
            doc,
            &[ForgeQueryGraphObligationSupportMatrix::MILESTONE_9_9_AUTHORITY_CERTIFICATION_MATRIX_NAME],
        );
        assert_contains_all(doc, &expected_lane_labels);
    }
}

#[test]
fn graph_composition_docs_demote_manual_invariant_packs() {
    assert_contains_all(
        GRAPH_COMPOSITION_DOC,
        &[
            "registered graph obligations are the primary covered path",
            "manual invariant packs are compatibility/custom extension surfaces",
            "not the primary covered graph obligation path",
        ],
    );
    assert_contains_all(
        AI_README,
        &[
            "Manual invariant packs are compatibility/custom extension surfaces",
            "registered graph obligations are the covered path",
        ],
    );
}

#[test]
fn graph_obligation_docs_frame_authority_beyond_dag_traversal_and_keep_read_lanes() {
    assert_contains_all(
        AI_README,
        &[
            "not a DAG traversal helper",
            "A write, read, live view, preview, branch",
            "live read, policy-aware mutation, construction",
            "downstream adoption",
            "Do not reduce this to \"index reads for a DAG.\"",
        ],
    );
    assert_contains_all(
        AUTHORITY_DOC,
        &[
            "Most runtimes can traverse a DAG",
            "Query turns declared graph meaning into",
            "runtime obligations with evidence",
            "This applies to graph-bearing writes and graph-bearing reads",
            "Read-family,",
            "live-read, preview, branch, construction",
            "operator-catalog, and policy-aware",
        ],
    );
    assert_contains_all(
        READ_COMPOSITION_DOC,
        &[
            "Graph Touch Obligation",
            "Authority is the check-selection path",
            "Read composition declares graph-shaped",
            "access; obligation authority decides",
        ],
    );
    assert_contains_all(
        LIVE_VIEWS_DOC,
        &[
            "Graph Touch Obligation",
            "is the covered path for graph obligation meaning",
            "live-read graph obligations",
        ],
    );
    assert_contains_all(
        READS_OBSERVE_MATERIALIZE_DOC,
        &[
            "Graph Touch",
            "Obligation Authority owns the graph obligation posture",
            "graph-bearing read or live-read consumption",
        ],
    );
}

#[test]
fn graph_obligation_docs_do_not_reintroduce_primary_invariant_pack_authority() {
    for (doc_name, doc) in [
        ("AI_README", AI_README),
        ("graph composition", GRAPH_COMPOSITION_DOC),
        ("authority", AUTHORITY_DOC),
        ("consumer kit", CONSUMER_KIT_DOC),
    ] {
        for line in doc.lines().filter(|line| {
            line.contains("manual invariant pack") && line.contains("primary covered")
        }) {
            assert!(
                line.contains("not the primary")
                    || line.contains("treating")
                    || line.contains("presenting")
                    || line.contains("compatibility/custom"),
                "{doc_name} reintroduced primary invariant-pack authority: {line}"
            );
        }
    }
}

#[test]
fn phase_19_markdown_links_resolve() {
    let docs_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("docs");
    for (relative_path, contents) in [
        ("AI_README.md", AI_README),
        ("README.md", DOCS_README),
        (
            "authoring/graph-composition-authoring.md",
            GRAPH_COMPOSITION_DOC,
        ),
        ("authoring/read-composition.md", READ_COMPOSITION_DOC),
        (
            "authoring/graph-touch-obligation-authority.md",
            AUTHORITY_DOC,
        ),
        (
            "authoring/graph-obligation-consumer-kit.md",
            CONSUMER_KIT_DOC,
        ),
        (
            "foundations/support-matrix-and-admission.md",
            SUPPORT_MATRIX_DOC,
        ),
        ("runtime-surfaces/live-views.md", LIVE_VIEWS_DOC),
        (
            "runtime-surfaces/reads-observe-materialize.md",
            READS_OBSERVE_MATERIALIZE_DOC,
        ),
    ] {
        assert_markdown_links_resolve(&docs_root, relative_path, contents);
    }
}

fn assert_contains_all(haystack: &str, needles: &[&str]) {
    for needle in needles {
        assert!(
            haystack.contains(needle),
            "expected docs to contain `{needle}`"
        );
    }
}

fn budget_terms_from_matrix(matrix: &ForgeQueryGraphObligationSupportMatrix) -> Vec<&'static str> {
    let sample = matrix.rows().first().expect("authority matrix row");
    vec![
        "BudgetExceeded",
        ForgeQueryGraphObligationExecutionStatus::BudgetExceeded.as_str(),
        sample.cost_class().as_str(),
        sample.state_load_counter_policy(),
        sample.diagnostic_artifact_policy(),
    ]
}

fn assert_authority_matrix_covers_all_kinds_and_lanes(
    matrix: &ForgeQueryGraphObligationSupportMatrix,
) {
    for kind in ForgeQueryGraphObligationKind::ALL {
        assert_eq!(
            matrix.rows_for_kind(kind).count(),
            ForgeQueryGraphObligationSupportLane::MILESTONE_9_9_COVERED.len(),
            "authority matrix must cover every lane for {kind:?}"
        );
    }
    for lane in ForgeQueryGraphObligationSupportLane::MILESTONE_9_9_COVERED {
        assert_eq!(
            matrix.rows_for_lane(lane).count(),
            ForgeQueryGraphObligationKind::ALL.len(),
            "authority matrix must cover every kind for {lane:?}"
        );
    }
}

fn assert_authority_matrix_names_every_support_status(
    matrix: &ForgeQueryGraphObligationSupportMatrix,
) {
    for status in ForgeQueryGraphObligationSupportStatus::ALL {
        assert!(
            matrix.rows().iter().any(|row| row.status() == status),
            "authority matrix must contain at least one {status:?} row"
        );
    }
}
