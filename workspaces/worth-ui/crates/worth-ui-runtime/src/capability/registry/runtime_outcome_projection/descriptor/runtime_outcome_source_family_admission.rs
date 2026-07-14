use worth_query::facade::foundation::{
    WorthQueryOrdinaryPostureKind,
    WorthQueryOrdinaryRuntimePostureKind,
};
use worth_query::facade::runtime::WorthQueryRuntimeAsyncResultStateKind;

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
    posture_kind: Option<WorthQueryOrdinaryPostureKind>,
) -> Option<RuntimeOutcomeFamily> {
    if kind == "bound" {
        return Some(RuntimeOutcomeFamily::ready());
    }
    posture_kind.and_then(ordinary_posture_family)
}

fn ordinary_posture_family(kind: WorthQueryOrdinaryPostureKind) -> Option<RuntimeOutcomeFamily> {
    match kind {
        WorthQueryOrdinaryPostureKind::Ambiguous
        | WorthQueryOrdinaryPostureKind::Deferred
        | WorthQueryOrdinaryPostureKind::ExplicitNarrowingRequired
        | WorthQueryOrdinaryPostureKind::MissingRequiredAspect => {
            Some(RuntimeOutcomeFamily::advisory())
        }
        WorthQueryOrdinaryPostureKind::AspectConflict
        | WorthQueryOrdinaryPostureKind::AuthorityMismatch
        | WorthQueryOrdinaryPostureKind::BasisMismatch
        | WorthQueryOrdinaryPostureKind::Refused
        | WorthQueryOrdinaryPostureKind::WrongHandle
        | WorthQueryOrdinaryPostureKind::WrongWorld => Some(RuntimeOutcomeFamily::violation()),
        WorthQueryOrdinaryPostureKind::Denied => Some(RuntimeOutcomeFamily::denied()),
        WorthQueryOrdinaryPostureKind::Failed | WorthQueryOrdinaryPostureKind::Unavailable => {
            Some(RuntimeOutcomeFamily::failed())
        }
        WorthQueryOrdinaryPostureKind::Unsupported => None,
        WorthQueryOrdinaryPostureKind::RebindRequired => Some(RuntimeOutcomeFamily::recoverable()),
        WorthQueryOrdinaryPostureKind::Stale => Some(RuntimeOutcomeFamily::stale()),
    }
}

fn runtime_posture_family(
    kind: WorthQueryOrdinaryRuntimePostureKind,
) -> Option<RuntimeOutcomeFamily> {
    match kind {
        WorthQueryOrdinaryRuntimePostureKind::Current => Some(RuntimeOutcomeFamily::ready()),
        WorthQueryOrdinaryRuntimePostureKind::Remasked => Some(RuntimeOutcomeFamily::advisory()),
        WorthQueryOrdinaryRuntimePostureKind::Pending => Some(RuntimeOutcomeFamily::loading()),
        WorthQueryOrdinaryRuntimePostureKind::Failed => Some(RuntimeOutcomeFamily::failed()),
        WorthQueryOrdinaryRuntimePostureKind::Stale => Some(RuntimeOutcomeFamily::stale()),
        WorthQueryOrdinaryRuntimePostureKind::Cancelled => Some(RuntimeOutcomeFamily::cancelled()),
        WorthQueryOrdinaryRuntimePostureKind::Retried => Some(RuntimeOutcomeFamily::retrying()),
        WorthQueryOrdinaryRuntimePostureKind::Revalidating => {
            Some(RuntimeOutcomeFamily::revalidating())
        }
        WorthQueryOrdinaryRuntimePostureKind::Superseded => Some(RuntimeOutcomeFamily::stopped()),
        WorthQueryOrdinaryRuntimePostureKind::Denied => Some(RuntimeOutcomeFamily::denied()),
        WorthQueryOrdinaryRuntimePostureKind::Unsupported => None,
    }
}

fn async_result_state_family(
    kind: WorthQueryRuntimeAsyncResultStateKind,
) -> Option<RuntimeOutcomeFamily> {
    match kind {
        WorthQueryRuntimeAsyncResultStateKind::Pending => Some(RuntimeOutcomeFamily::loading()),
        WorthQueryRuntimeAsyncResultStateKind::Current => Some(RuntimeOutcomeFamily::ready()),
        WorthQueryRuntimeAsyncResultStateKind::Failed => Some(RuntimeOutcomeFamily::failed()),
        WorthQueryRuntimeAsyncResultStateKind::Stale => Some(RuntimeOutcomeFamily::stale()),
        WorthQueryRuntimeAsyncResultStateKind::Cancelled => Some(RuntimeOutcomeFamily::cancelled()),
        WorthQueryRuntimeAsyncResultStateKind::Retried => Some(RuntimeOutcomeFamily::retrying()),
        WorthQueryRuntimeAsyncResultStateKind::Revalidating => {
            Some(RuntimeOutcomeFamily::revalidating())
        }
        WorthQueryRuntimeAsyncResultStateKind::Superseded => Some(RuntimeOutcomeFamily::stopped()),
        WorthQueryRuntimeAsyncResultStateKind::Denied => Some(RuntimeOutcomeFamily::denied()),
    }
}
