#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalFormatField {
    Magic,
    EnvelopeSchema,
    FormatVersion,
    EncodedLength,
    Checksum,
    StoreIdentity,
    ArtifactIdentity,
    PhysicalGeneration,
    SelectorRole,
    RootGeneration,
    LinkedSelector,
    ChildReference,
    Payload,
}
