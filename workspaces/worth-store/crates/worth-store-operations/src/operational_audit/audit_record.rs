use crate::{OperationalOperationId, OperationalTransitionId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OperationLocalSequence(u64);

impl OperationLocalSequence {
    pub(crate) const fn new(value: u64) -> Option<Self> {
        if value == 0 { None } else { Some(Self(value)) }
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuditCausalParent([u8; 32]);

impl AuditCausalParent {
    pub const fn record_identity(self) -> [u8; 32] {
        self.0
    }

    pub(crate) const fn from_record(record: &OperationalAuditRecord) -> Self {
        Self(record.record_identity)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationalAuditTransitionKind {
    WorkflowOpened,
    SourceLeasePersisted,
    MaterializationOpened,
    MaterializationRecorded,
    IndependentVerificationRecorded,
    Abandoned,
    AuthorizationConsumed,
    OwnerExecutionOpened,
    OwnerEffectStarted,
    OwnerReceiptPersisted,
    DispositionRecorded,
    StagingCompleted,
    PublicationPrepared,
    PublicationPending,
    PublicationDisposition,
    FenceReleased,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationalAuditRecord {
    pub(crate) operation_id: OperationalOperationId,
    pub(crate) transition_id: OperationalTransitionId,
    pub(crate) sequence: OperationLocalSequence,
    pub(crate) causal_parent: Option<AuditCausalParent>,
    pub(crate) transition_kind: OperationalAuditTransitionKind,
    pub(crate) source_artifact_identity: [u8; 32],
    pub(crate) record_identity: [u8; 32],
}

impl OperationalAuditRecord {
    pub const fn operation_id(&self) -> &OperationalOperationId {
        &self.operation_id
    }

    pub const fn transition_id(&self) -> &OperationalTransitionId {
        &self.transition_id
    }

    pub const fn sequence(&self) -> OperationLocalSequence {
        self.sequence
    }

    pub const fn causal_parent(&self) -> Option<AuditCausalParent> {
        self.causal_parent
    }

    pub const fn transition_kind(&self) -> OperationalAuditTransitionKind {
        self.transition_kind
    }

    pub const fn source_artifact_identity(&self) -> [u8; 32] {
        self.source_artifact_identity
    }

    pub const fn record_identity(&self) -> [u8; 32] {
        self.record_identity
    }
}
