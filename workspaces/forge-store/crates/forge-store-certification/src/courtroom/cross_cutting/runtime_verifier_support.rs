use crate::{
    RuntimeVerifierComparisonClassification, RuntimeVerifierComparisonDenial,
    RuntimeVerifierComparisonReport,
};
use forge_store_physical_format::{
    PhysicalShortcutBoundary, PhysicalStoreRuntimeDenial, PhysicalStoreRuntimeDenialKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeVerifierSupportReport {
    classification: RuntimeVerifierComparisonClassification,
    runtime_reference_count: u32,
    offline_reference_count: u32,
    semantic_decode_attempts: u32,
    forbidden_shortcuts: Vec<PhysicalShortcutBoundary>,
}

impl RuntimeVerifierSupportReport {
    pub fn from_comparison(
        report: &RuntimeVerifierComparisonReport,
    ) -> Result<Self, RuntimeVerifierSupportDenial> {
        if report.classification() != RuntimeVerifierComparisonClassification::Equivalent {
            return Err(RuntimeVerifierSupportDenial::UnexpectedMismatch(
                report.classification(),
            ));
        }
        Ok(Self {
            classification: report.classification(),
            runtime_reference_count: report.runtime_reference_count(),
            offline_reference_count: report.offline_reference_count(),
            semantic_decode_attempts: report.runtime_semantic_decode_attempts()
                + report.offline_semantic_decode_attempts(),
            forbidden_shortcuts: Vec::new(),
        })
    }

    pub fn from_mismatch(denial: &RuntimeVerifierComparisonDenial) -> Self {
        let report = denial.report();
        Self {
            classification: report.classification(),
            runtime_reference_count: report.runtime_reference_count(),
            offline_reference_count: report.offline_reference_count(),
            semantic_decode_attempts: report.runtime_semantic_decode_attempts()
                + report.offline_semantic_decode_attempts(),
            forbidden_shortcuts: Vec::new(),
        }
    }

    pub fn from_shortcut_facade_denial(
        denial: &PhysicalStoreRuntimeDenial,
    ) -> Result<Self, RuntimeVerifierSupportDenial> {
        if denial.kind() != PhysicalStoreRuntimeDenialKind::ShortcutBoundaryRejected {
            return Err(RuntimeVerifierSupportDenial::UnexpectedFacadeDenial(
                denial.kind(),
            ));
        }
        let shortcut_denial = denial
            .shortcut_denial()
            .ok_or(RuntimeVerifierSupportDenial::MissingShortcutBoundary)?;
        Ok(Self {
            classification: RuntimeVerifierComparisonClassification::Equivalent,
            runtime_reference_count: 0,
            offline_reference_count: 0,
            semantic_decode_attempts: 0,
            forbidden_shortcuts: vec![shortcut_denial.boundary()],
        })
    }

    pub const fn classification(&self) -> RuntimeVerifierComparisonClassification {
        self.classification
    }

    pub const fn runtime_reference_count(&self) -> u32 {
        self.runtime_reference_count
    }

    pub const fn offline_reference_count(&self) -> u32 {
        self.offline_reference_count
    }

    pub const fn semantic_decode_attempts(&self) -> u32 {
        self.semantic_decode_attempts
    }

    pub fn forbidden_shortcuts(&self) -> &[PhysicalShortcutBoundary] {
        &self.forbidden_shortcuts
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeVerifierSupportDenial {
    UnexpectedMismatch(RuntimeVerifierComparisonClassification),
    UnexpectedFacadeDenial(PhysicalStoreRuntimeDenialKind),
    MissingShortcutBoundary,
}
