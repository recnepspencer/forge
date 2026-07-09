pub type StoreContractResult<T> = Result<T, StoreContractError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreContractError {
    EmptyStableId,
    EmptyDigest,
    EmptyRequiredField,
    UnsupportedRoadmapClaim,
    MissingHandoffDigest,
    HandoffScopeMismatch,
    PhysicalAuthorityScopeMismatch,
}
