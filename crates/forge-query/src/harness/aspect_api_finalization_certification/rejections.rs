use crate::runtime::{ForgeQueryIntentDeclaration, ForgeQueryRuntimeError};

use super::fixture::stateful_bridge_task_runtime;
use super::{AspectApiFinalizationFailureClass, AspectApiFinalizationRejectionBundle};

pub(super) fn unsupported_intent_rejection() -> AspectApiFinalizationRejectionBundle {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("aspect-api.intent-denial")
        .expect("workspace should open");
    let report = workspace.public_mutation_surface_report();
    let closeout = workspace.public_aspect_api_finalization_closeout();
    let error = workspace
        .intent(ForgeQueryIntentDeclaration::strategy_commit(
            "unsupported-intent",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            serde_json::json!({ "entity": "task-1" }),
        ))
        .expect_err("unsupported runtime should deny intent typed and early");

    match error {
        ForgeQueryRuntimeError::UnsupportedFacadeFamily(denial) => {
            AspectApiFinalizationRejectionBundle {
                failure_class: AspectApiFinalizationFailureClass::SupportDenied,
                failure_kind: denial.family().to_string(),
                failure_digest: crate::harness::certification::digest_parts(&[
                    denial.family().to_string(),
                    denial.reason().to_string(),
                ]),
                support_matrix_digest: workspace
                    .public_support_matrix()
                    .matrix_digest()
                    .terminal_projection_for_reporting()
                    .to_string(),
                mutation_surface_report_digest: report.report_digest().to_string(),
                closeout_digest: closeout.closeout_digest().to_string(),
            }
        }
        other => panic!("expected typed support denial, got {other:?}"),
    }
}

pub(super) fn duplicate_aspect_authoring_rejection() -> AspectApiFinalizationRejectionBundle {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("aspect-api.duplicate-denial")
        .expect("workspace should open");
    let report = workspace.public_mutation_surface_report();
    let closeout = workspace.public_aspect_api_finalization_closeout();
    let error = workspace
        .update(
            crate::memory_workspace::admit_authored_entity_label("entity:1:1:1"),
            |task| task.clear("title.value").aspect("title.value", "Buy milk"),
        )
        .expect_err("duplicate aspect authoring should fail closed");

    match error {
        ForgeQueryRuntimeError::Workspace(error) => AspectApiFinalizationRejectionBundle {
            failure_class: AspectApiFinalizationFailureClass::AuthoringDenied,
            failure_kind: "workspace-authoring".to_string(),
            failure_digest: crate::harness::certification::digest_parts(&[error.to_string()]),
            support_matrix_digest: workspace
                .public_support_matrix()
                .matrix_digest()
                .terminal_projection_for_reporting()
                .to_string(),
            mutation_surface_report_digest: report.report_digest().to_string(),
            closeout_digest: closeout.closeout_digest().to_string(),
        },
        other => panic!("expected workspace authoring denial, got {other:?}"),
    }
}
