use crate::assumption::{
    BoundaryBridgedAuthorityRevalidationRequiredBasis, BoundaryBridgedRebindRequiredBasis,
    BoundaryBridgedStaleReadableBasis, CurrentValidity, FreshnessScopedBasis,
};
use crate::proof::{AuthorityMarker, AuthorityWitness};
use crate::recipe::{Admitted, Lowered, Recipe, Resolved};

pub trait ResolvedBridgedRecipeDxExt<T, B> {
    fn rebind_with<Auth, NextB>(
        self,
        authority: AuthorityWitness<Auth>,
        basis: NextB,
    ) -> Recipe<
        Resolved,
        T,
        FreshnessScopedBasis<CurrentValidity, crate::assumption::AssumptionBasis<NextB>>,
    >
    where
        Auth: AuthorityMarker;
}

impl<T, B> ResolvedBridgedRecipeDxExt<T, B>
    for Recipe<Resolved, T, BoundaryBridgedRebindRequiredBasis<B>>
{
    fn rebind_with<Auth, NextB>(
        self,
        authority: AuthorityWitness<Auth>,
        basis: NextB,
    ) -> Recipe<
        Resolved,
        T,
        FreshnessScopedBasis<CurrentValidity, crate::assumption::AssumptionBasis<NextB>>,
    >
    where
        Auth: AuthorityMarker,
    {
        self.rebind_with_authority(basis, authority)
    }
}

pub trait LoweredBridgedRecipeDxExt<T, B> {
    fn readmit_with<Auth, NextB>(
        self,
        authority: AuthorityWitness<Auth>,
        basis: NextB,
    ) -> Recipe<
        Lowered,
        T,
        FreshnessScopedBasis<CurrentValidity, crate::assumption::AssumptionBasis<NextB>>,
    >
    where
        Auth: AuthorityMarker;
}

impl<T, B> LoweredBridgedRecipeDxExt<T, B>
    for Recipe<Lowered, T, BoundaryBridgedStaleReadableBasis<B>>
{
    fn readmit_with<Auth, NextB>(
        self,
        authority: AuthorityWitness<Auth>,
        basis: NextB,
    ) -> Recipe<
        Lowered,
        T,
        FreshnessScopedBasis<CurrentValidity, crate::assumption::AssumptionBasis<NextB>>,
    >
    where
        Auth: AuthorityMarker,
    {
        self.readmit_with_authority(basis, authority)
    }
}

pub trait AdmittedBridgedRecipeDxExt<T, B> {
    fn readmit_with<Auth, NextB>(
        self,
        authority: AuthorityWitness<Auth>,
        basis: NextB,
    ) -> Recipe<
        Admitted,
        T,
        FreshnessScopedBasis<CurrentValidity, crate::assumption::AssumptionBasis<NextB>>,
    >
    where
        Auth: AuthorityMarker;
}

impl<T, B> AdmittedBridgedRecipeDxExt<T, B>
    for Recipe<Admitted, T, BoundaryBridgedAuthorityRevalidationRequiredBasis<B>>
{
    fn readmit_with<Auth, NextB>(
        self,
        authority: AuthorityWitness<Auth>,
        basis: NextB,
    ) -> Recipe<
        Admitted,
        T,
        FreshnessScopedBasis<CurrentValidity, crate::assumption::AssumptionBasis<NextB>>,
    >
    where
        Auth: AuthorityMarker,
    {
        self.readmit_with_authority(basis, authority)
    }
}

#[cfg(test)]
mod tests {
    use crate::assumption::{AssumptionBasis, CurrentValidity, FreshnessScopedBasis};
    use crate::proof::{mint_authority_witness, AuthorityMarker};
    use crate::recipe::{Admitted, Lowered, Recipe, Resolved};

    use super::{
        AdmittedBridgedRecipeDxExt, LoweredBridgedRecipeDxExt, ResolvedBridgedRecipeDxExt,
    };

    struct ReadmissionAuthority;
    impl AuthorityMarker for ReadmissionAuthority {}

    #[test]
    fn pleasant_boundary_verbs_match_raw_rebind_and_readmission_posture() {
        let resolved = Recipe::<Resolved, _, _>::with_stage(
            "resolved",
            FreshnessScopedBasis::<CurrentValidity, _>::new(AssumptionBasis::new(23_u8)),
        );
        let lowered = Recipe::<Lowered, _, _>::with_stage(
            "lowered",
            FreshnessScopedBasis::<CurrentValidity, _>::new(AssumptionBasis::new(27_u8)),
        );
        let admitted = Recipe::<Admitted, _, _>::with_stage(
            "admitted",
            FreshnessScopedBasis::<CurrentValidity, _>::new(AssumptionBasis::new(29_u8)),
        );

        let pleasant_rebound = resolved
            .bridge_trust_boundary()
            .rebind_with(mint_authority_witness::<ReadmissionAuthority>(), 31_u16);
        let pleasant_readmitted_lowered = lowered
            .bridge_trust_boundary()
            .readmit_with(mint_authority_witness::<ReadmissionAuthority>(), 35_u16);
        let pleasant_readmitted_admitted = admitted
            .bridge_trust_boundary()
            .readmit_with(mint_authority_witness::<ReadmissionAuthority>(), 37_u16);

        assert_eq!(pleasant_rebound.strong_basis().value(), &31_u16);
        assert_eq!(pleasant_readmitted_lowered.strong_basis().value(), &35_u16);
        assert_eq!(pleasant_readmitted_admitted.strong_basis().value(), &37_u16);
    }
}
