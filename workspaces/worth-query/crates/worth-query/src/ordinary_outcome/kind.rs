#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryOrdinaryPostureKind {
    Ambiguous,
    AspectConflict,
    AuthorityMismatch,
    BasisMismatch,
    Deferred,
    Denied,
    ExplicitNarrowingRequired,
    Failed,
    MissingRequiredAspect,
    RebindRequired,
    Refused,
    Stale,
    Unavailable,
    Unsupported,
    WrongHandle,
    WrongWorld,
}
