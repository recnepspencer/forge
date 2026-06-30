use super::row::EvidenceLookupForbiddenAuthorityKind;

#[derive(Clone, Copy)]
pub(crate) struct MatchedSemanticShape {
    pub(crate) kind: EvidenceLookupForbiddenAuthorityKind,
    pub(crate) matched_surface: &'static str,
    pub(crate) reason: &'static str,
}

pub(crate) fn matched_semantic_shapes(
    source_path: &str,
    source: &str,
) -> Vec<MatchedSemanticShape> {
    let normalized = source.to_ascii_lowercase();
    let mut matches = Vec::new();
    if is_public_lookup_vocabulary_path(source_path) {
        push_match_if(
            &mut matches,
            mentions_public_evidence_row_surface(&normalized),
            EvidenceLookupForbiddenAuthorityKind::PublicEvidenceRowExposure,
            "public workload evidence row or complete-ledger export",
            "public workload-vocabulary exports must not act as ordinary lookup-proof vocabulary",
        );
        push_match_if(
            &mut matches,
            mentions_query_lookup_product_substitution(&normalized),
            EvidenceLookupForbiddenAuthorityKind::QueryLookupProductSubstitution,
            "public query/lookup product bridge surface",
            "public workload-vocabulary lookup products must not stand in for Milestone 11 lookup-proof products",
        );
    }
    if is_receipt_lookup_surface_path(source_path) {
        push_match_if(
            &mut matches,
            mentions_broad_receipt_scan(&normalized),
            EvidenceLookupForbiddenAuthorityKind::BroadReceiptScan,
            "broad boolean receipt lookup helper",
            "broad ledger receipt lookup is forbidden ordinary lookup authority after migrated family selection exists",
        );
        push_match_if(
            &mut matches,
            mentions_stage_local_nearby_lookup(&normalized),
            EvidenceLookupForbiddenAuthorityKind::StageLocalNearbyLookup,
            "stage-local nearby evidence row lookup",
            "stage-local row fetch is nearby lookup folklore and cannot reappear as ordinary lookup proof",
        );
    }
    if is_raw_row_surface_path(source_path) {
        push_match_if(
            &mut matches,
            mentions_raw_evidence_vector_access(&normalized),
            EvidenceLookupForbiddenAuthorityKind::RawEvidenceVectorAccess,
            "manual workload evidence row fabrication",
            "manual workload evidence row construction is raw-vector authority and must not survive as ordinary lookup proof",
        );
    }
    if is_query_lookup_bridge_path(source_path) {
        push_match_if(
            &mut matches,
            mentions_query_lookup_product_substitution(&normalized),
            EvidenceLookupForbiddenAuthorityKind::QueryLookupProductSubstitution,
            "query descriptor / lookup product substitution bridge",
            "query-looking local proof must not bridge into spatial lookup product authority",
        );
    }
    if is_copied_digest_surface_path(source_path) {
        push_match_if(
            &mut matches,
            mentions_copied_digest_lookup(&normalized),
            EvidenceLookupForbiddenAuthorityKind::CopiedDigestLookup,
            "query descriptor digest substitution",
            "copied Query digest surfaces must not satisfy spatial lookup authority",
        );
    }
    if is_kernel_lookup_residue_path(source_path) {
        push_match_if(
            &mut matches,
            mentions_kernel_stage_local_lookup(&normalized),
            EvidenceLookupForbiddenAuthorityKind::StageLocalNearbyLookup,
            "kernel stage-local lookup residue",
            "kernel boolean helper residue is stage-local lookup folklore until family-specific lookup proof replaces it",
        );
        push_match_if(
            &mut matches,
            mentions_query_lookup_product_substitution(&normalized),
            EvidenceLookupForbiddenAuthorityKind::QueryLookupProductSubstitution,
            "kernel query descriptor bridge",
            "kernel query descriptor proof must not bridge into spatial lookup product authority",
        );
    }
    matches
}

fn push_match_if(
    matches: &mut Vec<MatchedSemanticShape>,
    condition: bool,
    kind: EvidenceLookupForbiddenAuthorityKind,
    matched_surface: &'static str,
    reason: &'static str,
) {
    if condition {
        matches.push(MatchedSemanticShape {
            kind,
            matched_surface,
            reason,
        });
    }
}

fn is_public_lookup_vocabulary_path(source_path: &str) -> bool {
    matches!(
        source_path,
        "crates/worth-spatial/src/facade/workload_vocabulary/mod.rs"
            | "crates/worth-spatial/src/workload_platform/evidence_ledger/surface_inventory/rows.rs"
    )
}

fn is_receipt_lookup_surface_path(source_path: &str) -> bool {
    matches!(
        source_path,
        "crates/worth-spatial/src/workload_platform/evidence_ledger/ledger.rs"
            | "crates/worth-spatial/src/workload_platform/evidence_ledger/stage_index/product.rs"
            | "crates/worth-spatial/src/certification/workload_evidence.rs"
            | "crates/worth-spatial/src/workload_platform/evidence_ledger/surface_inventory/rows.rs"
    )
}

fn is_raw_row_surface_path(source_path: &str) -> bool {
    matches!(
        source_path,
        "crates/worth-spatial/src/workload_platform/evidence_ledger/row.rs"
            | "crates/worth-spatial/src/certification/workload_evidence.rs"
            | "crates/worth-spatial/src/workload_platform/evidence_ledger/surface_inventory/rows.rs"
    )
}

fn is_query_lookup_bridge_path(source_path: &str) -> bool {
    is_public_lookup_vocabulary_path(source_path)
        || source_path == "crates/worth-spatial/src/query_adoption.rs"
}

fn is_copied_digest_surface_path(source_path: &str) -> bool {
    source_path.starts_with(
        "crates/worth-spatial/src/workload_platform/evidence_ledger/spatial_touch_admission",
    )
}

fn is_kernel_lookup_residue_path(source_path: &str) -> bool {
    source_path.starts_with("crates/worth-kernel/src/workload_composition")
}

fn mentions_public_evidence_row_surface(source: &str) -> bool {
    contains_any(
        source,
        &["workloadevidencerow", "completeworkloadevidenceledger"],
    )
}

fn mentions_raw_evidence_vector_access(source: &str) -> bool {
    contains_any(
        source,
        &[
            "pub fn new(",
            "workloadevidencerow::new",
            "vec<workloadevidencerow>",
            "additional_rows: vec<workloadevidencerow>",
        ],
    )
}

fn mentions_broad_receipt_scan(source: &str) -> bool {
    source.contains("require_boolean_receipt_lookup")
        || contains_all(source, &["boolean", "receipt", "lookup"])
}

fn mentions_stage_local_nearby_lookup(source: &str) -> bool {
    source.contains("row_for_stage")
        || contains_any(
            source,
            &[
                "require_boolean_split(",
                "require_boolean_loop_reconstruction(",
            ],
        )
        || (contains_all(source, &["stage", "row"])
            && contains_any(source, &["nearby", "lookup", "fetch"]))
}

fn mentions_copied_digest_lookup(source: &str) -> bool {
    source.contains("query_descriptor_digest")
        || (contains_all(source, &["query", "descriptor", "digest"])
            && contains_any(source, &["lookup", "authority", "product"]))
}

fn mentions_query_lookup_product_substitution(source: &str) -> bool {
    (contains_any(
        source,
        &[
            "query descriptor",
            "query_descriptor",
            "forgequerygraphtouchdescriptor",
        ],
    ) && contains_any(
        source,
        &[
            "lookup",
            "product",
            "digest",
            "spatialevidence",
            "selectedplan",
        ],
    )) || contains_any(
        source,
        &["spatialevidencelookupproduct", "lookup_product_digest"],
    )
}

fn mentions_kernel_stage_local_lookup(source: &str) -> bool {
    contains_any(
        source,
        &[
            "require_boolean_split(",
            "require_boolean_loop_reconstruction(",
        ],
    )
}

fn contains_all(source: &str, parts: &[&str]) -> bool {
    parts.iter().all(|part| source.contains(part))
}

fn contains_any(source: &str, parts: &[&str]) -> bool {
    parts.iter().any(|part| source.contains(part))
}
