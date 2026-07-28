use std::fmt;

use worth_store_buffer_pool::{OperationAllocationGrant, PhysicalOperationAllocationScope};
use worth_store_physical_format::store_namespace::StableStoreIdentity;

use crate::physical_runtime::{
    record_serving::residency::frame_ports::RecordFramePorts, LifecycleGeneration, RuntimeIdentity,
};

pub(super) struct StoreScopedAllocation<'runtime> {
    grant: OperationAllocationGrant,
    _runtime_owner: &'runtime RecordFramePorts,
    runtime: RuntimeIdentity,
    generation: LifecycleGeneration,
}

impl fmt::Debug for StoreScopedAllocation<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("StoreScopedAllocation")
            .field("grant", &self.grant)
            .field("runtime", &self.runtime)
            .field("generation", &self.generation)
            .finish_non_exhaustive()
    }
}

macro_rules! exact_scope_allocation {
    ($name:ident, $scope:ident) => {
        #[derive(Debug)]
        pub struct $name<'runtime> {
            allocation: StoreScopedAllocation<'runtime>,
        }

        impl<'runtime> $name<'runtime> {
            pub(super) fn bind(
                grant: OperationAllocationGrant,
                runtime_owner: &'runtime RecordFramePorts,
                runtime: RuntimeIdentity,
                generation: LifecycleGeneration,
            ) -> Self {
                debug_assert_eq!(grant.scope(), PhysicalOperationAllocationScope::$scope);
                Self {
                    allocation: StoreScopedAllocation {
                        grant,
                        _runtime_owner: runtime_owner,
                        runtime,
                        generation,
                    },
                }
            }

            pub fn store_identity(&self) -> StableStoreIdentity {
                self.allocation.grant.observation().store()
            }

            pub const fn store_generation(&self) -> LifecycleGeneration {
                self.allocation.generation
            }

            pub const fn runtime_identity(&self) -> RuntimeIdentity {
                self.allocation.runtime
            }

            pub const fn bytes(&self) -> u64 {
                self.allocation.grant.bytes()
            }
        }
    };
}

exact_scope_allocation!(RecoveryPhysicalAllocation, Recovery);
exact_scope_allocation!(ScrubPhysicalAllocation, Scrub);
exact_scope_allocation!(MaintenancePhysicalAllocation, Maintenance);
exact_scope_allocation!(VerificationPhysicalAllocation, Verification);
exact_scope_allocation!(BlobPhysicalAllocation, Blob);
