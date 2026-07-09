use crate::collections::Pair;
use crate::composition::{FamilyLifecycleAction, JoinInputs2, LoweredFamilyProgram2};
use crate::recipe::ExecutionReadyRecipe;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FamilyActionKind {
    Create,
    Rewrite,
    Supersede,
    Retire,
}

pub trait FamilyActionDxExt {
    fn kind(&self) -> FamilyActionKind;
}

impl<S, A, P> FamilyActionDxExt for FamilyLifecycleAction<S, A, P> {
    fn kind(&self) -> FamilyActionKind {
        match self {
            FamilyLifecycleAction::Create { .. } => FamilyActionKind::Create,
            FamilyLifecycleAction::Rewrite { .. } => FamilyActionKind::Rewrite,
            FamilyLifecycleAction::Supersede { .. } => FamilyActionKind::Supersede,
            FamilyLifecycleAction::Retire { .. } => FamilyActionKind::Retire,
        }
    }
}

pub trait LoweredFamilyProgramDxExt<S, A, P> {
    fn action_kinds(&self) -> Pair<FamilyActionKind>;
}

impl<S, A, P> LoweredFamilyProgramDxExt<S, A, P> for LoweredFamilyProgram2<S, A, P> {
    fn action_kinds(&self) -> Pair<FamilyActionKind> {
        Pair::new(self.actions().left().kind(), self.actions().right().kind())
    }
}

pub struct ReadyJoinSummary<'a, L, R, LA, RA> {
    left_payload: &'a L,
    right_payload: &'a R,
    left_basis: &'a LA,
    right_basis: &'a RA,
}

impl<'a, L, R, LA, RA> ReadyJoinSummary<'a, L, R, LA, RA> {
    pub fn left_payload(&self) -> &'a L {
        self.left_payload
    }

    pub fn right_payload(&self) -> &'a R {
        self.right_payload
    }

    pub fn left_basis(&self) -> &'a LA {
        self.left_basis
    }

    pub fn right_basis(&self) -> &'a RA {
        self.right_basis
    }
}

pub trait ReadyJoinRecipeDxExt<L, R, LA, RA> {
    fn summary(&self) -> ReadyJoinSummary<'_, L, R, LA, RA>;
}

impl<L, R, LA, RA> ReadyJoinRecipeDxExt<L, R, LA, RA>
    for ExecutionReadyRecipe<JoinInputs2<L, R>, JoinInputs2<LA, RA>>
{
    fn summary(&self) -> ReadyJoinSummary<'_, L, R, LA, RA> {
        ReadyJoinSummary {
            left_payload: self.payload().left(),
            right_payload: self.payload().right(),
            left_basis: self.basis().left(),
            right_basis: self.basis().right(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::assumption::{AssumptionBasis, CurrentValidity, FreshnessScopedBasis};
    use crate::collections::Pair;
    use crate::composition::{
        lower_deterministic_family_pair, AuthoritativeFamilyMember, CompositionFamilySymbol,
        FamilyLifecycleAction, JoinInputs2,
    };
    use crate::recipe::{ExecutionReadyRecipe, Lowered, Recipe};

    use super::{
        FamilyActionDxExt, FamilyActionKind, LoweredFamilyProgramDxExt, ReadyJoinRecipeDxExt,
    };

    #[test]
    fn family_action_kind_and_lowered_kinds_are_narrow() {
        let lowered = lower_deterministic_family_pair(
            Pair::new(
                FamilyLifecycleAction::Create {
                    symbol: CompositionFamilySymbol::new(2_u8),
                    payload: "create",
                },
                FamilyLifecycleAction::Retire {
                    target: AuthoritativeFamilyMember::new(11_u16),
                },
            ),
            |action| match action {
                FamilyLifecycleAction::Retire { target } => (0, None, Some(*target.value())),
                FamilyLifecycleAction::Rewrite { target, .. } => (1, None, Some(*target.value())),
                FamilyLifecycleAction::Supersede { target, .. } => (2, None, Some(*target.value())),
                FamilyLifecycleAction::Create { symbol, .. } => (3, Some(*symbol.value()), None),
            },
        );

        assert_eq!(lowered.actions().left().kind(), FamilyActionKind::Retire);
        assert_eq!(
            lowered.action_kinds(),
            Pair::new(FamilyActionKind::Retire, FamilyActionKind::Create)
        );
    }

    #[test]
    fn ready_join_summary_groups_common_reads() {
        let joined = ExecutionReadyRecipe::new(Recipe::<Lowered, _, _>::with_stage(
            JoinInputs2::new("left", "right"),
            JoinInputs2::new(
                FreshnessScopedBasis::<CurrentValidity, _>::new(AssumptionBasis::new(3_u8)),
                FreshnessScopedBasis::<CurrentValidity, _>::new(AssumptionBasis::new(5_u16)),
            ),
        ));

        let summary = joined.summary();

        assert_eq!(summary.left_payload(), &"left");
        assert_eq!(summary.right_payload(), &"right");
        assert_eq!(summary.left_basis().basis().value(), &3_u8);
        assert_eq!(summary.right_basis().basis().value(), &5_u16);
    }
}
