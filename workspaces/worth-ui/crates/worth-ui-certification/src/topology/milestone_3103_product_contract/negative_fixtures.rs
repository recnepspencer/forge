use std::path::{Path, PathBuf};

use crate::topology::WorkspaceSourceInventory;

use super::{
    audit_live_product_contract, manifest_contract, source_contract, visual_identity_contract,
};

const WORKSPACE: &str = r#"
[workspace]
members = ["apps/platform-pulse"]
"#;

const MANIFEST: &str = r#"
[package]
name = "worth-ui-platform-pulse"
autotests = false
[lib]
name = "worth_ui_platform_pulse"
path = "src/lib.rs"
[[bin]]
name = "worth-ui-platform-pulse"
path = "src/main.rs"
[features]
executable-world = []
[[test]]
name = "executable_world"
path = "tests/executable_world.rs"
required-features = ["executable-world"]
[dependencies]
eframe = { workspace = true, features = ["wgpu"] }
notify = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
worth-query-decl = { workspace = true }
worth-query-host = { workspace = true }
worth-ui = { workspace = true }
worth-ui-host-egui = { workspace = true }
[target.'cfg(windows)'.dev-dependencies]
uiautomation = { workspace = true }
winsafe = { workspace = true }
xcap = { workspace = true, features = ["wgc"] }
"#;

#[test]
fn live_phase2_product_sources_satisfy_the_contract_without_a_workflow_flag() {
    audit_live_product_contract(&WorkspaceSourceInventory::capture(workspace_root()))
        .expect("live Phase 2 product architecture");
}

#[test]
fn manifest_rejects_a_forbidden_product_dependency() {
    let mutated = MANIFEST.replace(
        "[dependencies]",
        "[dependencies]\nworth-ui-runtime = { workspace = true }",
    );
    let error = manifest_contract::audit(WORKSPACE, &mutated)
        .expect_err("a runtime deep dependency must fail");
    assert!(error.contains("dependency surface"));
}

#[test]
fn manifest_rejects_an_extra_workflow_feature() {
    let mutated = MANIFEST.replace(
        "executable-world = []",
        "executable-world = []\nshortcut-world = []",
    );
    let error = manifest_contract::audit(WORKSPACE, &mutated)
        .expect_err("extra workflow feature must fail");
    assert!(error.contains("only"));
}

#[test]
fn manifest_rejects_renderer_and_capture_feature_drift() {
    let missing_renderer = MANIFEST.replace(
        r#"eframe = { workspace = true, features = ["wgpu"] }"#,
        "eframe = { workspace = true }",
    );
    let error = manifest_contract::audit(WORKSPACE, &missing_renderer)
        .expect_err("missing governed renderer feature must fail");
    assert!(error.contains("`eframe` contract drifted"), "{error}");

    let widened_capture = MANIFEST.replace(
        r#"xcap = { workspace = true, features = ["wgc"] }"#,
        r#"xcap = { workspace = true, features = ["wgc", "shortcut"] }"#,
    );
    let error = manifest_contract::audit(WORKSPACE, &widened_capture)
        .expect_err("unplanned native capture feature must fail");
    assert!(error.contains("`xcap` contract drifted"), "{error}");
}

#[test]
fn library_rejects_exporting_application_implementation() {
    let error = source_contract::audit_library_surface(
        "pub mod observation_contract;\npub mod application;",
    )
    .expect_err("application implementation cannot leave the binary");
    assert!(error.contains("only"));
}

#[test]
fn product_source_rejects_an_executable_world_feature_branch() {
    let error = source_contract::audit_source_posture(
        Path::new("apps/platform-pulse/src/main.rs"),
        "#[cfg(feature = \"executable_world\")] fn main() {}",
    )
    .expect_err("product source feature branch must fail");
    assert!(error.contains("cannot branch"));
}

#[test]
fn protocol_rejects_a_missing_lifecycle_outcome() {
    let envelope = r#"
const SCHEMA_VERSION: u16 = 5;
const MAXIMUM_ENCODED_OBSERVATION_BYTES: usize = 1_048_576;
const ID: &str = "worth-ui.platform-pulse.lifecycle-observation";
const PREFIX: &str = "WORTH_UI_PLATFORM_PULSE_EVENT ";
"#;
    let lifecycle = r#"
pub enum PlatformPulseLifecycleObservation {
    ProcessStarted(PlatformPulseProcessStarted),
    FirstFramePublished(PlatformPulseFirstFramePublished),
    NativeInputReached(PlatformPulseNativeInputReached),
    QueryProjectionIssued(PlatformPulseQueryProjectionIssued),
    QueryProjectionPublished(PlatformPulseQueryProjectionPublished),
    VisualSnapshotCaptured(PlatformPulseVisualSnapshotCaptured),
    VisualPointTrace(PlatformPulseVisualPointTrace),
    VisualOverlayPublished(PlatformPulseVisualOverlayPublished),
    VisualOverlayCleared(PlatformPulseVisualOverlayCleared),
    VisualSnapshotRetired(PlatformPulseVisualSnapshotRetired),
    RebindPublished(PlatformPulseReplacementPublished),
    VisualComparison(PlatformPulseVisualComparison),
    ShutdownCompleted(PlatformPulseShutdownCompleted),
    TerminalFailure(PlatformPulseTerminalFailure),
}
pub struct PlatformPulseProcessStarted {}
pub struct PlatformPulseFirstFramePublished { value: u64 }
pub struct PlatformPulseNativeInputReached { value: u64 }
pub struct PlatformPulseQueryProjectionIssued { value: u64 }
pub struct PlatformPulseQueryProjectionPublished { value: u64 }
pub struct PlatformPulseVisualSnapshotCaptured { value: u64 }
pub struct PlatformPulseVisualPointTrace { value: u64 }
pub struct PlatformPulseVisualOverlayPublished { value: u64 }
pub struct PlatformPulseVisualOverlayCleared { value: u64 }
pub struct PlatformPulseVisualSnapshotRetired { value: u64 }
pub struct PlatformPulseReplacementPublished { value: u64 }
pub struct PlatformPulseVisualComparison { value: u64 }
pub struct PlatformPulseShutdownCompleted { value: u64 }
pub struct PlatformPulseTerminalFailure { value: u64 }
"#;
    let error = source_contract::audit_protocol(envelope, lifecycle)
        .expect_err("preservation outcome cannot disappear");
    assert!(error.contains("variants"));
}

#[test]
fn protocol_rejects_public_raw_payload_fields() {
    let envelope = r#"
const SCHEMA_VERSION: u16 = 5;
const MAXIMUM_ENCODED_OBSERVATION_BYTES: usize = 1_048_576;
const ID: &str = "worth-ui.platform-pulse.lifecycle-observation";
const PREFIX: &str = "WORTH_UI_PLATFORM_PULSE_EVENT ";
"#;
    let lifecycle = r#"
pub enum PlatformPulseLifecycleObservation {
    ProcessStarted(PlatformPulseProcessStarted),
    FirstFramePublished(PlatformPulseFirstFramePublished),
    NativeInputReached(PlatformPulseNativeInputReached),
    QueryProjectionIssued(PlatformPulseQueryProjectionIssued),
    QueryProjectionPublished(PlatformPulseQueryProjectionPublished),
    VisualSnapshotCaptured(PlatformPulseVisualSnapshotCaptured),
    VisualPointTrace(PlatformPulseVisualPointTrace),
    VisualOverlayPublished(PlatformPulseVisualOverlayPublished),
    VisualOverlayCleared(PlatformPulseVisualOverlayCleared),
    VisualSnapshotRetired(PlatformPulseVisualSnapshotRetired),
    RebindPublished(PlatformPulseReplacementPublished),
    RebindDeniedPreserving(PlatformPulseReplacementPreserved),
    VisualComparison(PlatformPulseVisualComparison),
    ShutdownCompleted(PlatformPulseShutdownCompleted),
    TerminalFailure(PlatformPulseTerminalFailure),
}
pub struct PlatformPulseProcessStarted {}
pub struct PlatformPulseFirstFramePublished { pub frame: u64 }
pub struct PlatformPulseNativeInputReached { value: u64 }
pub struct PlatformPulseQueryProjectionIssued { value: u64 }
pub struct PlatformPulseQueryProjectionPublished { value: u64 }
pub struct PlatformPulseVisualSnapshotCaptured { value: u64 }
pub struct PlatformPulseVisualPointTrace { value: u64 }
pub struct PlatformPulseVisualOverlayPublished { value: u64 }
pub struct PlatformPulseVisualOverlayCleared { value: u64 }
pub struct PlatformPulseVisualSnapshotRetired { value: u64 }
pub struct PlatformPulseReplacementPublished { value: u64 }
pub struct PlatformPulseReplacementPreserved { value: u64 }
pub struct PlatformPulseVisualComparison { value: u64 }
pub struct PlatformPulseShutdownCompleted { value: u64 }
pub struct PlatformPulseTerminalFailure { value: u64 }
"#;
    let error = source_contract::audit_protocol(envelope, lifecycle)
        .expect_err("caller-mintable payload must fail");
    assert!(error.contains("private"));
}

#[test]
fn projection_rejects_replacing_a_publication_receipt_with_a_raw_frame() {
    let live = canonical_live_projection().replace(
        "publication: &UiMountedFramePublicationReceipt",
        "publication: u64",
    );
    let error = source_contract::audit_projection_contract(&live, canonical_terminal(), "")
        .expect_err("raw frame input cannot project first publication");
    assert!(error.contains("receipt-derived"));
}

#[test]
fn projection_rejects_caller_supplied_preserved_generation() {
    let live = canonical_live_projection().replace(
        "denial: &UiSourceRebindAttemptFailure,",
        "generation: u64, denial: &UiSourceRebindAttemptFailure,",
    );
    let error = source_contract::audit_projection_contract(&live, canonical_terminal(), "")
        .expect_err("preservation must use stream-owned predecessor");
    assert!(error.contains("receipt-derived"));
}

#[test]
fn unchanged_frame_rejects_observation_publication() {
    let source = "Ok(UiMountedFrameOutcome::Unchanged(_)) if self.initial_source.is_none() => { self.publisher.changed(); }";
    let error = source_contract::audit_unchanged_frame(source)
        .expect_err("unchanged frame publication must fail");
    assert!(error.contains("zero observation work"));
}

#[test]
fn visual_product_rejects_test_only_and_direct_egui_shortcuts() {
    let inventory = WorkspaceSourceInventory::capture(workspace_root());
    let sources = visual_identity_contract::ProductVisualIdentitySources::capture(&inventory);

    let mut test_only = sources.clone();
    test_only
        .execution
        .push_str("\n#[cfg(test)] fn pulse() {}\n");
    let error = visual_identity_contract::audit_sources(&test_only)
        .expect_err("test-only product behavior must fail");
    assert!(error.contains("#[cfg(test)]"));

    let mut direct_egui = sources;
    direct_egui
        .execution
        .push_str("\nfn paint(context: &egui::Context) {}\n");
    let error = visual_identity_contract::audit_sources(&direct_egui)
        .expect_err("direct product egui drawing must fail");
    assert!(error.contains("egui::"));
}

#[test]
fn visual_product_rejects_missing_grant_and_wrong_target_oracles() {
    let inventory = WorkspaceSourceInventory::capture(workspace_root());
    let sources = visual_identity_contract::ProductVisualIdentitySources::capture(&inventory);

    let mut missing_grant = sources.clone();
    missing_grant.execution = mutate_required_edge(
        &missing_grant.execution,
        "issue_overlay_grant()",
        "assume_overlay_authority()",
    );
    let error = visual_identity_contract::audit_sources(&missing_grant)
        .expect_err("missing overlay grant must fail");
    assert!(error.contains("issue_overlay_grant"));

    let mut wrong_target = sources;
    wrong_target.adjudication = wrong_target.adjudication.replace(
        "PLATFORM_PULSE_IDENTITY_TARGET_AUTHORED_NAME",
        "CALLER_SUPPLIED_TARGET_NAME",
    );
    let error = visual_identity_contract::audit_sources(&wrong_target)
        .expect_err("wrong target oracle must fail");
    assert!(error.contains("PLATFORM_PULSE_IDENTITY_TARGET_AUTHORED_NAME"));
}

#[test]
fn visual_wire_rejects_pixel_payload_and_authority_reentry() {
    let inventory = WorkspaceSourceInventory::capture(workspace_root());
    let sources = visual_identity_contract::ProductVisualIdentitySources::capture(&inventory);

    let mut pixel_payload = sources.clone();
    pixel_payload
        .wire
        .push_str("\ntype CounterfeitScreenshot = Vec<u8>;\n");
    let error = visual_identity_contract::audit_sources(&pixel_payload)
        .expect_err("wire screenshot bytes must fail");
    assert!(error.contains("Vec<u8>"));

    let mut reentry = sources;
    reentry
        .projection
        .push_str("\nfn counterfeit() { UiVisualOverlayTarget::from_wire(); }\n");
    let error = visual_identity_contract::audit_sources(&reentry)
        .expect_err("wire identity re-entry must fail");
    assert!(error.contains("UiVisualOverlayTarget::"));
}

#[test]
fn visual_publication_rejects_reopening_positional_point_adjudication() {
    let inventory = WorkspaceSourceInventory::capture(workspace_root());
    let mut sources = visual_identity_contract::ProductVisualIdentitySources::capture(&inventory);
    sources.publication = sources.publication.replace(
        "input: PlatformPulseVisualPointTraceInput<'_>",
        "target_point: UiClientPhysicalPixel,\n        target: &UiVisualPointAdjudication,\n        \
         background_point: UiClientPhysicalPixel,\n        background: &UiVisualPointAdjudication",
    );
    let error = visual_identity_contract::audit_sources(&sources)
        .expect_err("positional point/adjudication pairs must fail");
    assert!(error.contains("typed visual trace publication"), "{error}");
}

fn canonical_live_projection() -> String {
    r#"
pub fn project_first_frame(
    &mut self,
    source: &WorthUiSourcePackageRevision,
    publication: &UiMountedFramePublicationReceipt,
) { actual_native_effect_count: publication.cost_report().adapter().translated_rows() }
pub fn project_replacement(
    &mut self,
    source: &WorthUiSourcePackageRevision,
    application: &WorthUiApplicationCutoverReceipt,
    mounted: &UiMountedFramePublicationReceipt,
) { actual_native_effect_count: mounted.cost_report().adapter().translated_rows() }
pub fn project_preserved_predecessor(
    &mut self,
    source: &WorthUiSourcePackageRevision,
    denial: &UiSourceRebindAttemptFailure,
) {}
"#
    .to_owned()
}

fn canonical_terminal() -> &'static str {
    r#"
pub fn project_shutdown(
    &mut self,
    watcher: &WorthUiFilesystemWatcherShutdownReceipt,
    application: WorthUiNativeApplicationShutdownReceipt,
) {}
"#
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates")
        .parent()
        .expect("workspace")
        .to_path_buf()
}

fn mutate_required_edge(source: &str, edge: &str, counterfeit: &str) -> String {
    assert!(
        source.contains(edge),
        "negative fixture cannot exercise absent edge `{edge}`"
    );
    source.replacen(edge, counterfeit, 1)
}
