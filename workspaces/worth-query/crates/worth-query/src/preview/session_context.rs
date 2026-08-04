use crate::preview::evaluation::PreviewEvaluationClass;
#[cfg(test)]
use worth_runtime_bridge::facade::{
    bridge_identity_reporting_label, BridgePreviewPromotionRecord, BridgePreviewReplayBundle,
};
use worth_runtime_bridge::facade::{
    BridgePreviewExecutionRecord, BridgePreviewLifecycleStateKind, BridgePreviewSession,
    BridgePreviewSessionDeclarationIdentity, BridgePreviewSessionIdentity, PreviewActive,
    PreviewAdmitted, PreviewDeclared, PreviewDiscarded, PreviewExecutionRecordIdentity,
    PreviewPromoted,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewSessionQueryContext {
    pub(super) source: PreviewContextSource,
    pub(super) evaluation_class: PreviewEvaluationClass,
    pub(super) replay_bundle: Option<PreviewReplaySnapshot>,
    pub(super) promotion_record: Option<PreviewPromotionSnapshot>,
}

impl PreviewSessionQueryContext {
    pub fn active(
        session: &BridgePreviewSession<PreviewActive>,
        execution_record: &BridgePreviewExecutionRecord,
        evaluation_class: PreviewEvaluationClass,
    ) -> Self {
        Self {
            source: PreviewContextSource::from_active(session, Some(execution_record)),
            evaluation_class,
            replay_bundle: None,
            promotion_record: None,
        }
    }

    #[cfg(test)]
    pub(crate) fn active_without_execution_record(
        session: &BridgePreviewSession<PreviewActive>,
        evaluation_class: PreviewEvaluationClass,
    ) -> Self {
        Self {
            source: PreviewContextSource::from_active(session, None),
            evaluation_class,
            replay_bundle: None,
            promotion_record: None,
        }
    }

    pub fn declared(
        session: &BridgePreviewSession<PreviewDeclared>,
        evaluation_class: PreviewEvaluationClass,
    ) -> Self {
        Self {
            source: PreviewContextSource::from_declared(session),
            evaluation_class,
            replay_bundle: None,
            promotion_record: None,
        }
    }

    pub fn admitted(
        session: &BridgePreviewSession<PreviewAdmitted>,
        evaluation_class: PreviewEvaluationClass,
    ) -> Self {
        Self {
            source: PreviewContextSource::from_admitted(session),
            evaluation_class,
            replay_bundle: None,
            promotion_record: None,
        }
    }

    pub fn discarded(
        session: &BridgePreviewSession<PreviewDiscarded>,
        evaluation_class: PreviewEvaluationClass,
    ) -> Self {
        Self {
            source: PreviewContextSource::from_discarded(session),
            evaluation_class,
            replay_bundle: None,
            promotion_record: None,
        }
    }

    pub fn promoted(
        session: &BridgePreviewSession<PreviewPromoted>,
        evaluation_class: PreviewEvaluationClass,
    ) -> Self {
        Self {
            source: PreviewContextSource::from_promoted(session),
            evaluation_class,
            replay_bundle: None,
            promotion_record: None,
        }
    }

    #[cfg(test)]
    pub(crate) fn with_replay_bundle(mut self, replay_bundle: &BridgePreviewReplayBundle) -> Self {
        self.replay_bundle = Some(PreviewReplaySnapshot::from_bundle(replay_bundle));
        self
    }

    #[cfg(test)]
    pub(crate) fn with_promotion_record(
        mut self,
        promotion_record: &BridgePreviewPromotionRecord,
    ) -> Self {
        self.promotion_record = Some(PreviewPromotionSnapshot::from_record(promotion_record));
        self
    }

    pub fn evaluation_class(&self) -> &PreviewEvaluationClass {
        &self.evaluation_class
    }

    pub fn lifecycle_state_kind(&self) -> BridgePreviewLifecycleStateKind {
        self.source.lifecycle_state_kind()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum PreviewContextSource {
    Active(PreviewSessionSnapshot),
    Declared(PreviewSessionSnapshot),
    Admitted(PreviewSessionSnapshot),
    Discarded(PreviewSessionSnapshot),
    Promoted(PreviewSessionSnapshot),
}

impl PreviewContextSource {
    fn from_active(
        session: &BridgePreviewSession<PreviewActive>,
        execution_record: Option<&BridgePreviewExecutionRecord>,
    ) -> Self {
        Self::Active(PreviewSessionSnapshot::from_active(
            session,
            execution_record,
        ))
    }

    fn from_declared(session: &BridgePreviewSession<PreviewDeclared>) -> Self {
        Self::Declared(PreviewSessionSnapshot::from_declared(session))
    }

    fn from_admitted(session: &BridgePreviewSession<PreviewAdmitted>) -> Self {
        Self::Admitted(PreviewSessionSnapshot::from_admitted(session))
    }

    fn from_discarded(session: &BridgePreviewSession<PreviewDiscarded>) -> Self {
        Self::Discarded(PreviewSessionSnapshot::from_discarded(session))
    }

    fn from_promoted(session: &BridgePreviewSession<PreviewPromoted>) -> Self {
        Self::Promoted(PreviewSessionSnapshot::from_promoted(session))
    }

    pub(super) fn snapshot(&self) -> &PreviewSessionSnapshot {
        match self {
            Self::Active(snapshot)
            | Self::Declared(snapshot)
            | Self::Admitted(snapshot)
            | Self::Discarded(snapshot)
            | Self::Promoted(snapshot) => snapshot,
        }
    }

    pub(super) fn lifecycle_state_kind(&self) -> BridgePreviewLifecycleStateKind {
        self.snapshot().lifecycle_state_kind
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PreviewSessionSnapshot {
    pub(super) preview_session_identity: BridgePreviewSessionIdentity,
    pub(super) declaration_identity: BridgePreviewSessionDeclarationIdentity,
    pub(super) declaration_digest: String,
    pub(super) lifecycle_state_kind: BridgePreviewLifecycleStateKind,
    pub(super) execution_record_identity: Option<PreviewExecutionRecordIdentity>,
    pub(super) session_execution_record_identity: Option<PreviewExecutionRecordIdentity>,
    pub(super) execution_record_digest: Option<String>,
    pub(super) execution_record_preview_session_identity: Option<String>,
    pub(super) execution_record_declaration_digest: Option<String>,
}

impl PreviewSessionSnapshot {
    fn from_declared(session: &BridgePreviewSession<PreviewDeclared>) -> Self {
        Self {
            preview_session_identity: session.session_identity().clone(),
            declaration_identity: session
                .declaration()
                .declaration()
                .declaration_identity()
                .clone(),
            declaration_digest: session.declaration().digest().to_string(),
            lifecycle_state_kind: session.lifecycle_state_kind(),
            execution_record_identity: None,
            session_execution_record_identity: None,
            execution_record_digest: None,
            execution_record_preview_session_identity: None,
            execution_record_declaration_digest: None,
        }
    }

    fn from_admitted(session: &BridgePreviewSession<PreviewAdmitted>) -> Self {
        Self {
            preview_session_identity: session.session_identity().clone(),
            declaration_identity: session
                .declaration()
                .declaration()
                .declaration_identity()
                .clone(),
            declaration_digest: session.declaration().digest().to_string(),
            lifecycle_state_kind: session.lifecycle_state_kind(),
            execution_record_identity: None,
            session_execution_record_identity: None,
            execution_record_digest: None,
            execution_record_preview_session_identity: None,
            execution_record_declaration_digest: None,
        }
    }

    fn from_active(
        session: &BridgePreviewSession<PreviewActive>,
        execution_record: Option<&BridgePreviewExecutionRecord>,
    ) -> Self {
        Self {
            preview_session_identity: session.session_identity().clone(),
            declaration_identity: session
                .declaration()
                .declaration()
                .declaration_identity()
                .clone(),
            declaration_digest: session.declaration().digest().to_string(),
            lifecycle_state_kind: session.lifecycle_state_kind(),
            execution_record_identity: execution_record
                .map(|record| record.record_identity().clone())
                .or_else(|| session.execution_record_identity().cloned()),
            session_execution_record_identity: session.execution_record_identity().cloned(),
            execution_record_digest: execution_record.map(|record| record.digest().to_string()),
            execution_record_preview_session_identity: execution_record
                .map(|record| record.preview_session_identity().to_string()),
            execution_record_declaration_digest: execution_record
                .map(|record| record.preview_declaration_digest().to_string()),
        }
    }

    fn from_discarded(session: &BridgePreviewSession<PreviewDiscarded>) -> Self {
        Self {
            preview_session_identity: session.session_identity().clone(),
            declaration_identity: session
                .declaration()
                .declaration()
                .declaration_identity()
                .clone(),
            declaration_digest: session.declaration().digest().to_string(),
            lifecycle_state_kind: session.lifecycle_state_kind(),
            execution_record_identity: session.execution_record_identity().cloned(),
            session_execution_record_identity: session.execution_record_identity().cloned(),
            execution_record_digest: None,
            execution_record_preview_session_identity: None,
            execution_record_declaration_digest: None,
        }
    }

    fn from_promoted(session: &BridgePreviewSession<PreviewPromoted>) -> Self {
        Self {
            preview_session_identity: session.session_identity().clone(),
            declaration_identity: session
                .declaration()
                .declaration()
                .declaration_identity()
                .clone(),
            declaration_digest: session.declaration().digest().to_string(),
            lifecycle_state_kind: session.lifecycle_state_kind(),
            execution_record_identity: session.execution_record_identity().cloned(),
            session_execution_record_identity: session.execution_record_identity().cloned(),
            execution_record_digest: None,
            execution_record_preview_session_identity: None,
            execution_record_declaration_digest: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PreviewReplaySnapshot {
    pub(super) digest: String,
}

impl PreviewReplaySnapshot {
    #[cfg(test)]
    fn from_bundle(bundle: &BridgePreviewReplayBundle) -> Self {
        Self {
            digest: bundle.digest().to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PreviewPromotionSnapshot {
    pub(super) record_identity: String,
    pub(super) proof_digest: String,
}

impl PreviewPromotionSnapshot {
    #[cfg(test)]
    fn from_record(record: &BridgePreviewPromotionRecord) -> Self {
        Self {
            record_identity: bridge_identity_reporting_label(
                &record.record_identity().bridge_admission_evidence(),
            )
            .to_string(),
            proof_digest: record.promotion_proof_digest().to_string(),
        }
    }
}
