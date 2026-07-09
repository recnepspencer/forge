use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::speculation::{BridgePreviewSessionIdentity, PreviewExecutionRecordIdentity};
use crate::subscription::BridgeSubscriptionCounters;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionPreviewBasisRejectionKind {
    PreviewExecutionRecordMismatch,
    PreviewDeclarationDigestMismatch,
}

impl BridgeSubscriptionPreviewBasisRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewExecutionRecordMismatch => "preview_execution_record_mismatch",
            Self::PreviewDeclarationDigestMismatch => "preview_declaration_digest_mismatch",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BridgeSubscriptionPreviewBasisRejectionContext {
    SessionExecutionRecordMismatch {
        preview_session_identity: BridgePreviewSessionIdentity,
        session_execution_record_identity: PreviewExecutionRecordIdentity,
        supplied_execution_record_identity: PreviewExecutionRecordIdentity,
    },
    ExecutionRecordSessionMismatch {
        preview_session_identity: BridgePreviewSessionIdentity,
        record_preview_session_identity: Arc<str>,
        supplied_execution_record_identity: PreviewExecutionRecordIdentity,
    },
    DeclarationDigestMismatch {
        preview_session_identity: BridgePreviewSessionIdentity,
        session_declaration_digest: Arc<str>,
        record_declaration_digest: Arc<str>,
    },
}

impl BridgeSubscriptionPreviewBasisRejectionContext {
    pub(super) fn session_execution_record_mismatch(
        preview_session_identity: BridgePreviewSessionIdentity,
        session_execution_record_identity: PreviewExecutionRecordIdentity,
        supplied_execution_record_identity: PreviewExecutionRecordIdentity,
    ) -> Self {
        Self::SessionExecutionRecordMismatch {
            preview_session_identity,
            session_execution_record_identity,
            supplied_execution_record_identity,
        }
    }

    pub(super) fn execution_record_session_mismatch(
        preview_session_identity: BridgePreviewSessionIdentity,
        record_preview_session_identity: impl Into<Arc<str>>,
        supplied_execution_record_identity: PreviewExecutionRecordIdentity,
    ) -> Self {
        Self::ExecutionRecordSessionMismatch {
            preview_session_identity,
            record_preview_session_identity: record_preview_session_identity.into(),
            supplied_execution_record_identity,
        }
    }

    pub(super) fn declaration_digest_mismatch(
        preview_session_identity: BridgePreviewSessionIdentity,
        session_declaration_digest: impl Into<Arc<str>>,
        record_declaration_digest: impl Into<Arc<str>>,
    ) -> Self {
        Self::DeclarationDigestMismatch {
            preview_session_identity,
            session_declaration_digest: session_declaration_digest.into(),
            record_declaration_digest: record_declaration_digest.into(),
        }
    }

    pub fn preview_session_identity(&self) -> &BridgePreviewSessionIdentity {
        match self {
            Self::SessionExecutionRecordMismatch {
                preview_session_identity,
                ..
            }
            | Self::ExecutionRecordSessionMismatch {
                preview_session_identity,
                ..
            }
            | Self::DeclarationDigestMismatch {
                preview_session_identity,
                ..
            } => preview_session_identity,
        }
    }

    pub fn supplied_execution_record_identity(&self) -> Option<&PreviewExecutionRecordIdentity> {
        match self {
            Self::SessionExecutionRecordMismatch {
                supplied_execution_record_identity,
                ..
            }
            | Self::ExecutionRecordSessionMismatch {
                supplied_execution_record_identity,
                ..
            } => Some(supplied_execution_record_identity),
            Self::DeclarationDigestMismatch { .. } => None,
        }
    }

    fn canonical_basis(&self) -> String {
        match self {
            Self::SessionExecutionRecordMismatch {
                preview_session_identity,
                session_execution_record_identity,
                supplied_execution_record_identity,
            } => format!(
                "session={}|session-execution={}|record={}",
                preview_session_identity.as_str(),
                session_execution_record_identity.as_str(),
                supplied_execution_record_identity.as_str()
            ),
            Self::ExecutionRecordSessionMismatch {
                preview_session_identity,
                record_preview_session_identity,
                supplied_execution_record_identity,
            } => format!(
                "session={}|record-session={}|record={}",
                preview_session_identity.as_str(),
                record_preview_session_identity,
                supplied_execution_record_identity.as_str()
            ),
            Self::DeclarationDigestMismatch {
                preview_session_identity,
                session_declaration_digest,
                record_declaration_digest,
            } => format!(
                "session={}|session-declaration={}|record-declaration={}",
                preview_session_identity.as_str(),
                session_declaration_digest,
                record_declaration_digest
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionPreviewBasisRejection {
    rejection_kind: BridgeSubscriptionPreviewBasisRejectionKind,
    rejection_context: BridgeSubscriptionPreviewBasisRejectionContext,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionPreviewBasisRejection {
    pub(super) fn new(
        rejection_kind: BridgeSubscriptionPreviewBasisRejectionKind,
        rejection_context: BridgeSubscriptionPreviewBasisRejectionContext,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-preview-basis-rejection|kind={}|context={}",
            rejection_kind.as_str(),
            rejection_context.canonical_basis()
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            rejection_context,
            counters: BridgeSubscriptionCounters::from_subscription_preview_basis_rejection(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-preview-basis-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeSubscriptionPreviewBasisRejectionKind {
        self.rejection_kind
    }

    pub fn rejection_context(&self) -> &BridgeSubscriptionPreviewBasisRejectionContext {
        &self.rejection_context
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
