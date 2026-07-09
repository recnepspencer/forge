#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobCorruptionDetectionSource {
    VerifiedRead,
    Scrub,
    ColdFetch,
    ImportReadmission,
    CapsuleMaterialization,
}
