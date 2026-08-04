use worth_store_physical_backend::{
    ArtifactRangeWriteDurability, ArtifactRangeWriteDurabilityRequirement,
};

use super::super::super::PhysicalWorkDurabilityRequirement;

pub(in crate::physical_runtime::work) fn durability_satisfies(
    declared: PhysicalWorkDurabilityRequirement,
    observed: ArtifactRangeWriteDurability,
) -> bool {
    match declared {
        PhysicalWorkDurabilityRequirement::ReadOnly
        | PhysicalWorkDurabilityRequirement::WalAppend
        | PhysicalWorkDurabilityRequirement::WalDurabilityBarrier
        | PhysicalWorkDurabilityRequirement::CheckpointCapture
        | PhysicalWorkDurabilityRequirement::WalReclamation
        | PhysicalWorkDurabilityRequirement::RootPublication => false,
        PhysicalWorkDurabilityRequirement::ArtifactRangeWrite(requirement) => match requirement {
            ArtifactRangeWriteDurabilityRequirement::BufferedWrite => true,
            ArtifactRangeWriteDurabilityRequirement::FileDataSynchronization => {
                observed == ArtifactRangeWriteDurability::FileDataSynchronized
            }
        },
    }
}
