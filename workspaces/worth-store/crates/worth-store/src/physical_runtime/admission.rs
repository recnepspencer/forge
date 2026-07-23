use std::{
    fmt,
    path::{Component, Path, PathBuf},
};

use super::{
    diagnostics::{
        process_counter_snapshot, record_admission_attempt, record_admission_cancellation,
        record_admission_denial, record_admission_panic_before_return, record_admitted_incarnation,
    },
    identity::{DeclaredStoreRoot, RuntimeIdentity},
    root_admission::RootAdmission,
    runtime::AdmittedPhysicalRuntime,
    ProcessRuntimeCounterSnapshot,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclaredStoreRootDenialKind {
    Empty,
    Relative,
    LexicallyAmbiguous,
    WindowsDeviceNamespace,
    WindowsVerbatimNamespace,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdmissionError {
    InvalidDeclaredStoreRoot {
        declared_root: PathBuf,
        kind: DeclaredStoreRootDenialKind,
    },
    DeclaredRootAlreadyAdmitted(DeclaredStoreRoot),
    RuntimeIdentityExhausted,
}

impl fmt::Display for DeclaredStoreRootDenialKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reason = match self {
            Self::Empty => "the path is empty",
            Self::Relative => "the path is not absolute",
            Self::LexicallyAmbiguous => "the path contains `.` or `..` components",
            Self::WindowsDeviceNamespace => "Windows device namespace roots are not admitted",
            Self::WindowsVerbatimNamespace => "Windows verbatim namespace roots are not admitted",
        };
        formatter.write_str(reason)
    }
}

impl fmt::Display for AdmissionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDeclaredStoreRoot {
                declared_root,
                kind,
            } => write!(
                formatter,
                "declared store root `{}` is invalid: {kind}",
                declared_root.display()
            ),
            Self::DeclaredRootAlreadyAdmitted(declared_root) => write!(
                formatter,
                "declared store root `{}` already has a live process-local owner",
                declared_root.as_path().display()
            ),
            Self::RuntimeIdentityExhausted => {
                formatter.write_str("physical runtime identity space is exhausted")
            }
        }
    }
}

impl std::error::Error for AdmissionError {}

/// Complete validated request for one physical runtime incarnation.
pub struct PhysicalRuntimeAdmission {
    declared_root: DeclaredStoreRoot,
}

impl PhysicalRuntimeAdmission {
    pub fn new(declared_root: impl Into<PathBuf>) -> Result<Self, AdmissionError> {
        record_admission_attempt();
        let declared_root = validate_declared_store_root(declared_root.into())?;

        Ok(Self { declared_root })
    }

    pub fn cancel(self) -> CancelledPhysicalRuntimeAdmission {
        let Self { declared_root } = self;
        record_admission_cancellation();
        CancelledPhysicalRuntimeAdmission { declared_root }
    }
}

/// Non-authoritative summary of a validated request cancelled before admission.
pub struct CancelledPhysicalRuntimeAdmission {
    declared_root: DeclaredStoreRoot,
}

impl CancelledPhysicalRuntimeAdmission {
    pub fn declared_store_root(&self) -> &DeclaredStoreRoot {
        &self.declared_root
    }
}

/// Sole public entry point for creating the physical runtime composition root.
pub struct PhysicalStore;

impl PhysicalStore {
    /// Admits one process-local owner for the exact declared root.
    ///
    /// This transition performs no filesystem access and therefore does not
    /// claim durable namespace locking or cross-process exclusion.
    pub fn admit(
        admission: PhysicalRuntimeAdmission,
    ) -> Result<AdmittedPhysicalRuntime, AdmissionError> {
        admit_with_runtime_identity_source(admission, RuntimeIdentity::generate)
    }

    pub fn diagnostics() -> ProcessRuntimeCounterSnapshot {
        process_counter_snapshot()
    }
}

fn admit_with_runtime_identity_source(
    admission: PhysicalRuntimeAdmission,
    runtime_identity_source: impl FnOnce() -> Option<RuntimeIdentity>,
) -> Result<AdmittedPhysicalRuntime, AdmissionError> {
    let PhysicalRuntimeAdmission { declared_root } = admission;
    let root_admission = RootAdmission::admit(declared_root).inspect_err(|_| {
        record_admission_denial();
    })?;
    let mut assembly = AdmissionAssemblyGuard::after_root_reservation();
    let runtime_identity = runtime_identity_source().ok_or_else(|| {
        record_admission_denial();
        AdmissionError::RuntimeIdentityExhausted
    })?;
    let admitted_runtime =
        AdmittedPhysicalRuntime::from_admission(runtime_identity, root_admission);
    record_admitted_incarnation();
    assembly.complete();

    Ok(admitted_runtime)
}

fn validate_declared_store_root(path: PathBuf) -> Result<DeclaredStoreRoot, AdmissionError> {
    let kind = if path.as_os_str().is_empty() {
        Some(DeclaredStoreRootDenialKind::Empty)
    } else if let Some(kind) = disallowed_windows_namespace(&path) {
        Some(kind)
    } else if path.is_relative() {
        Some(DeclaredStoreRootDenialKind::Relative)
    } else if path
        .components()
        .any(|component| matches!(component, Component::CurDir | Component::ParentDir))
    {
        Some(DeclaredStoreRootDenialKind::LexicallyAmbiguous)
    } else {
        None
    };

    match kind {
        Some(kind) => {
            record_admission_denial();
            Err(AdmissionError::InvalidDeclaredStoreRoot {
                declared_root: path,
                kind,
            })
        }
        None => Ok(DeclaredStoreRoot::from_validated_path(path)),
    }
}

#[cfg(windows)]
fn disallowed_windows_namespace(path: &Path) -> Option<DeclaredStoreRootDenialKind> {
    use std::path::Prefix;

    let Component::Prefix(prefix) = path.components().next()? else {
        return None;
    };
    match prefix.kind() {
        Prefix::DeviceNS(_) => Some(DeclaredStoreRootDenialKind::WindowsDeviceNamespace),
        Prefix::Verbatim(_) | Prefix::VerbatimDisk(_) | Prefix::VerbatimUNC(_, _) => {
            Some(DeclaredStoreRootDenialKind::WindowsVerbatimNamespace)
        }
        Prefix::Disk(_) | Prefix::UNC(_, _) => None,
    }
}

#[cfg(not(windows))]
const fn disallowed_windows_namespace(_path: &Path) -> Option<DeclaredStoreRootDenialKind> {
    None
}

struct AdmissionAssemblyGuard {
    completed: bool,
}

impl AdmissionAssemblyGuard {
    const fn after_root_reservation() -> Self {
        Self { completed: false }
    }

    fn complete(&mut self) {
        self.completed = true;
    }
}

impl Drop for AdmissionAssemblyGuard {
    fn drop(&mut self) {
        if !self.completed && std::thread::panicking() {
            record_admission_panic_before_return();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    #[test]
    fn declared_root_validation_classifies_each_non_filesystem_denial() {
        let cases = [
            (PathBuf::new(), DeclaredStoreRootDenialKind::Empty),
            (
                PathBuf::from("relative-store-root"),
                DeclaredStoreRootDenialKind::Relative,
            ),
            (
                std::env::temp_dir()
                    .join("lexical-alias")
                    .join("..")
                    .join("store"),
                DeclaredStoreRootDenialKind::LexicallyAmbiguous,
            ),
        ];

        for (path, expected_kind) in cases {
            assert!(matches!(
                PhysicalRuntimeAdmission::new(path),
                Err(AdmissionError::InvalidDeclaredStoreRoot { kind, .. })
                    if kind == expected_kind
            ));
        }
    }

    #[cfg(windows)]
    #[test]
    fn declared_root_validation_rejects_device_and_verbatim_namespaces() {
        for (path, expected_kind) in [
            (
                PathBuf::from(r"\\.\C:\worth-store"),
                DeclaredStoreRootDenialKind::WindowsDeviceNamespace,
            ),
            (
                PathBuf::from(r"\\?\C:\worth-store"),
                DeclaredStoreRootDenialKind::WindowsVerbatimNamespace,
            ),
            (
                PathBuf::from(r"\\?\UNC\server\share\worth-store"),
                DeclaredStoreRootDenialKind::WindowsVerbatimNamespace,
            ),
        ] {
            assert!(matches!(
                PhysicalRuntimeAdmission::new(path),
                Err(AdmissionError::InvalidDeclaredStoreRoot { kind, .. })
                    if kind == expected_kind
            ));
        }
    }

    #[test]
    fn panic_after_root_reservation_releases_admission_without_reporting_success() {
        let root = std::env::temp_dir().join(format!(
            "worth-store-c3-admission-unwind-{}",
            std::process::id()
        ));
        let before = PhysicalStore::diagnostics();
        let admission = PhysicalRuntimeAdmission::new(root.clone()).unwrap();
        let panic = catch_unwind(AssertUnwindSafe(|| {
            let _ = admit_with_runtime_identity_source(admission, || {
                panic!("controlled assembly panic")
            });
        }));
        assert!(panic.is_err());

        let readmitted = PhysicalStore::admit(PhysicalRuntimeAdmission::new(root).unwrap())
            .expect("the unwind guard must release the reserved root");
        let after = PhysicalStore::diagnostics();
        assert_eq!(
            after.admission_panics_before_return(),
            before.admission_panics_before_return() + 1
        );
        assert_eq!(
            after.admitted_incarnations(),
            before.admitted_incarnations() + 1
        );
        readmitted.abort();
    }
}
