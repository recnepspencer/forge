use crate::domain_artifacts::core_artifact::{require_non_empty, HadwigerArtifactShapeError};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentSourceRecord {
    agent_identity: String,
    session_identity: String,
    transcript_digest: String,
    tool_digest: String,
}

impl AgentSourceRecord {
    pub fn new(
        agent_identity: impl Into<String>,
        session_identity: impl Into<String>,
        transcript_digest: impl Into<String>,
        tool_digest: impl Into<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        Ok(Self {
            agent_identity: require_non_empty(agent_identity, "agent_identity")?,
            session_identity: require_non_empty(session_identity, "session_identity")?,
            transcript_digest: require_non_empty(transcript_digest, "transcript_digest")?,
            tool_digest: require_non_empty(tool_digest, "tool_digest")?,
        })
    }

    pub(crate) fn declaration_advisory(candidate_id: &str, detail: &str) -> Self {
        Self {
            agent_identity: "external-agent".to_string(),
            session_identity: "declaration-advisory".to_string(),
            transcript_digest: format!("transcript:declaration-advisory:{candidate_id}:{detail}"),
            tool_digest: "tool:unspecified".to_string(),
        }
    }

    pub fn agent_identity(&self) -> &str {
        &self.agent_identity
    }

    pub fn session_identity(&self) -> &str {
        &self.session_identity
    }

    pub fn transcript_digest(&self) -> &str {
        &self.transcript_digest
    }

    pub fn tool_digest(&self) -> &str {
        &self.tool_digest
    }

    pub fn source_digest(&self) -> String {
        self.stable_token()
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}:{}",
            self.agent_identity, self.session_identity, self.transcript_digest, self.tool_digest
        )
    }
}
