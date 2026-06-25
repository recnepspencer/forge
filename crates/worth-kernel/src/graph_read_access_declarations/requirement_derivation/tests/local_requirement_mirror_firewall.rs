use crate::graph_read_access_declarations::current_worth_graph_read_requirement_derivation_closeout;

use super::common::{phase_two_closeout_from_seed, production_seed, rust_sources_under};

#[test]
fn worth_local_requirement_rows_are_rejected_as_query_derivation_proof() {
    let phase_two = phase_two_closeout_from_seed(&production_seed());
    let phase_four = current_worth_graph_read_requirement_derivation_closeout(&phase_two)
        .expect("Phase 4 should reject local row mirrors by producing gaps");

    assert_eq!(phase_four.derivation_summary().derived_row_count(), 0);
    assert_eq!(
        phase_four
            .derivation_summary()
            .distinct_requirement_kind_count(),
        0
    );
    assert!(phase_four.requirement_records().iter().all(|record| !record
        .derivation_outcome()
        .claims_query_requirement_rows_derived()));
}

#[test]
fn production_declaration_sources_do_not_add_local_requirement_mirrors() {
    let source_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("graph_read_access_declarations");
    let forbidden = [
        "WorthGraphReadRequirementKind",
        "directional_adjacency",
        "reverse_adjacency",
        "traversal_workset",
        "visited_set",
        "dedup_set",
        "result_buffer",
        "no N+1",
        "no_n_plus_one",
    ];
    let mut offenders = Vec::new();
    for path in rust_sources_under(&source_root)
        .into_iter()
        .filter(|path| {
            !path
                .components()
                .any(|component| component.as_os_str() == "tests")
        })
        .filter(|path| !is_query_requirement_evidence_wrapper(path))
        .filter(|path| !is_source_firewall_pattern_catalog(path))
    {
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
        for needle in forbidden {
            if text.contains(needle) {
                offenders.push(format!("{} contains {needle}", path.display()));
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "Worth-local requirement mirrors must not appear outside Query evidence wrappers: {offenders:#?}"
    );
}

fn is_query_requirement_evidence_wrapper(path: &std::path::Path) -> bool {
    let components = path
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>();
    components
        .windows(2)
        .any(|window| window[0] == "query_requirement_evidence" && window[1] == "row.rs")
        || components
            .windows(2)
            .any(|window| window[0] == "query_requirement_evidence" && window[1] == "set.rs")
}

fn is_source_firewall_pattern_catalog(path: &std::path::Path) -> bool {
    let components = path
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>();
    components.windows(3).any(|window| {
        window[0] == "deletion_firewall"
            && window[1] == "source_firewall"
            && window[2] == "forbidden_pattern.rs"
    })
}
