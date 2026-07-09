use crate::collections::Pair;
use crate::composition::{
    lower_deterministic_family_pair, AuthoritativeFamilyMember, CompositionFamilySymbol,
    FamilyLifecycleAction, LoweredFamilyProgram2,
};

pub fn create<S, A, P>(
    symbol: CompositionFamilySymbol<S>,
    payload: P,
) -> FamilyLifecycleAction<S, A, P> {
    FamilyLifecycleAction::Create { symbol, payload }
}

pub fn rewrite<S, A, P>(
    target: AuthoritativeFamilyMember<A>,
    payload: P,
) -> FamilyLifecycleAction<S, A, P> {
    FamilyLifecycleAction::Rewrite { target, payload }
}

pub fn supersede<S, A, P>(
    target: AuthoritativeFamilyMember<A>,
    replacement: CompositionFamilySymbol<S>,
    payload: P,
) -> FamilyLifecycleAction<S, A, P> {
    FamilyLifecycleAction::Supersede {
        target,
        replacement,
        payload,
    }
}

pub fn retire<S, A, P>(target: AuthoritativeFamilyMember<A>) -> FamilyLifecycleAction<S, A, P> {
    FamilyLifecycleAction::Retire { target }
}

pub fn family_pair<S, A, P>(
    left: FamilyLifecycleAction<S, A, P>,
    right: FamilyLifecycleAction<S, A, P>,
) -> Pair<FamilyLifecycleAction<S, A, P>> {
    Pair::new(left, right)
}

pub trait FamilyPairDxExt<S, A, P> {
    fn lower_by<K>(
        self,
        canonical_key: impl Fn(&FamilyLifecycleAction<S, A, P>) -> K,
    ) -> LoweredFamilyProgram2<S, A, P>
    where
        K: Ord;
}

impl<S, A, P> FamilyPairDxExt<S, A, P> for Pair<FamilyLifecycleAction<S, A, P>> {
    fn lower_by<K>(
        self,
        canonical_key: impl Fn(&FamilyLifecycleAction<S, A, P>) -> K,
    ) -> LoweredFamilyProgram2<S, A, P>
    where
        K: Ord,
    {
        lower_deterministic_family_pair(self, canonical_key)
    }
}

#[cfg(test)]
mod tests {
    use crate::composition::{
        lower_deterministic_family_pair, AuthoritativeFamilyMember, CompositionFamilySymbol,
        FamilyLifecycleAction,
    };

    use super::{create, family_pair, retire, rewrite, supersede, FamilyPairDxExt};

    #[test]
    fn family_intent_helpers_match_raw_lifecycle_actions() {
        assert_eq!(
            create::<u8, u16, _>(CompositionFamilySymbol::new(2_u8), "create"),
            FamilyLifecycleAction::Create {
                symbol: CompositionFamilySymbol::new(2_u8),
                payload: "create",
            }
        );
        assert_eq!(
            rewrite::<u8, u16, _>(AuthoritativeFamilyMember::new(11_u16), "rewrite"),
            FamilyLifecycleAction::Rewrite {
                target: AuthoritativeFamilyMember::new(11_u16),
                payload: "rewrite",
            }
        );
        assert_eq!(
            supersede::<u8, u16, _>(
                AuthoritativeFamilyMember::new(11_u16),
                CompositionFamilySymbol::new(3_u8),
                "replace",
            ),
            FamilyLifecycleAction::Supersede {
                target: AuthoritativeFamilyMember::new(11_u16),
                replacement: CompositionFamilySymbol::new(3_u8),
                payload: "replace",
            }
        );
        assert_eq!(
            retire::<u8, u16, &'static str>(AuthoritativeFamilyMember::new(11_u16)),
            FamilyLifecycleAction::Retire {
                target: AuthoritativeFamilyMember::new(11_u16),
            }
        );
    }

    #[test]
    fn family_pair_lower_by_matches_raw_family_lowering() {
        let pleasant = family_pair(
            create::<u8, u16, _>(CompositionFamilySymbol::new(2_u8), "create"),
            retire::<u8, u16, &'static str>(AuthoritativeFamilyMember::new(11_u16)),
        )
        .lower_by(family_action_key);

        let raw = lower_deterministic_family_pair(
            crate::collections::Pair::new(
                FamilyLifecycleAction::Create {
                    symbol: CompositionFamilySymbol::new(2_u8),
                    payload: "create",
                },
                FamilyLifecycleAction::Retire {
                    target: AuthoritativeFamilyMember::new(11_u16),
                },
            ),
            family_action_key,
        );

        assert_eq!(pleasant.actions(), raw.actions());
    }

    fn family_action_key<S: Ord + Copy, A: Ord + Copy, P>(
        action: &FamilyLifecycleAction<S, A, P>,
    ) -> (u8, Option<S>, Option<A>) {
        match action {
            FamilyLifecycleAction::Retire { target } => (0, None, Some(*target.value())),
            FamilyLifecycleAction::Rewrite { target, .. } => (1, None, Some(*target.value())),
            FamilyLifecycleAction::Supersede { target, .. } => (2, None, Some(*target.value())),
            FamilyLifecycleAction::Create { symbol, .. } => (3, Some(*symbol.value()), None),
        }
    }
}
