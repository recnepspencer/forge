use crate::{
    RuntimeVerifierComparisonClassification, RuntimeVerifierComparisonDenial,
    RuntimeVerifierComparisonReport, RuntimeVerifierParityTrace, RuntimeVerifierRelationship,
};
use forge_store_physical_format::{
    PhysicalShortcutBoundary, PlatformPhysicalFacadeDenial, PlatformPhysicalFacadeDenialKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeVerifierDiagnosticKind {
    LayoutParity,
    LayoutMismatch,
    ShortcutRejected,
    SemanticDecodeRejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeVerifierDiagnosticReport {
    kind: RuntimeVerifierDiagnosticKind,
    comparison: Option<RuntimeVerifierComparisonClassification>,
    shortcut_boundary: Option<PhysicalShortcutBoundary>,
    parity_trace: RuntimeVerifierParityTrace,
}

impl RuntimeVerifierDiagnosticReport {
    pub fn from_comparison(
        report: &RuntimeVerifierComparisonReport,
    ) -> Result<Self, RuntimeVerifierDiagnosticDenial> {
        if report.classification() != RuntimeVerifierComparisonClassification::Equivalent {
            return Err(RuntimeVerifierDiagnosticDenial::UnexpectedMismatch(
                report.classification(),
            ));
        }
        Ok(Self {
            kind: RuntimeVerifierDiagnosticKind::LayoutParity,
            comparison: Some(report.classification()),
            shortcut_boundary: None,
            parity_trace: report.parity_trace(),
        })
    }

    pub fn from_mismatch(denial: &RuntimeVerifierComparisonDenial) -> Self {
        let kind = if denial.classification()
            == RuntimeVerifierComparisonClassification::SemanticDecodeAttempted
        {
            RuntimeVerifierDiagnosticKind::SemanticDecodeRejected
        } else {
            RuntimeVerifierDiagnosticKind::LayoutMismatch
        };
        Self {
            kind,
            comparison: Some(denial.classification()),
            shortcut_boundary: None,
            parity_trace: denial.parity_trace(),
        }
    }

    pub fn from_shortcut_facade_denial(
        denial: &PlatformPhysicalFacadeDenial,
    ) -> Result<Self, RuntimeVerifierDiagnosticDenial> {
        if denial.kind() != PlatformPhysicalFacadeDenialKind::ShortcutBoundaryRejected {
            return Err(RuntimeVerifierDiagnosticDenial::UnexpectedFacadeDenial(
                denial.kind(),
            ));
        }
        let shortcut_denial = denial
            .shortcut_denial()
            .ok_or(RuntimeVerifierDiagnosticDenial::MissingShortcutBoundary)?;
        Ok(Self {
            kind: RuntimeVerifierDiagnosticKind::ShortcutRejected,
            comparison: None,
            shortcut_boundary: Some(shortcut_denial.boundary()),
            parity_trace: RuntimeVerifierParityTrace::new(
                RuntimeVerifierRelationship::NotApplicable,
            ),
        })
    }

    pub const fn kind(&self) -> RuntimeVerifierDiagnosticKind {
        self.kind
    }

    pub const fn comparison(&self) -> Option<RuntimeVerifierComparisonClassification> {
        self.comparison
    }

    pub const fn shortcut_boundary(&self) -> Option<PhysicalShortcutBoundary> {
        self.shortcut_boundary
    }

    pub const fn parity_trace(&self) -> RuntimeVerifierParityTrace {
        self.parity_trace
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeVerifierDiagnosticDenial {
    UnexpectedMismatch(RuntimeVerifierComparisonClassification),
    UnexpectedFacadeDenial(PlatformPhysicalFacadeDenialKind),
    MissingShortcutBoundary,
}
