use crate::diagnostics::{
    FoundationalDiagnosticOutcomeKind, FoundationalDiagnosticRow, FoundationalDiagnosticSubject,
};

use super::super::vocabulary::{
    FoundationalDiagnosticAssemblyDebt, FoundationalDiagnosticCounterSnapshot,
    FoundationalDiagnosticPartiality, FoundationalDiagnosticSupportClaimStrength,
    FoundationalDiagnosticSurfaceAvailability,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalDiagnosticSupportInput {
    pub(super) subject: FoundationalDiagnosticSubject,
    pub(super) outcome_kind: FoundationalDiagnosticOutcomeKind,
    pub(super) required_rows: Vec<FoundationalDiagnosticRow>,
    pub(super) standard_rows: Vec<FoundationalDiagnosticRow>,
    pub(super) forensic_rows: Vec<FoundationalDiagnosticRow>,
    pub(super) availability: FoundationalDiagnosticSurfaceAvailability,
    pub(super) support_claim_strength: FoundationalDiagnosticSupportClaimStrength,
    pub(super) partiality: FoundationalDiagnosticPartiality,
    pub(super) counter_snapshot: FoundationalDiagnosticCounterSnapshot,
    pub(super) assembly_debts: Vec<FoundationalDiagnosticAssemblyDebt>,
}

impl FoundationalDiagnosticSupportInput {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        subject: FoundationalDiagnosticSubject,
        outcome_kind: FoundationalDiagnosticOutcomeKind,
        required_rows: Vec<FoundationalDiagnosticRow>,
        standard_rows: Vec<FoundationalDiagnosticRow>,
        forensic_rows: Vec<FoundationalDiagnosticRow>,
        availability: FoundationalDiagnosticSurfaceAvailability,
        support_claim_strength: FoundationalDiagnosticSupportClaimStrength,
        partiality: FoundationalDiagnosticPartiality,
        counter_snapshot: FoundationalDiagnosticCounterSnapshot,
        assembly_debts: Vec<FoundationalDiagnosticAssemblyDebt>,
    ) -> Self {
        Self {
            subject,
            outcome_kind,
            required_rows,
            standard_rows,
            forensic_rows,
            availability,
            support_claim_strength,
            partiality,
            counter_snapshot,
            assembly_debts,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalDiagnosticExplanationInput {
    pub(super) subject: FoundationalDiagnosticSubject,
    pub(super) outcome_kind: FoundationalDiagnosticOutcomeKind,
    pub(super) required_rows: Vec<FoundationalDiagnosticRow>,
    pub(super) standard_rows: Vec<FoundationalDiagnosticRow>,
    pub(super) forensic_rows: Vec<FoundationalDiagnosticRow>,
    pub(super) availability: FoundationalDiagnosticSurfaceAvailability,
    pub(super) partiality: FoundationalDiagnosticPartiality,
    pub(super) counter_snapshot: FoundationalDiagnosticCounterSnapshot,
    pub(super) assembly_debts: Vec<FoundationalDiagnosticAssemblyDebt>,
}

impl FoundationalDiagnosticExplanationInput {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        subject: FoundationalDiagnosticSubject,
        outcome_kind: FoundationalDiagnosticOutcomeKind,
        required_rows: Vec<FoundationalDiagnosticRow>,
        standard_rows: Vec<FoundationalDiagnosticRow>,
        forensic_rows: Vec<FoundationalDiagnosticRow>,
        availability: FoundationalDiagnosticSurfaceAvailability,
        partiality: FoundationalDiagnosticPartiality,
        counter_snapshot: FoundationalDiagnosticCounterSnapshot,
        assembly_debts: Vec<FoundationalDiagnosticAssemblyDebt>,
    ) -> Self {
        Self {
            subject,
            outcome_kind,
            required_rows,
            standard_rows,
            forensic_rows,
            availability,
            partiality,
            counter_snapshot,
            assembly_debts,
        }
    }
}
