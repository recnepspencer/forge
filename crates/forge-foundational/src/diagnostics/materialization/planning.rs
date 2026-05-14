use std::marker::PhantomData;

use crate::diagnostics::{
    evaluate_diagnostic_materialization_legality, FoundationalDiagnosticAvailability,
    FoundationalDiagnosticDeliveryClass, FoundationalDiagnosticOutcomeKind,
    FoundationalDiagnosticRow, FoundationalDiagnosticSubject,
    FoundationalExplanationBundleArtifactKind, FoundationalSupportReportArtifactKind,
};
use crate::profiles::{
    DiagnosticRichnessProfile, FoundationalProfileSet, RetentionDeliveryProfile,
    SupportPostureProfile,
};

use super::vocabulary::{
    FoundationalDiagnosticAssemblyDebt, FoundationalDiagnosticAssemblyDebtClass,
    FoundationalDiagnosticCounterSnapshot, FoundationalDiagnosticMaterializationDenial,
    FoundationalDiagnosticPartiality, FoundationalDiagnosticSupportClaimStrength,
    FoundationalDiagnosticSurfaceAvailability,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalDiagnosticSupportInput {
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
    subject: FoundationalDiagnosticSubject,
    outcome_kind: FoundationalDiagnosticOutcomeKind,
    required_rows: Vec<FoundationalDiagnosticRow>,
    standard_rows: Vec<FoundationalDiagnosticRow>,
    forensic_rows: Vec<FoundationalDiagnosticRow>,
    availability: FoundationalDiagnosticSurfaceAvailability,
    partiality: FoundationalDiagnosticPartiality,
    counter_snapshot: FoundationalDiagnosticCounterSnapshot,
    assembly_debts: Vec<FoundationalDiagnosticAssemblyDebt>,
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

pub fn plan_diagnostic_support_report(
    input: FoundationalDiagnosticSupportInput,
    profile: FoundationalProfileSet,
    delivery_class: FoundationalDiagnosticDeliveryClass,
) -> Result<
    FoundationalDiagnosticMaterializationPlan<FoundationalSupportReportArtifactKind>,
    FoundationalDiagnosticMaterializationDenial,
> {
    validate_common(
        input.availability,
        &input.partiality,
        &input.assembly_debts,
        delivery_class,
        FoundationalDiagnosticAvailability::from(input.availability),
        crate::diagnostics::FoundationalDiagnosticArtifactKind::SupportReport,
    )?;

    if input.support_claim_strength
        == FoundationalDiagnosticSupportClaimStrength::DurableSupportReady
    {
        if profile.support_posture() == SupportPostureProfile::InternalOnly {
            return Err(FoundationalDiagnosticMaterializationDenial::InternalSupportCannotClaimDurableSupport);
        }
        if matches!(
            profile.retention_delivery(),
            RetentionDeliveryProfile::Ephemeral
        ) || !matches!(
            input.availability.availability(),
            FoundationalDiagnosticAvailability::RetainedHot
                | FoundationalDiagnosticAvailability::DeferredCold
                | FoundationalDiagnosticAvailability::Reconstructable
        ) {
            return Err(
                FoundationalDiagnosticMaterializationDenial::DurableSupportRequiresVisibleEvidence,
            );
        }
        if !has_visible_rows_for_profile(
            &input.required_rows,
            &input.standard_rows,
            &input.forensic_rows,
            profile,
        ) {
            return Err(
                FoundationalDiagnosticMaterializationDenial::DurableSupportRequiresVisibleRowsAtChosenRichness,
            );
        }
    }
    if input.support_claim_strength
        == FoundationalDiagnosticSupportClaimStrength::CertifiedSupportReady
    {
        if profile.support_posture() == SupportPostureProfile::InternalOnly {
            return Err(
                FoundationalDiagnosticMaterializationDenial::InternalSupportCannotClaimCertifiedSupport,
            );
        }
        if profile.certification_posture()
            != crate::profiles::CertificationPostureProfile::ProductionCertified
        {
            return Err(
                FoundationalDiagnosticMaterializationDenial::CertifiedSupportRequiresProductionCertifiedProfile,
            );
        }
        if matches!(
            profile.retention_delivery(),
            RetentionDeliveryProfile::Ephemeral
        ) || !matches!(
            input.availability.availability(),
            FoundationalDiagnosticAvailability::RetainedHot
                | FoundationalDiagnosticAvailability::DeferredCold
                | FoundationalDiagnosticAvailability::Reconstructable
        ) {
            return Err(
                FoundationalDiagnosticMaterializationDenial::DurableSupportRequiresVisibleEvidence,
            );
        }
        if !has_visible_rows_for_profile(
            &input.required_rows,
            &input.standard_rows,
            &input.forensic_rows,
            profile,
        ) {
            return Err(
                FoundationalDiagnosticMaterializationDenial::DurableSupportRequiresVisibleRowsAtChosenRichness,
            );
        }
    }

    Ok(FoundationalDiagnosticMaterializationPlan {
        subject: input.subject,
        outcome_kind: input.outcome_kind,
        required_rows: input.required_rows,
        standard_rows: input.standard_rows,
        forensic_rows: input.forensic_rows,
        profile,
        delivery_class,
        availability: input.availability,
        partiality: input.partiality,
        counter_snapshot: input.counter_snapshot,
        assembly_debts: input.assembly_debts,
        support_claim_strength: Some(input.support_claim_strength),
        _kind: PhantomData,
    })
}

pub fn plan_diagnostic_explanation_bundle(
    input: FoundationalDiagnosticExplanationInput,
    profile: FoundationalProfileSet,
    delivery_class: FoundationalDiagnosticDeliveryClass,
) -> Result<
    FoundationalDiagnosticMaterializationPlan<FoundationalExplanationBundleArtifactKind>,
    FoundationalDiagnosticMaterializationDenial,
> {
    validate_common(
        input.availability,
        &input.partiality,
        &input.assembly_debts,
        delivery_class,
        FoundationalDiagnosticAvailability::from(input.availability),
        crate::diagnostics::FoundationalDiagnosticArtifactKind::ExplanationBundle,
    )?;

    Ok(FoundationalDiagnosticMaterializationPlan {
        subject: input.subject,
        outcome_kind: input.outcome_kind,
        required_rows: input.required_rows,
        standard_rows: input.standard_rows,
        forensic_rows: input.forensic_rows,
        profile,
        delivery_class,
        availability: input.availability,
        partiality: input.partiality,
        counter_snapshot: input.counter_snapshot,
        assembly_debts: input.assembly_debts,
        support_claim_strength: None,
        _kind: PhantomData,
    })
}

fn validate_common(
    availability: FoundationalDiagnosticSurfaceAvailability,
    partiality: &FoundationalDiagnosticPartiality,
    assembly_debts: &[FoundationalDiagnosticAssemblyDebt],
    delivery_class: FoundationalDiagnosticDeliveryClass,
    raw_availability: FoundationalDiagnosticAvailability,
    kind: crate::diagnostics::FoundationalDiagnosticArtifactKind,
) -> Result<(), FoundationalDiagnosticMaterializationDenial> {
    match availability.availability() {
        FoundationalDiagnosticAvailability::RetainedHot
        | FoundationalDiagnosticAvailability::DeferredCold
        | FoundationalDiagnosticAvailability::Reconstructable => {
            if availability.absence_cause().is_some() {
                return Err(
                    FoundationalDiagnosticMaterializationDenial::UnavailableAvailabilityRequiresCause,
                );
            }
        }
        FoundationalDiagnosticAvailability::Redacted => {
            if availability.absence_cause()
                != Some(crate::diagnostics::FoundationalDiagnosticAbsenceCause::Redacted)
            {
                return Err(
                    FoundationalDiagnosticMaterializationDenial::RedactedAvailabilityMustUseRedactedCause,
                );
            }
        }
        FoundationalDiagnosticAvailability::Unavailable => {
            if availability.absence_cause().is_none() {
                return Err(
                    FoundationalDiagnosticMaterializationDenial::UnavailableAvailabilityRequiresCause,
                );
            }
        }
    }

    evaluate_diagnostic_materialization_legality(kind, delivery_class, raw_availability).map_err(
        |_| FoundationalDiagnosticMaterializationDenial::DurableSupportRequiresVisibleEvidence,
    )?;

    match partiality {
        FoundationalDiagnosticPartiality::Complete => {}
        FoundationalDiagnosticPartiality::PartialWithNamedGaps(gaps) => {
            if gaps.is_empty() {
                return Err(
                    FoundationalDiagnosticMaterializationDenial::PartialityRequiresNamedGaps,
                );
            }
        }
    }

    for debt in assembly_debts {
        match debt.class() {
            FoundationalDiagnosticAssemblyDebtClass::RowScanFallback if debt.count() == 0 => {
                return Err(FoundationalDiagnosticMaterializationDenial::RowScanFallbackMustRemainExplicitDebt);
            }
            FoundationalDiagnosticAssemblyDebtClass::WholeViewFallback if debt.count() == 0 => {
                return Err(FoundationalDiagnosticMaterializationDenial::WholeViewFallbackMustRemainExplicitDebt);
            }
            FoundationalDiagnosticAssemblyDebtClass::RepeatedRediscovery if debt.count() == 0 => {
                return Err(FoundationalDiagnosticMaterializationDenial::RepeatedRediscoveryMustRemainExplicitDebt);
            }
            _ => {}
        }
    }

    Ok(())
}

fn has_visible_rows_for_profile(
    required_rows: &[FoundationalDiagnosticRow],
    standard_rows: &[FoundationalDiagnosticRow],
    forensic_rows: &[FoundationalDiagnosticRow],
    profile: FoundationalProfileSet,
) -> bool {
    !required_rows.is_empty()
        || (profile.diagnostic_richness() != DiagnosticRichnessProfile::OperationalMinimal
            && !standard_rows.is_empty())
        || (profile.diagnostic_richness() == DiagnosticRichnessProfile::Forensic
            && !forensic_rows.is_empty())
}

impl From<FoundationalDiagnosticSurfaceAvailability> for FoundationalDiagnosticAvailability {
    fn from(value: FoundationalDiagnosticSurfaceAvailability) -> Self {
        value.availability()
    }
}
