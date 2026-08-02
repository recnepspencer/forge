use super::{PhysicalMutationIdempotencyMaterial, RecordAppendBatch};

/// One explicit logical mutation admitted into a certification WAL group.
pub struct CertificationDurableMutationInput {
    material: PhysicalMutationIdempotencyMaterial,
    batch: RecordAppendBatch,
}

impl CertificationDurableMutationInput {
    pub fn new(material: PhysicalMutationIdempotencyMaterial, batch: RecordAppendBatch) -> Self {
        Self { material, batch }
    }

    pub(in crate::physical_runtime) fn into_parts(
        self,
    ) -> (PhysicalMutationIdempotencyMaterial, RecordAppendBatch) {
        (self.material, self.batch)
    }
}
