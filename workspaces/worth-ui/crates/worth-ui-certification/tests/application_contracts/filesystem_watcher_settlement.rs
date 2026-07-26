use std::fs;
use std::time::{Duration, Instant};

use worth_ui::facade::graph::UiGraphWorldDifferenceKind;
use worth_ui::facade::source::{
    WorthUiFilesystemSourceProvider, WorthUiFilesystemSourceWatcher,
    WorthUiFilesystemWatcherBackend, WorthUiFilesystemWatcherDenial, WorthUiReloadDebounce,
    WorthUiWatchedCandidateSubmission, WorthUiWatchedCandidateSubmissionDenial,
};
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;
use worth_ui_dsl::{WorthUiDslCompileDiagnosticCode, WorthUiDslCompileReport};

use super::filesystem_contract_workspace::FilesystemContractWorkspace;

const SETTLEMENT_TIMEOUT: Duration = Duration::from_secs(5);

#[test]
fn native_watcher_preserves_active_generation_until_stable_replacement_activation() {
    let scenario = FilesystemApplicationLifecycleScenario::new("native-watcher-activation");
    let workspace = FilesystemContractWorkspace::new("native-watcher-activation");
    workspace.write(
        "app/main.wui",
        &FilesystemApplicationLifecycleScenario::current_source_text(),
    );
    let provider = WorthUiFilesystemSourceProvider::new(workspace.root());
    let mut watcher = WorthUiFilesystemSourceWatcher::start(provider)
        .expect("native watcher should register and freeze its initial source");
    assert_eq!(
        watcher.readiness().root(),
        fs::canonicalize(workspace.root()).expect("watch root should canonicalize")
    );
    assert_platform_backend(watcher.readiness().backend());

    let capabilities = scenario.capability_application();
    let initial = watcher
        .take_initial_snapshot()
        .expect("ready watcher should own one initial snapshot");
    let submission = FilesystemApplicationLifecycleScenario::lower_snapshot(
        initial,
        capabilities.capabilities(),
    );
    let app = scenario.prepare_application(submission);
    let mut session = app.launch().expect("filesystem application should launch");
    let initial_generation = session.generation_identity().clone();

    let invalid_deadline = watcher
        .settle(Duration::MAX)
        .expect_err("unrepresentable settlement deadlines must deny explicitly");
    assert!(matches!(
        invalid_deadline,
        WorthUiFilesystemWatcherDenial::SettlementDeadlineUnrepresentable(_)
    ));
    let timeout = watcher
        .settle(Duration::from_millis(50))
        .expect_err("no source change should settle before the total timeout");
    assert!(matches!(
        timeout,
        WorthUiFilesystemWatcherDenial::SettlementTimedOut { .. }
    ));

    let malformed_source = "component workspace.component.broken {";
    workspace.write("app/main.wui", malformed_source);
    let malformed = watcher
        .settle(SETTLEMENT_TIMEOUT)
        .expect("completed malformed bytes still form a stable filesystem snapshot");
    let filesystem_report = dsl_compilation_report(
        malformed.lower_to_candidate_submission(session.capabilities()),
        "filesystem-authored malformed source",
    );
    assert!(
        !filesystem_report.diagnostics().is_empty(),
        "malformed filesystem-authored source must retain its DSL diagnostics"
    );
    assert_eq!(session.generation_identity(), &initial_generation);

    workspace.write_atomic(
        "app/main.wui",
        &FilesystemApplicationLifecycleScenario::candidate_source_text(),
    );
    let candidate = watcher
        .settle(SETTLEMENT_TIMEOUT)
        .expect("atomic editor replacement should settle from native events");
    let candidate =
        FilesystemApplicationLifecycleScenario::lower_snapshot(candidate, session.capabilities());
    let cutover =
        FilesystemApplicationLifecycleScenario::activate_submission(&mut session, candidate);
    assert_eq!(cutover.prior_generation(), &initial_generation);
    assert_eq!(cutover.active_generation(), session.generation_identity());
    assert_ne!(session.generation_identity(), &initial_generation);
    let active_generation = session.generation_identity().clone();

    let duplicate = watcher
        .settle(Duration::from_millis(150))
        .expect_err("duplicate native notifications must not emit another source revision");
    assert!(matches!(
        duplicate,
        WorthUiFilesystemWatcherDenial::SettlementTimedOut { .. }
    ));
    assert_eq!(session.generation_identity(), &active_generation);

    let _ = session.shutdown();
    let shutdown = watcher
        .shutdown()
        .expect("native watcher should unwatch its source root");
    assert_platform_backend(shutdown.backend());
    assert!(shutdown.observed_notification_count() > 0);
    workspace.close();
}

#[test]
fn native_watcher_rejects_a_zero_quiet_window_before_registration() {
    let workspace = FilesystemContractWorkspace::new("native-watcher-zero-window");
    workspace.write(
        "app/main.wui",
        &FilesystemApplicationLifecycleScenario::current_source_text(),
    );
    let provider = WorthUiFilesystemSourceProvider::new(workspace.root());

    let result = WorthUiFilesystemSourceWatcher::start_with_debounce(
        provider,
        WorthUiReloadDebounce::stable_window(Duration::ZERO),
    );
    let denial = match result {
        Ok(_) => panic!("a real watcher must retain a nonzero settlement window"),
        Err(denial) => denial,
    };
    assert_eq!(
        denial,
        WorthUiFilesystemWatcherDenial::EmptySettlementWindow(
            fs::canonicalize(workspace.root()).expect("watch root should canonicalize")
        )
    );
    workspace.close();
}

#[test]
fn deadline_limited_settlement_retains_the_consumed_change_for_the_next_call() {
    let workspace = FilesystemContractWorkspace::new("native-watcher-deadline");
    workspace.write(
        "app/main.wui",
        &FilesystemApplicationLifecycleScenario::current_source_text(),
    );
    let provider = WorthUiFilesystemSourceProvider::new(workspace.root());
    let mut watcher = WorthUiFilesystemSourceWatcher::start_with_debounce(
        provider,
        WorthUiReloadDebounce::stable_window(Duration::from_secs(1)),
    )
    .expect("native watcher should establish its initial quiet snapshot");
    let _ = watcher
        .take_initial_snapshot()
        .expect("deadline contract should begin from settled truth");

    workspace.write(
        "app/main.wui",
        &FilesystemApplicationLifecycleScenario::candidate_source_text(),
    );
    let started = Instant::now();
    let denial = watcher
        .settle(Duration::from_millis(750))
        .expect_err("the total deadline is shorter than the required quiet window");
    let elapsed = started.elapsed();
    let observed_notification_count = match denial {
        WorthUiFilesystemWatcherDenial::SettlementTimedOut {
            observed_notification_count,
            ..
        } => observed_notification_count,
        other => panic!("deadline-limited settlement returned {other:?}"),
    };
    assert!(
        observed_notification_count > 0,
        "the deadline path must actually consume a native notification"
    );
    assert!(
        elapsed < Duration::from_secs(1),
        "settlement exceeded its total deadline by starting another quiet-window read: {elapsed:?}"
    );

    let settled = watcher
        .settle(Duration::from_secs(3))
        .expect("the consumed notification must survive as a retained resnapshot obligation");
    assert_eq!(settled.source_revision().sequence(), 2);

    watcher
        .shutdown()
        .expect("deadline watcher should release its native registration");
    workspace.close();
}

#[test]
fn native_watcher_reconstructs_imported_module_churn_from_final_tree_truth() {
    let scenario = FilesystemApplicationLifecycleScenario::new("native-watcher-imports");
    let workspace = FilesystemContractWorkspace::new("native-watcher-imports");
    workspace.write(
        "app/main.wui",
        &format!(
            "import \"app/panels/inspector.wui\";\n{}",
            FilesystemApplicationLifecycleScenario::current_source_text()
        ),
    );
    workspace.write(
        "app/panels/inspector.wui",
        &FilesystemApplicationLifecycleScenario::imported_current_source_text(),
    );
    let filesystem = WorthUiFilesystemSourceProvider::new(workspace.root());
    let mut watcher = WorthUiFilesystemSourceWatcher::start(filesystem.clone())
        .expect("native watcher should register recursively");
    let capabilities = scenario.capability_application();
    let initial = watcher
        .take_initial_snapshot()
        .expect("ready watcher should freeze imported source");
    let initial = FilesystemApplicationLifecycleScenario::lower_snapshot(
        initial,
        capabilities.capabilities(),
    );
    let initial_app = scenario.prepare_application(initial);

    workspace.write(
        "app/panels/inspector.wui",
        &FilesystemApplicationLifecycleScenario::imported_candidate_source_text(),
    );
    let modified = watcher
        .settle(SETTLEMENT_TIMEOUT)
        .expect("imported module modification should settle");
    assert_eq!(modified.source_revision().sequence(), 2);
    assert_eq!(modified.ordering_receipt().sequence(), 2);
    let modified = FilesystemApplicationLifecycleScenario::lower_snapshot(
        modified,
        capabilities.capabilities(),
    );
    let modified_app = scenario.prepare_application(modified);
    assert_ne!(
        modified_app.generation_identity(),
        initial_app.generation_identity(),
        "changing the imported declaration must change canonical application meaning"
    );
    let active_snapshot = filesystem
        .read()
        .expect("direct acquisition should observe modified imported source");
    let active_submission = FilesystemApplicationLifecycleScenario::lower_snapshot(
        active_snapshot,
        capabilities.capabilities(),
    );
    let session = scenario
        .prepare_application(active_submission)
        .launch()
        .expect("modified imported source should become active truth");
    let active_generation = session.generation_identity().clone();
    assert_eq!(
        session.generation_identity(),
        modified_app.generation_identity()
    );

    workspace.remove("app/panels/inspector.wui");
    let missing = watcher
        .settle(SETTLEMENT_TIMEOUT)
        .expect("imported module deletion should settle as final tree truth");
    assert_eq!(missing.source_revision().sequence(), 3);
    let missing_digest = missing.source_revision().final_package_digest();
    let denial = missing
        .lower_to_candidate_submission(session.capabilities())
        .expect_err("a still-declared import cannot silently lose its target module");
    assert!(matches!(
        denial,
        WorthUiWatchedCandidateSubmissionDenial::DslCompilation(report)
            if report.diagnostics()[0].identity().code()
                == WorthUiDslCompileDiagnosticCode::UnknownImportTarget
    ));
    assert_eq!(session.generation_identity(), &active_generation);

    workspace.write(
        "app/panels/inspector.wui",
        &FilesystemApplicationLifecycleScenario::imported_candidate_source_text(),
    );
    let restored = watcher
        .settle(SETTLEMENT_TIMEOUT)
        .expect("restored imported module should settle");
    assert_eq!(restored.source_revision().sequence(), 4);
    assert_eq!(restored.ordering_receipt().sequence(), 4);
    let restored_digest = restored.source_revision().final_package_digest();
    assert_ne!(restored_digest, missing_digest);
    let restored = FilesystemApplicationLifecycleScenario::lower_snapshot(
        restored,
        capabilities.capabilities(),
    );
    let restored_app = scenario.prepare_application(restored);
    assert_eq!(
        restored_app.generation_identity(),
        modified_app.generation_identity(),
        "restoring equal imported meaning must converge despite a later event sequence"
    );
    assert_eq!(
        restored_app.graph().compare_to(modified_app.graph()).kind(),
        UiGraphWorldDifferenceKind::SameWorldEquivalent
    );
    assert_eq!(session.generation_identity(), &active_generation);
    let direct = filesystem
        .read()
        .expect("direct reader should observe the same final source tree");
    assert_eq!(
        restored_digest,
        direct.source_revision().final_package_digest()
    );

    let _ = session.shutdown();
    let shutdown = watcher
        .shutdown()
        .expect("native watcher should release imported tree handles");
    assert!(shutdown.observed_notification_count() >= 3);
    workspace.close();
}

fn assert_platform_backend(backend: WorthUiFilesystemWatcherBackend) {
    #[cfg(target_os = "windows")]
    assert_eq!(
        backend,
        WorthUiFilesystemWatcherBackend::ReadDirectoryChanges
    );
    #[cfg(not(target_os = "windows"))]
    assert_ne!(
        backend,
        WorthUiFilesystemWatcherBackend::ReadDirectoryChanges
    );
}

fn dsl_compilation_report(
    result: Result<WorthUiWatchedCandidateSubmission, WorthUiWatchedCandidateSubmissionDenial>,
    gateway: &str,
) -> WorthUiDslCompileReport {
    match result {
        Err(WorthUiWatchedCandidateSubmissionDenial::DslCompilation(report)) => report,
        Err(other) => panic!("{gateway} stopped outside DSL compilation: {other:?}"),
        Ok(_) => panic!("{gateway} unexpectedly produced a runtime candidate"),
    }
}
