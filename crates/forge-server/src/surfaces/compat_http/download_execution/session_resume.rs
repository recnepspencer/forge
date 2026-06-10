use super::ForgeServerBinaryEgressSession;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerBinarySessionResume {
    previous_session_digest: String,
    expected_next_start: usize,
    content_type: String,
    full_representation_digest: String,
    validator_entity_tag: String,
    workspace_digest: String,
    branch_digest: String,
    authorization_digest: String,
    canonical_digest: String,
}

impl ForgeServerBinarySessionResume {
    pub(crate) fn from_session(session: &ForgeServerBinaryEgressSession) -> Self {
        let previous_session_digest = session.canonical_digest().to_string();
        let expected_next_start = session.selected_end_exclusive();
        let content_type = session.download_request().content_type().to_string();
        let full_representation_digest = session.download_request().payload_digest().to_string();
        let validator_entity_tag = session.validator().entity_tag().to_string();
        let workspace_digest = session
            .read()
            .direct_context()
            .workspace_digest()
            .to_string();
        let branch_digest = session.read().direct_context().branch_digest().to_string();
        let authorization_digest = session
            .download_request()
            .authorization()
            .canonical_digest()
            .to_string();
        let canonical_digest = format!(
            "compat-http-binary-session-resume-v1|session={previous_session_digest}|next_start={expected_next_start}|content_type={content_type}|full={full_representation_digest}|validator={validator_entity_tag}|workspace={workspace_digest}|branch={branch_digest}|authorization={authorization_digest}"
        );
        Self {
            previous_session_digest,
            expected_next_start,
            content_type,
            full_representation_digest,
            validator_entity_tag,
            workspace_digest,
            branch_digest,
            authorization_digest,
            canonical_digest,
        }
    }

    pub fn previous_session_digest(&self) -> &str {
        &self.previous_session_digest
    }

    pub fn expected_next_start(&self) -> usize {
        self.expected_next_start
    }

    pub fn content_type(&self) -> &str {
        &self.content_type
    }

    pub fn full_representation_digest(&self) -> &str {
        &self.full_representation_digest
    }

    pub fn validator_entity_tag(&self) -> &str {
        &self.validator_entity_tag
    }

    pub fn workspace_digest(&self) -> &str {
        &self.workspace_digest
    }

    pub fn branch_digest(&self) -> &str {
        &self.branch_digest
    }

    pub fn authorization_digest(&self) -> &str {
        &self.authorization_digest
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
