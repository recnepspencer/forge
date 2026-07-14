use super::{declare, preview, WorthQueryWorkflowStopSource};
use crate::ordinary::mutation;
use crate::runtime::{
    tests::support::stateful_bridge_task_runtime, WorthQueryAspectTouch,
    WorthQueryAuthoredAspectValue,
};
use crate::session_label::WorthQuerySessionLabel;

fn label(name: &str) -> WorthQuerySessionLabel {
    WorthQuerySessionLabel::scoped_strs("ordinary-workflow-tests", [name])
        .expect("label should build")
}

fn mutation(title: &str) -> mutation::WorthQueryMutationDeclaration {
    mutation::declare(|builder| {
        builder
            .set_aspect(
                WorthQueryAspectTouch::from_authoring_ingress_text("identity.id")?,
                WorthQueryAuthoredAspectValue::string(format!("task-{title}")),
            )
            .set_aspect(
                WorthQueryAspectTouch::from_authoring_ingress_text("title.value")?,
                WorthQueryAuthoredAspectValue::string(title),
            )
            .build_insert("Task")
    })
    .expect("mutation should declare")
}

#[test]
fn ordinary_workflow_matches_explicit_lower_runtime_orchestration_identities() {
    let session_label = label("parity");
    let declaration = declare(session_label.clone(), mutation("parity"));
    let explicit_mutation = declaration.mutation().clone();
    let explicit_identity = declaration.identity().evidence_identity().clone();
    let mut explicit_workspace = stateful_bridge_task_runtime()
        .workspace("ordinary-workflow-explicit")
        .expect("workspace should open");
    let explicit = explicit_workspace
        .execute_ordinary_preview_promotion(
            session_label.clone(),
            &explicit_identity,
            explicit_mutation.into_command(),
        )
        .expect("explicit orchestration should complete");

    let mut ordinary_workspace = stateful_bridge_task_runtime()
        .workspace("ordinary-workflow-declarative")
        .expect("workspace should open");
    let context = preview(&ordinary_workspace, session_label).expect("context should admit");
    let outcome = declaration.using(context).run(&mut ordinary_workspace);
    let completion = outcome
        .completed()
        .expect("ordinary workflow should complete");

    assert_eq!(
        completion.lowered_plan().request_identity(),
        explicit.request_identity()
    );
    assert_eq!(
        completion.aftermath().receipt_identity(),
        explicit.receipt_identity()
    );
    assert_eq!(
        completion.aftermath().identity(),
        explicit.aftermath_identity()
    );
    assert_eq!(
        completion.aftermath().inspection_identity(),
        explicit.inspection_identity()
    );
}

#[test]
fn cross_session_denial_does_no_lower_runtime_work_and_leaves_no_label_residue() {
    let declaration_label = label("cross-session");
    let wrong_label = label("foreign-session");
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("ordinary-workflow-cross-session")
        .expect("workspace should open");
    let wrong_context = preview(&workspace, wrong_label).expect("context should admit");
    let stopped = declare(declaration_label.clone(), mutation("cross-session"))
        .using(wrong_context)
        .run(&mut workspace);
    let stop = stopped.stop().expect("session mismatch should stop");
    assert_eq!(stop.source(), WorthQueryWorkflowStopSource::CrossSession);
    assert_eq!(stop.counters().session_open_attempt_count(), 0);
    assert_eq!(stop.counters().lower_runtime_execution_attempt_count(), 0);

    let matching = preview(&workspace, declaration_label.clone()).expect("context should admit");
    let retry = declare(declaration_label, mutation("cross-session-retry"))
        .using(matching)
        .run(&mut workspace);
    assert!(
        retry.completed().is_some(),
        "same label must remain available"
    );
}

#[test]
fn foreign_authority_and_stale_preview_deny_before_session_open() {
    let foreign_label = label("foreign-authority");
    let left = stateful_bridge_task_runtime()
        .workspace("ordinary-workflow-left")
        .expect("left should open");
    let foreign_context = preview(&left, foreign_label.clone()).expect("context should admit");
    let mut right = stateful_bridge_task_runtime()
        .workspace("ordinary-workflow-right")
        .expect("right should open");
    let foreign = declare(foreign_label, mutation("foreign"))
        .using(foreign_context)
        .run(&mut right);
    let stop = foreign.stop().expect("foreign authority should stop");
    assert_eq!(
        stop.source(),
        WorthQueryWorkflowStopSource::ForeignAuthority
    );
    assert_eq!(stop.counters().session_open_attempt_count(), 0);

    let stale_label = label("stale");
    let stale_context = preview(&right, stale_label.clone()).expect("context should admit");
    right
        .insert("Task", |builder| {
            builder.set_aspect(
                WorthQueryAspectTouch::from_authoring_ingress_text("identity.id")
                    .expect("touch should build"),
                WorthQueryAuthoredAspectValue::string("stale-basis-advance"),
            )
        })
        .expect("authoritative mutation should advance basis");
    let stale = declare(stale_label, mutation("stale"))
        .using(stale_context)
        .run(&mut right);
    let stop = stale.stop().expect("stale preview should stop");
    assert_eq!(stop.source(), WorthQueryWorkflowStopSource::StalePreview);
    assert_eq!(stop.counters().session_open_attempt_count(), 0);
}

#[test]
fn unsupported_writeback_denies_before_open_and_preserves_retry_authority() {
    let session_label = label("writeback-denial");
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("ordinary-workflow-writeback")
        .expect("workspace should open");
    let context = preview(&workspace, session_label.clone()).expect("context should admit");
    let stopped = declare(session_label.clone(), mutation("writeback"))
        .deferred_writeback()
        .using(context)
        .run(&mut workspace);
    let stop = stopped.stop().expect("writeback should deny");
    assert_eq!(
        stop.source(),
        WorthQueryWorkflowStopSource::UnsupportedWriteback
    );
    assert_eq!(stop.counters().session_open_attempt_count(), 0);

    let retry_context = preview(&workspace, session_label.clone()).expect("context should admit");
    let retry = declare(session_label, mutation("writeback-retry"))
        .using(retry_context)
        .run(&mut workspace);
    assert!(retry.completed().is_some());
}
