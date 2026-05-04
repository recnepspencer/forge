use std::marker::PhantomData;

use crate::assumption::NoAssumptionBasis;

pub trait RecipeStageMarker: 'static {}

pub struct Unresolved;
impl RecipeStageMarker for Unresolved {}

pub struct Resolved;
impl RecipeStageMarker for Resolved {}

pub struct Lowered;
impl RecipeStageMarker for Lowered {}

pub struct Admitted;
impl RecipeStageMarker for Admitted {}

#[derive(Debug, PartialEq, Eq)]
pub struct Recipe<S, T, A = NoAssumptionBasis>
where
    S: RecipeStageMarker,
{
    pub(crate) payload: T,
    pub(crate) basis: A,
    pub(crate) stage: PhantomData<S>,
}

impl<T> Recipe<Unresolved, T, NoAssumptionBasis> {
    pub fn new(payload: T) -> Self {
        Self {
            payload,
            basis: NoAssumptionBasis,
            stage: PhantomData,
        }
    }
}

impl<S, T, A> Recipe<S, T, A>
where
    S: RecipeStageMarker,
{
    pub(crate) fn with_stage(payload: T, basis: A) -> Self {
        Self {
            payload,
            basis,
            stage: PhantomData,
        }
    }

    pub fn payload(&self) -> &T {
        &self.payload
    }

    pub fn basis(&self) -> &A {
        &self.basis
    }

    pub fn into_parts(self) -> (T, A) {
        (self.payload, self.basis)
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use crate::assumption::{AssumptionBasis, NoAssumptionBasis};

    use super::{Recipe, Resolved, Unresolved};

    #[test]
    fn unresolved_recipe_uses_empty_basis_default() {
        let recipe = Recipe::<Unresolved, _>::new("payload");

        assert_eq!(recipe.payload(), &"payload");
        assert_eq!(recipe.basis(), &NoAssumptionBasis);
    }

    #[test]
    fn recipe_is_size_honest_for_representation() {
        assert_eq!(size_of::<Recipe<Unresolved, u64>>(), size_of::<u64>());
        assert_eq!(
            size_of::<Recipe<Resolved, u64, AssumptionBasis<u8>>>(),
            size_of::<(u64, AssumptionBasis<u8>)>()
        );
    }
}
