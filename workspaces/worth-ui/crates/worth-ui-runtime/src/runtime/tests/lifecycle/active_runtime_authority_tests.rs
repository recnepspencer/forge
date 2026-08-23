use std::{collections::BTreeMap, path::Path};
use worth_ui_dsl::WorthUiSourceModuleId;

use crate::facade::{WorthUi, WorthUiApp};
use crate::runtime::{
    WorthUiRuntimeDiagnosticPolicy, WorthUiRuntimeFrameEpoch, WorthUiRuntimeLaunch,
    WorthUiRuntimeLifecycle,
};
use crate::source::{
    WorthUiArtifact, WorthUiArtifactHandle, WorthUiArtifactIdentitySeed,
    WorthUiArtifactImportHandle, WorthUiArtifactImportNode, WorthUiArtifactModule,
    WorthUiArtifactNode, WorthUiDurableStateEligibility, WorthUiDurableStateIneligibilityReason,
};

#[test]
fn equivalent_runtime_hosts_start_with_equivalent_active_state() {
    let app = WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("application preparation should succeed");
    let left = launch_runtime(&app, import_artifact(["app/panels/inspector.wui"]));
    let right = launch_runtime(&app, import_artifact(["app/panels/inspector.wui"]));

    assert_eq!(left.inspect_active(), right.inspect_active());
    assert_eq!(left.lifecycle(), WorthUiRuntimeLifecycle::Active);
    assert_eq!(left.frame_epoch(), WorthUiRuntimeFrameEpoch::initial());
}

#[test]
fn different_canonical_artifact_meaning_changes_active_runtime_truth() {
    let app = WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("application preparation should succeed");
    let inspector_runtime = launch_runtime(&app, import_artifact(["app/panels/inspector.wui"]));
    let settings_runtime = launch_runtime(&app, import_artifact(["app/panels/settings.wui"]));

    assert_ne!(
        inspector_runtime.inspect_active().artifact_digest(),
        settings_runtime.inspect_active().artifact_digest()
    );
    assert_eq!(
        inspector_runtime.inspect_active().active_plan_digest(),
        settings_runtime.inspect_active().active_plan_digest()
    );
    assert_ne!(
        inspector_runtime.last_valid(),
        settings_runtime.last_valid()
    );
}

#[test]
fn last_valid_state_exists_before_first_reload_candidate() {
    let app = WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("application preparation should succeed");
    let runtime = launch_runtime(&app, import_artifact(["app/panels/inspector.wui"]));
    let active = runtime.inspect_active();
    let last_valid = runtime.last_valid();

    assert!(last_valid.was_recorded_before_candidates());
    assert_eq!(last_valid.recorded_frame_epoch(), active.frame_epoch());
    assert_eq!(last_valid.artifact_digest(), active.artifact_digest());
    assert_eq!(last_valid.active_plan_digest(), active.active_plan_digest());
}

#[test]
fn diagnostic_policy_does_not_change_active_runtime_truth() {
    let app = WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("application preparation should succeed");
    let artifact = import_artifact(["app/panels/inspector.wui", "app/panels/settings.wui"]);
    let minimal = app
        .launch_runtime(
            WorthUiRuntimeLaunch::from_canonical_artifact(artifact.clone())
                .with_diagnostics(WorthUiRuntimeDiagnosticPolicy::minimal()),
        )
        .expect("minimal diagnostics runtime launches");
    let rich = app
        .launch_runtime(
            WorthUiRuntimeLaunch::from_canonical_artifact(artifact)
                .with_diagnostics(WorthUiRuntimeDiagnosticPolicy::rich()),
        )
        .expect("rich diagnostics runtime launches");

    assert_eq!(minimal.inspect_active(), rich.inspect_active());
    assert_eq!(minimal.last_valid(), rich.last_valid());
}

#[test]
fn shutdown_receipt_preserves_final_frame_epoch() {
    let app = WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("application preparation should succeed");
    let runtime = launch_runtime(&app, import_artifact(["app/panels/inspector.wui"]));
    let frame_epoch = runtime.frame_epoch();

    let receipt = runtime.shutdown();

    assert_eq!(receipt.final_frame_epoch(), frame_epoch);
}

fn launch_runtime(
    app: &WorthUiApp,
    artifact: WorthUiArtifact,
) -> crate::runtime::WorthUiRuntimeFrameworkLoop {
    app.launch_runtime(WorthUiRuntimeLaunch::from_canonical_artifact(artifact))
        .expect("runtime launches from canonical artifact")
}

fn import_artifact<const N: usize>(targets: [&str; N]) -> WorthUiArtifact {
    let module_id = module_id("app/main.wui");
    let nodes = targets
        .into_iter()
        .enumerate()
        .map(|(node_index, target)| import_node(&module_id, node_index, target))
        .collect::<Vec<_>>();
    let module = WorthUiArtifactModule::new(module_id.clone(), nodes);
    let mut modules = BTreeMap::new();
    modules.insert(module_id.clone(), module);

    WorthUiArtifact::new(modules, vec![module_id])
}

fn import_node(
    module_id: &WorthUiSourceModuleId,
    node_index: usize,
    target: &str,
) -> WorthUiArtifactNode {
    WorthUiArtifactNode::Import(WorthUiArtifactImportNode::new(
        WorthUiArtifactHandle::Import(WorthUiArtifactImportHandle::new(
            module_id.clone(),
            node_index,
        )),
        crate::source::test_compilation::semantic_import(target)
            .target()
            .clone(),
        0,
        WorthUiArtifactIdentitySeed::structural_fallback(format!(
            "module:{}|import:{}",
            module_id.as_str(),
            target
        )),
        WorthUiDurableStateEligibility::Ineligible {
            reason: WorthUiDurableStateIneligibilityReason::NoDurableStateSurface,
        },
    ))
}

fn module_id(path: &str) -> WorthUiSourceModuleId {
    WorthUiSourceModuleId::from_relative_path(Path::new(path)).expect("valid module id")
}
