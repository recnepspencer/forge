use std::collections::BTreeSet;

use super::evidence_document::{require_exact_ids, toml_rows, toml_text, toml_texts};

const MUTATION_IDS: &[&str] = &[
    "M01_DIRECT_EGUI_DRAWING",
    "M02_INJECTED_SOURCE",
    "M03_COUNT_ONLY_PAINT",
    "M04_DETACHED_SCREENSHOT",
];
const ORACLE_IDS: &[&str] = &[
    "O01_AUTHORED_EXPECTATION",
    "O02_HEADLESS_TRANSCRIPT",
    "O03_NATIVE_SHAPE",
    "O04_PUBLICATION",
];
const PUBLIC_GAP_IDS: &[&str] = &[
    "G01_INITIAL_SOURCE_CAPABILITY_FREEZE",
    "G02_ORDINARY_MOUNT_AND_ALLOCATION",
];
const EARLY_APPEARANCE_SCOPE: &[&str] = &[
    "appearance-roles",
    "component-defaults",
    "interaction-state-styling",
    "theme-switching",
    "theme-invalidation",
    "typography",
    "borders-radii-shadows",
    "renderer-fallbacks",
];
const MUTATION_EVIDENCE: &[(&str, &[&str])] = &[
    (
        "M01_DIRECT_EGUI_DRAWING",
        &[
            "mounted-frame-identity",
            "mounted-node-receipt",
            "committed-allocation",
            "surface-binding",
            "native-paint-effect",
        ],
    ),
    (
        "M02_INJECTED_SOURCE",
        &[
            "filesystem-provider-kind",
            "watcher-readiness",
            "settled-snapshot",
            "filesystem-revision",
            "watcher-shutdown",
        ],
    ),
    (
        "M03_COUNT_ONLY_PAINT",
        &[
            "complete-paint-primitive",
            "native-paint-admission",
            "native-shape",
        ],
    ),
    (
        "M04_DETACHED_SCREENSHOT",
        &[
            "application-generation",
            "mounted-frame-identity",
            "surface-binding",
            "mounted-node-receipt",
            "publication-receipt",
            "native-shape",
        ],
    ),
];

pub(super) fn audit(document: &toml::Value) -> Result<(), String> {
    audit_scenario(document)?;
    audit_composition_root(document)?;
    audit_native_shell(document)?;
    require_exact_ids(document, "mutation", MUTATION_IDS)?;
    require_exact_ids(document, "oracle", ORACLE_IDS)?;
    require_exact_ids(document, "public_gap", PUBLIC_GAP_IDS)?;
    audit_named_rows(document, "mutation", &["fault", "detection"])?;
    audit_mutation_evidence(document)?;
    audit_named_rows(document, "oracle", &["independent_observation"])?;
    audit_named_rows(
        document,
        "public_gap",
        &["current_constraint", "required_contract"],
    )?;
    audit_early_appearance_exclusion(document)
}

fn audit_mutation_evidence(document: &toml::Value) -> Result<(), String> {
    let rows = toml_rows(document, "mutation")?;
    for (id, expected) in MUTATION_EVIDENCE {
        let row = rows
            .iter()
            .find(|row| row.get("id").and_then(toml::Value::as_str) == Some(*id))
            .ok_or_else(|| format!("required mutation `{id}` is missing"))?;
        let actual = toml_texts(row, "invalidated_evidence")?
            .into_iter()
            .collect::<BTreeSet<_>>();
        let expected = expected.iter().copied().collect::<BTreeSet<_>>();
        if actual != expected {
            return Err(format!(
                "mutation `{id}` should invalidate exactly {expected:?}; found {actual:?}"
            ));
        }
    }
    Ok(())
}

fn audit_native_shell(document: &toml::Value) -> Result<(), String> {
    let shell = document
        .get("native_shell")
        .ok_or_else(|| "Phase 1 evidence should freeze [native_shell]".to_owned())?;
    if toml_text(shell, "package")? != "eframe"
        || toml_text(shell, "version_requirement")? != "=0.31.1"
        || shell
            .get("uses_default_features")
            .and_then(toml::Value::as_bool)
            != Some(false)
    {
        return Err(
            "native shell should remain eframe =0.31.1 with default features disabled".to_owned(),
        );
    }
    let actual = toml_texts(shell, "features")?
        .into_iter()
        .collect::<BTreeSet<_>>();
    let expected = ["default_fonts", "glow", "wayland", "x11"]
        .into_iter()
        .collect::<BTreeSet<_>>();
    if actual != expected {
        return Err(format!(
            "native shell features should be exactly {expected:?}; found {actual:?}"
        ));
    }
    toml_text(shell, "adjudication")?;
    Ok(())
}

fn audit_scenario(document: &toml::Value) -> Result<(), String> {
    let scenario = document
        .get("scenario")
        .ok_or_else(|| "Phase 1 evidence should freeze [scenario]".to_owned())?;
    let exact_text = [
        ("identity", "worth-ui.platform-pulse.seed"),
        ("package", "worth-ui-platform-pulse"),
        (
            "source_path",
            "workspaces/worth-ui/apps/platform-pulse/app/main.wui",
        ),
        ("source_module", "app/main.wui"),
        ("valid_edit_from", "theme.platform_pulse.blue"),
        ("valid_edit_to", "theme.platform_pulse.green"),
        (
            "replacement_classification",
            "whole-application-replacement",
        ),
    ];
    for (field, expected) in exact_text {
        let actual = toml_text(scenario, field)?;
        if actual != expected {
            return Err(format!(
                "scenario `{field}` should be `{expected}`; found `{actual}`"
            ));
        }
    }
    let initial_source = toml_text(scenario, "initial_source")?;
    for required in [
        "component platform.pulse.component.seed {}",
        "surface platform.pulse.surface.main {}",
        "token theme.platform_pulse.fill = \"theme.platform_pulse.blue\";",
    ] {
        if !initial_source.lines().any(|line| line == required) {
            return Err(format!(
                "canonical source should contain the exact line `{required}`"
            ));
        }
    }
    require_integer(scenario, "canonical_x", 0)?;
    require_integer(scenario, "canonical_y", 0)?;
    require_integer(scenario, "canonical_width", 160)?;
    require_integer(scenario, "canonical_height", 96)?;
    require_integer(scenario, "layer_semantic_order", 0)?;
    require_integer_array(scenario, "initial_rgba", &[47, 129, 247, 255])?;
    require_integer_array(scenario, "successor_rgba", &[63, 185, 80, 255])
}

fn audit_composition_root(document: &toml::Value) -> Result<(), String> {
    let root = document
        .get("composition_root")
        .ok_or_else(|| "Phase 1 evidence should freeze [composition_root]".to_owned())?;
    if toml_text(root, "owner")? != "worth-ui-platform-pulse" {
        return Err("the permanent pulse package should own application composition".to_owned());
    }
    for field in ["rationale", "facade_exclusion", "adapter_exclusion"] {
        toml_text(root, field)?;
    }
    Ok(())
}

fn audit_named_rows(document: &toml::Value, family: &str, fields: &[&str]) -> Result<(), String> {
    for row in toml_rows(document, family)? {
        let id = toml_text(row, "id")?;
        for field in fields {
            toml_text(row, field).map_err(|error| format!("{id}: {error}"))?;
        }
    }
    Ok(())
}

fn audit_early_appearance_exclusion(document: &toml::Value) -> Result<(), String> {
    let static_paint = document
        .get("static_paint")
        .ok_or_else(|| "Phase 1 evidence should freeze [static_paint]".to_owned())?;
    let actual = toml_texts(static_paint, "forbidden_early_scope")?
        .into_iter()
        .collect::<BTreeSet<_>>();
    let expected = EARLY_APPEARANCE_SCOPE
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    if actual != expected {
        return Err(format!(
            "Phase 1 should exclude exactly the early appearance scope {expected:?}; found {actual:?}"
        ));
    }
    Ok(())
}

fn require_integer(row: &toml::Value, field: &str, expected: i64) -> Result<(), String> {
    let actual = row
        .get(field)
        .and_then(toml::Value::as_integer)
        .ok_or_else(|| format!("scenario `{field}` should be an integer"))?;
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "scenario `{field}` should be {expected}; found {actual}"
        ))
    }
}

fn require_integer_array(row: &toml::Value, field: &str, expected: &[i64]) -> Result<(), String> {
    let actual = row
        .get(field)
        .and_then(toml::Value::as_array)
        .ok_or_else(|| format!("scenario `{field}` should be an array"))?
        .iter()
        .map(|value| {
            value
                .as_integer()
                .ok_or_else(|| format!("scenario `{field}` values should be integers"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "scenario `{field}` should be {expected:?}; found {actual:?}"
        ))
    }
}
