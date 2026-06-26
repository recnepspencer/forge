use forge_query::facade::{
    ForgeQueryOrdinaryPostureKind, ForgeQueryOrdinaryRuntimePostureKind,
    ForgeQueryRuntimeAsyncResultStateKind,
};

use super::{RuntimeOutcomeFamily, RuntimeOutcomeSourceReference};

impl RuntimeOutcomeSourceReference {
    pub(crate) fn admits_family(&self, family: &RuntimeOutcomeFamily) -> bool {
        self.admitted_family().as_ref() == Some(family)
    }

    pub(crate) fn admitted_family(&self) -> Option<RuntimeOutcomeFamily> {
        match self {
            Self::QueryOrdinaryOutcome { kind, posture_kind } => {
                ordinary_outcome_family(kind, *posture_kind)
            }
            Self::QueryOrdinaryPosture { kind } => ordinary_posture_family(*kind),
            Self::QueryOrdinaryRuntimePosture { kind, .. } => runtime_posture_family(*kind),
            Self::QueryRuntimeAsyncResultState { kind, .. } => async_result_state_family(*kind),
        }
    }
}

fn ordinary_outcome_family(
    kind: &str,
    posture_kind: Option<ForgeQueryOrdinaryPostureKind>,
) -> Option<RuntimeOutcomeFamily> {
    if kind == "bound" {
        return Some(RuntimeOutcomeFamily::ready());
    }
    posture_kind.and_then(ordinary_posture_family)
}

fn ordinary_posture_family(kind: ForgeQueryOrdinaryPostureKind) -> Option<RuntimeOutcomeFamily> {
    match kind {
        ForgeQueryOrdinaryPostureKind::Ambiguous
        | ForgeQueryOrdinaryPostureKind::Deferred
        | ForgeQueryOrdinaryPostureKind::ExplicitNarrowingRequired
        | ForgeQueryOrdinaryPostureKind::MissingRequiredAspect => {
            Some(RuntimeOutcomeFamily::advisory())
        }
        ForgeQueryOrdinaryPostureKind::AspectConflict
        | ForgeQueryOrdinaryPostureKind::AuthorityMismatch
        | ForgeQueryOrdinaryPostureKind::BasisMismatch
        | ForgeQueryOrdinaryPostureKind::Refused
        | ForgeQueryOrdinaryPostureKind::WrongHandle
        | ForgeQueryOrdinaryPostureKind::WrongWorld => Some(RuntimeOutcomeFamily::violation()),
        ForgeQueryOrdinaryPostureKind::Denied => Some(RuntimeOutcomeFamily::denied()),
        ForgeQueryOrdinaryPostureKind::Failed | ForgeQueryOrdinaryPostureKind::Unavailable => {
            Some(RuntimeOutcomeFamily::failed())
        }
        ForgeQueryOrdinaryPostureKind::Unsupported => None,
        ForgeQueryOrdinaryPostureKind::RebindRequired => Some(RuntimeOutcomeFamily::recoverable()),
        ForgeQueryOrdinaryPostureKind::Stale => Some(RuntimeOutcomeFamily::stale()),
    }
}

fn runtime_posture_family(
    kind: ForgeQueryOrdinaryRuntimePostureKind,
) -> Option<RuntimeOutcomeFamily> {
    match kind {
        ForgeQueryOrdinaryRuntimePostureKind::Current => Some(RuntimeOutcomeFamily::ready()),
        ForgeQueryOrdinaryRuntimePostureKind::Remasked => Some(RuntimeOutcomeFamily::advisory()),
        ForgeQueryOrdinaryRuntimePostureKind::Pending => Some(RuntimeOutcomeFamily::loading()),
        ForgeQueryOrdinaryRuntimePostureKind::Failed => Some(RuntimeOutcomeFamily::failed()),
        ForgeQueryOrdinaryRuntimePostureKind::Stale => Some(RuntimeOutcomeFamily::stale()),
        ForgeQueryOrdinaryRuntimePostureKind::Cancelled => Some(RuntimeOutcomeFamily::cancelled()),
        ForgeQueryOrdinaryRuntimePostureKind::Retried => Some(RuntimeOutcomeFamily::retrying()),
        ForgeQueryOrdinaryRuntimePostureKind::Revalidating => {
            Some(RuntimeOutcomeFamily::revalidating())
        }
        ForgeQueryOrdinaryRuntimePostureKind::Superseded => Some(RuntimeOutcomeFamily::stopped()),
        ForgeQueryOrdinaryRuntimePostureKind::Denied => Some(RuntimeOutcomeFamily::denied()),
        ForgeQueryOrdinaryRuntimePostureKind::Unsupported => None,
    }
}

fn async_result_state_family(
    kind: ForgeQueryRuntimeAsyncResultStateKind,
) -> Option<RuntimeOutcomeFamily> {
    match kind {
        ForgeQueryRuntimeAsyncResultStateKind::Pending => Some(RuntimeOutcomeFamily::loading()),
        ForgeQueryRuntimeAsyncResultStateKind::Current => Some(RuntimeOutcomeFamily::ready()),
        ForgeQueryRuntimeAsyncResultStateKind::Failed => Some(RuntimeOutcomeFamily::failed()),
        ForgeQueryRuntimeAsyncResultStateKind::Stale => Some(RuntimeOutcomeFamily::stale()),
        ForgeQueryRuntimeAsyncResultStateKind::Cancelled => Some(RuntimeOutcomeFamily::cancelled()),
        ForgeQueryRuntimeAsyncResultStateKind::Retried => Some(RuntimeOutcomeFamily::retrying()),
        ForgeQueryRuntimeAsyncResultStateKind::Revalidating => {
            Some(RuntimeOutcomeFamily::revalidating())
        }
        ForgeQueryRuntimeAsyncResultStateKind::Superseded => Some(RuntimeOutcomeFamily::stopped()),
        ForgeQueryRuntimeAsyncResultStateKind::Denied => Some(RuntimeOutcomeFamily::denied()),
    }
}
