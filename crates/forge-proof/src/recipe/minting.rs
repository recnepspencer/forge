use crate::assumption::{AssumptionBasis, CurrentValidity, FreshnessScopedBasis};
use crate::proof::{AuthorityMarker, AuthorityWitness, CapabilityMarker, CapabilityWitness};

use super::stages::{Admitted, Lowered, Recipe, Resolved, Unresolved};

impl<T> Recipe<Unresolved, T> {
    pub fn resolve_with_authority<Auth, B>(
        self,
        basis: B,
        _authority: AuthorityWitness<Auth>,
    ) -> Recipe<Resolved, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>
    where
        Auth: AuthorityMarker,
    {
        Recipe::with_stage(
            self.payload,
            FreshnessScopedBasis::new(AssumptionBasis::new(basis)),
        )
    }
}

impl<T, B> Recipe<Resolved, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>> {
    pub fn lower_with_capability<C>(
        self,
        _capability: CapabilityWitness<C>,
    ) -> Recipe<Lowered, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>
    where
        C: CapabilityMarker,
    {
        Recipe::with_stage(self.payload, self.basis)
    }
}

impl<T, B> Recipe<Lowered, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>> {
    pub fn admit_with_authority<Auth>(
        self,
        _authority: AuthorityWitness<Auth>,
    ) -> Recipe<Admitted, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>
    where
        Auth: AuthorityMarker,
    {
        Recipe::with_stage(self.payload, self.basis)
    }
}

#[cfg(test)]
mod tests {
    use crate::proof::{
        mint_authority_witness, mint_capability_witness, AuthorityMarker, CapabilityMarker,
    };

    use super::{Recipe, Unresolved};

    struct ResolutionAuthority;
    impl AuthorityMarker for ResolutionAuthority {}

    struct LoweringCapability;
    impl CapabilityMarker for LoweringCapability {}

    struct AdmissionAuthority;
    impl AuthorityMarker for AdmissionAuthority {}

    #[test]
    fn recipe_progression_requires_explicit_stages() {
        let unresolved = Recipe::<Unresolved, _>::new("payload");
        let resolved = unresolved
            .resolve_with_authority(17_u8, mint_authority_witness::<ResolutionAuthority>());
        let lowered =
            resolved.lower_with_capability(mint_capability_witness::<LoweringCapability>());
        let admitted = lowered.admit_with_authority(mint_authority_witness::<AdmissionAuthority>());

        assert_eq!(admitted.payload(), &"payload");
        assert_eq!(admitted.strong_basis().value(), &17_u8);
    }
}
