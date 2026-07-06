#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalVocabularyError {
    ZeroPhysicalIdentifier,
    MissingVocabularyTerm,
    WrongAuthorityScope,
    InvalidFreeSpaceReuseAllocationClass,
}
