use std::collections::{BTreeMap, BTreeSet};

use crate::durability::data::{DurabilityError, PartitionCheckpointImage};
use crate::identity::data::PartitionId;
use crate::runtime::RelationalRuntime;
use crate::storage::overlay::PartitionState;

pub(super) type RestoredPartitions = BTreeMap<PartitionId, PartitionState>;

pub(super) fn restore_unique_partition_images(
    restored: &RelationalRuntime,
    images: &[PartitionCheckpointImage],
    aspect_contracts: &crate::durability::checkpoints::aspect_state_images::CheckpointAspectContractCatalog,
    owner: &str,
) -> Result<RestoredPartitions, DurabilityError> {
    restore_unique_partition_images_with_schema(
        images,
        &restored.schema_contract_runtime.aspect_contract_plans,
        aspect_contracts,
        owner,
    )
}

pub(super) fn restore_unique_partition_images_with_schema(
    images: &[PartitionCheckpointImage],
    plans: &crate::schema::data::AspectContractPlanCatalog,
    aspect_contracts: &crate::durability::checkpoints::aspect_state_images::CheckpointAspectContractCatalog,
    owner: &str,
) -> Result<RestoredPartitions, DurabilityError> {
    reject_duplicate_partition_images(images, owner)?;
    images
        .iter()
        .cloned()
        .map(|image| {
            let partition_id = image.partition_id;
            crate::durability::checkpoints::images::partition_from_image(
                image,
                plans,
                aspect_contracts,
            )
            .map(|partition| (partition_id, partition))
        })
        .collect()
}

pub(super) fn reject_duplicate_partition_images(
    images: &[PartitionCheckpointImage],
    owner: &str,
) -> Result<(), DurabilityError> {
    let mut seen = BTreeSet::new();
    for image in images {
        if !seen.insert(image.partition_id) {
            return Err(DurabilityError::new(
                crate::durability::data::RecoveryFailureClass::CorruptCheckpoint,
                format!(
                    "{owner} contains duplicate partition image `{}`",
                    image.partition_id.as_u32()
                ),
            ));
        }
    }
    Ok(())
}
