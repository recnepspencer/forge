use std::path::{Path, PathBuf};

use worth_foundational::facade::{
    BoundaryProtocolCompatibilityWindow, BoundaryProtocolIdentity, BoundaryProtocolVersion,
};

pub static PHYSICAL_INTEGRITY_OBSERVATION_PROTOCOL_IDENTITY: BoundaryProtocolIdentity =
    BoundaryProtocolIdentity::new("store.physical.integrity-observation");
pub const PHYSICAL_INTEGRITY_OBSERVATION_PROTOCOL_VERSION: BoundaryProtocolVersion =
    BoundaryProtocolVersion::new(1);
pub const PHYSICAL_INTEGRITY_OBSERVATION_COMPATIBILITY: BoundaryProtocolCompatibilityWindow =
    BoundaryProtocolCompatibilityWindow::inclusive(
        PHYSICAL_INTEGRITY_OBSERVATION_PROTOCOL_VERSION,
        PHYSICAL_INTEGRITY_OBSERVATION_PROTOCOL_VERSION,
    );

#[derive(Debug, Clone, PartialEq, Eq)]
enum OfflineIntegrityReportDestinationKind {
    StandardOutput,
    File(PathBuf),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfflineIntegrityReportDestination {
    kind: OfflineIntegrityReportDestinationKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfflineIntegrityReportDestinationDenial {
    EmptyFilePath,
}

impl OfflineIntegrityReportDestination {
    pub const fn standard_output() -> Self {
        Self {
            kind: OfflineIntegrityReportDestinationKind::StandardOutput,
        }
    }

    pub fn file(path: PathBuf) -> Result<Self, OfflineIntegrityReportDestinationDenial> {
        if path.as_os_str().is_empty() {
            return Err(OfflineIntegrityReportDestinationDenial::EmptyFilePath);
        }
        Ok(Self {
            kind: OfflineIntegrityReportDestinationKind::File(path),
        })
    }

    pub fn file_path(&self) -> Option<&Path> {
        match &self.kind {
            OfflineIntegrityReportDestinationKind::StandardOutput => None,
            OfflineIntegrityReportDestinationKind::File(path) => Some(path),
        }
    }

    pub const fn is_standard_output(&self) -> bool {
        matches!(
            &self.kind,
            OfflineIntegrityReportDestinationKind::StandardOutput
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfflineIntegrityReportBoundary {
    destination: OfflineIntegrityReportDestination,
    maximum_bytes: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfflineIntegrityReportBoundaryDenial {
    DestinationInsideDeclaredStoreRoot,
}

impl OfflineIntegrityReportBoundary {
    pub(crate) fn new(
        store_root: &Path,
        destination: OfflineIntegrityReportDestination,
        maximum_bytes: u64,
    ) -> Result<Self, OfflineIntegrityReportBoundaryDenial> {
        if let Some(file_path) = destination.file_path() {
            if file_path == store_root || file_path.starts_with(store_root) {
                return Err(
                    OfflineIntegrityReportBoundaryDenial::DestinationInsideDeclaredStoreRoot,
                );
            }
        }
        Ok(Self {
            destination,
            maximum_bytes,
        })
    }

    pub const fn destination(&self) -> &OfflineIntegrityReportDestination {
        &self.destination
    }

    pub const fn maximum_bytes(&self) -> u64 {
        self.maximum_bytes
    }

    pub const fn protocol_identity(&self) -> &'static BoundaryProtocolIdentity {
        &PHYSICAL_INTEGRITY_OBSERVATION_PROTOCOL_IDENTITY
    }

    pub const fn protocol_version(&self) -> BoundaryProtocolVersion {
        PHYSICAL_INTEGRITY_OBSERVATION_PROTOCOL_VERSION
    }

    pub const fn compatibility_window(&self) -> BoundaryProtocolCompatibilityWindow {
        PHYSICAL_INTEGRITY_OBSERVATION_COMPATIBILITY
    }
}
