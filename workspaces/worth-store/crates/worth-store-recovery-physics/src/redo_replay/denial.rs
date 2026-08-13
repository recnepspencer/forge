#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalRedoPlanningDenial {
    MalformedMember,
    WrongDomain,
    RecordCountLimit,
    TargetLimit,
    DistinctTargetLimit,
    InvalidRecordOrder,
    NonCanonicalTargetOrder,
    LsnRangeMismatch,
    InvalidTarget,
    InvalidRecoveryProjection,
    MissingPageObservation,
    GenerationMismatch,
    PageDigestMismatch,
    ProvenNoEffectHasWalAttempt,
    CounterOverflow,
}
