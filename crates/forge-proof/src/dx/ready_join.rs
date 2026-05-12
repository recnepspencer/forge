use crate::composition::{compose_join_ready_recipe_pair, join_ready_recipe_pair, JoinInputs2};
use crate::recipe::ExecutionReadyRecipe;
use crate::transition::TransitionOutcome;

pub fn join_ready<L, R, LA, RA>(
    left: ExecutionReadyRecipe<L, LA>,
    right: ExecutionReadyRecipe<R, RA>,
) -> ExecutionReadyRecipe<JoinInputs2<L, R>, JoinInputs2<LA, RA>> {
    join_ready_recipe_pair(JoinInputs2::new(left, right))
}

pub fn compose_ready<L, R, LA, RA, D, De, St, Rb, F>(
    left: TransitionOutcome<ExecutionReadyRecipe<L, LA>, D, De, St, Rb, F>,
    right: impl FnOnce() -> TransitionOutcome<ExecutionReadyRecipe<R, RA>, D, De, St, Rb, F>,
) -> TransitionOutcome<ExecutionReadyRecipe<JoinInputs2<L, R>, JoinInputs2<LA, RA>>, D, De, St, Rb, F>
{
    compose_join_ready_recipe_pair(left, right)
}

#[cfg(test)]
mod tests {
    use crate::assumption::{AssumptionBasis, CurrentValidity, FreshnessScopedBasis};
    use crate::recipe::{ExecutionReadyRecipe, Lowered, Recipe};
    use crate::transition::TransitionOutcome;

    use super::{compose_ready, join_ready};

    #[test]
    fn pleasant_ready_join_matches_raw_join_shape() {
        let left = ExecutionReadyRecipe::new(Recipe::<Lowered, _, _>::with_stage(
            "left",
            FreshnessScopedBasis::<CurrentValidity, _>::new(AssumptionBasis::new(3_u8)),
        ));
        let right = ExecutionReadyRecipe::new(Recipe::<Lowered, _, _>::with_stage(
            "right",
            FreshnessScopedBasis::<CurrentValidity, _>::new(AssumptionBasis::new(5_u16)),
        ));

        let joined = join_ready(left, right);

        assert_eq!(joined.payload().left(), &"left");
        assert_eq!(joined.payload().right(), &"right");
        assert_eq!(joined.basis().left().basis().value(), &3_u8);
        assert_eq!(joined.basis().right().basis().value(), &5_u16);
    }

    #[test]
    fn pleasant_ready_join_composition_short_circuits_like_raw_lane() {
        type LeftBasis = FreshnessScopedBasis<CurrentValidity, AssumptionBasis<u8>>;

        let left = ExecutionReadyRecipe::new(Recipe::<Lowered, _, _>::with_stage(
            "left",
            FreshnessScopedBasis::<CurrentValidity, _>::new(AssumptionBasis::new(3_u8)),
        ));
        let right = ExecutionReadyRecipe::new(Recipe::<Lowered, _, _>::with_stage(
            "right",
            FreshnessScopedBasis::<CurrentValidity, _>::new(AssumptionBasis::new(5_u16)),
        ));

        let joined = compose_ready(TransitionOutcome::<_, &'static str>::success(left), || {
            TransitionOutcome::success(right)
        });
        assert!(matches!(joined, TransitionOutcome::Success(_)));

        let denied = compose_ready(
            TransitionOutcome::<ExecutionReadyRecipe<&'static str, LeftBasis>, &'static str>::denied(
                "denied",
            ),
            || -> TransitionOutcome<ExecutionReadyRecipe<&'static str, LeftBasis>, &'static str> {
                panic!("right lane must not run after left denial")
            },
        );

        assert!(matches!(denied, TransitionOutcome::Denied("denied")));
    }
}
