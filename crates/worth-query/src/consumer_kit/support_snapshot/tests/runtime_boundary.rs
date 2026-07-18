#[path = "runtime_boundary/source_scan.rs"]
mod source_scan;

use source_scan::{
    collect_production_string_map_residue, collect_runtime_support_snapshot_imports,
    collect_rust_source_marker_residue,
};

#[test]
fn runtime_sources_do_not_depend_on_consumer_support_snapshots() {
    let runtime_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("runtime");
    let mut offending_files = Vec::new();
    collect_runtime_support_snapshot_imports(&runtime_dir, &mut offending_files);

    assert!(
        offending_files.is_empty(),
        "runtime authority must not import or mention consumer support snapshots: {offending_files:?}"
    );
}

#[test]
fn support_terminal_json_codecs_do_not_become_aspect_compatibility_bridges() {
    let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let terminal_codec_files = [
        "src/consumer_kit/support_pinning/document/terminal_json_codec.rs",
        "src/consumer_kit/support_snapshot/document/terminal_json_codec.rs",
    ];
    let forbidden_aspect_authority_markers = [
        "worth_foundational",
        "AspectContract",
        "AspectValue",
        "AuthoritativeRecordAspect",
        "ContractValidatedAspect",
        "JsonCompatibilityAspectInput",
        "lower_json_",
        "compatibility().json()",
    ];
    let mut offenders = Vec::new();

    for relative_path in terminal_codec_files {
        let source = std::fs::read_to_string(crate_root.join(relative_path))
            .expect("terminal codec source should be readable");
        for marker in forbidden_aspect_authority_markers {
            if source.contains(marker) {
                offenders.push(format!("{relative_path} contains {marker}"));
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "support terminal JSON codecs serialize durable support documents only; JSON-as-aspect-truth must use worth-foundational compatibility instead: {offenders:?}"
    );
}

#[test]
fn query_sources_do_not_define_local_foundational_json_compatibility_bridge() {
    let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let forbidden_markers = [
        "JsonCompatibilityAspectInput",
        "JsonCompatibilityFrontDoor",
        "JsonCompatibilityLowering",
        "JsonCompatibilityLoweringOutcome",
        "compatibility().json()",
        "lower_json_",
    ];
    let mut offenders = Vec::new();

    for marker in forbidden_markers {
        let mut marker_offenders = Vec::new();
        collect_rust_source_marker_residue(crate_root, crate_root, marker, &mut marker_offenders);
        for offender in marker_offenders {
            if offender != "src/consumer_kit/support_snapshot/tests/runtime_boundary.rs" {
                offenders.push(format!("{offender} contains {marker}"));
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "Query has no approved JSON-as-aspect compatibility bridge; external JSON-aspect lowering must be owned by worth-foundational before Query receives native artifacts: {offenders:?}"
    );
}

#[test]
fn bridge_readmission_fixtures_do_not_key_authority_by_terminal_strings() {
    let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let fixture_path =
        "src/lower_runtime_routing/certification/surface/fixtures/phase_six/readmission_support.rs";
    let source = std::fs::read_to_string(crate_root.join(fixture_path))
        .expect("readmission fixture source should be readable");
    let forbidden_markers = [
        "committed_patches: BTreeMap<String",
        "branch_heads: BTreeMap<String",
        "snapshots: BTreeMap<String",
        ".bridge_admission_evidence()\n                .terminal_projection_for_reporting()",
    ];
    let offenders = forbidden_markers
        .into_iter()
        .filter(|marker| source.contains(marker))
        .collect::<Vec<_>>();

    assert!(
        offenders.is_empty(),
        "bridge readmission fixtures must retain runtime-bridge identity carriers as map keys instead of terminal projection strings: {offenders:?}"
    );
}

#[test]
fn production_string_map_residue_is_classified_grammar_or_reporting_only() {
    let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let source_root = crate_root.join("src");
    let mut residue_files = Vec::new();
    collect_production_string_map_residue(&source_root, crate_root, &mut residue_files);

    assert_eq!(
        residue_files,
        vec![
            "src/composition/templates/instantiation.rs".to_string(),
            "src/consumer_kit/declarative_surface/consumer_orchestration/audit.rs".to_string(),
            "src/consumer_kit/evidence_report/report.rs".to_string(),
            "src/consumer_kit/evidence_report_adoption/syntax.rs".to_string(),
            "src/memory_workspace/entity_row.rs".to_string(),
            "src/program.rs".to_string(),
            "src/runtime/intent/input.rs".to_string(),
        ],
        "production string-keyed maps must stay confined to classified grammar/reporting files, not runtime authority storage"
    );
}

#[test]
fn graph_obligation_index_does_not_expose_touch_digest_as_lookup_key_value() {
    let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let index_entry_path =
        "src/runtime/mutation/graph_composition/obligation/index/construction/index_entry.rs";
    let lookup_key_path =
        "src/runtime/mutation/graph_composition/obligation/index/lookup/lookup_key.rs";
    let index_entry_source = std::fs::read_to_string(crate_root.join(index_entry_path))
        .expect("graph obligation index entry source should be readable");
    let lookup_key_source = std::fs::read_to_string(crate_root.join(lookup_key_path))
        .expect("graph obligation lookup key source should be readable");

    let mut offenders = Vec::new();
    if index_entry_source.contains("pub fn touch_key_value(") {
        offenders.push(format!("{index_entry_path} exposes touch_key_value"));
    }
    if lookup_key_source.contains(" fn value(") {
        offenders.push(format!("{lookup_key_path} exposes ambiguous value()"));
    }
    if !index_entry_source.contains("terminal_touch_key_value_projection") {
        offenders.push(format!(
            "{index_entry_path} does not name touch key text as terminal projection"
        ));
    }
    if !lookup_key_source.contains("terminal_value_projection") {
        offenders.push(format!(
            "{lookup_key_path} does not name lookup key text as terminal projection"
        ));
    }

    assert!(
        offenders.is_empty(),
        "graph obligation index matching must use native lookup keys; rendered touch digest text may only be exposed as terminal projection evidence: {offenders:?}"
    );
}

#[test]
fn write_receipt_touch_digest_helpers_are_terminal_projections_only() {
    let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let write_receipt_path = "src/runtime/surface/mutation/write_receipt/mod.rs";
    let source = std::fs::read_to_string(crate_root.join(write_receipt_path))
        .expect("write receipt source should be readable");
    let mut offenders = Vec::new();
    if source.contains("fn touched_aspect_digest_parts(") {
        offenders.push(format!(
            "{write_receipt_path} exposes ambiguous touched_aspect_digest_parts helper"
        ));
    }
    if !source.contains("fn terminal_touched_aspect_digest_projections(") {
        offenders.push(format!(
            "{write_receipt_path} does not name replay touch digest text as terminal projection"
        ));
    }

    assert!(
        offenders.is_empty(),
        "write receipt replay evidence may render touch digest text only as terminal projection evidence, not as reusable authority parts: {offenders:?}"
    );
}

#[test]
fn batch_receipt_touch_digest_helpers_are_terminal_projections_only() {
    let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let helper_files = [
        "src/runtime/surface/mutation/batch_receipt_identity.rs",
        "src/runtime/inspection/unified/batch_write_digest.rs",
        "src/runtime/inspection/unified/batch_write_digest_components.rs",
    ];
    let mut offenders = Vec::new();

    for helper_file in helper_files {
        let source = std::fs::read_to_string(crate_root.join(helper_file))
            .expect("batch receipt helper source should be readable");
        if source.contains("evidence_touch_identities") {
            offenders.push(format!(
                "{helper_file} exposes ambiguous evidence_touch_identities helper"
            ));
        }
        if !source.contains("terminal_touch_projection_identities") {
            offenders.push(format!(
                "{helper_file} does not name batch touch digest text as terminal projection"
            ));
        }
    }

    assert!(
        offenders.is_empty(),
        "batch receipt evidence may render touch digest text only as terminal projection evidence, not as reusable authority identities: {offenders:?}"
    );
}

#[test]
fn summary_and_effect_touch_digest_rendering_stays_terminal_evidence_only() {
    let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let effect_helper_path = "src/runtime/effect/delivery_helpers.rs";
    let effect_helper_source = std::fs::read_to_string(crate_root.join(effect_helper_path))
        .expect("effect delivery helper source should be readable");
    let evidence_files = [
        "src/runtime/computed/surface.rs",
        "src/runtime/preview/evidence/execution.rs",
        "src/runtime/surface/verified_assumption_set.rs",
        "src/runtime/surface/graph_composition_assumption_summary.rs",
    ];
    let mut offenders = Vec::new();

    if effect_helper_source.contains("terminal_touch_digest_sequence") {
        offenders.push(format!(
            "{effect_helper_path} exposes ambiguous terminal_touch_digest_sequence helper"
        ));
    }
    if !effect_helper_source.contains("terminal_touch_digest_projection_sequence") {
        offenders.push(format!(
            "{effect_helper_path} does not name effect touch digest text as terminal projection"
        ));
    }

    for evidence_file in evidence_files {
        let source = std::fs::read_to_string(crate_root.join(evidence_file))
            .expect("summary evidence source should be readable");
        if source.contains("admitted_touch_digest_part()")
            && !source.contains("WorthQueryEvidenceTag::new(")
        {
            offenders.push(format!(
                "{evidence_file} renders touch digests outside evidence identity construction"
            ));
        }
    }

    assert!(
        offenders.is_empty(),
        "computed, preview, verified-assumption, and effect touch digest rendering must stay terminal evidence projection only: {offenders:?}"
    );
}
