use crate::assumption::AssumptionBasis;
use crate::proof::{AuthorityMarker, AuthorityWitness, CapabilityMarker, CapabilityWitness};

use super::stages::{Admitted, Lowered, Recipe, Resolved, Unresolved};

impl<T> Recipe<Unresolved, T> {
    pub fn resolve_with_authority<Auth, B>(
        self,
        basis: B,
        _authority: AuthorityWitness<Auth>,
    ) -> Recipe<Resolved, T, AssumptionBasis<B>>
    where
        Auth: AuthorityMarker,
    {
        Recipe::with_stage(self.payload, AssumptionBasis::new(basis))
    }
}

impl<T, A> Recipe<Resolved, T, A> {
    pub fn lower_with_capability<C>(
        self,
        _capability: CapabilityWitness<C>,
    ) -> Recipe<Lowered, T, A>
    where
        C: CapabilityMarker,
    {
        Recipe::with_stage(self.payload, self.basis)
    }
}

impl<T, A> Recipe<Lowered, T, A> {
    pub fn admit_with_authority<Auth>(
        self,
        _authority: AuthorityWitness<Auth>,
    ) -> Recipe<Admitted, T, A>
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
        assert_eq!(admitted.basis().value(), &17_u8);
    }
}
