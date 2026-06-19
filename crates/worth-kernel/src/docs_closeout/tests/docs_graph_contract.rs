use crate::docs_closeout::{worth_docs_graph_for_root, WorthDocsGraphEdgeKind};

use super::hostile_workspace::WorthDocsTestWorkspace;

#[test]
fn docs_graph_exposes_direct_edges_and_no_unresolved_links_on_clean_workspace() {
    let workspace = WorthDocsTestWorkspace::new();
    let graph = worth_docs_graph_for_root(workspace.root()).expect("docs graph should build");

    assert!(graph.unresolved_links().is_empty());
    assert!(graph.edges().iter().any(|edge| {
        edge.from_path() == "crates/worth-kernel/docs/README.md"
            && edge.to_path() == "crates/worth-kernel/docs/features/primitive-construction.md"
            && edge.kind() == WorthDocsGraphEdgeKind::CrateMap
    }));
    assert!(graph.edges().iter().any(|edge| {
        edge.from_path() == "crates/worth-kernel/docs/features/primitive-construction.md"
            && edge.to_path() == "crates/worth-topo/docs/features/topology-workloads-and-seeds.md"
            && edge.kind() == WorthDocsGraphEdgeKind::RelatedDoc
    }));
}

#[test]
fn docs_graph_reports_unresolved_internal_markdown_links() {
    let workspace = WorthDocsTestWorkspace::new();
    workspace.replace_once(
        "crates/worth-kernel/docs/features/primitive-construction.md",
        "../../../worth-topo/docs/features/topology-workloads-and-seeds.md",
        "../../../worth-topo/docs/features/topology-workloads-and-seeds-typo.md",
    );

    let graph = worth_docs_graph_for_root(workspace.root()).expect("docs graph should build");
    let unresolved = graph
        .unresolved_links()
        .iter()
        .find(|link| {
            link.from_path() == "crates/worth-kernel/docs/features/primitive-construction.md"
        })
        .expect("primitive construction should report unresolved link drift");

    assert_eq!(
        unresolved.raw_target(),
        "../../../worth-topo/docs/features/topology-workloads-and-seeds-typo.md"
    );
    assert_eq!(
        unresolved.attempted_path(),
        "crates/worth-topo/docs/features/topology-workloads-and-seeds-typo.md"
    );
}
