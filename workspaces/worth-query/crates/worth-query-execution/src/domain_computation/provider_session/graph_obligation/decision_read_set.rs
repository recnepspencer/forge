use worth_query_installation::facade::WorthQueryInstalledGraphObligationKind;

use super::{
    WorthQueryManagedGraphWorkSession, WorthQueryMutationGraphWorkLane, WorthQueryReadGraphWorkLane,
};

pub(in crate::domain_computation) struct WorthQueryCompleteGraphWorkDecisionReadSet<Lane, Basis> {
    pub(super) session: WorthQueryManagedGraphWorkSession<Lane, Basis>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::domain_computation) enum WorthQueryGraphWorkDecisionReadSetDenial {
    IncompleteOwnerProgression,
}

impl<Basis> WorthQueryManagedGraphWorkSession<WorthQueryReadGraphWorkLane, Basis> {
    pub(in crate::domain_computation) fn complete_decision_read_set(
        self,
    ) -> Result<
        WorthQueryCompleteGraphWorkDecisionReadSet<WorthQueryReadGraphWorkLane, Basis>,
        WorthQueryGraphWorkDecisionReadSetDenial,
    > {
        complete(self, read_decision_kind)
    }
}

impl<Basis> WorthQueryManagedGraphWorkSession<WorthQueryMutationGraphWorkLane, Basis> {
    pub(in crate::domain_computation) fn complete_decision_read_set(
        self,
    ) -> Result<
        WorthQueryCompleteGraphWorkDecisionReadSet<WorthQueryMutationGraphWorkLane, Basis>,
        WorthQueryGraphWorkDecisionReadSetDenial,
    > {
        complete(self, mutation_decision_kind)
    }
}

fn complete<Lane, Basis>(
    session: WorthQueryManagedGraphWorkSession<Lane, Basis>,
    included: fn(WorthQueryInstalledGraphObligationKind) -> bool,
) -> Result<
    WorthQueryCompleteGraphWorkDecisionReadSet<Lane, Basis>,
    WorthQueryGraphWorkDecisionReadSetDenial,
> {
    let complete = session
        .plan()
        .required_obligations()
        .iter()
        .filter(|row| included(row.kind()))
        .all(|row| {
            row.owner_progression()
                .iter()
                .enumerate()
                .all(|(ordinal, _)| {
                    session
                        .completed_owner_steps
                        .contains(&(row.identity().slot(), ordinal))
                })
        });
    complete
        .then_some(WorthQueryCompleteGraphWorkDecisionReadSet { session })
        .ok_or(WorthQueryGraphWorkDecisionReadSetDenial::IncompleteOwnerProgression)
}

fn read_decision_kind(kind: WorthQueryInstalledGraphObligationKind) -> bool {
    matches!(
        kind,
        WorthQueryInstalledGraphObligationKind::GraphRead
            | WorthQueryInstalledGraphObligationKind::AuthorizationObservation
    )
}

fn mutation_decision_kind(kind: WorthQueryInstalledGraphObligationKind) -> bool {
    read_decision_kind(kind) || kind == WorthQueryInstalledGraphObligationKind::MutationTouch
}
