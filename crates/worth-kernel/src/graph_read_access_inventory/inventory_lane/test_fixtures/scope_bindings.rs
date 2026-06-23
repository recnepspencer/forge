use super::super::{WorthGraphReadAccessScopeBinding, WorthGraphReadAccessScopeFamily};

pub(super) fn deleted_source_scope(source_path: &str) -> WorthGraphReadAccessScopeBinding {
    WorthGraphReadAccessScopeBinding::deleted_graph_read_source(source_path, "adoption-a").unwrap()
}

pub(super) fn declaration_scope() -> WorthGraphReadAccessScopeBinding {
    WorthGraphReadAccessScopeBinding::selected_obligation(
        "crates/worth-topo/src/projection/read_views/domain",
        0,
        WorthGraphReadAccessScopeFamily::TopologyReadLedger,
        "authority-a",
        "touch-a",
        "execution-a",
        "registration-a",
    )
    .unwrap()
}

pub(super) fn spatial_scope(source_path: &str) -> WorthGraphReadAccessScopeBinding {
    WorthGraphReadAccessScopeBinding::spatial_continuation_proof(
        source_path,
        1,
        "authority-a",
        "touch-a",
        "execution-a",
    )
    .unwrap()
}

pub(super) fn certification_scope(source_path: &str) -> WorthGraphReadAccessScopeBinding {
    WorthGraphReadAccessScopeBinding::certification_boundary(
        source_path,
        format!("certification-boundary:{source_path}"),
    )
    .unwrap()
}

pub(super) fn out_of_scope_binding(source_path: &str) -> WorthGraphReadAccessScopeBinding {
    WorthGraphReadAccessScopeBinding::out_of_scope_non_graph_read(
        source_path,
        "non-graph-read-boundary",
    )
    .unwrap()
}
