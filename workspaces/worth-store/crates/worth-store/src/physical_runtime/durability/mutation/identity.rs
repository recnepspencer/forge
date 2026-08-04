use crate::physical_runtime::{PhysicalOperationIdentity, PhysicalWorkIdentity, RuntimeIdentity};
use worth_store_physical_format::store_namespace::StableStoreIdentity;

/// Store-owned identity for one complete physical mutation obligation.
///
/// The wrapped C.5.1 identity is observable correlation. It does not authorize
/// work submission, media effects, settlement, or acknowledgment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicalMutationIdentity(PhysicalWorkIdentity);

impl PhysicalMutationIdentity {
    pub(in crate::physical_runtime) const fn from_reserved_operation(
        identity: PhysicalWorkIdentity,
    ) -> Self {
        Self(identity)
    }

    pub const fn store_identity(self) -> StableStoreIdentity {
        self.0.store()
    }

    pub const fn runtime_identity(self) -> RuntimeIdentity {
        self.0.runtime()
    }

    pub const fn operation_identity(self) -> PhysicalOperationIdentity {
        self.0.operation()
    }

    pub const fn lifecycle_generation(self) -> u64 {
        self.0.generation().lifecycle().get()
    }
}
