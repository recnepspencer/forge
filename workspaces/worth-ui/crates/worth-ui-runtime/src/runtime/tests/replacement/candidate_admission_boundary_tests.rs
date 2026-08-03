use std::{collections::BTreeMap, path::Path};
use worth_ui_dsl::WorthUiSourceModuleId;

use crate::facade::{WorthUi, WorthUiApp};
use crate::runtime::replacement::admission::{
    WorthUiCandidateAdmission, WorthUiCandidateAdmissionDenial, WorthUiRuntimeReplacementPosture,
};
use crate::runtime::replacement::candidate::rust_authored_replacement_candidate;
use crate::runtime::{
    WorthUiCandidateArtifactBundle, WorthUiCandidateDependencyMetadata,
    WorthUiCandidateLoweringBasis, WorthUiReplacementCandidate, WorthUiReplacementCause,
    WorthUiRuntimeLaunch,
};
use crate::source::{
    WorthUiArtifact, WorthUiArtifactDigestor, WorthUiArtifactEquivalenceBasis,
    WorthUiArtifactHandle, WorthUiArtifactIdentitySeed, WorthUiArtifactImportHandle,
    WorthUiArtifactImportNode, WorthUiArtifactModule, WorthUiArtifactNode,
    WorthUiDurableStateEligibility, WorthUiDurableStateIneligibilityReason,
};

mod candidate_admission_artifact_nodes;
use candidate_admission_artifact_nodes::{import_node, module_id};

#[test]
fn same_candidate_and_same_active_basis_admit_equivalently() {
    let app = WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .expect("application preparation should succeed");
    let left = replacement_candidate(&app, ["app/panels/inspector.wui"]);
    let right = replacement_candidate(&app, ["app/panels/inspector.wui"]);
    let runtime = launch_runtime(&app, import_artifact(["app/panels/inspector.wui"]));
    let active_basis = runtime.replacement_admission_basis();

    let left_admitted = WorthUiCandidateAdmission::for_active_basis(active_basis)
        .admit(left)
        .expect("left candidate admits");
    let right_admitted = WorthUiCandidateAdmission::for_active_basis(active_basis)
        .admit(right)
        .expect("right candidate admits");

    assert_eq!(left_admitted.report(), right_admitted.report());
    assert_eq!(left_admitted.active_basis(), active_basis);
}

#[test]
fn snapshot_mismatch_rejected_before_equivalence_comparison() {
    let app = WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .expect("application preparation should succeed");
    let runtime = launch_runtime(&app, import_artifact(["app/panels/inspector.wui"]));
    let active_basis = runtime.replacement_admission_basis();
    let candidate = candidate_with_lowering_basis(
        ["app/panels/inspector.wui"],
        WorthUiCandidateLoweringBasis::from_raw_parts_for_test(
            active_basis.snapshot_digest() ^ 0x55aa,
        ),
    );

    let report = WorthUiCandidateAdmission::for_active_basis(active_basis)
        .admit(candidate)
        .expect_err("mismatched snapshot denies");

    assert_eq!(
        report.denial(),
        Some(WorthUiCandidateAdmissionDenial::SnapshotMismatch {
            candidate_snapshot_digest: active_basis.snapshot_digest() ^ 0x55aa,
            active_snapshot_digest: active_basis.snapshot_digest(),
        })
    );
    assert_eq!(report.counters().snapshot_compatibility_checks(), 1);
    assert_eq!(report.counters().runtime_posture_checks(), 0);
    assert_eq!(report.counters().artifact_comparisons(), 0);
    assert_eq!(report.counters().plan_lowering_attempts(), 0);
}

#[test]
fn deferred_runtime_posture_rejected_before_plan_lowering() {
    let app = WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .expect("application preparation should succeed");
    let runtime = launch_runtime(&app, import_artifact(["app/panels/inspector.wui"]));
    let active_basis = runtime
        .replacement_admission_basis()
        .with_replacement_posture_for_test(WorthUiRuntimeReplacementPosture::Deferred);
    let candidate = replacement_candidate(&app, ["app/panels/inspector.wui"]);

    let report = WorthUiCandidateAdmission::for_active_basis(active_basis)
        .admit(candidate)
        .expect_err("deferred runtime posture denies");

    assert_eq!(
        report.denial(),
        Some(WorthUiCandidateAdmissionDenial::DeferredRuntimePosture {
            posture: WorthUiRuntimeReplacementPosture::Deferred,
        })
    );
    assert_eq!(report.counters().runtime_posture_checks(), 1);
    assert_eq!(report.counters().artifact_comparisons(), 0);
    assert_eq!(report.counters().plan_lowering_attempts(), 0);
}

#[test]
fn unsupported_runtime_posture_rejected_before_plan_lowering() {
    let app = WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .expect("application preparation should succeed");
    let runtime = launch_runtime(&app, import_artifact(["app/panels/inspector.wui"]));
    let active_basis = runtime
        .replacement_admission_basis()
        .with_replacement_posture_for_test(WorthUiRuntimeReplacementPosture::Unsupported);
    let candidate = replacement_candidate(&app, ["app/panels/inspector.wui"]);

    let report = WorthUiCandidateAdmission::for_active_basis(active_basis)
        .admit(candidate)
        .expect_err("unsupported runtime posture denies");

    assert_eq!(
        report.denial(),
        Some(WorthUiCandidateAdmissionDenial::UnsupportedRuntimePosture {
            posture: WorthUiRuntimeReplacementPosture::Unsupported,
        })
    );
    assert_eq!(report.counters().runtime_posture_checks(), 1);
    assert_eq!(report.counters().artifact_comparisons(), 0);
    assert_eq!(report.counters().plan_lowering_attempts(), 0);
}

fn replacement_candidate<const N: usize>(
    app: &WorthUiApp,
    targets: [&str; N],
) -> WorthUiReplacementCandidate {
    rust_authored_replacement_candidate(
        import_artifact(targets),
        app.capabilities().digest(),
        WorthUiReplacementCause::manual_refresh(1),
    )
    .expect("candidate seals")
}

fn candidate_with_lowering_basis<const N: usize>(
    targets: [&str; N],
    lowering_basis: WorthUiCandidateLoweringBasis,
) -> WorthUiReplacementCandidate {
    let artifact = import_artifact(targets);
    let artifact_digest =
        WorthUiArtifactDigestor::digest(&artifact, WorthUiArtifactEquivalenceBasis::semantic());
    let dependency_metadata = WorthUiCandidateDependencyMetadata::derive_for_artifact(&artifact);
    let bundle = WorthUiCandidateArtifactBundle::from_optional_parts_for_test(
        artifact,
        Some(artifact_digest),
        Some(dependency_metadata),
        Some(lowering_basis),
    )
    .expect("test candidate bundle seals");
    WorthUiReplacementCandidate::from_artifact_bundle(
        bundle,
        WorthUiReplacementCause::manual_refresh(2),
        crate::runtime::WorthUiCandidateAuthoringLane::rust_authored(),
    )
    .expect("test candidate seals")
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
