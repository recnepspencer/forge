mod topology;

use crate::identity::hash_parts;

use super::manifests::{
    compile_fail_boundary_digest, crate_doc_example_target_digest,
    forge_query_intent_admission_compile_fail_targets,
    forge_query_intent_admission_crate_doc_example_targets,
    forge_query_intent_admission_golden_transcripts, golden_transcript_digest,
    ForgeQueryIntentAdmissionCompileFailTarget, ForgeQueryIntentAdmissionCrateDocExampleTarget,
    ForgeQueryIntentAdmissionGoldenTranscript,
};
use crate::intent_admission::{
    forge_query_intent_admission_coverage_inventory, forge_query_intent_admission_family_inventory,
    ForgeQueryIntentDecisionTraceEnvelopeKind, ForgeQueryIntentDecisionTraceStage,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionPublicBoundaryAudit {
    compile_fail_targets: &'static [ForgeQueryIntentAdmissionCompileFailTarget],
    golden_transcripts: &'static [ForgeQueryIntentAdmissionGoldenTranscript],
    crate_doc_example_targets: &'static [ForgeQueryIntentAdmissionCrateDocExampleTarget],
    compile_fail_boundary_digest: String,
    negative_dx_boundary_digest: String,
    golden_transcript_digest: String,
    crate_doc_example_target_digest: String,
    public_surface_digest: String,
    target_dx_digest: String,
}

impl ForgeQueryIntentAdmissionPublicBoundaryAudit {
    pub(crate) fn new() -> Self {
        let compile_fail_targets = forge_query_intent_admission_compile_fail_targets();
        let golden_transcripts = forge_query_intent_admission_golden_transcripts();
        let crate_doc_example_targets = forge_query_intent_admission_crate_doc_example_targets();
        let compile_fail_boundary_digest = compile_fail_boundary_digest();
        let golden_transcript_digest = golden_transcript_digest();
        let crate_doc_example_target_digest = crate_doc_example_target_digest();
        let public_surface_digest = hash_parts(
            &forge_query_intent_admission_family_inventory()
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
            &forge_query_intent_admission_coverage_inventory()
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
        let negative_dx_boundary_digest = hash_parts(&[
            compile_fail_boundary_digest.clone(),
            "runtime-floor-covered-execution-requires-typed-handoff".to_string(),
        ]);
        Self {
            compile_fail_targets,
            golden_transcripts,
            crate_doc_example_targets,
            compile_fail_boundary_digest,
            negative_dx_boundary_digest,
            golden_transcript_digest,
            crate_doc_example_target_digest,
            public_surface_digest,
            target_dx_digest,
        }
    }

    pub fn compile_fail_targets(&self) -> &[ForgeQueryIntentAdmissionCompileFailTarget] {
        self.compile_fail_targets
    }

    pub fn golden_transcripts(&self) -> &[ForgeQueryIntentAdmissionGoldenTranscript] {
        self.golden_transcripts
    }

    pub fn crate_doc_example_targets(&self) -> &[ForgeQueryIntentAdmissionCrateDocExampleTarget] {
        self.crate_doc_example_targets
    }

    pub fn compile_fail_boundary_digest(&self) -> &str {
        &self.compile_fail_boundary_digest
    }

    pub fn negative_dx_boundary_digest(&self) -> &str {
        &self.negative_dx_boundary_digest
    }

    pub fn golden_transcript_digest(&self) -> &str {
        &self.golden_transcript_digest
    }

    pub fn crate_doc_example_target_digest(&self) -> &str {
        &self.crate_doc_example_target_digest
    }

    pub fn public_surface_digest(&self) -> &str {
        &self.public_surface_digest
    }

    pub fn target_dx_digest(&self) -> &str {
        &self.target_dx_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionProofShapeAudit {
    admitted_phase_progression: Vec<ForgeQueryIntentDecisionTraceStage>,
    advisory_phase_progression: Vec<ForgeQueryIntentDecisionTraceStage>,
    violation_phase_progression: Vec<ForgeQueryIntentDecisionTraceStage>,
    decision_phase_progression_digest: String,
    decision_proof_shape_digest: String,
}

pub use topology::{
    ForgeQueryIntentAdmissionTopologyAudit, ForgeQueryIntentAdmissionTopologyAuditRow,
    ForgeQueryIntentAdmissionTopologyDomain,
};

impl ForgeQueryIntentAdmissionProofShapeAudit {
    pub(crate) fn new() -> Self {
        let admitted_phase_progression = vec![
            ForgeQueryIntentDecisionTraceStage::RawIntent,
            ForgeQueryIntentDecisionTraceStage::Eligibility,
            ForgeQueryIntentDecisionTraceStage::AdmittedDecision,
            ForgeQueryIntentDecisionTraceStage::ExecutionHandoff,
            ForgeQueryIntentDecisionTraceStage::ExecutionOutcome,
        ];
        let advisory_phase_progression = vec![
            ForgeQueryIntentDecisionTraceStage::RawIntent,
            ForgeQueryIntentDecisionTraceStage::Eligibility,
            ForgeQueryIntentDecisionTraceStage::AdvisoryStop,
        ];
        let violation_phase_progression = vec![
            ForgeQueryIntentDecisionTraceStage::RawIntent,
            ForgeQueryIntentDecisionTraceStage::Eligibility,
            ForgeQueryIntentDecisionTraceStage::ViolationStop,
        ];
        let decision_phase_progression_digest = hash_parts(&[
            stage_digest("admitted", &admitted_phase_progression),
            stage_digest("advisory", &advisory_phase_progression),
            stage_digest("violation", &violation_phase_progression),
        ]);
        let decision_proof_shape_digest = hash_parts(&[
            format!(
                "admitted:{}",
                ForgeQueryIntentDecisionTraceEnvelopeKind::AdmittedExecution.as_str()
            ),
            format!(
                "advisory:{}",
                ForgeQueryIntentDecisionTraceEnvelopeKind::AdvisoryStop.as_str()
            ),
            format!(
                "violation:{}",
                ForgeQueryIntentDecisionTraceEnvelopeKind::ViolationStop.as_str()
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

    pub fn admitted_phase_progression(&self) -> &[ForgeQueryIntentDecisionTraceStage] {
        &self.admitted_phase_progression
    }

    pub fn advisory_phase_progression(&self) -> &[ForgeQueryIntentDecisionTraceStage] {
        &self.advisory_phase_progression
    }

    pub fn violation_phase_progression(&self) -> &[ForgeQueryIntentDecisionTraceStage] {
        &self.violation_phase_progression
    }

    pub fn decision_phase_progression_digest(&self) -> &str {
        &self.decision_phase_progression_digest
    }

    pub fn decision_proof_shape_digest(&self) -> &str {
        &self.decision_proof_shape_digest
    }
}

pub(crate) fn forge_query_intent_admission_proof_shape_audit(
) -> ForgeQueryIntentAdmissionProofShapeAudit {
    ForgeQueryIntentAdmissionProofShapeAudit::new()
}

fn stage_digest(label: &str, stages: &[ForgeQueryIntentDecisionTraceStage]) -> String {
    hash_parts(
        &std::iter::once(format!("lane:{label}"))
            .chain(stages.iter().map(|stage| stage.as_str().to_string()))
            .collect::<Vec<_>>(),
    )
}
