use std::path::Path;

use crate::topology::WorkspaceSourceInventory;

use super::{
    audit, audit_product_root_source, collect_file_exports, declared_exports, ledger, ProductExport,
};

fn workspace_inventory() -> WorkspaceSourceInventory {
    WorkspaceSourceInventory::capture(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("crate parent")
            .parent()
            .expect("workspace root"),
    )
}

fn product_manifest() -> toml::Value {
    let repository_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
        .and_then(Path::parent)
        .expect("repository root");
    ledger::load(&repository_root.join("_docs/worth-ui/milestone-3.10.1-phase-5-product-api.toml"))
        .expect("Phase 5 product manifest")
}

fn export_group_mut<'a>(document: &'a mut toml::Value, audience: &str) -> &'a mut toml::Value {
    document
        .get_mut("export_group")
        .and_then(toml::Value::as_array_mut)
        .expect("export groups")
        .iter_mut()
        .find(|row| row.get("audience").and_then(toml::Value::as_str) == Some(audience))
        .expect("named export group")
}

#[test]
fn phase5_manifest_rejects_unmanifested_public_growth() {
    let inventory = workspace_inventory();
    let mut document = product_manifest();
    let symbols = export_group_mut(&mut document, "app")
        .get_mut("symbols")
        .and_then(toml::Value::as_array_mut)
        .expect("app symbols");
    let removed = symbols.pop().expect("app has a public symbol");
    let error = audit(&inventory, &document).expect_err("missing symbol should fail exactness");
    assert!(error.contains("differs from its exact manifest"));
    assert!(
        error.contains(removed.as_str().expect("symbol text")),
        "removed={removed:?}; {error}"
    );
}

#[test]
fn phase5_manifest_requires_overlay_observation_exports() {
    let inventory = workspace_inventory();
    let mut document = product_manifest();
    let overlay = overlay_export_group_mut(&mut document);
    let symbols = overlay["symbols"].as_array_mut().expect("overlay symbols");
    let removed = symbols.pop().expect("overlay group has a public symbol");
    let error = audit(&inventory, &document).expect_err("missing overlay export must fail");
    assert!(error.contains("differs from its exact manifest"));
    assert!(
        error.contains(removed.as_str().expect("symbol text")),
        "removed={removed:?}; {error}"
    );
}

#[test]
fn phase5_overlay_observations_cannot_leave_the_inspection_audience() {
    let inventory = workspace_inventory();
    let mut document = product_manifest();
    overlay_export_group_mut(&mut document)["audience"] = toml::Value::String("app".to_owned());
    let error = audit(&inventory, &document).expect_err("overlay audience reassignment must fail");
    assert!(error.contains("belongs to another audience"), "{error}");
}

#[test]
fn phase5_manifest_rejects_duplicate_audience_ownership() {
    let document = product_manifest();
    let journeys = super::audit_journeys(
        &workspace_inventory(),
        ledger::tables(&document, "journey").expect("journeys"),
    )
    .expect("real journeys");
    let mut rows = ledger::tables(&document, "export_group")
        .expect("export groups")
        .clone();
    let duplicate = rows[0]
        .get("symbols")
        .and_then(toml::Value::as_array)
        .and_then(|symbols| symbols.first())
        .cloned()
        .expect("first symbol");
    rows[1]
        .get_mut("symbols")
        .and_then(toml::Value::as_array_mut)
        .expect("second symbols")
        .push(duplicate);
    let error = declared_exports(&rows, &journeys).expect_err("duplicate owner should fail");
    assert!(error.contains("duplicate audience ownership"));
}

#[test]
fn phase5_manifest_rejects_a_group_without_its_real_caller() {
    let inventory = workspace_inventory();
    let mut document = product_manifest();
    export_group_mut(&mut document, "app")
        .as_table_mut()
        .expect("app group")
        .insert(
            "caller".to_owned(),
            toml::Value::String("absent-caller".to_owned()),
        );
    let error = audit(&inventory, &document).expect_err("unknown caller should fail");
    assert!(error.contains("absent or belongs to another audience"));
}

#[test]
fn phase5_product_facade_rejects_wildcard_publication() {
    let mut exports = std::collections::BTreeSet::<ProductExport>::new();
    let error = collect_file_exports(
        "pub use worth_ui_runtime::facade::entry::*;",
        "app",
        "facade/app.rs",
        &mut exports,
    )
    .expect_err("glob should fail");
    assert!(error.contains("wildcard"));
}

#[test]
fn phase5_product_root_rejects_forwarding_around_named_audiences() {
    let error = audit_product_root_source("pub mod facade {}\npub use facade::app::WorthUi;\n")
        .expect_err("root forwarding should fail");
    assert!(error.contains("only `facade`"));
}

#[test]
fn phase5_app_audience_rejects_host_or_certification_authority() {
    for forbidden in [
        "WorthUiRuntime",
        "WorthUiHostAdapter",
        "WorthUiLaneCertification",
        "UiPreparedMountedFrame",
    ] {
        let error = super::reject_audience_authority("app", forbidden)
            .expect_err("forbidden app authority should fail");
        assert!(error.contains(forbidden));
    }
}

#[test]
fn phase5_inspection_audience_rejects_storage_or_materialization_authority() {
    for forbidden in [
        "FrozenCommandCapabilities",
        "UiEvidenceSliceAssembly",
        "WorthUiFrameReportMaterializationBoundary",
        "WorthUiSteadyFrameReportPlanner",
    ] {
        let error = super::reject_audience_authority("inspection", forbidden)
            .expect_err("forbidden inspection authority should fail");
        assert!(error.contains(forbidden));
    }
}

fn overlay_export_group_mut(document: &mut toml::Value) -> &mut toml::Value {
    document["export_group"]
        .as_array_mut()
        .expect("export groups")
        .iter_mut()
        .find(|row| {
            row["authority"].as_str()
                == Some("snapshot-bound overlay identity, typed failure, and shutdown observation")
        })
        .expect("overlay observation export group")
}
