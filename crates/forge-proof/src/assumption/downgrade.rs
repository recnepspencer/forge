use crate::artifact::Artifact;
use crate::recipe::{Admitted, Lowered, Recipe, Resolved};

use super::{
    AssumptionBasis, AuthorityRevalidationRequired, CurrentValidity, FreshnessScopedBasis,
    RebindRequired, StaleReadable,
};

pub type StaleReadableBasis<B> = FreshnessScopedBasis<StaleReadable, AssumptionBasis<B>>;
pub type RebindRequiredBasis<B> = FreshnessScopedBasis<RebindRequired, AssumptionBasis<B>>;
pub type AuthorityRevalidationRequiredBasis<B> =
    FreshnessScopedBasis<AuthorityRevalidationRequired, AssumptionBasis<B>>;

impl<P, T, S, B> Artifact<P, T, S, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>> {
    pub fn strong_basis(&self) -> &AssumptionBasis<B> {
        self.basis.basis()
    }

    pub fn downgrade_to_stale_readable(self) -> Artifact<P, T, S, StaleReadableBasis<B>> {
        Artifact::with_state(
            self.payload,
            self.proofs,
            FreshnessScopedBasis::new(self.basis.into_basis()),
        )
    }

    pub fn downgrade_to_rebind_required(self) -> Artifact<P, T, S, RebindRequiredBasis<B>> {
        Artifact::with_state(
            self.payload,
            self.proofs,
            FreshnessScopedBasis::new(self.basis.into_basis()),
        )
    }

    pub fn downgrade_to_authority_revalidation_required(
        self,
    ) -> Artifact<P, T, S, AuthorityRevalidationRequiredBasis<B>> {
        Artifact::with_state(
            self.payload,
            self.proofs,
            FreshnessScopedBasis::new(self.basis.into_basis()),
        )
    }
}

impl<T, B> Recipe<Resolved, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>> {
    pub fn strong_basis(&self) -> &AssumptionBasis<B> {
        self.basis.basis()
    }

    pub fn downgrade_to_rebind_required(self) -> Recipe<Resolved, T, RebindRequiredBasis<B>> {
        Recipe::with_stage(
            self.payload,
            FreshnessScopedBasis::new(self.basis.into_basis()),
        )
    }
}

impl<T, B> Recipe<Lowered, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>> {
    pub fn strong_basis(&self) -> &AssumptionBasis<B> {
        self.basis.basis()
    }

    pub fn downgrade_to_stale_readable(self) -> Recipe<Lowered, T, StaleReadableBasis<B>> {
        Recipe::with_stage(
            self.payload,
            FreshnessScopedBasis::new(self.basis.into_basis()),
        )
    }
}

impl<T, B> Recipe<Admitted, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>> {
    pub fn strong_basis(&self) -> &AssumptionBasis<B> {
        self.basis.basis()
    }

    pub fn downgrade_to_stale_readable(self) -> Recipe<Admitted, T, StaleReadableBasis<B>> {
        Recipe::with_stage(
            self.payload,
            FreshnessScopedBasis::new(self.basis.into_basis()),
        )
    }

    pub fn downgrade_to_authority_revalidation_required(
        self,
    ) -> Recipe<Admitted, T, AuthorityRevalidationRequiredBasis<B>> {
        Recipe::with_stage(
            self.payload,
            FreshnessScopedBasis::new(self.basis.into_basis()),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::artifact::Artifact;
    use crate::assumption::{
        AssumptionBasis, AuthorityRevalidationRequiredBasis, CurrentValidity, FreshnessScopedBasis,
        RebindRequiredBasis, StaleReadableBasis,
    };
    use crate::phase::PhaseMarker;
    use crate::proof::{mint_proof, CanonicalOrder};
    use crate::recipe::{Admitted, Lowered, Recipe, Resolved};

    struct ValidatedPhase;
    impl PhaseMarker for ValidatedPhase {}

    #[test]
    fn artifact_downgrade_preserves_payload_proofs_and_basis() {
        let artifact = Artifact::<ValidatedPhase, _, _, _>::with_state(
            "payload",
            mint_proof::<CanonicalOrder>(),
            FreshnessScopedBasis::<CurrentValidity, _>::new(AssumptionBasis::new(7_u8)),
        );

        let stale: Artifact<ValidatedPhase, _, _, StaleReadableBasis<u8>> =
            artifact.downgrade_to_stale_readable();
        assert_eq!(stale.payload(), &"payload");
        assert_eq!(stale.basis().basis().value(), &7_u8);
    }

    #[test]
    fn recipe_downgrade_preserves_stage_payload_and_basis() {
        let admitted = Recipe::<Admitted, _, _>::with_stage(
            "payload",
            FreshnessScopedBasis::<CurrentValidity, _>::new(AssumptionBasis::new(11_u8)),
        );

        let stale: Recipe<Admitted, _, StaleReadableBasis<u8>> =
            admitted.downgrade_to_stale_readable();
        assert_eq!(stale.payload(), &"payload");
        assert_eq!(stale.basis().basis().value(), &11_u8);
    }

    #[test]
    fn recipe_downgrade_surfaces_distinguish_rebind_and_authority_loss() {
        let resolved = Recipe::<Resolved, _, _>::with_stage(
            "payload",
            FreshnessScopedBasis::<CurrentValidity, _>::new(AssumptionBasis::new(13_u8)),
        );
        let admitted = Recipe::<Admitted, _, _>::with_stage(
            "payload",
            FreshnessScopedBasis::<CurrentValidity, _>::new(AssumptionBasis::new(17_u8)),
        );

        let rebind: Recipe<Resolved, _, RebindRequiredBasis<u8>> =
            resolved.downgrade_to_rebind_required();
        let authority: Recipe<Admitted, _, AuthorityRevalidationRequiredBasis<u8>> =
            admitted.downgrade_to_authority_revalidation_required();

        assert_eq!(rebind.basis().basis().value(), &13_u8);
        assert_eq!(authority.basis().basis().value(), &17_u8);
    }

    #[test]
    fn strong_basis_access_is_available_for_current_validity_forms() {
        let lowered = Recipe::<Lowered, _, _>::with_stage(
            "payload",
            FreshnessScopedBasis::<CurrentValidity, _>::new(AssumptionBasis::new(19_u8)),
        );

        assert_eq!(lowered.strong_basis().value(), &19_u8);
    }
}
