use crate::assumption::{
    AssumptionBasis, AuthorityRevalidationRequiredBasis,
    BoundaryBridgedAuthorityRevalidationRequiredBasis, BoundaryBridgedRebindRequiredBasis,
    BoundaryBridgedStaleReadableBasis, CurrentValidity, FreshnessScopedBasis, NoAssumptionBasis,
    RebindRequiredBasis, StaleReadableBasis,
};
use crate::recipe::{ExecutedRecipe, ExecutionReadyRecipe, Recipe, RecipeStageMarker};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasisPostureKind {
    None,
    CurrentValidity,
    StaleReadable,
    RebindRequired,
    AuthorityRevalidationRequired,
    BoundaryBridgedStaleReadable,
    BoundaryBridgedRebindRequired,
    BoundaryBridgedAuthorityRevalidationRequired,
}

pub trait BasisPostureDxExt {
    fn basis_posture(&self) -> BasisPostureKind;
    fn has_strong_basis(&self) -> bool;
}

trait BasisPostureMarker {
    const BASIS_POSTURE: BasisPostureKind;
}

impl BasisPostureMarker for NoAssumptionBasis {
    const BASIS_POSTURE: BasisPostureKind = BasisPostureKind::None;
}

impl<B> BasisPostureMarker for FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>> {
    const BASIS_POSTURE: BasisPostureKind = BasisPostureKind::CurrentValidity;
}

impl<B> BasisPostureMarker for StaleReadableBasis<B> {
    const BASIS_POSTURE: BasisPostureKind = BasisPostureKind::StaleReadable;
}

impl<B> BasisPostureMarker for RebindRequiredBasis<B> {
    const BASIS_POSTURE: BasisPostureKind = BasisPostureKind::RebindRequired;
}

impl<B> BasisPostureMarker for AuthorityRevalidationRequiredBasis<B> {
    const BASIS_POSTURE: BasisPostureKind = BasisPostureKind::AuthorityRevalidationRequired;
}

impl<B> BasisPostureMarker for BoundaryBridgedStaleReadableBasis<B> {
    const BASIS_POSTURE: BasisPostureKind = BasisPostureKind::BoundaryBridgedStaleReadable;
}

impl<B> BasisPostureMarker for BoundaryBridgedRebindRequiredBasis<B> {
    const BASIS_POSTURE: BasisPostureKind = BasisPostureKind::BoundaryBridgedRebindRequired;
}

impl<B> BasisPostureMarker for BoundaryBridgedAuthorityRevalidationRequiredBasis<B> {
    const BASIS_POSTURE: BasisPostureKind =
        BasisPostureKind::BoundaryBridgedAuthorityRevalidationRequired;
}

impl<S, T, A> BasisPostureDxExt for Recipe<S, T, A>
where
    S: RecipeStageMarker,
    A: BasisPostureMarker,
{
    fn basis_posture(&self) -> BasisPostureKind {
        A::BASIS_POSTURE
    }

    fn has_strong_basis(&self) -> bool {
        matches!(self.basis_posture(), BasisPostureKind::CurrentValidity)
    }
}

impl<T, A> BasisPostureDxExt for ExecutionReadyRecipe<T, A>
where
    A: BasisPostureMarker,
{
    fn basis_posture(&self) -> BasisPostureKind {
        A::BASIS_POSTURE
    }

    fn has_strong_basis(&self) -> bool {
        matches!(self.basis_posture(), BasisPostureKind::CurrentValidity)
    }
}

impl<T, A> BasisPostureDxExt for ExecutedRecipe<T, A>
where
    A: BasisPostureMarker,
{
    fn basis_posture(&self) -> BasisPostureKind {
        A::BASIS_POSTURE
    }

    fn has_strong_basis(&self) -> bool {
        matches!(self.basis_posture(), BasisPostureKind::CurrentValidity)
    }
}

#[cfg(test)]
mod tests {
    use crate::assumption::{
        AssumptionBasis, CurrentValidity, FreshnessScopedBasis, RebindRequiredBasis,
    };
    use crate::recipe::{ExecutionReadyRecipe, Lowered, Recipe, Resolved};

    use super::{BasisPostureDxExt, BasisPostureKind};

    #[test]
    fn basis_posture_distinguishes_current_and_rebind_forms() {
        let current = Recipe::<Resolved, _, _>::with_stage(
            "payload",
            FreshnessScopedBasis::<CurrentValidity, _>::new(AssumptionBasis::new(7_u8)),
        );
        let rebind = Recipe::<Resolved, _, _>::with_stage(
            "payload",
            RebindRequiredBasis::new(AssumptionBasis::new(7_u8)),
        );
        let ready = ExecutionReadyRecipe::new(Recipe::<Lowered, _, _>::with_stage(
            "payload",
            FreshnessScopedBasis::<CurrentValidity, _>::new(AssumptionBasis::new(9_u8)),
        ));

        assert_eq!(current.basis_posture(), BasisPostureKind::CurrentValidity);
        assert!(current.has_strong_basis());
        assert_eq!(rebind.basis_posture(), BasisPostureKind::RebindRequired);
        assert!(!rebind.has_strong_basis());
        assert_eq!(ready.basis_posture(), BasisPostureKind::CurrentValidity);
        assert!(ready.has_strong_basis());
    }
}
