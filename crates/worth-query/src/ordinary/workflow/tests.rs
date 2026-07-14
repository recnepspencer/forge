use super::{declare, preview, WorthQueryWorkflowStopSource, WorthQueryWorkflowViolationKind};
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
    let declaration = declare(session_label.clone(), mutation("parity")).with_rich_inspection();
    let explicit_mutation = declaration.mutation().clone();
    let explicit_identity = declaration.identity().evidence_identity().clone();
    let mut explicit_workspace = stateful_bridge_task_runtime()
        .workspace("ordinary-workflow-explicit")
        .expect("workspace should open");
    let explicit_context =
        preview(&explicit_workspace, session_label.clone()).expect("context should admit");
    let explicit_basis = explicit_context
        .authority
        .into_preview_basis()
        .expect("context should carry admitted basis");
    let explicit_basis_identity = explicit_basis.admission_identity().clone();
    let (explicit_command, _) = explicit_mutation.into_parts();
    let explicit = explicit_workspace
        .execute_ordinary_preview_promotion(
            explicit_basis,
            &explicit_identity,
            explicit_command,
            true,
        )
        .expect("explicit orchestration should complete");

    let mut ordinary_workspace = stateful_bridge_task_runtime()
        .workspace("ordinary-workflow-declarative")
        .expect("workspace should open");
    let context = preview(&ordinary_workspace, session_label).expect("context should admit");
    let ordinary_basis_identity = context
        .authority
        .preview_basis()
        .expect("context should carry admitted basis")
        .admission_identity()
        .clone();
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
    assert_eq!(explicit_basis_identity, ordinary_basis_identity);
    assert_eq!(
        completion
            .preview_outcome()
            .closeout_evidence()
            .basis_admission_identity(),
        &ordinary_basis_identity
    );
}

#[test]
fn diagnostic_policy_changes_only_inspection_materialization() {
    let session_label = label("diagnostic-policy");
    let minimal_declaration = declare(session_label.clone(), mutation("diagnostic-policy"));
    let rich_declaration = minimal_declaration.clone().with_rich_inspection();

    let mut minimal_workspace = stateful_bridge_task_runtime()
        .workspace("ordinary-workflow-minimal-diagnostics")
        .expect("workspace should open");
    let minimal_context =
        preview(&minimal_workspace, session_label.clone()).expect("context should admit");
    let minimal = minimal_declaration
        .using(minimal_context)
        .run(&mut minimal_workspace);
    let minimal = minimal.completed().expect("workflow should complete");

    let mut rich_workspace = stateful_bridge_task_runtime()
        .workspace("ordinary-workflow-rich-diagnostics")
        .expect("workspace should open");
    let rich_context = preview(&rich_workspace, session_label).expect("context should admit");
    let rich = rich_declaration
        .using(rich_context)
        .run(&mut rich_workspace);
    let rich = rich.completed().expect("workflow should complete");

    assert_eq!(
        minimal.lowered_plan().request_identity(),
        rich.lowered_plan().request_identity()
    );
    assert_eq!(
        minimal.aftermath().receipt_identity(),
        rich.aftermath().receipt_identity()
    );
    assert_eq!(minimal.aftermath().identity(), rich.aftermath().identity());
    assert!(minimal.aftermath().inspection_identity().is_none());
    assert!(minimal.advisories().is_empty());
    assert_eq!(minimal.counters().inspection_materialization_count(), 0);
    assert!(rich.aftermath().inspection_identity().is_some());
    assert_eq!(rich.counters().inspection_materialization_count(), 1);
    assert!(rich.advisories().is_empty());
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
    assert_eq!(
        stop.violation().kind(),
        WorthQueryWorkflowViolationKind::CrossSession
    );
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
    assert_eq!(
        stop.violation().kind(),
        WorthQueryWorkflowViolationKind::ForeignAuthority
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
    assert_eq!(
        stop.violation().kind(),
        WorthQueryWorkflowViolationKind::StalePreview
    );
    assert_eq!(stop.counters().session_open_attempt_count(), 0);
}

#[path = "tests/writeback.rs"]
mod writeback;
