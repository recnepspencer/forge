use std::{error::Error, fmt};

use crate::facade::WorthUiRuntimeLaunchPreparationDenial;

impl WorthUiRuntimeLaunchPreparationDenial {
    pub fn diagnostic_lines(&self) -> &[String] {
        match self {
            Self::SourcePackageRejected { diagnostics }
            | Self::ParseRejected { diagnostics }
            | Self::AuthoringEntryRejected { diagnostics }
            | Self::SnapshotResolutionRejected { diagnostics }
            | Self::StructuralLegalityRejected { diagnostics }
            | Self::BindingSemanticsRejected { diagnostics }
            | Self::IdentitySeedingRejected { diagnostics }
            | Self::ArtifactAssemblyRejected { diagnostics }
            | Self::ContentSlotCatalogRejected { diagnostics } => diagnostics,
            Self::EmptySourcePackage => &[],
        }
    }
}

impl fmt::Display for WorthUiRuntimeLaunchPreparationDenial {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySourcePackage => write!(formatter, "No source modules were provided."),
            other => {
                writeln!(formatter, "{}", launch_denial_kind(other))?;
                for diagnostic in other.diagnostic_lines() {
                    writeln!(formatter, "- {diagnostic}")?;
                }
                Ok(())
            }
        }
    }
}

impl Error for WorthUiRuntimeLaunchPreparationDenial {}

pub(super) fn source_package_diagnostics(
    report: &crate::source::WorthUiSourcePackageReport,
) -> Vec<String> {
    report
        .diagnostics()
        .iter()
        .map(|diagnostic| {
            format!(
                "{:?}: {} path={:?} module={:?} related={:?}",
                diagnostic.code(),
                diagnostic.message(),
                diagnostic.module_path(),
                diagnostic.module_id_text(),
                diagnostic.related_module_id_text()
            )
        })
        .collect()
}

pub(super) fn parse_diagnostics(report: &crate::source::WorthUiParseReport) -> Vec<String> {
    report
        .diagnostics()
        .iter()
        .map(|diagnostic| {
            format!(
                "{:?}: {} at {}:{}..{}",
                diagnostic.code(),
                diagnostic.message(),
                diagnostic.span().module_id().as_str(),
                diagnostic.span().start_byte(),
                diagnostic.span().end_byte()
            )
        })
        .collect()
}

pub(super) fn authoring_entry_diagnostics(
    report: &crate::source::WorthUiAuthoringEntryReport,
) -> Vec<String> {
    report
        .diagnostics()
        .iter()
        .map(|diagnostic| {
            format!(
                "{:?}: {} at {}:{}..{}",
                diagnostic.code(),
                diagnostic.message(),
                diagnostic.span().module_id().as_str(),
                diagnostic.span().start_byte(),
                diagnostic.span().end_byte()
            )
        })
        .collect()
}

pub(super) fn snapshot_resolution_diagnostics(
    report: &crate::source::WorthUiResolutionReport,
) -> Vec<String> {
    report
        .diagnostics()
        .iter()
        .map(|diagnostic| {
            format!(
                "{:?}: '{}' in {} provenance={:?}",
                diagnostic.code(),
                diagnostic.authored_text(),
                diagnostic.module_id().as_str(),
                diagnostic.provenance()
            )
        })
        .collect()
}

pub(super) fn debug_diagnostics<T: fmt::Debug>(diagnostics: &[T]) -> Vec<String> {
    diagnostics
        .iter()
        .map(|diagnostic| format!("{diagnostic:?}"))
        .collect()
}

pub(super) fn content_slot_diagnostics(
    diagnostics: &[crate::source::WorthUiContentSlotDiagnostic],
) -> Vec<String> {
    diagnostics
        .iter()
        .map(|diagnostic| {
            format!(
                "{:?}: page={} {}",
                diagnostic.code(),
                diagnostic.page_name(),
                diagnostic.detail()
            )
        })
        .collect()
}

fn launch_denial_kind(denial: &WorthUiRuntimeLaunchPreparationDenial) -> &'static str {
    match denial {
        WorthUiRuntimeLaunchPreparationDenial::EmptySourcePackage => "empty source package",
        WorthUiRuntimeLaunchPreparationDenial::SourcePackageRejected { .. } => {
            "source package rejected"
        }
        WorthUiRuntimeLaunchPreparationDenial::ParseRejected { .. } => "parse rejected",
        WorthUiRuntimeLaunchPreparationDenial::AuthoringEntryRejected { .. } => {
            "authoring entry rejected"
        }
        WorthUiRuntimeLaunchPreparationDenial::SnapshotResolutionRejected { .. } => {
            "snapshot resolution rejected"
        }
        WorthUiRuntimeLaunchPreparationDenial::StructuralLegalityRejected { .. } => {
            "structural legality rejected"
        }
        WorthUiRuntimeLaunchPreparationDenial::BindingSemanticsRejected { .. } => {
            "binding semantics rejected"
        }
        WorthUiRuntimeLaunchPreparationDenial::IdentitySeedingRejected { .. } => {
            "identity seeding rejected"
        }
        WorthUiRuntimeLaunchPreparationDenial::ArtifactAssemblyRejected { .. } => {
            "artifact assembly rejected"
        }
        WorthUiRuntimeLaunchPreparationDenial::ContentSlotCatalogRejected { .. } => {
            "content slot catalog rejected"
        }
    }
}
