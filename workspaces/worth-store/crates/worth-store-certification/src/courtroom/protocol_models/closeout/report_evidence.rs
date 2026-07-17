use worth_store_aspect_native::{
    StoreDiagnosticExplanationBundleEvidence, StoreDiagnosticSupportReportEvidence,
};

use crate::courtroom::protocol_models::mutants::ControlledMutantRejection;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CounterexampleDiagnosticEvidence {
    controlled_defect: ControlledMutantRejection,
    support_report: StoreDiagnosticSupportReportEvidence,
    explanation_bundle: StoreDiagnosticExplanationBundleEvidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CounterexampleDiagnosticEvidenceDenial {
    PhysicalWitnessMismatch,
}

impl CounterexampleDiagnosticEvidence {
    pub fn bind(
        controlled_defect: ControlledMutantRejection,
        support_report: StoreDiagnosticSupportReportEvidence,
        explanation_bundle: StoreDiagnosticExplanationBundleEvidence,
    ) -> Result<Self, CounterexampleDiagnosticEvidenceDenial> {
        if support_report.physical_witness() != explanation_bundle.physical_witness() {
            return Err(CounterexampleDiagnosticEvidenceDenial::PhysicalWitnessMismatch);
        }
        Ok(Self {
            controlled_defect,
            support_report,
            explanation_bundle,
        })
    }

    pub const fn controlled_defect(&self) -> &ControlledMutantRejection {
        &self.controlled_defect
    }

    pub const fn support_report(&self) -> &StoreDiagnosticSupportReportEvidence {
        &self.support_report
    }

    pub const fn explanation_bundle(&self) -> &StoreDiagnosticExplanationBundleEvidence {
        &self.explanation_bundle
    }
}
