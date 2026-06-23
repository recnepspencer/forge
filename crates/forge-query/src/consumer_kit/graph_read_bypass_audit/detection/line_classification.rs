use crate::ForgeQueryBoundaryAuditSourceSite;

use super::super::registry::{
    registry_row_for_class, ForgeQueryGraphReadBypassClass, ForgeQueryGraphReadBypassRegistryRow,
};
use super::scan_state::GraphReadBypassScanState;

#[derive(Clone, Debug)]
pub(in crate::consumer_kit::graph_read_bypass_audit) struct GraphReadBypassCandidate {
    pub(in crate::consumer_kit::graph_read_bypass_audit) row:
        &'static ForgeQueryGraphReadBypassRegistryRow,
    pub(in crate::consumer_kit::graph_read_bypass_audit) source_site:
        ForgeQueryBoundaryAuditSourceSite,
}

pub(in crate::consumer_kit::graph_read_bypass_audit) fn detect_graph_read_bypass_candidates(
    source_label: &str,
    source_path: Option<&str>,
    masked_source: &str,
) -> Vec<GraphReadBypassCandidate> {
    let mut scan_state = GraphReadBypassScanState::default();
    let mut candidates = Vec::new();
    for (line_index, line) in masked_source.lines().enumerate() {
        scan_state.observe_line(line);
        candidates.extend(detect_line_classes(line, &scan_state).into_iter().map(
            |(class, column)| GraphReadBypassCandidate {
                row: registry_row_for_class(class),
                source_site: ForgeQueryBoundaryAuditSourceSite::new(
                    source_label,
                    source_path,
                    line_index + 1,
                    column + 1,
                ),
            },
        ));
    }
    candidates
}

fn detect_line_classes(
    line: &str,
    scan_state: &GraphReadBypassScanState,
) -> Vec<(ForgeQueryGraphReadBypassClass, usize)> {
    let mut classes = Vec::new();
    push_if(
        &mut classes,
        line,
        ForgeQueryGraphReadBypassClass::ManualRelationRowLoop,
        |line| is_manual_relation_row_loop(line, scan_state),
    );
    push_if(
        &mut classes,
        line,
        ForgeQueryGraphReadBypassClass::PerNodeNeighborLookup,
        |line| is_per_node_neighbor_lookup(line, scan_state),
    );
    push_if(
        &mut classes,
        line,
        ForgeQueryGraphReadBypassClass::AdHocAdjacencyMap,
        |line| is_ad_hoc_adjacency_map(line, scan_state),
    );
    push_if(
        &mut classes,
        line,
        ForgeQueryGraphReadBypassClass::ManualFrontierScan,
        |line| is_manual_frontier_scan(line, scan_state),
    );
    push_if(
        &mut classes,
        line,
        ForgeQueryGraphReadBypassClass::ManualVisitedSetTraversal,
        |line| is_manual_visited_set_traversal(line, scan_state),
    );
    push_if(
        &mut classes,
        line,
        ForgeQueryGraphReadBypassClass::SurfaceLocalGraphCache,
        is_surface_local_graph_cache,
    );
    push_if(
        &mut classes,
        line,
        ForgeQueryGraphReadBypassClass::BroadBooleanGraphScan,
        |line| is_broad_boolean_graph_scan(line, scan_state),
    );
    push_if(
        &mut classes,
        line,
        ForgeQueryGraphReadBypassClass::HiddenGraphReadFallback,
        is_hidden_graph_read_fallback,
    );
    push_if(
        &mut classes,
        line,
        ForgeQueryGraphReadBypassClass::RuntimeReadLoweringBypass,
        is_runtime_read_lowering_bypass,
    );
    push_if(
        &mut classes,
        line,
        ForgeQueryGraphReadBypassClass::TestSupportClaimingProductionProof,
        is_test_support_claiming_production_proof,
    );
    classes
}

fn push_if(
    classes: &mut Vec<(ForgeQueryGraphReadBypassClass, usize)>,
    line: &str,
    class: ForgeQueryGraphReadBypassClass,
    predicate: impl Fn(&str) -> Option<usize>,
) {
    if let Some(column) = predicate(line) {
        classes.push((class, column));
    }
}

fn is_manual_relation_row_loop(line: &str, scan_state: &GraphReadBypassScanState) -> Option<usize> {
    if code_has_any(line, ["for ", ".iter()", ".into_iter()"])
        && scan_state.line_mentions_relation_rows(line)
    {
        first_code_match(line, ["relation_rows", "relations()", ".iter()", "for "])
    } else {
        None
    }
}

fn is_per_node_neighbor_lookup(line: &str, scan_state: &GraphReadBypassScanState) -> Option<usize> {
    if (code_has_any(line, ["neighbor", "neighbour"]) || scan_state.line_mentions_adjacency(line))
        && code_has_any(line, [".find(", ".filter(", "relations()"])
    {
        first_code_match(line, ["neighbor", "neighbour", ".find(", ".filter("])
    } else {
        None
    }
}

fn is_ad_hoc_adjacency_map(line: &str, scan_state: &GraphReadBypassScanState) -> Option<usize> {
    if scan_state.line_mentions_adjacency(line)
        && code_has_any(line, ["BTreeMap", "HashMap", ".insert(", "Vec<"])
    {
        first_code_match(line, ["adjacency", "BTreeMap", "HashMap", ".insert("])
    } else {
        None
    }
}

fn is_manual_frontier_scan(line: &str, scan_state: &GraphReadBypassScanState) -> Option<usize> {
    if scan_state.line_mentions_frontier(line)
        && code_has_any(line, ["while ", "loop", ".pop(", ".pop_front("])
    {
        first_code_match(line, ["frontier", ".pop(", ".pop_front(", "while ", "loop"])
    } else {
        None
    }
}

fn is_manual_visited_set_traversal(
    line: &str,
    scan_state: &GraphReadBypassScanState,
) -> Option<usize> {
    if scan_state.line_mentions_visited(line) && code_has_any(line, [".contains(", ".insert("]) {
        first_code_match(line, ["visited", ".contains(", ".insert("])
    } else {
        None
    }
}

fn is_surface_local_graph_cache(line: &str) -> Option<usize> {
    first_code_match(line, ["local_graph_cache", "graph_cache"])
}

fn is_broad_boolean_graph_scan(line: &str, scan_state: &GraphReadBypassScanState) -> Option<usize> {
    if code_has_any(line, [".filter(", "filter_map("])
        && (scan_state.line_mentions_relation_rows(line) || code_has_any(line, ["row_matches"]))
    {
        first_code_match(line, [".filter(", "filter_map("])
    } else {
        None
    }
}

fn is_hidden_graph_read_fallback(line: &str) -> Option<usize> {
    first_code_match(line, ["hidden_graph_read_fallback", "fallback_graph_read"])
}

fn is_runtime_read_lowering_bypass(line: &str) -> Option<usize> {
    first_code_match(
        line,
        [
            "execute_live_read_by_name",
            "execute_read_execution_binding",
            "prepare_read_execution_binding",
        ],
    )
}

fn is_test_support_claiming_production_proof(line: &str) -> Option<usize> {
    first_code_match(line, ["claim_production_graph_read_proof_for_test"])
}

fn code_has_any<const N: usize>(line: &str, needles: [&str; N]) -> bool {
    needles.into_iter().any(|needle| line.contains(needle))
}

fn first_code_match<const N: usize>(line: &str, needles: [&str; N]) -> Option<usize> {
    needles
        .into_iter()
        .filter_map(|needle| line.find(needle))
        .min()
}
