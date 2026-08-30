//! Typed failures at host release-ceremony boundaries.

use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) enum WorthQueryReleaseCeremonyError {
    InputRead {
        path: PathBuf,
        error: std::io::Error,
    },
    InputByteBudgetExceeded {
        path: PathBuf,
        maximum: u64,
    },
    InvalidPackageIdentity,
    Archive {
        stage: &'static str,
        denial: worth_query_package_archive::facade::WorthQueryPackageArchiveDenial,
    },
    Reconstruction {
        denial: worth_query_installation::facade::WorthQueryPortablePackageReconstructionDenial,
    },
    Export {
        denial: worth_query_installation::facade::WorthQueryPortablePackageExportDenial,
    },
    ReleaseDescriptionMismatch,
    ExpectationMismatch {
        field: &'static str,
    },
    OutputPathConflict,
    OutputAlreadyExists {
        path: PathBuf,
    },
    OutputWrite {
        path: PathBuf,
        error: std::io::Error,
    },
    ReportEncoding(serde_json::Error),
}

impl Display for WorthQueryReleaseCeremonyError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InputRead { path, error } => {
                write!(formatter, "cannot read {}: {error}", path.display())
            }
            Self::InputByteBudgetExceeded { path, maximum } => write!(
                formatter,
                "input {} exceeds the {} byte ceiling",
                path.display(),
                maximum
            ),
            Self::InvalidPackageIdentity => formatter
                .write_str("expected package identity must be exactly 64 hexadecimal digits"),
            Self::Archive { stage, denial } => {
                write!(formatter, "archive {stage} denied: {:?}", denial.kind())
            }
            Self::Reconstruction { denial } => {
                write!(formatter, "fresh Query readmission denied: {denial:?}")
            }
            Self::Export { denial } => {
                write!(formatter, "fresh Query re-export denied: {denial:?}")
            }
            Self::ReleaseDescriptionMismatch => formatter.write_str(
                "release description does not match the freshly re-derived Query package",
            ),
            Self::ExpectationMismatch { field } => {
                write!(formatter, "release expectation mismatch: {field}")
            }
            Self::OutputPathConflict => {
                formatter.write_str("release envelope and report must use distinct output paths")
            }
            Self::OutputAlreadyExists { path } => {
                write!(
                    formatter,
                    "refusing to replace existing output {}",
                    path.display()
                )
            }
            Self::OutputWrite { path, error } => {
                write!(formatter, "cannot write {}: {error}", path.display())
            }
            Self::ReportEncoding(error) => {
                write!(formatter, "cannot encode release report: {error}")
            }
        }
    }
}

impl std::error::Error for WorthQueryReleaseCeremonyError {}
