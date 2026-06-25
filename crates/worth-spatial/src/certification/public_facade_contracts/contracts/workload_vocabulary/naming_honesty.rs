use std::collections::BTreeSet;

const WORKLOAD_VOCABULARY_FACADE_RS: &str =
    include_str!("../../../../facade/workload_vocabulary/mod.rs");
const QUERY_ADOPTION_FACADE_RS: &str = include_str!("../../../../facade/query_adoption.rs");

#[test]
fn public_names_keep_receipt_lookup_query_adoption_and_topology_basis_distinct() {
    let workload_exports = exported_names(WORKLOAD_VOCABULARY_FACADE_RS);
    let query_exports = exported_names(QUERY_ADOPTION_FACADE_RS);

    for required in [
        "BooleanEvidenceReceipt",
        "BooleanEvidenceRowAuthority",
        "SpatialGeometryEvidenceTouchAuthority",
        "SpatialEvidenceLookupProduct",
        "SpatialEvidenceLookupKey",
        "SpatialEvidenceQueryTouchDescriptor",
        "SpatialEvidenceQueryTouchDescriptorDigest",
    ] {
        assert!(
            workload_exports.contains(required),
            "workload_vocabulary facade must expose the named {required} boundary"
        );
    }
    for required in [
        "current_spatial_query_consumer_kit_adoption_status",
        "spatial_query_graph_obligation_adoption_proof",
        "spatial_query_graph_obligation_residue_manifest",
        "WorthSpatialQueryConsumerKitAdoptionStatus",
    ] {
        assert!(
            query_exports.contains(required),
            "query_adoption facade must expose the named {required} boundary"
        );
    }

    assert!(
        !workload_exports.contains("TopologyTouchedGraphBasis"),
        "topology basis must not be exported as spatial workload authority"
    );
    assert!(
        !workload_exports.contains("TopologyDeclaredTouchedGraphBasisProof"),
        "topology declared basis proof must not be exported as spatial workload authority"
    );
}

#[test]
fn phase_nine_public_names_do_not_use_adapter_helper_manager_or_bridge_language() {
    for (surface, names) in [
        (
            "workload_vocabulary",
            exported_names(WORKLOAD_VOCABULARY_FACADE_RS),
        ),
        ("query_adoption", exported_names(QUERY_ADOPTION_FACADE_RS)),
    ] {
        for name in names {
            let lower = name.to_ascii_lowercase();
            for forbidden in ["adapter", "shim", "wrapper", "manager", "bridge"] {
                assert!(
                    !lower.contains(forbidden),
                    "{surface} public name {name} uses adapter-shaped language {forbidden}"
                );
            }
            if lower.contains("helper") {
                assert!(
                    name.contains("deny_"),
                    "{surface} public name {name} must not expose helper-shaped authority"
                );
            }
        }
    }
}

fn exported_names(source: &'static str) -> BTreeSet<String> {
    let mut exports = BTreeSet::new();
    let mut inside_export_block = false;

    for line in source.lines() {
        let trimmed = line
            .split_once("//")
            .map_or(line, |(before, _)| before)
            .trim();
        if trimmed.is_empty() {
            continue;
        }
        if inside_export_block {
            if let Some(closing_items) = trimmed.strip_suffix("};") {
                insert_exports(&mut exports, closing_items);
                inside_export_block = false;
                continue;
            }
            insert_exports(&mut exports, trimmed);
            continue;
        }

        let Some(export_path) = trimmed.strip_prefix("pub use ") else {
            continue;
        };
        if export_path.ends_with("::{") {
            inside_export_block = true;
            continue;
        }
        if let Some((_, grouped)) = export_path.split_once("::{") {
            if let Some(items) = grouped.strip_suffix("};") {
                insert_exports(&mut exports, items);
            }
            continue;
        }
        if let Some(single) = export_path
            .strip_suffix(';')
            .and_then(|path| path.rsplit_once("::"))
            .map(|(_, item)| item)
        {
            insert_exports(&mut exports, single);
        }
    }

    assert!(!inside_export_block, "facade export block must close");
    exports
}

fn insert_exports(exports: &mut BTreeSet<String>, raw_exports: &str) {
    for export in raw_exports.split(',') {
        let export = export.trim().trim_end_matches(';').trim();
        if export.is_empty() {
            continue;
        }
        let name = export
            .rsplit_once(" as ")
            .map_or(export, |(_, alias)| alias.trim());
        exports.insert(name.to_string());
    }
}
