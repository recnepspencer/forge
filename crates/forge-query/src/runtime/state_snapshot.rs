use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::ordinary_outcome::ForgeQueryOrdinaryRuntimePosture;

use super::{
    ForgeQueryAuthorityLane, ForgeQueryRuntimeAsyncResultState, ForgeQueryRuntimeRemaskPosture,
};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryRuntimeStateKind {
    Ready,
    Remasked,
    Pending,
    Stale,
    Failed,
    Cancelled,
    Retried,
    Revalidating,
    Superseded,
    Denied,
    Unsupported,
}

impl ForgeQueryRuntimeStateKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Remasked => "remasked",
            Self::Pending => "pending",
            Self::Stale => "stale",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::Retried => "retried",
            Self::Revalidating => "revalidating",
            Self::Superseded => "superseded",
            Self::Denied => "denied",
            Self::Unsupported => "unsupported",
        }
    }
}

impl std::fmt::Display for ForgeQueryRuntimeStateKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimeStateSnapshot {
    kind: ForgeQueryRuntimeStateKind,
    basis_digest: String,
    result_shape_digest: String,
    authority_lane: ForgeQueryAuthorityLane,
    explanation: String,
    ordinary_runtime_posture: Option<ForgeQueryOrdinaryRuntimePosture>,
    async_result_state: Option<ForgeQueryRuntimeAsyncResultState>,
    remask_posture: Option<ForgeQueryRuntimeRemaskPosture>,
    state_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryRuntimeStateSnapshot {
    pub fn ready(
        basis_digest: impl Into<String>,
        result_shape_digest: impl Into<String>,
        authority_lane: ForgeQueryAuthorityLane,
        explanation: impl Into<String>,
    ) -> Self {
        Self::new(
            ForgeQueryRuntimeStateKind::Ready,
            basis_digest,
            result_shape_digest,
            authority_lane,
            explanation,
        )
    }

    pub fn deferred(
        kind: ForgeQueryRuntimeStateKind,
        basis_digest: impl Into<String>,
        result_shape_digest: impl Into<String>,
        authority_lane: ForgeQueryAuthorityLane,
        explanation: impl Into<String>,
    ) -> Self {
        assert!(
            kind != ForgeQueryRuntimeStateKind::Ready,
            "ready state should use ForgeQueryRuntimeStateSnapshot::ready"
        );
        Self::new(
            kind,
            basis_digest,
            result_shape_digest,
            authority_lane,
            explanation,
        )
    }

    pub fn with_async_result_state(
        mut self,
        async_result_state: ForgeQueryRuntimeAsyncResultState,
    ) -> Self {
        if self.remask_posture.is_none() {
            self.kind = async_result_state.kind().state_kind();
        }
        self.async_result_state = Some(async_result_state);
        self.state_digest = compute_state_digest(
            self.kind,
            &self.basis_digest,
            &self.result_shape_digest,
            self.authority_lane,
            &self.explanation,
            self.ordinary_runtime_posture.as_ref(),
            self.async_result_state.as_ref(),
            self.remask_posture.as_ref(),
        );
        self
    }

    pub fn with_ordinary_runtime_posture(
        mut self,
        ordinary_runtime_posture: ForgeQueryOrdinaryRuntimePosture,
    ) -> Self {
        self.ordinary_runtime_posture = Some(ordinary_runtime_posture);
        self.state_digest = compute_state_digest(
            self.kind,
            &self.basis_digest,
            &self.result_shape_digest,
            self.authority_lane,
            &self.explanation,
            self.ordinary_runtime_posture.as_ref(),
            self.async_result_state.as_ref(),
            self.remask_posture.as_ref(),
        );
        self
    }

    pub fn with_remask_posture(mut self, remask_posture: ForgeQueryRuntimeRemaskPosture) -> Self {
        self.kind = remask_posture.disposition_kind().state_kind();
        self.remask_posture = Some(remask_posture);
        self.state_digest = compute_state_digest(
            self.kind,
            &self.basis_digest,
            &self.result_shape_digest,
            self.authority_lane,
            &self.explanation,
            self.ordinary_runtime_posture.as_ref(),
            self.async_result_state.as_ref(),
            self.remask_posture.as_ref(),
        );
        self
    }

    fn new(
        kind: ForgeQueryRuntimeStateKind,
        basis_digest: impl Into<String>,
        result_shape_digest: impl Into<String>,
        authority_lane: ForgeQueryAuthorityLane,
        explanation: impl Into<String>,
    ) -> Self {
        let basis_digest = basis_digest.into();
        let result_shape_digest = result_shape_digest.into();
        let explanation = explanation.into();
        let ordinary_runtime_posture = None;
        let async_result_state = None;
        let remask_posture = None;
        let state_digest = compute_state_digest(
            kind,
            &basis_digest,
            &result_shape_digest,
            authority_lane,
            &explanation,
            ordinary_runtime_posture.as_ref(),
            async_result_state.as_ref(),
            remask_posture.as_ref(),
        );
        Self {
            kind,
            basis_digest,
            result_shape_digest,
            authority_lane,
            explanation,
            ordinary_runtime_posture,
            async_result_state,
            remask_posture,
            state_digest,
        }
    }

    pub fn kind(&self) -> ForgeQueryRuntimeStateKind {
        self.kind
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn result_shape_digest(&self) -> &str {
        &self.result_shape_digest
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }

    pub fn explanation(&self) -> &str {
        &self.explanation
    }

    pub fn ordinary_runtime_posture(&self) -> Option<&ForgeQueryOrdinaryRuntimePosture> {
        self.ordinary_runtime_posture.as_ref()
    }

    pub fn async_result_state(&self) -> Option<&ForgeQueryRuntimeAsyncResultState> {
        self.async_result_state.as_ref()
    }

    pub fn remask_posture(&self) -> Option<&ForgeQueryRuntimeRemaskPosture> {
        self.remask_posture.as_ref()
    }

    pub fn state_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.state_digest
    }
}

fn compute_state_digest(
    kind: ForgeQueryRuntimeStateKind,
    basis_digest: &str,
    result_shape_digest: &str,
    authority_lane: ForgeQueryAuthorityLane,
    explanation: &str,
    ordinary_runtime_posture: Option<&ForgeQueryOrdinaryRuntimePosture>,
    async_result_state: Option<&ForgeQueryRuntimeAsyncResultState>,
    remask_posture: Option<&ForgeQueryRuntimeRemaskPosture>,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::RuntimeStateSnapshot)
        .field_shape(ForgeQueryEvidenceTag::new("kind"), kind.as_str())
        .field_identity(ForgeQueryEvidenceTag::new("basis_digest"), basis_digest)
        .field_identity(
            ForgeQueryEvidenceTag::new("result_shape_digest"),
            result_shape_digest,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("authority_lane"),
            authority_lane.as_str(),
        )
        .field_value(ForgeQueryEvidenceTag::new("explanation"), explanation)
        .optional_identity(
            ForgeQueryEvidenceTag::new("ordinary_runtime_posture"),
            ordinary_runtime_posture.map(ForgeQueryOrdinaryRuntimePosture::posture_digest),
        )
        .optional_identity(
            ForgeQueryEvidenceTag::new("async_result_state"),
            async_result_state.map(ForgeQueryRuntimeAsyncResultState::result_state_digest),
        )
        .optional_identity(
            ForgeQueryEvidenceTag::new("remask_posture"),
            remask_posture.map(ForgeQueryRuntimeRemaskPosture::remask_digest),
        )
        .seal()
}
