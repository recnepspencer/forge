use std::marker::PhantomData;

pub trait AuthorityMarker: 'static {}

pub trait CapabilityMarker: 'static {}

#[derive(Debug, PartialEq, Eq)]
pub struct AuthorityWitness<A>(PhantomData<A>)
where
    A: AuthorityMarker;

impl<A> AuthorityWitness<A>
where
    A: AuthorityMarker,
{
    #[allow(dead_code)]
    pub(crate) fn mint() -> Self {
        Self(PhantomData)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CapabilityWitness<C>(PhantomData<C>)
where
    C: CapabilityMarker;

impl<C> CapabilityWitness<C>
where
    C: CapabilityMarker,
{
    #[allow(dead_code)]
    pub(crate) fn mint() -> Self {
        Self(PhantomData)
    }
}

#[cfg(test)]
pub(crate) fn mint_authority_witness<A>() -> AuthorityWitness<A>
where
    A: AuthorityMarker,
{
    AuthorityWitness::mint()
}

#[cfg(test)]
pub(crate) fn mint_capability_witness<C>() -> CapabilityWitness<C>
where
    C: CapabilityMarker,
{
    CapabilityWitness::mint()
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use super::{
        mint_authority_witness, mint_capability_witness, AuthorityMarker, AuthorityWitness,
        CapabilityMarker, CapabilityWitness,
    };

    struct DeploymentAuthority;
    impl AuthorityMarker for DeploymentAuthority {}

    struct CanonicalizationCapability;
    impl CapabilityMarker for CanonicalizationCapability {}

    #[test]
    fn witnesses_are_zero_sized_and_crate_internal() {
        let authority = mint_authority_witness::<DeploymentAuthority>();
        let capability = mint_capability_witness::<CanonicalizationCapability>();
        let second_authority = AuthorityWitness::<DeploymentAuthority>::mint();
        let second_capability = CapabilityWitness::<CanonicalizationCapability>::mint();

        let _ = authority;
        let _ = capability;
        let _ = second_authority;
        let _ = second_capability;
        assert_eq!(size_of::<AuthorityWitness<DeploymentAuthority>>(), 0);
        assert_eq!(
            size_of::<CapabilityWitness<CanonicalizationCapability>>(),
            0
        );
    }
}
