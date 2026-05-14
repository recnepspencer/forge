use forge_proof::{
    Artifact, AssumptionBasis, AuthorityMarker, AuthorityProves, AuthorityWitness, CurrentValidity,
    FreshnessScopedBasis, Proof,
};

use super::categories::{ArtifactCategory, FoundationalBoundaryCategorySurface};
use super::roles::{AuthoritativeCurrentRole, FoundationalBoundaryRoleClaim};
use crate::boundary_artifacts::FoundationalBoundaryAuthorityAdmitted;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalAuthoritativeCurrentBoundaryPhase;
impl forge_proof::PhaseMarker for FoundationalAuthoritativeCurrentBoundaryPhase {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalBoundaryAuthorityAdmissionBasis;

type FoundationalAuthoritativeClaimInner<Surface> = Artifact<
    FoundationalAuthoritativeCurrentBoundaryPhase,
    FoundationalBoundaryRoleClaim<Surface, AuthoritativeCurrentRole>,
    Proof<FoundationalBoundaryAuthorityAdmitted, FoundationalBoundaryAuthorityAdmission>,
    FreshnessScopedBasis<
        CurrentValidity,
        AssumptionBasis<FoundationalBoundaryAuthorityAdmissionBasis>,
    >,
>;

pub struct FoundationalAuthoritativeBoundaryClaim<Surface> {
    inner: FoundationalAuthoritativeClaimInner<Surface>,
}

impl<Surface> FoundationalAuthoritativeBoundaryClaim<Surface>
where
    Surface: FoundationalBoundaryCategorySurface,
{
    fn new(inner: FoundationalAuthoritativeClaimInner<Surface>) -> Self {
        Self { inner }
    }

    pub fn claim(&self) -> &FoundationalBoundaryRoleClaim<Surface, AuthoritativeCurrentRole> {
        self.inner.payload()
    }

    pub fn proofs(
        &self,
    ) -> &Proof<FoundationalBoundaryAuthorityAdmitted, FoundationalBoundaryAuthorityAdmission> {
        self.inner.proofs()
    }

    pub fn surface(&self) -> &Surface {
        self.inner.payload().surface()
    }

    pub fn into_surface(self) -> Surface {
        let (claim, _proofs, _basis) = self.inner.into_parts().into_parts();
        claim.into_surface()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalBoundaryAuthorityAdmission(());

impl FoundationalBoundaryAuthorityAdmission {
    pub(crate) const fn milestone_4_phase_2() -> Self {
        Self(())
    }
}

impl AuthorityMarker for FoundationalBoundaryAuthorityAdmission {}
impl AuthorityProves<FoundationalBoundaryAuthorityAdmitted>
    for FoundationalBoundaryAuthorityAdmission
{
}

pub fn foundational_boundary_authority_admission(
) -> AuthorityWitness<FoundationalBoundaryAuthorityAdmission> {
    AuthorityWitness::from_authority_marker(
        FoundationalBoundaryAuthorityAdmission::milestone_4_phase_2(),
    )
}

pub fn admit_authoritative_current_boundary_surface<Surface>(
    surface: Surface,
    authority: AuthorityWitness<FoundationalBoundaryAuthorityAdmission>,
) -> FoundationalAuthoritativeBoundaryClaim<Surface>
where
    Surface: FoundationalBoundaryCategorySurface<Category = ArtifactCategory>,
{
    let proof = Proof::from_authority_witness(&authority);
    FoundationalAuthoritativeBoundaryClaim::new(Artifact::with_proofs_and_current_basis(
        FoundationalBoundaryRoleClaim::<Surface, AuthoritativeCurrentRole>::new(surface),
        proof,
        FoundationalBoundaryAuthorityAdmissionBasis,
        authority,
    ))
}
