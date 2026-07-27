use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

use worth_ui_platform_pulse::observation_contract::PlatformPulseLaunchConfigurationDenial;

pub(crate) struct AdmittedPlatformPulseLaunchConfiguration {
    source_root: PathBuf,
}

impl AdmittedPlatformPulseLaunchConfiguration {
    pub(crate) fn from_process() -> Result<Self, PlatformPulseLaunchConfigurationDenial> {
        admit(
            std::env::args_os().skip(1),
            Path::new(env!("CARGO_MANIFEST_DIR")).join("app"),
        )
    }

    pub(crate) fn source_root(&self) -> &Path {
        &self.source_root
    }
}

fn admit(
    arguments: impl IntoIterator<Item = OsString>,
    default_source_root: PathBuf,
) -> Result<AdmittedPlatformPulseLaunchConfiguration, PlatformPulseLaunchConfigurationDenial> {
    let mut arguments = arguments.into_iter();
    let source_root = match arguments.next() {
        None => default_source_root,
        Some(option) if option == OsStr::new("--source-root") => arguments
            .next()
            .ok_or(PlatformPulseLaunchConfigurationDenial::MissingSourceRootValue)?
            .into(),
        Some(_) => return Err(PlatformPulseLaunchConfigurationDenial::UnexpectedArgument),
    };
    if arguments.next().is_some() {
        return Err(PlatformPulseLaunchConfigurationDenial::SurplusArgument);
    }
    admit_source_root(source_root)
}

fn admit_source_root(
    source_root: PathBuf,
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
    Ok(AdmittedPlatformPulseLaunchConfiguration { source_root })
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

    #[test]
    fn no_arguments_admit_the_checked_in_canonical_installation() {
        let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let admitted = admit(Vec::new(), manifest.join("app")).expect("canonical launch");
        assert_eq!(admitted.source_root(), manifest.join("app"));
    }

    #[test]
    fn explicit_absolute_installation_is_admitted_without_test_authority() {
        let installation = IsolatedInstallation::with_entry();
        let admitted = admit(
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
        let admitted = admit(
            [
                "--source-root".into(),
                installation.root.clone().into_os_string(),
            ],
            PathBuf::from("unused"),
        )
        .expect("explicit launch");
        let prepared = crate::application::prepare(eframe::egui::Context::default(), &admitted)
            .expect("isolated source reaches real filesystem and application preparation");
        prepared
            .watcher
            .shutdown()
            .expect("isolated source watcher shuts down");
    }

    #[test]
    fn invalid_launch_shapes_are_distinct_typed_denials() {
        let empty = IsolatedInstallation::empty();
        let missing = empty.root.join("missing");
        let file = empty.root.join("not-a-directory");
        std::fs::write(&file, b"not a directory").expect("write file");

        assert!(matches!(
            admit(["--source-root".into()], PathBuf::from("unused")),
            Err(PlatformPulseLaunchConfigurationDenial::MissingSourceRootValue)
        ));
        assert!(matches!(
            admit(
                ["--source-root".into(), "relative".into()],
                PathBuf::from("unused")
            ),
            Err(PlatformPulseLaunchConfigurationDenial::RelativeSourceRoot(
                _
            ))
        ));
        assert!(matches!(
            admit(
                ["--source-root".into(), missing.into_os_string()],
                PathBuf::from("unused")
            ),
            Err(PlatformPulseLaunchConfigurationDenial::MissingSourceRoot(_))
        ));
        assert!(matches!(
            admit(
                ["--source-root".into(), file.into_os_string()],
                PathBuf::from("unused")
            ),
            Err(PlatformPulseLaunchConfigurationDenial::SourceRootNotDirectory(_))
        ));
        assert!(matches!(
            admit(
                ["--source-root".into(), empty.root.clone().into_os_string()],
                PathBuf::from("unused")
            ),
            Err(PlatformPulseLaunchConfigurationDenial::MissingEntrySource(
                _
            ))
        ));
        assert!(matches!(
            admit(["--unknown".into()], PathBuf::from("unused")),
            Err(PlatformPulseLaunchConfigurationDenial::UnexpectedArgument)
        ));
        assert!(matches!(
            admit(
                ["--source-root".into(), "one".into(), "two".into()],
                PathBuf::from("unused")
            ),
            Err(PlatformPulseLaunchConfigurationDenial::SurplusArgument)
        ));
    }
}
