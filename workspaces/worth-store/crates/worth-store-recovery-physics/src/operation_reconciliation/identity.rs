#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RecoveryOperationIdentity {
    store: [u8; 16],
    runtime: u64,
    lifecycle: u64,
    operation: u64,
    idempotency: [u8; 32],
}

impl RecoveryOperationIdentity {
    pub fn new(
        store: [u8; 16],
        runtime: u64,
        lifecycle: u64,
        operation: u64,
        idempotency: [u8; 32],
    ) -> Option<Self> {
        if store == [0; 16] || runtime == 0 || lifecycle == 0 || operation == 0 {
            return None;
        }
        Some(Self {
            store,
            runtime,
            lifecycle,
            operation,
            idempotency,
        })
    }

    pub const fn store(self) -> [u8; 16] {
        self.store
    }
    pub const fn runtime(self) -> u64 {
        self.runtime
    }
    pub const fn lifecycle(self) -> u64 {
        self.lifecycle
    }
    pub const fn operation(self) -> u64 {
        self.operation
    }
    pub const fn idempotency(self) -> [u8; 32] {
        self.idempotency
    }
}
