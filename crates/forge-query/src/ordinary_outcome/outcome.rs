use super::ForgeQueryOrdinaryPosture;

#[derive(Debug, Eq, PartialEq)]
pub enum ForgeQueryOrdinaryOutcome<T> {
    Bound(T),
    Ambiguous(ForgeQueryOrdinaryPosture),
    AspectConflict(ForgeQueryOrdinaryPosture),
    AuthorityMismatch(ForgeQueryOrdinaryPosture),
    BasisMismatch(ForgeQueryOrdinaryPosture),
    Deferred(ForgeQueryOrdinaryPosture),
    Denied(ForgeQueryOrdinaryPosture),
    ExplicitNarrowingRequired(ForgeQueryOrdinaryPosture),
    Failed(ForgeQueryOrdinaryPosture),
    MissingRequiredAspect(ForgeQueryOrdinaryPosture),
    RebindRequired(ForgeQueryOrdinaryPosture),
    Refused(ForgeQueryOrdinaryPosture),
    Stale(ForgeQueryOrdinaryPosture),
    Unavailable(ForgeQueryOrdinaryPosture),
    Unsupported(ForgeQueryOrdinaryPosture),
    WrongHandle(ForgeQueryOrdinaryPosture),
    WrongWorld(ForgeQueryOrdinaryPosture),
}

impl<T: Clone> Clone for ForgeQueryOrdinaryOutcome<T> {
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
