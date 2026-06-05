use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, require_non_empty, HadwigerArtifactAuthorityOwner,
    HadwigerArtifactCore, HadwigerArtifactKind, HadwigerArtifactReference,
    HadwigerArtifactShapeError, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;

use super::patterns::MotifObservation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InvariantCandidateAuthority {
    HadwigerLocalCandidate,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvariantHypothesis {
    core: HadwigerArtifactCore,
    hypothesis_statement: String,
    motif_reference: HadwigerArtifactReference,
}

impl InvariantHypothesis {
    pub(crate) fn from_motif(motif: &MotifObservation) -> Result<Self, HadwigerArtifactShapeError> {
        let hypothesis_statement = format!(
            "failure motif {} may define a suppressible research invariant",
            motif.pattern_signature().stable_token()
        );
        let motif_reference = motif.reference();
        let core = artifact_core(
            HadwigerArtifactKind::InvariantHypothesis,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "invariant_hypothesis".to_string(),
            },
            vec![motif_reference.clone()],
            vec![
                HadwigerArtifactPayloadEntry::text(
                    "hypothesis_statement",
                    hypothesis_statement.clone(),
                ),
                HadwigerArtifactPayloadEntry::text(
                    "motif_reference",
                    motif_reference.stable_token(),
                ),
            ],
        )?;
        Ok(Self {
            core,
            hypothesis_statement,
            motif_reference,
        })
    }

    pub fn hypothesis_statement(&self) -> &str {
        &self.hypothesis_statement
    }

    pub fn motif_reference(&self) -> &HadwigerArtifactReference {
        &self.motif_reference
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(InvariantHypothesis, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvariantCandidate {
    core: HadwigerArtifactCore,
    authority: InvariantCandidateAuthority,
}

impl InvariantCandidate {
    pub(crate) fn from_hypothesis(
        hypothesis: &InvariantHypothesis,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let core = artifact_core(
            HadwigerArtifactKind::InvariantCandidate,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "invariant_candidate".to_string(),
            },
            vec![hypothesis.reference()],
            vec![
                HadwigerArtifactPayloadEntry::text("authority", "hadwiger_local_candidate"),
                HadwigerArtifactPayloadEntry::text(
                    "hypothesis_statement",
                    hypothesis.hypothesis_statement(),
                ),
            ],
        )?;
        Ok(Self {
            core,
            authority: InvariantCandidateAuthority::HadwigerLocalCandidate,
        })
    }

    pub fn authority(&self) -> InvariantCandidateAuthority {
        self.authority
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(InvariantCandidate, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CounterexampleObligation {
    core: HadwigerArtifactCore,
    obligation: String,
}

impl CounterexampleObligation {
    pub fn new(
        candidate: &InvariantCandidate,
        obligation: impl Into<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let obligation = require_non_empty(obligation, "counterexample_obligation")?;
        let core = artifact_core(
            HadwigerArtifactKind::CounterexampleObligation,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "counterexample_obligation".to_string(),
            },
            vec![candidate.reference()],
            vec![HadwigerArtifactPayloadEntry::text(
                "obligation",
                obligation.clone(),
            )],
        )?;
        Ok(Self { core, obligation })
    }

    pub fn obligation(&self) -> &str {
        &self.obligation
    }
}

impl_hadwiger_artifact!(CounterexampleObligation, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReactivationCondition {
    core: HadwigerArtifactCore,
    qualifying_evidence: HadwigerArtifactReference,
}

impl ReactivationCondition {
    pub fn from_new_evidence(
        retired_reference: HadwigerArtifactReference,
        qualifying_evidence: HadwigerArtifactReference,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let core = artifact_core(
            HadwigerArtifactKind::ReactivationCondition,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "reactivation_condition".to_string(),
            },
            vec![retired_reference, qualifying_evidence.clone()],
            vec![HadwigerArtifactPayloadEntry::text(
                "qualifying_evidence",
                qualifying_evidence.stable_token(),
            )],
        )?;
        Ok(Self {
            core,
            qualifying_evidence,
        })
    }

    pub fn qualifying_evidence(&self) -> &HadwigerArtifactReference {
        &self.qualifying_evidence
    }
}

impl_hadwiger_artifact!(ReactivationCondition, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetiredHypothesisRecord {
    core: HadwigerArtifactCore,
    hypothesis: InvariantHypothesis,
}

impl RetiredHypothesisRecord {
    pub fn retire(
        hypothesis: InvariantHypothesis,
        reason: impl Into<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let reason = require_non_empty(reason, "retirement_reason")?;
        let core = artifact_core(
            HadwigerArtifactKind::RetiredHypothesisRecord,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "retired_hypothesis_record".to_string(),
            },
            vec![hypothesis.reference()],
            vec![HadwigerArtifactPayloadEntry::text("reason", reason)],
        )?;
        Ok(Self { core, hypothesis })
    }

    pub fn hypothesis(&self) -> &InvariantHypothesis {
        &self.hypothesis
    }
}

impl_hadwiger_artifact!(RetiredHypothesisRecord, core);
