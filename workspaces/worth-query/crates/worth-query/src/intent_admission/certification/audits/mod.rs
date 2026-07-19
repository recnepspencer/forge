mod topology;

use crate::identity::hash_parts;

use crate::intent_admission::{
    worth_query_intent_admission_coverage_inventory, worth_query_intent_admission_family_inventory,
    WorthQueryIntentDecisionTraceEnvelopeKind, WorthQueryIntentDecisionTraceStage,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryIntentAdmissionPublicBoundaryAudit {
    public_surface_digest: String,
    target_dx_digest: String,
}

impl WorthQueryIntentAdmissionPublicBoundaryAudit {
    pub(crate) fn new() -> Self {
        let public_surface_digest = hash_parts(
            &worth_query_intent_admission_family_inventory()
                .rows()
                .iter()
                .flat_map(|row| {
                    [
                        format!("raw:{}", row.raw_authoring_constructor().label()),
                        format!("common:{}", row.common_path_front_door().label()),
                        format!("advanced:{}", row.advanced_path_front_door().label()),
                    ]
                })
                .collect::<Vec<_>>(),
        );
        let target_dx_digest = hash_parts(
            &worth_query_intent_admission_coverage_inventory()
                .rows()
                .iter()
                .flat_map(|row| {
                    [
                        format!("entrypoint:{}", row.entrypoint().as_str()),
                        format!("common:{}", row.common_path_front_door().label()),
                        format!("advanced:{}", row.advanced_path_front_door().label()),
                    ]
                })
                .collect::<Vec<_>>(),
        );
        Self {
            public_surface_digest,
            target_dx_digest,
        }
    }

    pub fn public_surface_digest(&self) -> &str {
        &self.public_surface_digest
    }

    pub fn target_dx_digest(&self) -> &str {
        &self.target_dx_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryIntentAdmissionProofShapeAudit {
    admitted_phase_progression: Vec<WorthQueryIntentDecisionTraceStage>,
    advisory_phase_progression: Vec<WorthQueryIntentDecisionTraceStage>,
    violation_phase_progression: Vec<WorthQueryIntentDecisionTraceStage>,
    decision_phase_progression_digest: String,
    decision_proof_shape_digest: String,
}

pub use topology::{
    WorthQueryIntentAdmissionTopologyAudit, WorthQueryIntentAdmissionTopologyAuditRow,
    WorthQueryIntentAdmissionTopologyDomain,
};

impl WorthQueryIntentAdmissionProofShapeAudit {
    pub(crate) fn new() -> Self {
        let admitted_phase_progression = vec![
            WorthQueryIntentDecisionTraceStage::RawIntent,
            WorthQueryIntentDecisionTraceStage::Eligibility,
            WorthQueryIntentDecisionTraceStage::AdmittedDecision,
            WorthQueryIntentDecisionTraceStage::ExecutionHandoff,
            WorthQueryIntentDecisionTraceStage::ExecutionOutcome,
        ];
        let advisory_phase_progression = vec![
            WorthQueryIntentDecisionTraceStage::RawIntent,
            WorthQueryIntentDecisionTraceStage::Eligibility,
            WorthQueryIntentDecisionTraceStage::AdvisoryStop,
        ];
        let violation_phase_progression = vec![
            WorthQueryIntentDecisionTraceStage::RawIntent,
            WorthQueryIntentDecisionTraceStage::Eligibility,
            WorthQueryIntentDecisionTraceStage::ViolationStop,
        ];
        let decision_phase_progression_digest = hash_parts(&[
            stage_digest("admitted", &admitted_phase_progression),
            stage_digest("advisory", &advisory_phase_progression),
            stage_digest("violation", &violation_phase_progression),
        ]);
        let decision_proof_shape_digest = hash_parts(&[
            format!(
                "admitted:{}",
                WorthQueryIntentDecisionTraceEnvelopeKind::AdmittedExecution.as_str()
            ),
            format!(
                "advisory:{}",
                WorthQueryIntentDecisionTraceEnvelopeKind::AdvisoryStop.as_str()
            ),
            format!(
                "violation:{}",
                WorthQueryIntentDecisionTraceEnvelopeKind::ViolationStop.as_str()
            ),
            decision_phase_progression_digest.clone(),
        ]);
        Self {
            admitted_phase_progression,
            advisory_phase_progression,
            violation_phase_progression,
            decision_phase_progression_digest,
            decision_proof_shape_digest,
        }
    }

    pub fn admitted_phase_progression(&self) -> &[WorthQueryIntentDecisionTraceStage] {
        &self.admitted_phase_progression
    }

    pub fn advisory_phase_progression(&self) -> &[WorthQueryIntentDecisionTraceStage] {
        &self.advisory_phase_progression
    }

    pub fn violation_phase_progression(&self) -> &[WorthQueryIntentDecisionTraceStage] {
        &self.violation_phase_progression
    }

    pub fn decision_phase_progression_digest(&self) -> &str {
        &self.decision_phase_progression_digest
    }

    pub fn decision_proof_shape_digest(&self) -> &str {
        &self.decision_proof_shape_digest
    }
}

pub(crate) fn worth_query_intent_admission_proof_shape_audit(
) -> WorthQueryIntentAdmissionProofShapeAudit {
    WorthQueryIntentAdmissionProofShapeAudit::new()
}

fn stage_digest(label: &str, stages: &[WorthQueryIntentDecisionTraceStage]) -> String {
    hash_parts(
        &std::iter::once(format!("lane:{label}"))
            .chain(stages.iter().map(|stage| stage.as_str().to_string()))
            .collect::<Vec<_>>(),
    )
}
