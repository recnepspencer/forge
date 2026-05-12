use std::convert::Infallible;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuccessfulTransitionOutcome<S> {
    value: S,
}

impl<S> SuccessfulTransitionOutcome<S> {
    pub fn new(value: S) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &S {
        &self.value
    }

    pub fn into_value(self) -> S {
        self.value
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransitionOutcome<
    S,
    D = Infallible,
    De = Infallible,
    St = Infallible,
    R = Infallible,
    F = Infallible,
> {
    Success(S),
    Denied(D),
    Deferred(De),
    Stale(St),
    RebindRequired(R),
    Failed(F),
}
pub type DenialTransitionOutcome<S, D, F = Infallible> =
    TransitionOutcome<S, D, Infallible, Infallible, Infallible, F>;
pub type DeferredTransitionOutcome<S, D, De, F = Infallible> =
    TransitionOutcome<S, D, De, Infallible, Infallible, F>;
pub type FreshnessTransitionOutcome<S, St, R, F = Infallible> =
    TransitionOutcome<S, Infallible, Infallible, St, R, F>;

impl<S, D, De, St, R, F> TransitionOutcome<S, D, De, St, R, F> {
    pub fn success(value: S) -> Self {
        Self::Success(value)
    }

    pub fn denied(value: D) -> Self {
        Self::Denied(value)
    }

    pub fn deferred(value: De) -> Self {
        Self::Deferred(value)
    }

    pub fn stale(value: St) -> Self {
        Self::Stale(value)
    }

    pub fn rebind_required(value: R) -> Self {
        Self::RebindRequired(value)
    }

    pub fn failed(value: F) -> Self {
        Self::Failed(value)
    }

    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success(_))
    }

    pub fn map_success<Next>(
        self,
        map: impl FnOnce(S) -> Next,
    ) -> TransitionOutcome<Next, D, De, St, R, F> {
        match self {
            Self::Success(value) => TransitionOutcome::Success(map(value)),
            Self::Denied(value) => TransitionOutcome::Denied(value),
            Self::Deferred(value) => TransitionOutcome::Deferred(value),
            Self::Stale(value) => TransitionOutcome::Stale(value),
            Self::RebindRequired(value) => TransitionOutcome::RebindRequired(value),
            Self::Failed(value) => TransitionOutcome::Failed(value),
        }
    }
}

impl<S, D, De, St, R, F> From<SuccessfulTransitionOutcome<S>>
    for TransitionOutcome<S, D, De, St, R, F>
{
    fn from(value: SuccessfulTransitionOutcome<S>) -> Self {
        TransitionOutcome::Success(value.into_value())
    }
}

#[cfg(test)]
mod tests {
    use crate::assumption::{RebindRequiredBasis, StaleReadableBasis};
    use crate::recipe::{Lowered, Recipe, Resolved};

    use super::{
        DeferredTransitionOutcome, DenialTransitionOutcome, FreshnessTransitionOutcome,
        SuccessfulTransitionOutcome, TransitionOutcome,
    };

    #[test]
    fn narrower_aliases_preserve_outcome_category_distinctions() {
        let success = SuccessfulTransitionOutcome::new(7);
        let denied: DenialTransitionOutcome<u64, &'static str> =
            TransitionOutcome::denied("denied");
        let deferred: DeferredTransitionOutcome<u64, &'static str, &'static str> =
            TransitionOutcome::deferred("deferred");

        assert_eq!(success.value(), &7);
        assert!(matches!(denied, TransitionOutcome::Denied("denied")));
        assert!(matches!(deferred, TransitionOutcome::Deferred("deferred")));
    }

    #[test]
    fn freshness_alias_carries_milestone3_state_types_without_generic_error_collapse() {
        let stale_recipe = Recipe::<Lowered, _, StaleReadableBasis<u8>>::with_stage(
            "lowered",
            StaleReadableBasis::new(crate::assumption::AssumptionBasis::new(7_u8)),
        );
        let rebind_recipe = Recipe::<Resolved, _, RebindRequiredBasis<u8>>::with_stage(
            "resolved",
            RebindRequiredBasis::new(crate::assumption::AssumptionBasis::new(11_u8)),
        );

        let stale: FreshnessTransitionOutcome<
            u64,
            Recipe<Lowered, &str, StaleReadableBasis<u8>>,
            Recipe<Resolved, &str, RebindRequiredBasis<u8>>,
            &'static str,
        > = TransitionOutcome::stale(stale_recipe);
        let rebind: FreshnessTransitionOutcome<
            u64,
            Recipe<Lowered, &str, StaleReadableBasis<u8>>,
            Recipe<Resolved, &str, RebindRequiredBasis<u8>>,
            &'static str,
        > = TransitionOutcome::rebind_required(rebind_recipe);

        assert!(matches!(stale, TransitionOutcome::Stale(_)));
        assert!(matches!(rebind, TransitionOutcome::RebindRequired(_)));
    }

    #[test]
    fn success_mapping_preserves_non_success_categories() {
        let denied: DenialTransitionOutcome<u64, &'static str> = TransitionOutcome::denied("nope");
        let remapped = denied.map_success(|value| value + 1);

        assert!(matches!(remapped, TransitionOutcome::Denied("nope")));
    }

    #[test]
    fn successful_outcome_only_carries_success_value() {
        let success = SuccessfulTransitionOutcome::new(9_u64);
        let widened: TransitionOutcome<u64, &'static str> = success.into();

        assert!(matches!(widened, TransitionOutcome::Success(9)));
    }
}
