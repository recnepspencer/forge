use std::marker::PhantomData;

use crate::diagnostics::{
    FoundationalDiagnosticDeliveryClass, FoundationalDiagnosticOutcomeKind,
    FoundationalDiagnosticRow, FoundationalDiagnosticSubject,
    FoundationalExplanationBundleArtifactKind, FoundationalSupportReportArtifactKind,
};
use crate::profiles::{DiagnosticRichnessProfile, FoundationalProfileSet};

use super::super::vocabulary::{
    FoundationalDiagnosticAssemblyDebt, FoundationalDiagnosticCounterSnapshot,
    FoundationalDiagnosticPartiality, FoundationalDiagnosticSupportClaimStrength,
    FoundationalDiagnosticSurfaceAvailability,
};
use super::inputs::{FoundationalDiagnosticExplanationInput, FoundationalDiagnosticSupportInput};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalDiagnosticMaterializationPlan<Kind> {
    subject: FoundationalDiagnosticSubject,
    outcome_kind: FoundationalDiagnosticOutcomeKind,
    required_rows: Vec<FoundationalDiagnosticRow>,
    standard_rows: Vec<FoundationalDiagnosticRow>,
    forensic_rows: Vec<FoundationalDiagnosticRow>,
    profile: FoundationalProfileSet,
    delivery_class: FoundationalDiagnosticDeliveryClass,
    availability: FoundationalDiagnosticSurfaceAvailability,
    partiality: FoundationalDiagnosticPartiality,
    counter_snapshot: FoundationalDiagnosticCounterSnapshot,
    assembly_debts: Vec<FoundationalDiagnosticAssemblyDebt>,
    support_claim_strength: Option<FoundationalDiagnosticSupportClaimStrength>,
    _kind: PhantomData<Kind>,
}

impl FoundationalDiagnosticMaterializationPlan<FoundationalSupportReportArtifactKind> {
    pub(super) fn from_support_input(
        input: FoundationalDiagnosticSupportInput,
        profile: FoundationalProfileSet,
        delivery_class: FoundationalDiagnosticDeliveryClass,
    ) -> Self {
        let FoundationalDiagnosticSupportInput {
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
        } = input;

        Self {
            subject,
            outcome_kind,
            required_rows,
            standard_rows,
            forensic_rows,
            profile,
            delivery_class,
            availability,
            partiality,
            counter_snapshot,
            assembly_debts,
            support_claim_strength: Some(support_claim_strength),
            _kind: PhantomData,
        }
    }
}

impl FoundationalDiagnosticMaterializationPlan<FoundationalExplanationBundleArtifactKind> {
    pub(super) fn from_explanation_input(
        input: FoundationalDiagnosticExplanationInput,
        profile: FoundationalProfileSet,
        delivery_class: FoundationalDiagnosticDeliveryClass,
    ) -> Self {
        let FoundationalDiagnosticExplanationInput {
            subject,
            outcome_kind,
            required_rows,
            standard_rows,
            forensic_rows,
            availability,
            partiality,
            counter_snapshot,
            assembly_debts,
        } = input;

        Self {
            subject,
            outcome_kind,
            required_rows,
            standard_rows,
            forensic_rows,
            profile,
            delivery_class,
            availability,
            partiality,
            counter_snapshot,
            assembly_debts,
            support_claim_strength: None,
            _kind: PhantomData,
        }
    }
}

impl<Kind> FoundationalDiagnosticMaterializationPlan<Kind> {
    pub fn subject(&self) -> &FoundationalDiagnosticSubject {
        &self.subject
    }

    pub const fn outcome_kind(&self) -> FoundationalDiagnosticOutcomeKind {
        self.outcome_kind
    }

    pub fn required_rows(&self) -> &[FoundationalDiagnosticRow] {
        &self.required_rows
    }

    pub fn standard_rows(&self) -> &[FoundationalDiagnosticRow] {
        &self.standard_rows
    }

    pub fn forensic_rows(&self) -> &[FoundationalDiagnosticRow] {
        &self.forensic_rows
    }

    pub const fn profile(&self) -> FoundationalProfileSet {
        self.profile
    }

    pub const fn delivery_class(&self) -> FoundationalDiagnosticDeliveryClass {
        self.delivery_class
    }

    pub const fn availability(&self) -> FoundationalDiagnosticSurfaceAvailability {
        self.availability
    }

    pub fn partiality(&self) -> &FoundationalDiagnosticPartiality {
        &self.partiality
    }

    pub const fn counter_snapshot(&self) -> FoundationalDiagnosticCounterSnapshot {
        self.counter_snapshot
    }

    pub fn assembly_debts(&self) -> &[FoundationalDiagnosticAssemblyDebt] {
        &self.assembly_debts
    }

    pub const fn support_claim_strength(
        &self,
    ) -> Option<FoundationalDiagnosticSupportClaimStrength> {
        self.support_claim_strength
    }

    pub fn selected_rows(&self) -> Vec<FoundationalDiagnosticRow> {
        let mut rows = self.required_rows.clone();
        if self.profile.diagnostic_richness() != DiagnosticRichnessProfile::OperationalMinimal {
            rows.extend(self.standard_rows.clone());
        }
        if self.profile.diagnostic_richness() == DiagnosticRichnessProfile::Forensic {
            rows.extend(self.forensic_rows.clone());
        }
        crate::diagnostics::sort_foundational_diagnostic_rows(&mut rows);
        rows
    }
}
