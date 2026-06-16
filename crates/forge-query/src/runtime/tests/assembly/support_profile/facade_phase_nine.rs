use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use super::super::super::support::*;
use crate::memory_workspace::ForgeQueryCommitIdentity;

#[derive(Clone)]
struct PhaseNineMaintainer {
    invocations: Arc<AtomicUsize>,
}

impl ForgeQueryDerivedViewMaintainer for PhaseNineMaintainer {
    fn maintain(
        &mut self,
        view: &crate::program::ForgeQueryDerivedView,
        _delta: &crate::memory_workspace::ForgeQueryMutationDelta,
        materialization: &mut ForgeQueryDerivedViewMaterialization,
    ) -> ForgeQueryDerivedPatch {
        let next = self.invocations.fetch_add(1, Ordering::SeqCst) + 1;
        materialization.replace_rows([json!({ "title": { "value": "Phase Nine" } })]);
        ForgeQueryDerivedPatch::whole_refresh_materialized(
            view.name(),
            crate::memory_workspace::admit_external_commit_label(format!(
                "phase-nine-refresh-{next}"
            )),
            ["title.value".to_string()],
            json!({"published": true, "title": "Phase Nine"}),
            format!("phase-nine-publication-{next}"),
        )
    }
}

#[test]
fn phase_nine_facade_families_are_supported_and_admitted() {
    let workspace = stateful_bridge_task_runtime()
        .workspace("task.phase-nine-support")
        .expect("workspace should build");

    let contract = workspace.public_api_contract();
    for family in [
        ForgeQueryRuntimeFacadeFamily::Read,
        ForgeQueryRuntimeFacadeFamily::Live,
        ForgeQueryRuntimeFacadeFamily::Computed,
        ForgeQueryRuntimeFacadeFamily::Effect,
        ForgeQueryRuntimeFacadeFamily::BranchPreview,
        ForgeQueryRuntimeFacadeFamily::Write,
        ForgeQueryRuntimeFacadeFamily::Inspect,
    ] {
        assert_eq!(
            contract
                .family(family)
                .expect("legacy family must be present")
                .owner_closure(),
            "Milestone 9.3"
        );
    }

    for family in [
        ForgeQueryRuntimeFacadeFamily::SharedRead,
        ForgeQueryRuntimeFacadeFamily::Submission,
    ] {
        let support = contract
            .family(family)
            .expect("phase-nine family must be present");
        assert_eq!(
            support.status(),
            ForgeQueryRuntimeFamilySupportStatus::Supported
        );
        assert_eq!(
            support.teaching_posture(),
            ForgeQueryRuntimeFamilyTeachingPosture::OrdinaryRuntimeDx
        );
        assert_eq!(support.owner_closure(), "Milestone 9.7");
        assert!(!support.admission_fail_closed());
        workspace
            .admit_public_api_family(family)
            .expect("supported phase-nine family should admit");
    }
}

#[test]
fn workspace_submission_lane_matches_existing_workspace_convenience_receipts() {
    let command = ForgeQueryAspectMutationBuilder::new()
        .aspect("identity.id", "task-1")
        .aspect("title.value", "Phase Nine")
        .build_insert("Task")
        .expect("insert command should build");

    let convenience_receipt = stateful_bridge_task_runtime()
        .workspace("task.phase-nine-convenience")
        .expect("workspace should build")
        .write(command.clone())
        .expect("workspace convenience write should succeed");
    let submission_receipt = {
        let mut workspace = stateful_bridge_task_runtime()
            .workspace("task.phase-nine-submission")
            .expect("workspace should build");
        workspace
            .submissions()
            .expect("submission lane should mint")
            .submit(command)
            .expect("submission lane write should succeed")
    };

    assert_eq!(
        convenience_receipt.commit_identity(),
        submission_receipt.commit_identity()
    );
    assert_eq!(
        mutation_receipt_summary(&convenience_receipt),
        mutation_receipt_summary(&submission_receipt)
    );
}

#[test]
fn workspace_shared_read_lane_matches_runtime_owned_context_without_recomputation() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("task.phase-nine-shared-read")
        .expect("workspace should build");
    let live: ForgeQueryLiveView<Value> = workspace
        .live_view_request("tasks.phase-nine", task_live_request(), task_schema())
        .expect("live view should declare");
    let invocations = Arc::new(AtomicUsize::new(0));
    let derived: ForgeQueryDerivedViewHandle<Value> = workspace
        .computed_view(
            crate::program::ForgeQueryDerivedView::new(
                "derived.phase-nine",
                ["title.value".to_string()],
            )
            .depends_on_live(&live),
            PhaseNineMaintainer {
                invocations: Arc::clone(&invocations),
            },
        )
        .expect("derived view should declare");

    workspace
        .insert("Task", |builder| {
            builder
                .aspect("identity.id", "task-1")
                .aspect("title.value", "Phase Nine")
        })
        .expect("insert should publish derived artifact");

    let facade_artifact = workspace
        .shared_read_context()
        .expect("workspace shared read context should mint")
        .published_derived_artifact(&derived)
        .expect("workspace shared read artifact should mint");
    let decomposed_artifact = ForgeQuerySharedReadContext::from_runtime(&workspace.runtime)
        .published_derived_artifact(&derived)
        .expect("runtime shared read artifact should mint");

    assert_eq!(facade_artifact, decomposed_artifact);
    assert_eq!(
        invocations.load(Ordering::SeqCst),
        1,
        "shared read parity must not trigger recomputation"
    );
}

fn mutation_receipt_summary(receipt: &ForgeQueryWriteReceipt) -> serde_json::Value {
    json!({
        "snapshot_identity": receipt.snapshot_evidence_identity().as_str().to_string(),
        "authority_lane": receipt.authority_lane().as_str(),
        "mutation_family": receipt.mutation_family().as_str(),
        "declared_collection": receipt.declared_collection(),
        "declared_entity_identity": receipt
            .declared_entity_identity()
            .map(|identity| identity.terminal_projection_for_reporting().to_string()),
        "target_collection": receipt.target_collection(),
        "target_entity_identity": receipt
            .target_entity_identity()
            .map(|identity| identity.terminal_projection_for_reporting().to_string()),
        "declared_aspect_value_digest": receipt.declared_aspect_value_digest(),
        "affected_live_view_ids": receipt.affected_live_view_ids(),
        "affected_derived_view_ids": receipt.affected_derived_view_ids(),
        "pending_write_intent_count": receipt.pending_write_intent_count(),
        "suppressed_effect_count": receipt.suppressed_effect_count(),
        "meaningful_effect_suppression_count": receipt.meaningful_effect_suppression_count(),
        "effect_expression_failure_count": receipt.effect_expression_failure_count(),
        "refresh_fallback": receipt.refresh_fallback(),
        "deltas": receipt
            .deltas()
            .iter()
            .map(|delta| {
                json!({
                    "collection": delta.collection,
                    "entity_identity": delta.entity_identity.terminal_projection_for_reporting().to_string(),
                    "kind": format!("{:?}", delta.kind),
                    "aspect_paths": delta.aspect_paths,
                })
            })
            .collect::<Vec<_>>(),
    })
}
