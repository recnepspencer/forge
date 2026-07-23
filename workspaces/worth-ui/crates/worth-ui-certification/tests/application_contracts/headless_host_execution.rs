use worth_ui::facade::app::{
    WorthUiOrdinaryFrameTarget, WorthUiOrdinaryPlanAvailability, WorthUiOrdinaryPlanSummary,
    WorthUiOrdinaryPlanSummaryRequest, WorthUiOrdinarySummaryTarget, WorthUiOrdinaryTouchBreadth,
};
use worth_ui::facade::host::{
    WorthUiHostOutputDisposition, WorthUiHostOutputPayload, WorthUiOrdinaryHostOutputTarget,
};
use worth_ui::facade::source::WorthUiFilesystemSourceProvider;
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;

use super::filesystem_contract_workspace::FilesystemContractWorkspace;
use super::headless_output_observer::ObservingHeadlessHost;

#[test]
fn real_wui_bytes_execute_through_the_public_active_session_and_headless_host() {
    let scenario = FilesystemApplicationLifecycleScenario::new("headless-host-execution");
    let workspace = FilesystemContractWorkspace::new("headless-host-execution");
    workspace.write(
        "app/main.wui",
        &FilesystemApplicationLifecycleScenario::ordinary_execution_source_text(),
    );
    let snapshot = WorthUiFilesystemSourceProvider::new(workspace.root())
        .read()
        .expect("production filesystem acquisition should read real .wui bytes");
    let capabilities = scenario.capability_application();
    let submission = FilesystemApplicationLifecycleScenario::lower_snapshot(
        snapshot,
        capabilities.capabilities(),
    );
    let (host, host_observation) = ObservingHeadlessHost::new();
    let app = scenario.prepare_application_with_host(submission, host);
    let expected_generation = app.generation_identity().clone();
    let mut session = app
        .launch()
        .expect("the filesystem-authored application should launch publicly");

    assert_eq!(
        session.ordinary_plan_availability(),
        WorthUiOrdinaryPlanAvailability::Executable
    );
    let expected_host_session = session.host_session_identity().as_u64();
    let active = session.inspect_runtime();
    let expected_plan_digest = active.active_plan_digest();
    let expected_artifact_digest = active.artifact_digest();
    let expected_frame_epoch = active.frame_epoch().as_u64();
    let requests = [
        WorthUiOrdinaryPlanSummaryRequest::Component,
        WorthUiOrdinaryPlanSummaryRequest::ChildRange,
        WorthUiOrdinaryPlanSummaryRequest::Command,
        WorthUiOrdinaryPlanSummaryRequest::Token,
        WorthUiOrdinaryPlanSummaryRequest::StateSlot,
    ];
    let targets = requests.map(|request| {
        let summary = session
            .inspect_ordinary_plan(request)
            .expect("the active family index should produce a bounded summary");
        assert_eq!(summary.request(), request);
        assert!(summary.family_row_count() > 0);
        assert!(summary.target_semantic_digest().is_some());
        assert_eq!(summary.family_index_lookup_count(), 1);
        assert_eq!(summary.direct_row_lookup_count(), 1);
        assert_native_summary_meaning(&summary);
        summary
            .target()
            .expect("the real .wui source should lower the requested family")
    });

    {
        let execution = session
            .execute_framework_turn(|_| {})
            .into_execution()
            .unwrap_or_else(|_| {
                panic!("an empty public framework turn should lend ordinary execution")
            });
        let frame = execution
            .execute_ordinary_frame(WorthUiOrdinaryFrameTarget::root_shell())
            .expect("the active ordinary root shell should execute");

        assert_eq!(frame.generation_identity(), &expected_generation);
        assert!(frame.receipt().touch().row_count() > 0);
        assert_eq!(
            frame.receipt().touch().breadth(),
            WorthUiOrdinaryTouchBreadth::RootShell
        );
        assert_eq!(
            frame.receipt().counters().root_shell_row_touch_count(),
            frame.receipt().touch().row_count()
        );
        assert_eq!(frame.disposition(), WorthUiHostOutputDisposition::Consumed);
        let envelope = *frame.output();
        let generation = envelope.generation();
        assert_eq!(generation.host_session_identity(), expected_host_session);
        assert_eq!(generation.active_plan_digest(), expected_plan_digest);
        assert_eq!(generation.frame_epoch(), expected_frame_epoch);
        let output = match envelope.payload() {
            WorthUiHostOutputPayload::Ordinary(output) => output,
            _ => panic!("headless execution emitted an unsupported output payload"),
        };
        assert_eq!(output.target(), WorthUiOrdinaryHostOutputTarget::RootShell);
        assert_eq!(
            output.touched_row_count(),
            frame.receipt().touch().row_count()
        );
        assert_ne!(output.meaning_digest(), 0);
        assert_eq!(
            envelope.receipt_reference().digest(),
            frame.receipt().touch().touch_digest()
        );

        let observed = host_observation.snapshot();
        assert_eq!(observed.call_count, 1);
        assert_eq!(observed.host_session_identity, expected_host_session);
        assert_eq!(observed.active_artifact_digest, expected_artifact_digest);
        assert_eq!(observed.active_plan_digest, expected_plan_digest);
        assert_eq!(observed.frame_epoch, expected_frame_epoch);
        assert_eq!(observed.target, WorthUiOrdinaryHostOutputTarget::RootShell);
        assert_eq!(observed.touched_row_count, output.touched_row_count());
        assert_eq!(observed.meaning_digest, output.meaning_digest());

        for target in targets {
            let expected_host_target = host_target(target);
            let frame = execution
                .execute_ordinary_frame(target.frame_target())
                .expect("a summary-discovered typed target should execute");
            let counters = frame.receipt().counters();
            assert_eq!(counters.source_parse_count(), 0);
            assert_eq!(counters.registry_lookup_count(), 0);
            assert_eq!(counters.artifact_tree_scan_count(), 0);
            assert_eq!(counters.full_plan_scan_count(), 0);
            assert_eq!(counters.component_string_resolution_count(), 0);
            assert_eq!(counters.command_string_resolution_count(), 0);
            let output = match frame.output().payload() {
                WorthUiHostOutputPayload::Ordinary(output) => output,
                _ => panic!("typed ordinary execution emitted the wrong output payload"),
            };
            assert_eq!(output.target(), expected_host_target);
            assert_eq!(
                output.touched_row_count(),
                frame.receipt().touch().row_count()
            );
            match target {
                WorthUiOrdinarySummaryTarget::Component(_)
                | WorthUiOrdinarySummaryTarget::ChildRange(_) => {
                    assert_eq!(
                        frame.receipt().touch().breadth(),
                        WorthUiOrdinaryTouchBreadth::Subtree
                    );
                    assert_eq!(
                        counters.intentional_subtree_row_touch_count(),
                        frame.receipt().touch().row_count()
                    );
                    assert!(frame.receipt().touch().row_count() > 1);
                }
                _ => {
                    assert_eq!(
                        frame.receipt().touch().breadth(),
                        WorthUiOrdinaryTouchBreadth::Direct
                    );
                    assert_eq!(frame.receipt().touch().row_count(), 1);
                }
            }
        }

        let observed = host_observation.snapshot();
        assert_eq!(observed.call_count, 1 + targets.len() as u64);
        assert_eq!(observed.target, WorthUiOrdinaryHostOutputTarget::StateSlot);
    }

    let _ = session.shutdown();
    workspace.close();
}

fn assert_native_summary_meaning(summary: &WorthUiOrdinaryPlanSummary) {
    match summary.request() {
        WorthUiOrdinaryPlanSummaryRequest::Component => assert_eq!(
            summary.component_descriptor().unwrap().id().as_str(),
            "workspace.component.authority_current"
        ),
        WorthUiOrdinaryPlanSummaryRequest::ChildRange => {
            assert!(summary.child_target_count().is_some_and(|count| count > 0));
        }
        WorthUiOrdinaryPlanSummaryRequest::Command => {
            let command = summary.command_descriptor().unwrap();
            assert_eq!(command.id().as_str(), "workspace.command.authority_save");
            assert_eq!(command.label(), "Save");
        }
        WorthUiOrdinaryPlanSummaryRequest::Token => {
            let token = summary.token_entry().unwrap();
            let resolved = summary.resolved_token_entry().unwrap();
            assert_eq!(
                token.descriptor().id().as_str(),
                "theme.text.authority_default"
            );
            assert_eq!(
                resolved.descriptor().id().as_str(),
                "theme.text.authority_primary"
            );
            assert_eq!(
                resolved
                    .descriptor()
                    .value()
                    .expect("resolved token retains its native value"),
                &worth_ui::facade::registry::ThemeTokenValue::color(
                    worth_ui::facade::registry::ThemeColorValue::hex("#101820")
                        .expect("valid expected native color"),
                )
            );
        }
        WorthUiOrdinaryPlanSummaryRequest::StateSlot => {
            assert_eq!(
                summary.state_slot_descriptor().unwrap().id().as_str(),
                "workspace.state.authority_scroll"
            );
            assert_eq!(summary.state_succession_is_launch(), Some(true));
            assert!(summary.state_reconciliation_receipt().is_none());
        }
    }
}

fn host_target(target: WorthUiOrdinarySummaryTarget) -> WorthUiOrdinaryHostOutputTarget {
    match target {
        WorthUiOrdinarySummaryTarget::Component(_) => WorthUiOrdinaryHostOutputTarget::Component,
        WorthUiOrdinarySummaryTarget::ChildRange(_) => WorthUiOrdinaryHostOutputTarget::ChildRange,
        WorthUiOrdinarySummaryTarget::Command(_) => WorthUiOrdinaryHostOutputTarget::Command,
        WorthUiOrdinarySummaryTarget::Token(_) => WorthUiOrdinaryHostOutputTarget::TokenSupport,
        WorthUiOrdinarySummaryTarget::StateSlot(_) => WorthUiOrdinaryHostOutputTarget::StateSlot,
    }
}

#[test]
fn reordered_real_declarations_preserve_ordinary_execution_behavior() {
    let scenario = FilesystemApplicationLifecycleScenario::new("ordinary-reorder");
    let left_workspace = FilesystemContractWorkspace::new("ordinary-reorder-left");
    let right_workspace = FilesystemContractWorkspace::new("ordinary-reorder-right");
    left_workspace.write(
        "app/main.wui",
        &FilesystemApplicationLifecycleScenario::ordinary_execution_source_text(),
    );
    right_workspace.write(
        "app/main.wui",
        &FilesystemApplicationLifecycleScenario::reordered_ordinary_execution_source_text(),
    );
    let capabilities = scenario.capability_application();
    let left_submission = FilesystemApplicationLifecycleScenario::lower_snapshot(
        WorthUiFilesystemSourceProvider::new(left_workspace.root())
            .read()
            .expect("left real source should settle"),
        capabilities.capabilities(),
    );
    let right_submission = FilesystemApplicationLifecycleScenario::lower_snapshot(
        WorthUiFilesystemSourceProvider::new(right_workspace.root())
            .read()
            .expect("right real source should settle"),
        capabilities.capabilities(),
    );
    let mut left = scenario
        .prepare_application(left_submission)
        .launch()
        .expect("left equivalent source should launch");
    let mut right = scenario
        .prepare_application(right_submission)
        .launch()
        .expect("right equivalent source should launch");

    for request in [
        WorthUiOrdinaryPlanSummaryRequest::Component,
        WorthUiOrdinaryPlanSummaryRequest::ChildRange,
        WorthUiOrdinaryPlanSummaryRequest::Command,
        WorthUiOrdinaryPlanSummaryRequest::Token,
        WorthUiOrdinaryPlanSummaryRequest::StateSlot,
    ] {
        let left_summary = left.inspect_ordinary_plan(request).unwrap();
        let right_summary = right.inspect_ordinary_plan(request).unwrap();
        assert_eq!(
            left_summary.family_row_count(),
            right_summary.family_row_count()
        );
        assert_eq!(
            left_summary.target_semantic_digest(),
            right_summary.target_semantic_digest()
        );
        assert_equivalent_native_summary_meaning(&left_summary, &right_summary);
        assert_ne!(
            left_summary.target().unwrap().frame_target(),
            right_summary.target().unwrap().frame_target(),
            "equivalent meaning must not collapse distinct session authority"
        );
    }

    let left_execution = left
        .execute_framework_turn(|_| {})
        .into_execution()
        .unwrap_or_else(|_| panic!("left equivalent source should lend execution"));
    let left_frame = left_execution
        .execute_ordinary_frame(WorthUiOrdinaryFrameTarget::root_shell())
        .unwrap();
    let right_execution = right
        .execute_framework_turn(|_| {})
        .into_execution()
        .unwrap_or_else(|_| panic!("right equivalent source should lend execution"));
    let right_frame = right_execution
        .execute_ordinary_frame(WorthUiOrdinaryFrameTarget::root_shell())
        .unwrap();
    assert_eq!(
        left_frame.receipt().touch().row_count(),
        right_frame.receipt().touch().row_count()
    );
    assert_eq!(
        left_frame.receipt().counters(),
        right_frame.receipt().counters()
    );

    drop(left_frame);
    drop(right_frame);
    drop(left_execution);
    drop(right_execution);
    let _ = left.shutdown();
    let _ = right.shutdown();
    left_workspace.close();
    right_workspace.close();
}

fn assert_equivalent_native_summary_meaning(
    left: &WorthUiOrdinaryPlanSummary,
    right: &WorthUiOrdinaryPlanSummary,
) {
    match left.request() {
        WorthUiOrdinaryPlanSummaryRequest::Component => {
            assert_eq!(left.component_descriptor(), right.component_descriptor());
        }
        WorthUiOrdinaryPlanSummaryRequest::ChildRange => {
            assert_eq!(left.child_target_count(), right.child_target_count());
        }
        WorthUiOrdinaryPlanSummaryRequest::Command => {
            assert_eq!(left.command_descriptor(), right.command_descriptor());
        }
        WorthUiOrdinaryPlanSummaryRequest::Token => {
            assert_eq!(left.token_entry(), right.token_entry());
            assert_eq!(left.resolved_token_entry(), right.resolved_token_entry());
        }
        WorthUiOrdinaryPlanSummaryRequest::StateSlot => {
            assert_eq!(left.state_slot_descriptor(), right.state_slot_descriptor());
            assert_eq!(
                left.state_succession_is_launch(),
                right.state_succession_is_launch()
            );
            assert_eq!(
                left.state_reconciliation_receipt(),
                right.state_reconciliation_receipt()
            );
        }
    }
}
