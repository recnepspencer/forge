use super::{
    OperationalCounterReceipt, OperationalSafeNextAction, OperationalSessionIdentity,
    OperationalSessionKind, OperationalSessionRecoveryHandle,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationalProgressPosture {
    Executed,
    Interrupted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperationalProgressEvent {
    session: OperationalSessionIdentity,
    kind: OperationalSessionKind,
    posture: OperationalProgressPosture,
    durable_transitions: u64,
    source_bytes_read: u64,
    output_bytes_written: u64,
    safe_next_action: Option<OperationalSafeNextAction>,
}

impl OperationalProgressEvent {
    pub const fn from_counters(receipt: OperationalCounterReceipt) -> Self {
        Self {
            session: receipt.session(),
            kind: receipt.kind(),
            posture: OperationalProgressPosture::Executed,
            durable_transitions: receipt.durable_protocol_transitions(),
            source_bytes_read: receipt.source_bytes_read(),
            output_bytes_written: receipt.output_bytes_written(),
            safe_next_action: None,
        }
    }

    pub const fn from_recovery(handle: OperationalSessionRecoveryHandle) -> Self {
        Self {
            session: handle.session(),
            kind: handle.kind(),
            posture: OperationalProgressPosture::Interrupted,
            durable_transitions: handle.durable_transition_count(),
            source_bytes_read: 0,
            output_bytes_written: 0,
            safe_next_action: Some(handle.next_action()),
        }
    }

    pub const fn session(self) -> OperationalSessionIdentity {
        self.session
    }
    pub const fn kind(self) -> OperationalSessionKind {
        self.kind
    }
    pub const fn posture(self) -> OperationalProgressPosture {
        self.posture
    }
    pub const fn durable_transitions(self) -> u64 {
        self.durable_transitions
    }
    pub const fn source_bytes_read(self) -> u64 {
        self.source_bytes_read
    }
    pub const fn output_bytes_written(self) -> u64 {
        self.output_bytes_written
    }
    pub const fn safe_next_action(self) -> Option<OperationalSafeNextAction> {
        self.safe_next_action
    }
}
