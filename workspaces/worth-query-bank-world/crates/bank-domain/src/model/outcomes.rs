#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReadOutcome<T, D> {
    Delivered(T),
    Absent,
    Denied(D),
    Stale,
    Cancelled,
    Unavailable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MutationOutcome<C, P, D, V> {
    Committed(C),
    ApprovalRequired(P),
    Denied(D),
    InvariantViolated(V),
    Stale,
    AlreadyCommitted(C),
    Aborted,
    Cancelled,
    PartialEffect,
    Indeterminate,
}
