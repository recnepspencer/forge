#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransitionReadiness<C, D, De, St, R, F> {
    Ready(C),
    Denied(D),
    Deferred(De),
    Stale(St),
    RebindRequired(R),
    Failed(F),
}

impl<C, D, De, St, R, F> TransitionReadiness<C, D, De, St, R, F> {
    pub fn ready(context: C) -> Self {
        Self::Ready(context)
    }

    pub fn denied(reason: D) -> Self {
        Self::Denied(reason)
    }

    pub fn deferred(reason: De) -> Self {
        Self::Deferred(reason)
    }

    pub fn stale(value: St) -> Self {
        Self::Stale(value)
    }

    pub fn rebind_required(value: R) -> Self {
        Self::RebindRequired(value)
    }

    pub fn failed(reason: F) -> Self {
        Self::Failed(reason)
    }

    pub fn map_ready<Next>(
        self,
        map: impl FnOnce(C) -> Next,
    ) -> TransitionReadiness<Next, D, De, St, R, F> {
        match self {
            Self::Ready(context) => TransitionReadiness::Ready(map(context)),
            Self::Denied(reason) => TransitionReadiness::Denied(reason),
            Self::Deferred(reason) => TransitionReadiness::Deferred(reason),
            Self::Stale(value) => TransitionReadiness::Stale(value),
            Self::RebindRequired(value) => TransitionReadiness::RebindRequired(value),
            Self::Failed(reason) => TransitionReadiness::Failed(reason),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreConstructionGate<C, D, De> {
    Ready(C),
    Denied(D),
    Deferred(De),
}

impl<C, D, De> PreConstructionGate<C, D, De> {
    pub fn ready(context: C) -> Self {
        Self::Ready(context)
    }

    pub fn denied(reason: D) -> Self {
        Self::Denied(reason)
    }

    pub fn deferred(reason: De) -> Self {
        Self::Deferred(reason)
    }

    pub fn map_ready<Next>(self, map: impl FnOnce(C) -> Next) -> PreConstructionGate<Next, D, De> {
        match self {
            Self::Ready(context) => PreConstructionGate::Ready(map(context)),
            Self::Denied(reason) => PreConstructionGate::Denied(reason),
            Self::Deferred(reason) => PreConstructionGate::Deferred(reason),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PreConstructionGate, TransitionReadiness};

    #[test]
    fn pre_construction_gate_preserves_denial_and_defer_without_fake_success() {
        let denied = PreConstructionGate::<u64, _, &'static str>::denied("denied");
        let deferred = PreConstructionGate::<u64, &'static str, _>::deferred("deferred");

        assert!(matches!(denied, PreConstructionGate::Denied("denied")));
        assert!(matches!(
            deferred,
            PreConstructionGate::Deferred("deferred")
        ));
    }

    #[test]
    fn pre_construction_gate_maps_only_ready_context() {
        let ready = PreConstructionGate::<u64, &'static str, &'static str>::ready(7);
        let mapped = ready.map_ready(|value| value + 1);

        assert!(matches!(mapped, PreConstructionGate::Ready(8)));
    }

    #[test]
    fn transition_readiness_preserves_freshness_and_failure_categories() {
        let stale = TransitionReadiness::<
            u64,
            &'static str,
            &'static str,
            &'static str,
            &'static str,
            &'static str,
        >::stale("stale");
        let rebind = TransitionReadiness::<
            u64,
            &'static str,
            &'static str,
            &'static str,
            &'static str,
            &'static str,
        >::rebind_required("rebind");
        let failed = TransitionReadiness::<
            u64,
            &'static str,
            &'static str,
            &'static str,
            &'static str,
            &'static str,
        >::failed("failed");

        assert!(matches!(stale, TransitionReadiness::Stale("stale")));
        assert!(matches!(
            rebind,
            TransitionReadiness::RebindRequired("rebind")
        ));
        assert!(matches!(failed, TransitionReadiness::Failed("failed")));
    }

    #[test]
    fn transition_readiness_maps_only_ready_context() {
        let ready = TransitionReadiness::<
            u64,
            &'static str,
            &'static str,
            &'static str,
            &'static str,
            &'static str,
        >::ready(7);
        let mapped = ready.map_ready(|value| value + 1);

        assert!(matches!(mapped, TransitionReadiness::Ready(8)));
    }
}
