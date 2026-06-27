use serde::Serialize;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ReplayUndoSemanticGraphStageIndexIdentity {
    digest: String,
}

impl ReplayUndoSemanticGraphStageIndexIdentity {
    fn new(digest: impl Into<String>) -> Self {
        let digest = digest.into();
        assert!(
            !digest.trim().is_empty(),
            "replay/undo stage index identity requires a non-empty digest"
        );
        Self { digest }
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn digest_part(&self) -> String {
        format!("stage-index:{}", self.digest)
    }
}

pub fn admit_replay_undo_stage_index_identity(
    stage_index_identity: &str,
) -> ReplayUndoSemanticGraphStageIndexIdentity {
    ReplayUndoSemanticGraphStageIndexIdentity::new(stage_index_identity)
}
