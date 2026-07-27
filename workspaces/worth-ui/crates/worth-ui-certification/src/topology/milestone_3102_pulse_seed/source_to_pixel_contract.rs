use std::collections::BTreeSet;

use super::evidence_document::{require_exact_ids, toml_rows, toml_text, toml_texts};

const EDGE_IDS: &[&str] = &[
    "E01_CHECKED_SOURCE_TO_FILESYSTEM_PROVIDER",
    "E02_PROVIDER_TO_SETTLED_SNAPSHOT",
    "E03_SNAPSHOT_TO_SEALED_AUTHORED_MEANING",
    "E04_SEALED_MEANING_TO_PREPARED_GENERATION",
    "E05_GENERATION_TO_GRAPH_NODE",
    "E06_GRAPH_NODE_TO_MOUNTED_RECEIPT",
    "E07_LAYOUT_TO_COMMITTED_ALLOCATION",
    "E08_AUTHORED_TOKEN_TO_ADMITTED_COLOR",
    "E09_AUTHORITIES_TO_COMPLETE_PAINT",
    "E10_COMPLETE_PAINT_TO_HOST_ADMISSION",
    "E11_HOST_ADMISSION_TO_EGUI_SHAPE",
    "E12_EGUI_SHAPE_TO_VISIBLE_PIXEL",
    "E13_VALID_EDIT_TO_SUCCESSOR_PUBLICATION",
    "E14_MALFORMED_EDIT_TO_PREDECESSOR_PRESERVATION",
];

const REJECTION_IDS: &[&str] = &[
    "R01_MISSING_ALLOCATION",
    "R02_UNRESOLVED_OR_INVALID_COLOR",
    "R03_OMITTED_LAYER",
    "R04_OMITTED_OR_FOREIGN_CLIP",
    "R05_STALE_NODE_RECEIPT",
    "R06_FOREIGN_SURFACE_BINDING",
    "R07_UNSUPPORTED_PROTOCOL",
    "R08_MISSING_NATIVE_PAINT_CAPABILITY",
];

const REQUIRED_JOIN: &[&str] = &[
    "exact-mounted-node-receipt",
    "committed-allocation-box",
    "admitted-file-authored-color",
    "explicit-layer",
    "explicit-clip",
    "surface-binding-generation",
    "mounted-frame-identity",
];

pub(super) fn audit(document: &toml::Value) -> Result<(), String> {
    require_exact_ids(document, "edge", EDGE_IDS)?;
    require_exact_ids(document, "rejection", REJECTION_IDS)?;
    audit_edges(document)?;
    audit_rejections(document)?;
    audit_complete_static_paint(document)
}

fn audit_edges(document: &toml::Value) -> Result<(), String> {
    let required_fields = [
        "producer",
        "consumer",
        "cardinality",
        "lifetime",
        "authority_owner",
        "failure_owner",
        "effect_owner",
        "cost_class",
        "forbidden_shortcut",
    ];
    for row in toml_rows(document, "edge")? {
        let id = toml_text(row, "id")?;
        for field in required_fields {
            toml_text(row, field).map_err(|error| format!("{id}: {error}"))?;
        }
        let authority = toml_text(row, "authority_owner")?;
        if authority == "worth-ui-platform-pulse" || authority == "worth-ui-host-egui" {
            return Err(format!(
                "{id}: application and adapter mechanics cannot own semantic authority"
            ));
        }
    }
    Ok(())
}

fn audit_rejections(document: &toml::Value) -> Result<(), String> {
    for row in toml_rows(document, "rejection")? {
        let id = toml_text(row, "id")?;
        toml_text(row, "condition").map_err(|error| format!("{id}: {error}"))?;
        let result = toml_text(row, "required_result").map_err(|error| format!("{id}: {error}"))?;
        if result != "reject before effects and preserve predecessor" {
            return Err(format!(
                "{id}: every incomplete static-paint condition must reject before effects and preserve predecessor"
            ));
        }
    }
    Ok(())
}

fn audit_complete_static_paint(document: &toml::Value) -> Result<(), String> {
    let static_paint = document
        .get("static_paint")
        .ok_or_else(|| "Phase 1 evidence should freeze [static_paint]".to_owned())?;
    if toml_text(static_paint, "primitive")? != "filled-rectangle" {
        return Err("Phase 1 may freeze only the filled-rectangle primitive".to_owned());
    }
    let actual = toml_texts(static_paint, "required_join")?
        .into_iter()
        .collect::<BTreeSet<_>>();
    let expected = REQUIRED_JOIN.iter().copied().collect::<BTreeSet<_>>();
    if actual != expected {
        return Err(format!(
            "complete static paint should require exactly {expected:?}; found {actual:?}"
        ));
    }
    Ok(())
}
