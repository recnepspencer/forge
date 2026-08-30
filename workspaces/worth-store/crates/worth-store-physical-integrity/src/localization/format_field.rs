#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalFormatField {
    Magic,
    EnvelopeSchema,
    FormatVersion,
    FormatDeclaration,
    EncodedLength,
    Checksum,
    StoreIdentity,
    ArtifactFamily,
    ArtifactIdentity,
    PhysicalGeneration,
    SelectorRole,
    RootGeneration,
    LinkedSelector,
    ChildReference,
    Reserved,
    Payload,
}
