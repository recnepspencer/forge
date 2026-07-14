#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalStoreRuntimeOperation {
    AppendPhysicalRecord,
    ReadPhysicalRecord,
    ScanPhysicalManifest,
    LocatePhysicalReference,
    PublishPhysicalRoot,
    ReopenPhysicalStore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalStoreRuntimeVocabulary {
    operation: PhysicalStoreRuntimeOperation,
}

impl PhysicalStoreRuntimeVocabulary {
    pub const fn new(operation: PhysicalStoreRuntimeOperation) -> Self {
        Self { operation }
    }

    pub const fn operation(&self) -> PhysicalStoreRuntimeOperation {
        self.operation
    }
}
