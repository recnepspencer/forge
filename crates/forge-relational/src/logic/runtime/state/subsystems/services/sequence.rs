use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Default)]
pub(super) struct RuntimeSequenceState {
    runtime_instance_id: u64,
    next_transaction_id: u64,
    next_savepoint_id: u64,
}

impl RuntimeSequenceState {
    pub(super) fn new() -> Self {
        static NEXT_RUNTIME_INSTANCE_ID: AtomicU64 = AtomicU64::new(1);
        Self {
            runtime_instance_id: NEXT_RUNTIME_INSTANCE_ID.fetch_add(1, Ordering::Relaxed),
            next_transaction_id: 1,
            next_savepoint_id: 1,
        }
    }

    pub(super) fn next_transaction_id(&mut self) -> crate::transactions::data::TransactionId {
        let transaction_id = crate::transactions::data::TransactionId(self.next_transaction_id);
        self.next_transaction_id += 1;
        transaction_id
    }

    pub(super) fn next_savepoint_id(&mut self) -> crate::transactions::data::SavepointId {
        let savepoint_id = crate::transactions::data::SavepointId(self.next_savepoint_id);
        self.next_savepoint_id += 1;
        savepoint_id
    }

    pub(super) fn runtime_instance_id(&self) -> u64 {
        self.runtime_instance_id
    }
}
