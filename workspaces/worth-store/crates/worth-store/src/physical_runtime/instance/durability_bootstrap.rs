use worth_store_physical_backend::QualifiedFilesystemMedia;

use crate::physical_runtime::durability::{
    rebuild_idempotency, reopen_binding_compaction, reopen_wal_inventory,
    PhysicalDurabilityRuntimeOwner, PhysicalWalBindingReopenCutoff, PhysicalWalRuntimeOwner,
    ReopenedPhysicalBindingCompaction, ReopenedPhysicalDurabilityRuntimeOwner,
};
use crate::physical_runtime::{
    PhysicalBindingCompactionReopenFailure, PhysicalIdempotencyReopenFailure,
    PhysicalSignalProfileIdentity, PhysicalWalOpenFailure, RuntimeIdentity,
};

pub(in crate::physical_runtime) struct PhysicalDurabilityReopenBasis {
    rebuilt: crate::physical_runtime::durability::RebuiltPhysicalMutationIdempotency,
    wal: PhysicalWalRuntimeOwner,
}

pub(in crate::physical_runtime) struct ReopenedPhysicalDurabilityOwners {
    pub(in crate::physical_runtime) durability: ReopenedPhysicalDurabilityRuntimeOwner,
    pub(in crate::physical_runtime) wal: PhysicalWalRuntimeOwner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalDurabilityStateReopenFailure {
    Checkpoint(PhysicalBindingCompactionReopenFailure),
    Wal(PhysicalWalOpenFailure),
    Idempotency(PhysicalIdempotencyReopenFailure),
}

pub(in crate::physical_runtime) fn reopen_durability_basis(
    media: &QualifiedFilesystemMedia,
    runtime: RuntimeIdentity,
    signal_profile: PhysicalSignalProfileIdentity,
    durability: &PhysicalDurabilityRuntimeOwner,
) -> Result<PhysicalDurabilityReopenBasis, PhysicalDurabilityStateReopenFailure> {
    let observation = durability.observation();
    let checkpoint = reopen_binding_compaction(media)
        .map_err(PhysicalDurabilityStateReopenFailure::Checkpoint)?;
    let cutoff = match checkpoint {
        ReopenedPhysicalBindingCompaction::GenerationZero => {
            PhysicalWalBindingReopenCutoff::GenerationZero
        }
        ReopenedPhysicalBindingCompaction::NamespaceDurable(ref reopened) => {
            PhysicalWalBindingReopenCutoff::after_checkpoint(reopened.wal_cutoff_lsn_exclusive())
        }
    };
    let mut inventory = reopen_wal_inventory(media, observation.wal_policy(), cutoff)
        .map_err(PhysicalDurabilityStateReopenFailure::Wal)?;
    let members = inventory.take_members();
    let rebuilt = rebuild_idempotency(
        media,
        runtime,
        observation.policy_identity(),
        observation.idempotency_policy(),
        &checkpoint,
        members,
    )
    .map_err(PhysicalDurabilityStateReopenFailure::Idempotency)?;
    let wal = PhysicalWalRuntimeOwner::from_reopened(
        media,
        runtime,
        signal_profile,
        observation.wal_policy(),
        inventory,
    );
    Ok(PhysicalDurabilityReopenBasis { rebuilt, wal })
}

impl PhysicalDurabilityReopenBasis {
    pub(in crate::physical_runtime) fn install(
        self,
        durability: PhysicalDurabilityRuntimeOwner,
    ) -> ReopenedPhysicalDurabilityOwners {
        ReopenedPhysicalDurabilityOwners {
            durability: durability.install_rebuilt_idempotency(self.rebuilt),
            wal: self.wal,
        }
    }
}
