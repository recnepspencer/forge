use crate::transition::{SuccessfulTransitionOutcome, TransitionOutcome};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProofOutcomeKind {
    Success,
    Denied,
    Deferred,
    Stale,
    RebindRequired,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofOutcome<
    S,
    D = core::convert::Infallible,
    De = core::convert::Infallible,
    St = core::convert::Infallible,
    R = core::convert::Infallible,
    F = core::convert::Infallible,
> {
    raw: TransitionOutcome<S, D, De, St, R, F>,
}

impl<S, D, De, St, R, F> ProofOutcome<S, D, De, St, R, F> {
    pub fn kind(&self) -> ProofOutcomeKind {
        match &self.raw {
            TransitionOutcome::Success(_) => ProofOutcomeKind::Success,
            TransitionOutcome::Denied(_) => ProofOutcomeKind::Denied,
            TransitionOutcome::Deferred(_) => ProofOutcomeKind::Deferred,
            TransitionOutcome::Stale(_) => ProofOutcomeKind::Stale,
            TransitionOutcome::RebindRequired(_) => ProofOutcomeKind::RebindRequired,
            TransitionOutcome::Failed(_) => ProofOutcomeKind::Failed,
        }
    }

    pub fn is_success(&self) -> bool {
        matches!(self.raw, TransitionOutcome::Success(_))
    }

    pub fn as_raw(&self) -> &TransitionOutcome<S, D, De, St, R, F> {
        &self.raw
    }

    pub fn into_raw(self) -> TransitionOutcome<S, D, De, St, R, F> {
        self.raw
    }

    pub fn map_success<Next>(
        self,
        map: impl FnOnce(S) -> Next,
    ) -> ProofOutcome<Next, D, De, St, R, F> {
        ProofOutcome::from(self.raw.map_success(map))
    }
}

impl<S, D, De, St, R, F> From<TransitionOutcome<S, D, De, St, R, F>>
    for ProofOutcome<S, D, De, St, R, F>
{
    fn from(raw: TransitionOutcome<S, D, De, St, R, F>) -> Self {
        Self { raw }
    }
}

impl<S, D, De, St, R, F> From<SuccessfulTransitionOutcome<S>> for ProofOutcome<S, D, De, St, R, F> {
    fn from(value: SuccessfulTransitionOutcome<S>) -> Self {
        Self { raw: value.into() }
    }
}

#[cfg(test)]
mod tests {
    use crate::transition::TransitionOutcome;

    use super::{ProofOutcome, ProofOutcomeKind};

    #[test]
    fn proof_outcome_kind_preserves_all_transition_categories() {
        let success = ProofOutcome::<
            u64,
            &'static str,
            &'static str,
            &'static str,
            &'static str,
            &'static str,
        >::from(TransitionOutcome::success(1_u64));
        let denied = ProofOutcome::<
            u64,
            &'static str,
            &'static str,
            &'static str,
            &'static str,
            &'static str,
        >::from(TransitionOutcome::denied("denied"));
        let deferred = ProofOutcome::<
            u64,
            &'static str,
            &'static str,
            &'static str,
            &'static str,
            &'static str,
        >::from(TransitionOutcome::deferred("deferred"));
        let stale = ProofOutcome::<
            u64,
            &'static str,
            &'static str,
            &'static str,
            &'static str,
            &'static str,
        >::from(TransitionOutcome::stale("stale"));
        let rebind = ProofOutcome::<
            u64,
            &'static str,
            &'static str,
            &'static str,
            &'static str,
            &'static str,
        >::from(TransitionOutcome::rebind_required("rebind"));
        let failed = ProofOutcome::<
            u64,
            &'static str,
            &'static str,
            &'static str,
            &'static str,
            &'static str,
        >::from(TransitionOutcome::failed("failed"));

        assert_eq!(success.kind(), ProofOutcomeKind::Success);
        assert_eq!(denied.kind(), ProofOutcomeKind::Denied);
        assert_eq!(deferred.kind(), ProofOutcomeKind::Deferred);
        assert_eq!(stale.kind(), ProofOutcomeKind::Stale);
        assert_eq!(rebind.kind(), ProofOutcomeKind::RebindRequired);
        assert_eq!(failed.kind(), ProofOutcomeKind::Failed);
    }
}
