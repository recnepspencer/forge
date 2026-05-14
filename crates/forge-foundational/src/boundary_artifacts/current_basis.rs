use forge_proof::{
    Artifact, AssumptionBasis, AuthorityMarker, AuthorityProves, AuthorityWitness,
    BoundaryBridgedAuthorityRevalidationRequiredBasis, CurrentValidity, FreshnessScopedBasis,
    Proof, TransitionOutcome,
};

use super::basis::{
    prepare_materialized_boundary_artifact_for_canonical_basis,
    prepare_materialized_boundary_bundle_for_canonical_basis,
};
use super::materialization::{
    FoundationalBoundaryMaterializationBundle, FoundationalMaterializedBoundaryArtifact,
};
use crate::boundary_artifacts::FoundationalBoundaryCurrentBasisCertified;
use crate::canonicalization::{
    CanonicalBasisConstructionDenial, CanonicalBasisReadyArtifact, CanonicalizationRuleVersion,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalBoundaryCurrentBasisProofLane {
    CurrentBasisArtifactWithBoundaryReadmission,
}

pub const fn foundational_boundary_current_basis_proof_lane(
) -> FoundationalBoundaryCurrentBasisProofLane {
    FoundationalBoundaryCurrentBasisProofLane::CurrentBasisArtifactWithBoundaryReadmission
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentBasisBoundaryArtifactPhase;
impl forge_proof::PhaseMarker for CurrentBasisBoundaryArtifactPhase {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalBoundaryCurrentBasisAuthority(());

impl FoundationalBoundaryCurrentBasisAuthority {
    pub(crate) const fn milestone_4_phase_4_5() -> Self {
        Self(())
    }
}

impl AuthorityMarker for FoundationalBoundaryCurrentBasisAuthority {}
impl AuthorityProves<FoundationalBoundaryCurrentBasisCertified>
    for FoundationalBoundaryCurrentBasisAuthority
{
}

pub fn foundational_boundary_current_basis_authority(
) -> AuthorityWitness<FoundationalBoundaryCurrentBasisAuthority> {
    AuthorityWitness::from_authority_marker(
        FoundationalBoundaryCurrentBasisAuthority::milestone_4_phase_4_5(),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalBoundaryCurrentBasisReadmissionAuthority(());

impl FoundationalBoundaryCurrentBasisReadmissionAuthority {
    pub(crate) const fn milestone_4_phase_4_5_boundary() -> Self {
        Self(())
    }
}

impl AuthorityMarker for FoundationalBoundaryCurrentBasisReadmissionAuthority {}

pub fn foundational_boundary_current_basis_readmission_authority(
) -> AuthorityWitness<FoundationalBoundaryCurrentBasisReadmissionAuthority> {
    AuthorityWitness::from_authority_marker(
        FoundationalBoundaryCurrentBasisReadmissionAuthority::milestone_4_phase_4_5_boundary(),
    )
}

type CurrentBasisBoundaryArtifactInner<Surface> = Artifact<
    CurrentBasisBoundaryArtifactPhase,
    FoundationalMaterializedBoundaryArtifact<Surface>,
    Proof<FoundationalBoundaryCurrentBasisCertified, FoundationalBoundaryCurrentBasisAuthority>,
    FreshnessScopedBasis<CurrentValidity, AssumptionBasis<CanonicalBasisReadyArtifact>>,
>;

type BoundaryBridgedCurrentBasisBoundaryArtifactInner<Surface> = Artifact<
    CurrentBasisBoundaryArtifactPhase,
    FoundationalMaterializedBoundaryArtifact<Surface>,
    Proof<FoundationalBoundaryCurrentBasisCertified, FoundationalBoundaryCurrentBasisAuthority>,
    BoundaryBridgedAuthorityRevalidationRequiredBasis<CanonicalBasisReadyArtifact>,
>;

type CurrentBasisBoundaryBundleInner<Primary, ReportRow> = Artifact<
    CurrentBasisBoundaryArtifactPhase,
    FoundationalBoundaryMaterializationBundle<Primary, ReportRow>,
    Proof<FoundationalBoundaryCurrentBasisCertified, FoundationalBoundaryCurrentBasisAuthority>,
    FreshnessScopedBasis<CurrentValidity, AssumptionBasis<CanonicalBasisReadyArtifact>>,
>;

type BoundaryBridgedCurrentBasisBoundaryBundleInner<Primary, ReportRow> = Artifact<
    CurrentBasisBoundaryArtifactPhase,
    FoundationalBoundaryMaterializationBundle<Primary, ReportRow>,
    Proof<FoundationalBoundaryCurrentBasisCertified, FoundationalBoundaryCurrentBasisAuthority>,
    BoundaryBridgedAuthorityRevalidationRequiredBasis<CanonicalBasisReadyArtifact>,
>;

pub struct CurrentBasisBoundaryArtifact<Surface> {
    inner: CurrentBasisBoundaryArtifactInner<Surface>,
}

impl<Surface> CurrentBasisBoundaryArtifact<Surface> {
    fn new(inner: CurrentBasisBoundaryArtifactInner<Surface>) -> Self {
        Self { inner }
    }

    pub fn materialized(&self) -> &FoundationalMaterializedBoundaryArtifact<Surface> {
        self.inner.payload()
    }

    pub fn proofs(
        &self,
    ) -> &Proof<FoundationalBoundaryCurrentBasisCertified, FoundationalBoundaryCurrentBasisAuthority>
    {
        self.inner.proofs()
    }

    pub fn strong_basis(&self) -> &CanonicalBasisReadyArtifact {
        self.inner.strong_basis().value()
    }
}

pub struct BoundaryBridgedCurrentBasisBoundaryArtifact<Surface> {
    inner: BoundaryBridgedCurrentBasisBoundaryArtifactInner<Surface>,
}

impl<Surface> BoundaryBridgedCurrentBasisBoundaryArtifact<Surface> {
    fn new(inner: BoundaryBridgedCurrentBasisBoundaryArtifactInner<Surface>) -> Self {
        Self { inner }
    }

    pub fn materialized(&self) -> &FoundationalMaterializedBoundaryArtifact<Surface> {
        self.inner.payload()
    }
}

pub struct CurrentBasisBoundaryBundle<Primary, ReportRow = ()> {
    inner: CurrentBasisBoundaryBundleInner<Primary, ReportRow>,
}

impl<Primary, ReportRow> CurrentBasisBoundaryBundle<Primary, ReportRow> {
    fn new(inner: CurrentBasisBoundaryBundleInner<Primary, ReportRow>) -> Self {
        Self { inner }
    }

    pub fn bundle(&self) -> &FoundationalBoundaryMaterializationBundle<Primary, ReportRow> {
        self.inner.payload()
    }

    pub fn proofs(
        &self,
    ) -> &Proof<FoundationalBoundaryCurrentBasisCertified, FoundationalBoundaryCurrentBasisAuthority>
    {
        self.inner.proofs()
    }

    pub fn strong_basis(&self) -> &CanonicalBasisReadyArtifact {
        self.inner.strong_basis().value()
    }
}

pub struct BoundaryBridgedCurrentBasisBoundaryBundle<Primary, ReportRow = ()> {
    inner: BoundaryBridgedCurrentBasisBoundaryBundleInner<Primary, ReportRow>,
}

impl<Primary, ReportRow> BoundaryBridgedCurrentBasisBoundaryBundle<Primary, ReportRow> {
    fn new(inner: BoundaryBridgedCurrentBasisBoundaryBundleInner<Primary, ReportRow>) -> Self {
        Self { inner }
    }

    pub fn bundle(&self) -> &FoundationalBoundaryMaterializationBundle<Primary, ReportRow> {
        self.inner.payload()
    }
}

pub fn admit_current_basis_boundary_artifact<Surface>(
    version: CanonicalizationRuleVersion,
    artifact: FoundationalMaterializedBoundaryArtifact<Surface>,
    authority: AuthorityWitness<FoundationalBoundaryCurrentBasisAuthority>,
) -> TransitionOutcome<CurrentBasisBoundaryArtifact<Surface>, CanonicalBasisConstructionDenial> {
    let basis = match prepare_materialized_boundary_artifact_for_canonical_basis(version, &artifact)
    {
        TransitionOutcome::Success(ready) => ready,
        TransitionOutcome::Denied(denial) => return TransitionOutcome::denied(denial),
        TransitionOutcome::Deferred(_)
        | TransitionOutcome::Stale(_)
        | TransitionOutcome::RebindRequired(_)
        | TransitionOutcome::Failed(_) => {
            unreachable!("boundary basis preparation uses only denied")
        }
    };

    let proof = Proof::from_authority_witness(&authority);
    TransitionOutcome::success(CurrentBasisBoundaryArtifact::new(
        Artifact::with_proofs_and_current_basis(artifact, proof, basis, authority),
    ))
}

pub fn admit_current_basis_boundary_bundle<Primary, ReportRow>(
    version: CanonicalizationRuleVersion,
    bundle: FoundationalBoundaryMaterializationBundle<Primary, ReportRow>,
    authority: AuthorityWitness<FoundationalBoundaryCurrentBasisAuthority>,
) -> TransitionOutcome<
    CurrentBasisBoundaryBundle<Primary, ReportRow>,
    CanonicalBasisConstructionDenial,
> {
    let basis = match prepare_materialized_boundary_bundle_for_canonical_basis(version, &bundle) {
        TransitionOutcome::Success(ready) => ready,
        TransitionOutcome::Denied(denial) => return TransitionOutcome::denied(denial),
        TransitionOutcome::Deferred(_)
        | TransitionOutcome::Stale(_)
        | TransitionOutcome::RebindRequired(_)
        | TransitionOutcome::Failed(_) => {
            unreachable!("boundary basis preparation uses only denied")
        }
    };

    let proof = Proof::from_authority_witness(&authority);
    TransitionOutcome::success(CurrentBasisBoundaryBundle::new(
        Artifact::with_proofs_and_current_basis(bundle, proof, basis, authority),
    ))
}

pub fn bridge_current_basis_boundary_artifact_trust_boundary<Surface>(
    artifact: CurrentBasisBoundaryArtifact<Surface>,
) -> BoundaryBridgedCurrentBasisBoundaryArtifact<Surface> {
    BoundaryBridgedCurrentBasisBoundaryArtifact::new(artifact.inner.bridge_trust_boundary())
}

pub fn readmit_current_basis_boundary_artifact_after_boundary<Surface>(
    artifact: BoundaryBridgedCurrentBasisBoundaryArtifact<Surface>,
    basis: CanonicalBasisReadyArtifact,
    authority: AuthorityWitness<FoundationalBoundaryCurrentBasisReadmissionAuthority>,
) -> CurrentBasisBoundaryArtifact<Surface> {
    CurrentBasisBoundaryArtifact::new(artifact.inner.readmit_with_authority(basis, authority))
}

pub fn bridge_current_basis_boundary_bundle_trust_boundary<Primary, ReportRow>(
    bundle: CurrentBasisBoundaryBundle<Primary, ReportRow>,
) -> BoundaryBridgedCurrentBasisBoundaryBundle<Primary, ReportRow> {
    BoundaryBridgedCurrentBasisBoundaryBundle::new(bundle.inner.bridge_trust_boundary())
}

pub fn readmit_current_basis_boundary_bundle_after_boundary<Primary, ReportRow>(
    bundle: BoundaryBridgedCurrentBasisBoundaryBundle<Primary, ReportRow>,
    basis: CanonicalBasisReadyArtifact,
    authority: AuthorityWitness<FoundationalBoundaryCurrentBasisReadmissionAuthority>,
) -> CurrentBasisBoundaryBundle<Primary, ReportRow> {
    CurrentBasisBoundaryBundle::new(bundle.inner.readmit_with_authority(basis, authority))
}
