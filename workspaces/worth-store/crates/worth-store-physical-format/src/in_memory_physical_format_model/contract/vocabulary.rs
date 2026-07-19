#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InMemoryPhysicalFormatModelOperation {
    AppendPhysicalRecord,
    ReadPhysicalRecord,
    ScanPhysicalManifest,
    LocatePhysicalReference,
    PublishPhysicalRoot,
    ReopenPhysicalStore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InMemoryPhysicalFormatModelVocabulary {
    operation: InMemoryPhysicalFormatModelOperation,
}

impl InMemoryPhysicalFormatModelVocabulary {
    pub const fn new(operation: InMemoryPhysicalFormatModelOperation) -> Self {
        Self { operation }
    }

    pub const fn operation(&self) -> InMemoryPhysicalFormatModelOperation {
        self.operation
    }
}
