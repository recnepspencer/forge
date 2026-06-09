use crate::discovery_loop::{ExperimentSuppressionProof, ResearchEvidenceCorpus};

use super::equivalence_errors::{require_equivalence_non_empty, TilingEquivalenceError};
use super::equivalence_proofs::TilingCandidateEquivalenceProof;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TilingCandidateSuppressionRequest {
    suppression_id: String,
    corpus: ResearchEvidenceCorpus,
    equivalence: TilingCandidateEquivalenceProof,
    suppression_proof: ExperimentSuppressionProof,
}

impl TilingCandidateSuppressionRequest {
    pub fn from_existing_suppression_proof(
        suppression_id: impl Into<String>,
        corpus: &ResearchEvidenceCorpus,
        equivalence: &TilingCandidateEquivalenceProof,
        suppression_proof: ExperimentSuppressionProof,
    ) -> Result<Self, TilingEquivalenceError> {
        Ok(Self {
            suppression_id: require_equivalence_non_empty(suppression_id, "suppression_id")?,
            corpus: corpus.clone(),
            equivalence: equivalence.clone(),
            suppression_proof,
        })
    }

    pub(crate) fn suppression_id(&self) -> &str {
        &self.suppression_id
    }

    pub(crate) fn corpus(&self) -> &ResearchEvidenceCorpus {
        &self.corpus
    }

    pub(crate) fn equivalence(&self) -> &TilingCandidateEquivalenceProof {
        &self.equivalence
    }

    pub(crate) fn suppression_proof(&self) -> &ExperimentSuppressionProof {
        &self.suppression_proof
    }
}
