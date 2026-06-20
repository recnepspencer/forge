use crate::identity::hash_parts;

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryGraphReadStreamingCursorDenialKind {
    CursorReplayDenied,
    CursorBasisMismatch,
    CursorPlanMismatch,
    CursorSequenceSkipped,
    ForgedCursorDenied,
}

#[allow(dead_code)]
impl ForgeQueryGraphReadStreamingCursorDenialKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CursorReplayDenied => "cursor_replay_denied",
            Self::CursorBasisMismatch => "cursor_basis_mismatch",
            Self::CursorPlanMismatch => "cursor_plan_mismatch",
            Self::CursorSequenceSkipped => "cursor_sequence_skipped",
            Self::ForgedCursorDenied => "forged_cursor_denied",
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadStreamingCursorDenial {
    digest: String,
    kind: ForgeQueryGraphReadStreamingCursorDenialKind,
    streaming_plan_digest: String,
    cursor_digest: Option<String>,
    expected_identity_digest: Option<String>,
    observed_identity_digest: Option<String>,
}

#[allow(dead_code)]
impl ForgeQueryGraphReadStreamingCursorDenial {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn kind(&self) -> &ForgeQueryGraphReadStreamingCursorDenialKind {
        &self.kind
    }

    pub fn streaming_plan_digest(&self) -> &str {
        &self.streaming_plan_digest
    }

    pub fn cursor_digest(&self) -> Option<&str> {
        self.cursor_digest.as_deref()
    }

    pub fn expected_identity_digest(&self) -> Option<&str> {
        self.expected_identity_digest.as_deref()
    }

    pub fn observed_identity_digest(&self) -> Option<&str> {
        self.observed_identity_digest.as_deref()
    }

    pub(crate) fn new(
        kind: ForgeQueryGraphReadStreamingCursorDenialKind,
        streaming_plan_digest: impl Into<String>,
        cursor_digest: Option<String>,
        expected_identity_digest: Option<String>,
        observed_identity_digest: Option<String>,
    ) -> Self {
        let streaming_plan_digest = streaming_plan_digest.into();
        let digest = hash_parts(&[
            "forge_query_graph_read_streaming_cursor_denial_v1".to_string(),
            format!("kind:{}", kind.as_str()),
            format!("streaming_plan:{streaming_plan_digest}"),
            format!("cursor:{}", cursor_digest.as_deref().unwrap_or("none")),
            format!(
                "expected:{}",
                expected_identity_digest.as_deref().unwrap_or("none")
            ),
            format!(
                "observed:{}",
                observed_identity_digest.as_deref().unwrap_or("none")
            ),
        ]);
        Self {
            digest,
            kind,
            streaming_plan_digest,
            cursor_digest,
            expected_identity_digest,
            observed_identity_digest,
        }
    }
}
