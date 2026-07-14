use crate::diagnostics::{
    FoundationalDiagnosticArtifactKind, FoundationalDiagnosticComparisonRow,
    FoundationalDiagnosticDecisionRow, FoundationalDiagnosticDeliveryClass,
    FoundationalDiagnosticFailureRow, FoundationalDiagnosticOutcomeKind,
    FoundationalDiagnosticProvenanceReadyRow, FoundationalDiagnosticRow,
    FoundationalDiagnosticSubject, FoundationalDiagnosticSupportRow,
    FoundationalExplanationBundleArtifactKind, FoundationalSupportReportArtifactKind,
};
use crate::profiles::FoundationalProfileSet;

use super::planning::{
    plan_diagnostic_explanation_bundle, plan_diagnostic_support_report,
    FoundationalDiagnosticExplanationInput, FoundationalDiagnosticMaterializationPlan,
    FoundationalDiagnosticSupportInput,
};
use super::vocabulary::{
    FoundationalDiagnosticAssemblyDebt, FoundationalDiagnosticCounterSnapshot,
    FoundationalDiagnosticMaterializationDenial, FoundationalDiagnosticNamedGap,
    FoundationalDiagnosticPartiality, FoundationalDiagnosticSupportClaimStrength,
    FoundationalDiagnosticSurfaceAvailability,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalDiagnosticSupportReport {
    subject: FoundationalDiagnosticSubject,
    outcome_kind: FoundationalDiagnosticOutcomeKind,
    rows: Vec<FoundationalDiagnosticRow>,
    profile: FoundationalProfileSet,
    delivery_class: FoundationalDiagnosticDeliveryClass,
    availability: FoundationalDiagnosticSurfaceAvailability,
    partiality: FoundationalDiagnosticPartiality,
    counter_snapshot: FoundationalDiagnosticCounterSnapshot,
    assembly_debts: Vec<FoundationalDiagnosticAssemblyDebt>,
    support_claim_strength: FoundationalDiagnosticSupportClaimStrength,
    _kind: FoundationalSupportReportArtifactKind,
}

impl FoundationalDiagnosticSupportReport {
    pub const fn artifact_kind(&self) -> FoundationalDiagnosticArtifactKind {
        FoundationalDiagnosticArtifactKind::SupportReport
    }

    pub fn subject(&self) -> &FoundationalDiagnosticSubject {
        &self.subject
    }

    pub const fn outcome_kind(&self) -> FoundationalDiagnosticOutcomeKind {
        self.outcome_kind
    }

    pub fn rows(&self) -> &[FoundationalDiagnosticRow] {
        &self.rows
    }

    pub fn decision_rows(&self) -> impl Iterator<Item = &FoundationalDiagnosticDecisionRow> {
        self.rows.iter().filter_map(|row| match row {
            FoundationalDiagnosticRow::Decision(value) => Some(value),
            _ => None,
        })
    }

    pub fn comparison_rows(&self) -> impl Iterator<Item = &FoundationalDiagnosticComparisonRow> {
        self.rows.iter().filter_map(|row| match row {
            FoundationalDiagnosticRow::Comparison(value) => Some(value),
            _ => None,
        })
    }

    pub fn failure_rows(&self) -> impl Iterator<Item = &FoundationalDiagnosticFailureRow> {
        self.rows.iter().filter_map(|row| match row {
            FoundationalDiagnosticRow::Failure(value) => Some(value),
            _ => None,
        })
    }

    pub fn support_rows(&self) -> impl Iterator<Item = &FoundationalDiagnosticSupportRow> {
        self.rows.iter().filter_map(|row| match row {
            FoundationalDiagnosticRow::Support(value) => Some(value),
            _ => None,
        })
    }

    pub fn provenance_ready_rows(
        &self,
    ) -> impl Iterator<Item = &FoundationalDiagnosticProvenanceReadyRow> {
        self.rows.iter().filter_map(|row| match row {
            FoundationalDiagnosticRow::ProvenanceReady(value) => Some(value),
            _ => None,
        })
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

    pub fn named_gaps(&self) -> &[FoundationalDiagnosticNamedGap] {
        self.partiality.named_gaps()
    }

    pub const fn counter_snapshot(&self) -> FoundationalDiagnosticCounterSnapshot {
        self.counter_snapshot
    }

    pub fn assembly_debts(&self) -> &[FoundationalDiagnosticAssemblyDebt] {
        &self.assembly_debts
    }

    pub const fn support_claim_strength(&self) -> FoundationalDiagnosticSupportClaimStrength {
        self.support_claim_strength
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalDiagnosticExplanationBundle {
    subject: FoundationalDiagnosticSubject,
    outcome_kind: FoundationalDiagnosticOutcomeKind,
    rows: Vec<FoundationalDiagnosticRow>,
    profile: FoundationalProfileSet,
    delivery_class: FoundationalDiagnosticDeliveryClass,
    availability: FoundationalDiagnosticSurfaceAvailability,
    partiality: FoundationalDiagnosticPartiality,
    counter_snapshot: FoundationalDiagnosticCounterSnapshot,
    assembly_debts: Vec<FoundationalDiagnosticAssemblyDebt>,
    _kind: FoundationalExplanationBundleArtifactKind,
}

impl FoundationalDiagnosticExplanationBundle {
    pub const fn artifact_kind(&self) -> FoundationalDiagnosticArtifactKind {
        FoundationalDiagnosticArtifactKind::ExplanationBundle
    }

    pub fn subject(&self) -> &FoundationalDiagnosticSubject {
        &self.subject
    }

    pub const fn outcome_kind(&self) -> FoundationalDiagnosticOutcomeKind {
        self.outcome_kind
    }

    pub fn rows(&self) -> &[FoundationalDiagnosticRow] {
        &self.rows
    }

    pub fn decision_rows(&self) -> impl Iterator<Item = &FoundationalDiagnosticDecisionRow> {
        self.rows.iter().filter_map(|row| match row {
            FoundationalDiagnosticRow::Decision(value) => Some(value),
            _ => None,
        })
    }

    pub fn comparison_rows(&self) -> impl Iterator<Item = &FoundationalDiagnosticComparisonRow> {
        self.rows.iter().filter_map(|row| match row {
            FoundationalDiagnosticRow::Comparison(value) => Some(value),
            _ => None,
        })
    }

    pub fn failure_rows(&self) -> impl Iterator<Item = &FoundationalDiagnosticFailureRow> {
        self.rows.iter().filter_map(|row| match row {
            FoundationalDiagnosticRow::Failure(value) => Some(value),
            _ => None,
        })
    }

    pub fn support_rows(&self) -> impl Iterator<Item = &FoundationalDiagnosticSupportRow> {
        self.rows.iter().filter_map(|row| match row {
            FoundationalDiagnosticRow::Support(value) => Some(value),
            _ => None,
        })
    }

    pub fn provenance_ready_rows(
        &self,
    ) -> impl Iterator<Item = &FoundationalDiagnosticProvenanceReadyRow> {
        self.rows.iter().filter_map(|row| match row {
            FoundationalDiagnosticRow::ProvenanceReady(value) => Some(value),
            _ => None,
        })
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

    pub fn named_gaps(&self) -> &[FoundationalDiagnosticNamedGap] {
        self.partiality.named_gaps()
    }

    pub const fn counter_snapshot(&self) -> FoundationalDiagnosticCounterSnapshot {
        self.counter_snapshot
    }

    pub fn assembly_debts(&self) -> &[FoundationalDiagnosticAssemblyDebt] {
        &self.assembly_debts
    }
}

impl FoundationalDiagnosticMaterializationPlan<FoundationalSupportReportArtifactKind> {
    pub fn materialize(self) -> FoundationalDiagnosticSupportReport {
        FoundationalDiagnosticSupportReport {
            subject: self.subject().clone(),
            outcome_kind: self.outcome_kind(),
            rows: self.selected_rows(),
            profile: self.profile(),
            delivery_class: self.delivery_class(),
            availability: self.availability(),
            partiality: self.partiality().clone(),
            counter_snapshot: self.counter_snapshot(),
            assembly_debts: self.assembly_debts().to_vec(),
            support_claim_strength: self
                .support_claim_strength()
                .expect("support report plan always carries claim strength"),
            _kind: crate::diagnostics::foundational_support_report_artifact_kind(),
        }
    }
}

impl FoundationalDiagnosticMaterializationPlan<FoundationalExplanationBundleArtifactKind> {
    pub fn materialize(self) -> FoundationalDiagnosticExplanationBundle {
        FoundationalDiagnosticExplanationBundle {
            subject: self.subject().clone(),
            outcome_kind: self.outcome_kind(),
            rows: self.selected_rows(),
            profile: self.profile(),
            delivery_class: self.delivery_class(),
            availability: self.availability(),
            partiality: self.partiality().clone(),
            counter_snapshot: self.counter_snapshot(),
            assembly_debts: self.assembly_debts().to_vec(),
            _kind: crate::diagnostics::foundational_explanation_bundle_artifact_kind(),
        }
    }
}

pub fn materialize_diagnostic_support_report(
    input: FoundationalDiagnosticSupportInput,
    profile: FoundationalProfileSet,
    delivery_class: FoundationalDiagnosticDeliveryClass,
) -> Result<FoundationalDiagnosticSupportReport, FoundationalDiagnosticMaterializationDenial> {
    plan_diagnostic_support_report(input, profile, delivery_class).map(|plan| plan.materialize())
}

pub fn materialize_diagnostic_explanation_bundle(
    input: FoundationalDiagnosticExplanationInput,
    profile: FoundationalProfileSet,
    delivery_class: FoundationalDiagnosticDeliveryClass,
) -> Result<FoundationalDiagnosticExplanationBundle, FoundationalDiagnosticMaterializationDenial> {
    plan_diagnostic_explanation_bundle(input, profile, delivery_class)
        .map(|plan| plan.materialize())
}
