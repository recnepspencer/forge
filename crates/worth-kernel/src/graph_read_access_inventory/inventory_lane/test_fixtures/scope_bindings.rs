use super::super::{WorthGraphReadAccessScopeBinding, WorthGraphReadAccessScopeFamily};

pub(crate) fn deleted_source_scope(source_path: &str) -> WorthGraphReadAccessScopeBinding {
    WorthGraphReadAccessScopeBinding::deleted_graph_read_source(source_path, "adoption-a").unwrap()
}

pub(crate) fn declaration_scope() -> WorthGraphReadAccessScopeBinding {
    declaration_scope_for_tests(
        "crates/worth-topo/src/projection/read_views/domain",
        "authority-a",
    )
}

pub(crate) fn declaration_scope_for_tests(
    source_path: &str,
    authority_digest: &str,
) -> WorthGraphReadAccessScopeBinding {
    WorthGraphReadAccessScopeBinding::selected_obligation(
        source_path,
        0,
        WorthGraphReadAccessScopeFamily::TopologyReadLedger,
        authority_digest,
        "touch-a",
        "execution-a",
        "registration-a",
    )
    .unwrap()
}

pub(crate) fn spatial_scope(source_path: &str) -> WorthGraphReadAccessScopeBinding {
    spatial_scope_with_authority_for_tests(source_path, "authority-a")
}

pub(crate) fn spatial_scope_with_authority_for_tests(
    source_path: &str,
    authority_digest: &str,
) -> WorthGraphReadAccessScopeBinding {
    WorthGraphReadAccessScopeBinding::spatial_continuation_proof(
        source_path,
        1,
        authority_digest,
        "touch-a",
        "execution-a",
    )
    .unwrap()
}

pub(crate) fn spatial_declaration_scope_for_tests(
    source_path: &str,
    authority_digest: &str,
) -> WorthGraphReadAccessScopeBinding {
    WorthGraphReadAccessScopeBinding::spatial_declaration_authority(
        source_path,
        1,
        authority_digest,
        "touch-a",
        "execution-a",
    )
    .unwrap()
}

pub(crate) fn preview_declaration_scope_for_tests(
    source_path: &str,
    authority_digest: &str,
) -> WorthGraphReadAccessScopeBinding {
    WorthGraphReadAccessScopeBinding::preview_declaration_candidate(
        source_path,
        2,
        authority_digest,
        "touch-preview",
        "execution-preview",
    )
    .unwrap()
}

pub(crate) fn branch_declaration_scope_for_tests(
    source_path: &str,
    authority_digest: &str,
) -> WorthGraphReadAccessScopeBinding {
    WorthGraphReadAccessScopeBinding::branch_declaration_candidate(
        source_path,
        3,
        authority_digest,
        "touch-branch",
        "execution-branch",
    )
    .unwrap()
}

pub(crate) fn future_receipt_scope_for_tests(
    source_path: &str,
    authority_digest: &str,
) -> WorthGraphReadAccessScopeBinding {
    WorthGraphReadAccessScopeBinding::touched_authority_digest(
        source_path,
        4,
        WorthGraphReadAccessScopeFamily::TopologyReadLedger,
        authority_digest,
        "touch-future",
        "execution-future",
    )
    .unwrap()
}

pub(crate) fn certification_scope(source_path: &str) -> WorthGraphReadAccessScopeBinding {
    WorthGraphReadAccessScopeBinding::from_certification_boundary(
        source_path,
        format!("certification-boundary:{source_path}"),
    )
    .unwrap()
}

pub(crate) fn out_of_scope_binding(source_path: &str) -> WorthGraphReadAccessScopeBinding {
    WorthGraphReadAccessScopeBinding::out_of_scope_non_graph_read(
        source_path,
        "non-graph-read-boundary",
    )
    .unwrap()
}
