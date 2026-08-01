use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use super::admit;
use worth_ui_platform_pulse::observation_contract::PlatformPulseLaunchConfigurationDenial;

static NEXT_INSTALLATION: AtomicU64 = AtomicU64::new(1);

struct IsolatedInstallation {
    root: PathBuf,
}

impl IsolatedInstallation {
    fn empty() -> Self {
        let ordinal = NEXT_INSTALLATION.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "worth-ui-platform-pulse-phase5-{}-{ordinal}",
            std::process::id()
        ));
        std::fs::create_dir_all(&root).expect("create isolated installation");
        Self { root }
    }

    fn with_entry() -> Self {
        let installation = Self::empty();
        std::fs::write(
            installation.root.join("main.wui"),
            include_bytes!("../../app/main.wui"),
        )
        .expect("write canonical source");
        installation
    }
}

impl Drop for IsolatedInstallation {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.root).expect("remove isolated installation");
    }
}

fn canonical_query_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("query_samples")
}

fn canonical_intent_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("intent_samples")
}

fn admit_test(
    arguments: impl IntoIterator<Item = std::ffi::OsString>,
    default_source_root: PathBuf,
) -> Result<super::AdmittedPlatformPulseLaunchConfiguration, PlatformPulseLaunchConfigurationDenial>
{
    admit(
        arguments,
        default_source_root,
        canonical_query_root(),
        canonical_intent_root(),
    )
}

#[test]
fn no_arguments_admit_all_checked_in_canonical_installations() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let admitted = admit_test(Vec::new(), manifest.join("app")).expect("canonical launch");
    assert_eq!(admitted.source_root(), manifest.join("app"));
    assert_eq!(
        admitted.intent_source_root(),
        manifest.join("intent_samples")
    );
}

#[test]
fn explicit_absolute_installation_is_admitted_without_test_authority() {
    let installation = IsolatedInstallation::with_entry();
    let admitted = admit_test(
        [
            "--source-root".into(),
            installation.root.clone().into_os_string(),
        ],
        PathBuf::from("unused"),
    )
    .expect("explicit launch");
    assert_eq!(admitted.source_root(), installation.root);
}

#[test]
fn explicit_absolute_installation_reaches_real_application_preparation() {
    let installation = IsolatedInstallation::with_entry();
    let admitted = admit_test(
        [
            "--source-root".into(),
            installation.root.clone().into_os_string(),
        ],
        PathBuf::from("unused"),
    )
    .expect("explicit launch");
    let context = eframe::egui::Context::default();
    let _ = context.run_ui(eframe::egui::RawInput::default(), |_| {});
    let mut prepared = crate::application::prepare(context, &admitted)
        .expect("isolated source reaches real filesystem and application preparation");
    let definitions = prepared.app.capabilities().intent_definitions();
    assert_eq!(definitions.len(), 1);
    assert!(definitions
        .get(&worth_ui::facade::intent::UiIntentId::stable(
            worth_ui_platform_pulse::intent::PLATFORM_PULSE_ACTION_DEFINITION,
        ))
        .is_some());
    let action_view = worth_ui::facade::query_binding::WorthUiQueryViewIdentity::new(
        worth_ui_platform_pulse::intent::PLATFORM_PULSE_ACTION_QUERY_VIEW,
    )
    .expect("static Pulse action view identity");
    assert_ne!(
        action_view.as_str(),
        worth_ui_platform_pulse::PLATFORM_PULSE_STATUS_QUERY_VIEW,
        "Query action evidence and the visible scalar consequence remain distinct identities"
    );
    assert!(prepared
        .app
        .resolve_query_view(
            &action_view,
            worth_ui::facade::query_binding::WorthUiQueryViewShape::Collection,
        )
        .is_some());
    assert!(
        prepared.app.declaration_artifacts().iter().any(|artifact| {
            artifact.identity().authored_semantic_name()
                == worth_ui_platform_pulse::visual_identity_pulse::
                    PLATFORM_PULSE_IDENTITY_TARGET_AUTHORED_NAME
        }),
        "the checked-in target's authored identity survives lowering"
    );
    let mut shell = prepared
        .app
        .launch_native_surface()
        .expect("the prepared product reaches the canonical native shell");
    let pending = prepared
        .query_lifecycle
        .issue_initial()
        .expect("the product Query owner issues its initial pending fact");
    assert_eq!(
        pending.projection_identity().as_str(),
        worth_ui_platform_pulse::PLATFORM_PULSE_STATUS_QUERY_VIEW,
        "the owner-issued visible consequence retains the authored scalar identity"
    );
    let receipt = publish_initial_pending_projection(&mut shell, pending);
    assert!(receipt.mounted_publication().is_some());
    let fact = receipt
        .release_scalar_projection_predecessor()
        .unwrap_or_else(|_| panic!("the pending receipt returns its exact Query fact"));
    prepared
        .query_lifecycle
        .admit_publication(fact)
        .expect("the exact pending fact restores the Query owner");
    let query_shutdown = prepared
        .query_lifecycle
        .close()
        .expect("isolated Query lifecycle closes");
    assert!(query_shutdown.owner_terminal());
    assert_eq!(query_shutdown.live_source_count(), 0);
    prepared
        .query_watcher
        .shutdown()
        .expect("isolated Query watcher shuts down");
    let intent_shutdown = prepared
        .intent_watcher
        .shutdown()
        .expect("isolated intent watcher shuts down");
    assert!(intent_shutdown.worker_joined());
    assert_eq!(intent_shutdown.pending_event_count(), 0);
    let intent_census = prepared.intent_action_owner.census();
    assert_eq!(intent_census.submitted(), 0);
    assert_eq!(intent_census.received(), 0);
    assert_eq!(intent_census.settled(), 0);
    assert_eq!(intent_census.retained(), 0);
    prepared
        .watcher
        .shutdown()
        .expect("isolated source watcher shuts down");
    let application_shutdown = shell.shutdown();
    assert!(application_shutdown.host_session_released());
}

fn publish_initial_pending_projection(
    shell: &mut worth_ui::facade::app::WorthUiNativeApplicationShell,
    pending: worth_ui::facade::query_binding::UiProjectionObservation,
) -> worth_ui::facade::rebind::UiRebindReceipt {
    match shell
        .begin_projection_rebind(
            worth_ui::facade::rebind::UiProjectionRebindRequest::new(pending).observed_at_tick(1),
        )
        .expect("pending Query fact enters the canonical mounted rebind")
    {
        worth_ui::facade::rebind::UiRebindOutcome::Published(receipt) => receipt,
        worth_ui::facade::rebind::UiRebindOutcome::InFlight(completion) => {
            match completion.complete(1) {
                worth_ui::facade::rebind::UiRebindOutcome::Published(receipt) => receipt,
                _ => panic!("pending Query completion must publish the first mounted frame"),
            }
        }
        worth_ui::facade::rebind::UiRebindOutcome::Duplicate(_) => {
            panic!("pending Query fact was classified as duplicate")
        }
        worth_ui::facade::rebind::UiRebindOutcome::ObservedNoChange(_) => {
            panic!("pending Query fact was classified as no change")
        }
        worth_ui::facade::rebind::UiRebindOutcome::RejectedBeforeEffects(denial) => panic!(
            "pending Query fact was rejected at {:?}: {:?}",
            denial.stopped_phase(),
            denial.cause()
        ),
        worth_ui::facade::rebind::UiRebindOutcome::CancelledBeforeEffects(receipt) => {
            panic!(
                "pending Query fact was cancelled at {:?}",
                receipt.stopped_phase()
            )
        }
        worth_ui::facade::rebind::UiRebindOutcome::TimedOutBeforeEffects(receipt) => {
            panic!(
                "pending Query fact timed out at {:?}",
                receipt.stopped_phase()
            )
        }
        worth_ui::facade::rebind::UiRebindOutcome::SupersededBeforeEffects(receipt) => {
            panic!(
                "pending Query fact was superseded at {:?}",
                receipt.stopped_phase()
            )
        }
        worth_ui::facade::rebind::UiRebindOutcome::Indeterminate(_) => {
            panic!("pending Query fact became indeterminate")
        }
        worth_ui::facade::rebind::UiRebindOutcome::InternalDefect(defect) => {
            panic!(
                "pending Query fact exposed internal defect {:?}",
                defect.kind()
            )
        }
    }
}

#[test]
fn intent_source_root_requires_the_exact_versioned_product_input() {
    let installation = IsolatedInstallation::with_entry();
    let empty_intent = IsolatedInstallation::empty();
    let denial = match admit(
        [
            "--source-root".into(),
            installation.root.clone().into_os_string(),
            "--intent-source-root".into(),
            empty_intent.root.clone().into_os_string(),
        ],
        PathBuf::from("unused"),
        canonical_query_root(),
        PathBuf::from("unused"),
    ) {
        Err(denial) => denial,
        Ok(_) => panic!("missing typed intent input must stop launch"),
    };
    assert!(matches!(
        denial,
        PlatformPulseLaunchConfigurationDenial::MissingIntentSource(_)
    ));
}

#[test]
fn invalid_launch_shapes_are_distinct_typed_denials() {
    let empty = IsolatedInstallation::empty();
    let missing = empty.root.join("missing");
    let file = empty.root.join("not-a-directory");
    std::fs::write(&file, b"not a directory").expect("write file");

    assert!(matches!(
        admit_test(["--source-root".into()], PathBuf::from("unused")),
        Err(PlatformPulseLaunchConfigurationDenial::MissingSourceRootValue)
    ));
    assert!(matches!(
        admit_test(
            ["--source-root".into(), "relative".into()],
            PathBuf::from("unused")
        ),
        Err(PlatformPulseLaunchConfigurationDenial::RelativeSourceRoot(
            _
        ))
    ));
    assert!(matches!(
        admit_test(
            ["--source-root".into(), missing.into_os_string()],
            PathBuf::from("unused")
        ),
        Err(PlatformPulseLaunchConfigurationDenial::MissingSourceRoot(_))
    ));
    assert!(matches!(
        admit_test(
            ["--source-root".into(), file.into_os_string()],
            PathBuf::from("unused")
        ),
        Err(PlatformPulseLaunchConfigurationDenial::SourceRootNotDirectory(_))
    ));
    assert!(matches!(
        admit_test(
            ["--source-root".into(), empty.root.clone().into_os_string()],
            PathBuf::from("unused")
        ),
        Err(PlatformPulseLaunchConfigurationDenial::MissingEntrySource(
            _
        ))
    ));
    assert!(matches!(
        admit_test(["--unknown".into()], PathBuf::from("unused")),
        Err(PlatformPulseLaunchConfigurationDenial::UnexpectedArgument)
    ));
}
