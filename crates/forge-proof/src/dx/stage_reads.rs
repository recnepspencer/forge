use crate::recipe::{ExecutedRecipe, ExecutionReadyRecipe, Recipe, RecipeStageMarker};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecipeStageKind {
    Unresolved,
    Resolved,
    Lowered,
    Admitted,
    ExecutionReady,
    Executed,
}

pub trait RecipeStageDxExt {
    fn stage(&self) -> RecipeStageKind;
}

trait RecipeStageKindMarker: RecipeStageMarker {
    const STAGE_KIND: RecipeStageKind;
}

impl RecipeStageKindMarker for crate::recipe::Unresolved {
    const STAGE_KIND: RecipeStageKind = RecipeStageKind::Unresolved;
}

impl RecipeStageKindMarker for crate::recipe::Resolved {
    const STAGE_KIND: RecipeStageKind = RecipeStageKind::Resolved;
}

impl RecipeStageKindMarker for crate::recipe::Lowered {
    const STAGE_KIND: RecipeStageKind = RecipeStageKind::Lowered;
}

impl RecipeStageKindMarker for crate::recipe::Admitted {
    const STAGE_KIND: RecipeStageKind = RecipeStageKind::Admitted;
}

impl<S, T, A> RecipeStageDxExt for Recipe<S, T, A>
where
    S: RecipeStageKindMarker,
{
    fn stage(&self) -> RecipeStageKind {
        S::STAGE_KIND
    }
}

impl<T, A> RecipeStageDxExt for ExecutionReadyRecipe<T, A> {
    fn stage(&self) -> RecipeStageKind {
        RecipeStageKind::ExecutionReady
    }
}

impl<T, A> RecipeStageDxExt for ExecutedRecipe<T, A> {
    fn stage(&self) -> RecipeStageKind {
        RecipeStageKind::Executed
    }
}

#[cfg(test)]
mod tests {
    use crate::recipe::{ExecutedRecipe, ExecutionReadyRecipe, Lowered, Recipe, Unresolved};

    use super::{RecipeStageDxExt, RecipeStageKind};

    #[test]
    fn stage_inspector_distinguishes_recipe_and_execution_forms() {
        let unresolved = Recipe::<Unresolved, _>::new("payload");
        let ready = ExecutionReadyRecipe::new(Recipe::<Lowered, _, _>::with_stage(
            "payload",
            crate::assumption::NoAssumptionBasis,
        ));
        let executed = ExecutedRecipe::new(ready);

        assert_eq!(unresolved.stage(), RecipeStageKind::Unresolved);
        assert_eq!(executed.stage(), RecipeStageKind::Executed);
    }
}
