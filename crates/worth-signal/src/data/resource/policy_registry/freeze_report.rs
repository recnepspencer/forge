use serde::{Deserialize, Serialize};

use super::identity::ResourcePolicyDigest;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourcePolicyRegistryFreezeReport {
    descriptor_count: usize,
    id_index_width: usize,
    kind_name_index_width: usize,
    registry_digest: ResourcePolicyDigest,
}

impl ResourcePolicyRegistryFreezeReport {
    pub(crate) fn new(
        descriptor_count: usize,
        id_index_width: usize,
        kind_name_index_width: usize,
        registry_digest: ResourcePolicyDigest,
    ) -> Self {
        Self {
            descriptor_count,
            id_index_width,
            kind_name_index_width,
            registry_digest,
        }
    }

    pub fn descriptor_count(&self) -> usize {
        self.descriptor_count
    }

    pub fn id_index_width(&self) -> usize {
        self.id_index_width
    }

    pub fn kind_name_index_width(&self) -> usize {
        self.kind_name_index_width
    }

    pub fn registry_digest(&self) -> &ResourcePolicyDigest {
        &self.registry_digest
    }
}
