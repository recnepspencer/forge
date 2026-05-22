use forge_foundational::facade::{
    admit_requested_foundational_profile, materialize_admitted_foundational_profile,
    materialize_diagnostic_explanation_bundle, materialize_diagnostic_support_report,
    request_foundational_profile_set, FoundationalBoundaryEvidenceProvenanceArtifact,
    FoundationalBoundaryEvidenceProvenanceConstructionDenial,
    FoundationalDiagnosticCounterSnapshot, FoundationalDiagnosticDeliveryClass,
    FoundationalDiagnosticExplanationBundle, FoundationalDiagnosticExplanationInput,
    FoundationalDiagnosticMaterializationDenial, FoundationalDiagnosticOutcomeKind,
    FoundationalDiagnosticPartiality, FoundationalDiagnosticRow,
    FoundationalDiagnosticSupportClaimStrength, FoundationalDiagnosticSupportInput,
    FoundationalDiagnosticSupportReport, FoundationalProfileNarrowingRecord,
    FoundationalProfileProgressionDenial, FoundationalProfileSet,
    MaterializedFoundationalProfileArtifact,
};
use forge_proof::TransitionOutcome;

use crate::spatial_intent::refs::{SpatialDirectionWitnessRef, SpatialPointWitnessRef};
use crate::spatial_intent::resolution::{
    ResolvedSpatialDirectionWitness, ResolvedSpatialPointWitness, SpatialWitnessFailureClass,
};

use super::materialization_support::{
    availability_from_outcome, base_rows, outcome_kind, partiality_from_outcome,
    provenance_from_outcome,
};
use super::materialization_vocab::{requested_locator, witness_subject, WitnessKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpatialWitnessMaterializationProfilePlan {
    requested: FoundationalProfileSet,
    admitted: FoundationalProfileSet,
    materialized: FoundationalProfileSet,
    requested_to_admitted_narrowing: Option<FoundationalProfileNarrowingRecord>,
    admitted_to_materialized_narrowing: Option<FoundationalProfileNarrowingRecord>,
}

impl SpatialWitnessMaterializationProfilePlan {
    pub const fn new(
        requested: FoundationalProfileSet,
        admitted: FoundationalProfileSet,
        materialized: FoundationalProfileSet,
        requested_to_admitted_narrowing: Option<FoundationalProfileNarrowingRecord>,
        admitted_to_materialized_narrowing: Option<FoundationalProfileNarrowingRecord>,
    ) -> Self {
        Self {
            requested,
            admitted,
            materialized,
            requested_to_admitted_narrowing,
            admitted_to_materialized_narrowing,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpatialWitnessMaterializationDenial {
    DiagnosticPrimitive(
        forge_foundational::facade::FoundationalDiagnosticPrimitiveConstructionDenial,
    ),
    ProfileProgression(FoundationalProfileProgressionDenial),
    DiagnosticMaterialization(FoundationalDiagnosticMaterializationDenial),
    ProvenanceConstruction(FoundationalBoundaryEvidenceProvenanceConstructionDenial),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpatialPointWitnessSupportMaterialization {
    support_report: FoundationalDiagnosticSupportReport,
    explanation_bundle: FoundationalDiagnosticExplanationBundle,
    provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
    profile: MaterializedFoundationalProfileArtifact,
}

impl SpatialPointWitnessSupportMaterialization {
    pub fn support_report(&self) -> &FoundationalDiagnosticSupportReport {
        &self.support_report
    }

    pub fn explanation_bundle(&self) -> &FoundationalDiagnosticExplanationBundle {
        &self.explanation_bundle
    }

    pub fn provenance(&self) -> &FoundationalBoundaryEvidenceProvenanceArtifact {
        &self.provenance
    }

    pub fn profile(&self) -> &MaterializedFoundationalProfileArtifact {
        &self.profile
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpatialDirectionWitnessSupportMaterialization {
    support_report: FoundationalDiagnosticSupportReport,
    explanation_bundle: FoundationalDiagnosticExplanationBundle,
    provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
    profile: MaterializedFoundationalProfileArtifact,
}

impl SpatialDirectionWitnessSupportMaterialization {
    pub fn support_report(&self) -> &FoundationalDiagnosticSupportReport {
        &self.support_report
    }

    pub fn explanation_bundle(&self) -> &FoundationalDiagnosticExplanationBundle {
        &self.explanation_bundle
    }

    pub fn provenance(&self) -> &FoundationalBoundaryEvidenceProvenanceArtifact {
        &self.provenance
    }

    pub fn profile(&self) -> &MaterializedFoundationalProfileArtifact {
        &self.profile
    }
}

pub fn materialize_spatial_point_witness_support_report(
    requested: SpatialPointWitnessRef,
    outcome: Result<ResolvedSpatialPointWitness, SpatialWitnessFailureClass>,
    profile_plan: SpatialWitnessMaterializationProfilePlan,
) -> Result<SpatialPointWitnessSupportMaterialization, SpatialWitnessMaterializationDenial> {
    let profile = materialize_profile(profile_plan)?;
    let subject = witness_subject(WitnessKind::Point);
    let rows = point_rows(&requested, &outcome)?;
    let support_report = materialize_diagnostic_support_report(
        support_input(
            subject.clone(),
            outcome_kind_from_point(&outcome),
            rows.clone(),
            partiality_from_outcome(WitnessKind::Point, &outcome),
            availability_from_outcome(&outcome),
        ),
        *profile.payload().materialized(),
        FoundationalDiagnosticDeliveryClass::MustBeHot,
    )
    .map_err(SpatialWitnessMaterializationDenial::DiagnosticMaterialization)?;
    let explanation_bundle = materialize_diagnostic_explanation_bundle(
        explanation_input(
            subject,
            outcome_kind_from_point(&outcome),
            rows,
            partiality_from_outcome(WitnessKind::Point, &outcome),
            availability_from_outcome(&outcome),
        ),
        *profile.payload().materialized(),
        FoundationalDiagnosticDeliveryClass::MustBeHot,
    )
    .map_err(SpatialWitnessMaterializationDenial::DiagnosticMaterialization)?;

    Ok(SpatialPointWitnessSupportMaterialization {
        support_report,
        explanation_bundle,
        provenance: point_provenance(&outcome)?,
        profile,
    })
}

pub fn materialize_spatial_direction_witness_support_report(
    requested: SpatialDirectionWitnessRef,
    outcome: Result<ResolvedSpatialDirectionWitness, SpatialWitnessFailureClass>,
    profile_plan: SpatialWitnessMaterializationProfilePlan,
) -> Result<SpatialDirectionWitnessSupportMaterialization, SpatialWitnessMaterializationDenial> {
    let profile = materialize_profile(profile_plan)?;
    let subject = witness_subject(WitnessKind::Direction);
    let rows = direction_rows(&requested, &outcome)?;
    let support_report = materialize_diagnostic_support_report(
        support_input(
            subject.clone(),
            outcome_kind_from_direction(&outcome),
            rows.clone(),
            partiality_from_outcome(WitnessKind::Direction, &outcome),
            availability_from_outcome(&outcome),
        ),
        *profile.payload().materialized(),
        FoundationalDiagnosticDeliveryClass::MustBeHot,
    )
    .map_err(SpatialWitnessMaterializationDenial::DiagnosticMaterialization)?;
    let explanation_bundle = materialize_diagnostic_explanation_bundle(
        explanation_input(
            subject,
            outcome_kind_from_direction(&outcome),
            rows,
            partiality_from_outcome(WitnessKind::Direction, &outcome),
            availability_from_outcome(&outcome),
        ),
        *profile.payload().materialized(),
        FoundationalDiagnosticDeliveryClass::MustBeHot,
    )
    .map_err(SpatialWitnessMaterializationDenial::DiagnosticMaterialization)?;

    Ok(SpatialDirectionWitnessSupportMaterialization {
        support_report,
        explanation_bundle,
        provenance: direction_provenance(&outcome)?,
        profile,
    })
}

fn materialize_profile(
    plan: SpatialWitnessMaterializationProfilePlan,
) -> Result<MaterializedFoundationalProfileArtifact, SpatialWitnessMaterializationDenial> {
    let requested = request_foundational_profile_set(plan.requested);
    let admitted = match admit_requested_foundational_profile(
        requested,
        plan.admitted,
        plan.requested_to_admitted_narrowing,
        forge_foundational::facade::foundational_profile_progression_authority(),
    ) {
        TransitionOutcome::Success(value) => value,
        TransitionOutcome::Denied(denial) => {
            return Err(SpatialWitnessMaterializationDenial::ProfileProgression(
                denial,
            ));
        }
        _ => unreachable!("profile admission here is denial-only"),
    };
    match materialize_admitted_foundational_profile(
        admitted,
        plan.materialized,
        plan.admitted_to_materialized_narrowing,
        forge_foundational::facade::foundational_profile_progression_authority(),
    ) {
        TransitionOutcome::Success(value) => Ok(value),
        TransitionOutcome::Denied(denial) => Err(
            SpatialWitnessMaterializationDenial::ProfileProgression(denial),
        ),
        _ => unreachable!("profile materialization here is denial-only"),
    }
}

fn point_rows(
    requested: &SpatialPointWitnessRef,
    outcome: &Result<ResolvedSpatialPointWitness, SpatialWitnessFailureClass>,
) -> Result<Vec<FoundationalDiagnosticRow>, SpatialWitnessMaterializationDenial> {
    base_rows(
        WitnessKind::Point,
        requested_locator(WitnessKind::Point),
        requested,
        outcome.as_ref().map(|value| value.resolution_class()),
        outcome.as_ref().err().copied(),
        outcome
            .as_ref()
            .ok()
            .and_then(|value| value.parameter_admission()),
    )
}

fn direction_rows(
    requested: &SpatialDirectionWitnessRef,
    outcome: &Result<ResolvedSpatialDirectionWitness, SpatialWitnessFailureClass>,
) -> Result<Vec<FoundationalDiagnosticRow>, SpatialWitnessMaterializationDenial> {
    base_rows(
        WitnessKind::Direction,
        requested_locator(WitnessKind::Direction),
        requested,
        outcome.as_ref().map(|value| value.resolution_class()),
        outcome.as_ref().err().copied(),
        outcome
            .as_ref()
            .ok()
            .and_then(|value| value.parameter_admission()),
    )
}

fn point_provenance(
    outcome: &Result<ResolvedSpatialPointWitness, SpatialWitnessFailureClass>,
) -> Result<FoundationalBoundaryEvidenceProvenanceArtifact, SpatialWitnessMaterializationDenial> {
    provenance_from_outcome(
        WitnessKind::Point,
        outcome.as_ref().map(|value| value.resolution_class()),
        outcome
            .as_ref()
            .ok()
            .and_then(|value| value.parameter_admission()),
    )
}

fn direction_provenance(
    outcome: &Result<ResolvedSpatialDirectionWitness, SpatialWitnessFailureClass>,
) -> Result<FoundationalBoundaryEvidenceProvenanceArtifact, SpatialWitnessMaterializationDenial> {
    provenance_from_outcome(
        WitnessKind::Direction,
        outcome.as_ref().map(|value| value.resolution_class()),
        outcome
            .as_ref()
            .ok()
            .and_then(|value| value.parameter_admission()),
    )
}

fn outcome_kind_from_point(
    outcome: &Result<ResolvedSpatialPointWitness, SpatialWitnessFailureClass>,
) -> FoundationalDiagnosticOutcomeKind {
    outcome_kind(
        outcome.as_ref().map(|value| value.resolution_class()),
        outcome.as_ref().err().copied(),
    )
}

fn outcome_kind_from_direction(
    outcome: &Result<ResolvedSpatialDirectionWitness, SpatialWitnessFailureClass>,
) -> FoundationalDiagnosticOutcomeKind {
    outcome_kind(
        outcome.as_ref().map(|value| value.resolution_class()),
        outcome.as_ref().err().copied(),
    )
}

fn support_input(
    subject: forge_foundational::facade::FoundationalDiagnosticSubject,
    outcome_kind: FoundationalDiagnosticOutcomeKind,
    rows: Vec<FoundationalDiagnosticRow>,
    partiality: FoundationalDiagnosticPartiality,
    availability: forge_foundational::facade::FoundationalDiagnosticSurfaceAvailability,
) -> FoundationalDiagnosticSupportInput {
    FoundationalDiagnosticSupportInput::new(
        subject,
        outcome_kind,
        rows,
        Vec::new(),
        Vec::new(),
        availability,
        FoundationalDiagnosticSupportClaimStrength::DescriptiveOnly,
        partiality,
        FoundationalDiagnosticCounterSnapshot::new(1, 0, 0, 0, 0, 0),
        Vec::new(),
    )
}

fn explanation_input(
    subject: forge_foundational::facade::FoundationalDiagnosticSubject,
    outcome_kind: FoundationalDiagnosticOutcomeKind,
    rows: Vec<FoundationalDiagnosticRow>,
    partiality: FoundationalDiagnosticPartiality,
    availability: forge_foundational::facade::FoundationalDiagnosticSurfaceAvailability,
) -> FoundationalDiagnosticExplanationInput {
    FoundationalDiagnosticExplanationInput::new(
        subject,
        outcome_kind,
        rows,
        Vec::new(),
        Vec::new(),
        availability,
        partiality,
        FoundationalDiagnosticCounterSnapshot::new(1, 0, 0, 0, 0, 0),
        Vec::new(),
    )
}

#[cfg(test)]
#[path = "materialization_tests.rs"]
mod materialization_tests;
