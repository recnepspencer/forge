#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordPublicationStage {
    CandidateDataWrite,
    DataSynchronization,
    PayloadManifestSynchronization,
    ManifestSynchronization,
    CatalogCandidateSynchronization,
    CatalogReplacement,
    NamespaceSynchronization,
}
