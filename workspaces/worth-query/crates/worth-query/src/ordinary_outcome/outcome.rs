use super::WorthQueryOrdinaryPosture;

#[derive(Debug, Eq, PartialEq)]
pub enum WorthQueryOrdinaryOutcome<T> {
    Bound(T),
    Ambiguous(WorthQueryOrdinaryPosture),
    AspectConflict(WorthQueryOrdinaryPosture),
    AuthorityMismatch(WorthQueryOrdinaryPosture),
    BasisMismatch(WorthQueryOrdinaryPosture),
    Deferred(WorthQueryOrdinaryPosture),
    Denied(WorthQueryOrdinaryPosture),
    ExplicitNarrowingRequired(WorthQueryOrdinaryPosture),
    Failed(WorthQueryOrdinaryPosture),
    MissingRequiredAspect(WorthQueryOrdinaryPosture),
    RebindRequired(WorthQueryOrdinaryPosture),
    Refused(WorthQueryOrdinaryPosture),
    Stale(WorthQueryOrdinaryPosture),
    Unavailable(WorthQueryOrdinaryPosture),
    Unsupported(WorthQueryOrdinaryPosture),
    WrongHandle(WorthQueryOrdinaryPosture),
    WrongWorld(WorthQueryOrdinaryPosture),
}

impl<T: Clone> Clone for WorthQueryOrdinaryOutcome<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Bound(value) => Self::Bound(value.clone()),
            Self::Ambiguous(value) => Self::Ambiguous(value.clone()),
            Self::AspectConflict(value) => Self::AspectConflict(value.clone()),
            Self::AuthorityMismatch(value) => Self::AuthorityMismatch(value.clone()),
            Self::BasisMismatch(value) => Self::BasisMismatch(value.clone()),
            Self::Deferred(value) => Self::Deferred(value.clone()),
            Self::Denied(value) => Self::Denied(value.clone()),
            Self::ExplicitNarrowingRequired(value) => {
                Self::ExplicitNarrowingRequired(value.clone())
            }
            Self::Failed(value) => Self::Failed(value.clone()),
            Self::MissingRequiredAspect(value) => Self::MissingRequiredAspect(value.clone()),
            Self::RebindRequired(value) => Self::RebindRequired(value.clone()),
            Self::Refused(value) => Self::Refused(value.clone()),
            Self::Stale(value) => Self::Stale(value.clone()),
            Self::Unavailable(value) => Self::Unavailable(value.clone()),
            Self::Unsupported(value) => Self::Unsupported(value.clone()),
            Self::WrongHandle(value) => Self::WrongHandle(value.clone()),
            Self::WrongWorld(value) => Self::WrongWorld(value.clone()),
        }
    }
}
