use super::{
    adjudicate_layout_hazards, observe_layout_proof_outcomes, LayoutCourtroomTranscriptIdentity,
    LayoutEvidenceBundle, LayoutHazardAdjudicationDenial, LayoutHazardInventory,
    LayoutProofOutcomeObservation,
};
use crate::courtroom::layout::formal_observation::{
    observe_layout_formal_model, LayoutFormalObservation, LayoutFormalObservationDenial,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutCourtroomDenial {
    Hazard(LayoutHazardAdjudicationDenial),
    ProofOutcomeEvidenceIncomplete,
    FormalObservation(LayoutFormalObservationDenial),
    TranscriptIdentityMismatch,
}

#[derive(Debug, PartialEq, Eq)]
pub struct LayoutCourtroomReport {
    evidence: LayoutEvidenceBundle,
    hazards: LayoutHazardInventory,
    proof_outcomes: LayoutProofOutcomeObservation,
    formal_observation: LayoutFormalObservation,
}

pub fn adjudicate_layout_courtroom(
    evidence: LayoutEvidenceBundle,
) -> Result<LayoutCourtroomReport, LayoutCourtroomDenial> {
    let hazards = adjudicate_layout_hazards(&evidence).map_err(LayoutCourtroomDenial::Hazard)?;
    let proof_outcomes = observe_layout_proof_outcomes(&evidence)
        .ok_or(LayoutCourtroomDenial::ProofOutcomeEvidenceIncomplete)?;
    let formal_observation =
        observe_layout_formal_model(&evidence).map_err(LayoutCourtroomDenial::FormalObservation)?;
    let transcript = evidence.transcript_identity();
    if hazards.transcript_identity() != transcript
        || proof_outcomes.transcript_identity() != transcript
        || formal_observation.transcript_identity() != transcript
    {
        return Err(LayoutCourtroomDenial::TranscriptIdentityMismatch);
    }
    Ok(LayoutCourtroomReport {
        evidence,
        hazards,
        proof_outcomes,
        formal_observation,
    })
}

impl LayoutCourtroomReport {
    pub const fn transcript_identity(&self) -> LayoutCourtroomTranscriptIdentity {
        self.evidence.transcript_identity()
    }

    pub const fn evidence(&self) -> &LayoutEvidenceBundle {
        &self.evidence
    }

    pub const fn hazards(&self) -> &LayoutHazardInventory {
        &self.hazards
    }

    pub const fn proof_outcomes(&self) -> &LayoutProofOutcomeObservation {
        &self.proof_outcomes
    }

    pub const fn formal_observation(&self) -> &LayoutFormalObservation {
        &self.formal_observation
    }
}
