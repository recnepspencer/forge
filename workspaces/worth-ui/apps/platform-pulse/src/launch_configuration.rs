use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

use worth_ui_platform_pulse::observation_contract::PlatformPulseLaunchConfigurationDenial;

pub(crate) struct AdmittedPlatformPulseLaunchConfiguration {
    source_root: PathBuf,
    query_source_root: PathBuf,
}

impl AdmittedPlatformPulseLaunchConfiguration {
    pub(crate) fn from_process() -> Result<Self, PlatformPulseLaunchConfigurationDenial> {
        admit(
            std::env::args_os().skip(1),
            Path::new(env!("CARGO_MANIFEST_DIR")).join("app"),
            Path::new(env!("CARGO_MANIFEST_DIR")).join("query_samples"),
        )
    }

    pub(crate) fn source_root(&self) -> &Path {
        &self.source_root
    }

    pub(crate) fn query_source_root(&self) -> &Path {
        &self.query_source_root
    }
}

fn admit(
    arguments: impl IntoIterator<Item = OsString>,
    default_source_root: PathBuf,
    default_query_source_root: PathBuf,
) -> Result<AdmittedPlatformPulseLaunchConfiguration, PlatformPulseLaunchConfigurationDenial> {
    let mut arguments = arguments.into_iter();
    let mut source_root = None;
    let mut query_source_root = None;
    let mut admitted_option = false;
    while let Some(option) = arguments.next() {
        if option == OsStr::new("--source-root") && source_root.is_none() {
            source_root = Some(
                arguments
                    .next()
                    .ok_or(PlatformPulseLaunchConfigurationDenial::MissingSourceRootValue)?
                    .into(),
            );
            admitted_option = true;
        } else if option == OsStr::new("--query-source-root") && query_source_root.is_none() {
            query_source_root = Some(
                arguments
                    .next()
                    .ok_or(PlatformPulseLaunchConfigurationDenial::MissingQuerySourceRootValue)?
                    .into(),
            );
            admitted_option = true;
        } else {
            return Err(if admitted_option {
                PlatformPulseLaunchConfigurationDenial::SurplusArgument
            } else {
                PlatformPulseLaunchConfigurationDenial::UnexpectedArgument
            });
        }
    }
    admit_roots(
        source_root.unwrap_or(default_source_root),
        query_source_root.unwrap_or(default_query_source_root),
    )
}

fn admit_roots(
    source_root: PathBuf,
    query_source_root: PathBuf,
) -> Result<AdmittedPlatformPulseLaunchConfiguration, PlatformPulseLaunchConfigurationDenial> {
    if !source_root.is_absolute() {
        return Err(PlatformPulseLaunchConfigurationDenial::RelativeSourceRoot(
            source_root,
        ));
    }
    match source_root.try_exists() {
        Ok(false) => {
            return Err(PlatformPulseLaunchConfigurationDenial::MissingSourceRoot(
                source_root,
            ));
        }
        Err(_) => {
            return Err(
                PlatformPulseLaunchConfigurationDenial::SourceRootMetadataUnavailable(source_root),
            );
        }
        Ok(true) => {}
    }
    let metadata = std::fs::metadata(&source_root).map_err(|_| {
        PlatformPulseLaunchConfigurationDenial::SourceRootMetadataUnavailable(source_root.clone())
    })?;
    if !metadata.is_dir() {
        return Err(PlatformPulseLaunchConfigurationDenial::SourceRootNotDirectory(source_root));
    }
    let entry_source = source_root.join("main.wui");
    if !entry_source.is_file() {
        return Err(PlatformPulseLaunchConfigurationDenial::MissingEntrySource(
            entry_source,
        ));
    }
    if !query_source_root.is_absolute() {
        return Err(
            PlatformPulseLaunchConfigurationDenial::RelativeQuerySourceRoot(query_source_root),
        );
    }
    let query_metadata = std::fs::metadata(&query_source_root).map_err(|_| {
        PlatformPulseLaunchConfigurationDenial::QuerySourceRootMetadataUnavailable(
            query_source_root.clone(),
        )
    })?;
    if !query_metadata.is_dir() {
        return Err(
            PlatformPulseLaunchConfigurationDenial::QuerySourceRootNotDirectory(query_source_root),
        );
    }
    Ok(AdmittedPlatformPulseLaunchConfiguration {
        source_root,
        query_source_root,
    })
}

#[cfg(test)]
mod tests {
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
                "worth-ui-platform-pulse-phase2-{}-{ordinal}",
                std::process::id()
            ));
            std::fs::create_dir_all(&root).expect("create isolated installation");
            Self { root }
        }

        fn with_entry() -> Self {
            let installation = Self::empty();
            std::fs::write(
                installation.root.join("main.wui"),
                include_bytes!("../app/main.wui"),
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

    use std::path::PathBuf;

    fn admit_test(
        arguments: impl IntoIterator<Item = std::ffi::OsString>,
        default_source_root: PathBuf,
    ) -> Result<
        super::AdmittedPlatformPulseLaunchConfiguration,
        PlatformPulseLaunchConfigurationDenial,
    > {
        admit(
            arguments,
            default_source_root,
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("query_samples"),
        )
    }

    #[test]
    fn no_arguments_admit_the_checked_in_canonical_installation() {
        let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let admitted = admit_test(Vec::new(), manifest.join("app")).expect("canonical launch");
        assert_eq!(admitted.source_root(), manifest.join("app"));
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
        let _ = context.run(eframe::egui::RawInput::default(), |_| {});
        let mut prepared = crate::application::prepare(context, &admitted)
            .expect("isolated source reaches real filesystem and application preparation");
        assert!(
            prepared.app.declaration_artifacts().iter().any(|artifact| {
                artifact.identity().authored_semantic_name()
                    == worth_ui_platform_pulse::visual_identity_pulse::
                        PLATFORM_PULSE_IDENTITY_TARGET_AUTHORED_NAME
            }),
            "the checked-in target's family-qualified authored semantic identity survives lowering"
        );
        let mut shell = prepared
            .app
            .launch_native_surface()
            .expect("the prepared product reaches the canonical native shell");
        let pending = prepared
            .query_lifecycle
            .issue_initial()
            .expect("the product Query owner issues its initial pending fact");
        let receipt = publish_initial_pending_projection(&mut shell, pending);
        assert!(
            receipt.mounted_publication().is_some(),
            "the pending Query receipt carries the first mounted publication"
        );
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
                worth_ui::facade::rebind::UiProjectionRebindRequest::new(pending)
                    .observed_at_tick(1),
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
            worth_ui::facade::rebind::UiRebindOutcome::RejectedBeforeEffects(denial) => {
                panic!(
                    "pending Query fact was rejected at {:?}: {:?}",
                    denial.stopped_phase(),
                    denial.cause()
                )
            }
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
        assert!(matches!(
            admit_test(
                ["--source-root".into(), "one".into(), "two".into()],
                PathBuf::from("unused")
            ),
            Err(PlatformPulseLaunchConfigurationDenial::SurplusArgument)
        ));
    }
}
