use worth_store_buffer_pool::{OperationAllocationGrant, PhysicalOperationAllocationScope};
use worth_store_physical_format::store_namespace::StableStoreIdentity;

use crate::physical_runtime::LifecycleGeneration;

#[derive(Debug)]
pub(super) struct StoreScopedAllocation {
    grant: OperationAllocationGrant,
    generation: LifecycleGeneration,
}

macro_rules! exact_scope_allocation {
    ($name:ident, $scope:ident) => {
        #[derive(Debug)]
        pub struct $name {
            allocation: StoreScopedAllocation,
        }

        impl $name {
            pub(super) fn bind(
                grant: OperationAllocationGrant,
                generation: LifecycleGeneration,
            ) -> Self {
                debug_assert_eq!(grant.scope(), PhysicalOperationAllocationScope::$scope);
                Self {
                    allocation: StoreScopedAllocation { grant, generation },
                }
            }

            pub fn store_identity(&self) -> StableStoreIdentity {
                self.allocation.grant.observation().store()
            }

            pub const fn store_generation(&self) -> LifecycleGeneration {
                self.allocation.generation
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
