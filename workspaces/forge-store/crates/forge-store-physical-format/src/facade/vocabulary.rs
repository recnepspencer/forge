#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PlatformPhysicalFacadeOperation {
    AppendPhysicalRecord,
    ReadPhysicalRecord,
    ScanPhysicalManifest,
    LocatePhysicalReference,
    PublishPhysicalRoot,
    ReopenPhysicalStore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformPhysicalFacadeVocabulary {
    operation: PlatformPhysicalFacadeOperation,
}

impl PlatformPhysicalFacadeVocabulary {
    pub const fn new(operation: PlatformPhysicalFacadeOperation) -> Self {
        Self { operation }
    }

    pub const fn operation(&self) -> PlatformPhysicalFacadeOperation {
        self.operation
    }
}
