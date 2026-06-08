use crate::domain_artifacts::core_artifact::require_non_empty;
use crate::domain_artifacts::HadwigerArtifactShapeError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScreeningSolverTranscript {
    solver_name: String,
    solver_version: String,
    transcript_digest: String,
    candidate_status: String,
}

impl ScreeningSolverTranscript {
    pub fn new(
        solver_name: impl Into<String>,
        solver_version: impl Into<String>,
        transcript_digest: impl Into<String>,
        candidate_status: impl Into<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        Ok(Self {
            solver_name: require_non_empty(solver_name, "solver_name")?,
            solver_version: require_non_empty(solver_version, "solver_version")?,
            transcript_digest: require_non_empty(transcript_digest, "solver_transcript_digest")?,
            candidate_status: require_non_empty(candidate_status, "candidate_status")?,
        })
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}:{}",
            self.solver_name, self.solver_version, self.transcript_digest, self.candidate_status
        )
    }
}
