use crate::docs_closeout::worth_docs_graph_for_root;

use super::hostile_workspace::WorthDocsTestWorkspace;

#[test]
fn docs_graph_keeps_cross_crate_learning_paths_reachable() {
    let workspace = WorthDocsTestWorkspace::new();
    let graph = worth_docs_graph_for_root(workspace.root()).expect("docs graph should build");

    assert!(graph.has_path(
        "crates/worth-kernel/docs/README.md",
        "crates/worth-kernel/docs/features/primitive-construction.md"
    ));
    assert!(graph.has_path(
        "crates/worth-kernel/docs/features/primitive-construction.md",
        "crates/worth-topo/docs/features/topology-workloads-and-seeds.md"
    ));
    assert!(graph.has_path(
        "crates/worth-spatial/docs/features/construction-time-birth-bindings.md",
        "crates/worth-geom/docs/features/analytic-primitives-and-planes.md"
    ));
}
