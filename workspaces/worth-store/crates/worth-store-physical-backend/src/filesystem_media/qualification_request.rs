use std::path::{Path, PathBuf};

use super::qualification_basis::RootProfileBinding;
use super::{RootProfileQualificationBasis, RootProfileQualificationReport};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemQualificationMode {
    Production,
    Certification,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemAccessPosture {
    /// Declares the deployment contract that every supported writer uses the
    /// canonical Store lease. This declaration is not authority by itself;
    /// qualification binds it to a live OS-owned lease and admitted identity.
    CoordinatedServiceAccount,
    UnmanagedWritersPossible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemAccessContract {
    CoordinatedServiceAccount,
}

impl FilesystemAccessPosture {
    pub(super) const fn admitted_contract(self) -> Option<FilesystemAccessContract> {
        match self {
            Self::CoordinatedServiceAccount => {
                Some(FilesystemAccessContract::CoordinatedServiceAccount)
            }
            Self::UnmanagedWritersPossible => None,
        }
    }
}

#[derive(Debug)]
pub struct FilesystemQualificationRequest {
    pub(super) root: PathBuf,
    pub(super) mode: FilesystemQualificationMode,
    pub(super) access: FilesystemAccessPosture,
    pub(super) expected_basis: Option<RootProfileBinding>,
    pub(super) fault_schedule: super::MediaFaultSchedule,
    pub(super) runtime_incarnation: Option<u64>,
}

impl FilesystemQualificationRequest {
    #[cfg(any(test, feature = "store-runtime-owner"))]
    pub fn production(root: impl Into<PathBuf>, access: FilesystemAccessPosture) -> Self {
        Self {
            root: root.into(),
            mode: FilesystemQualificationMode::Production,
            access,
            expected_basis: None,
            fault_schedule: super::MediaFaultSchedule::default(),
            runtime_incarnation: None,
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub fn certification(root: impl Into<PathBuf>, access: FilesystemAccessPosture) -> Self {
        Self {
            root: root.into(),
            mode: FilesystemQualificationMode::Certification,
            access,
            expected_basis: None,
            fault_schedule: super::MediaFaultSchedule::default(),
            runtime_incarnation: None,
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub fn with_fault_schedule(mut self, schedule: super::MediaFaultSchedule) -> Self {
        self.fault_schedule = schedule;
        self
    }

    pub fn require_current_basis(mut self, basis: &RootProfileQualificationBasis) -> Self {
        self.expected_basis = Some(basis.binding().clone());
        self
    }

    pub fn for_runtime_incarnation(mut self, identity: u64) -> Self {
        self.runtime_incarnation = Some(identity);
        self
    }

    pub fn require_current_profile(mut self, report: RootProfileQualificationReport) -> Self {
        self.expected_basis = Some(report.into_binding());
        self
    }

    pub fn root(&self) -> &Path {
        &self.root
    }
}
