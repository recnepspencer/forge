use forge_foundational::{
    bridge_current_basis_boundary_artifact_trust_boundary,
    readmit_current_basis_boundary_artifact_after_boundary,
    BoundaryBridgedCurrentBasisBoundaryArtifact, CanonicalBasisReadyArtifact,
    CurrentBasisBoundaryArtifact, FoundationalBoundaryArtifactSurface,
    FoundationalBoundaryCurrentBasisAuthority, FoundationalBoundaryCurrentBasisCertified,
    FoundationalBoundaryCurrentBasisReadmissionAuthority, FoundationalMaterializedBoundaryArtifact,
};
use forge_proof::{AuthorityWitness, Proof};

use super::{RelationalMergeBranchBasis, RelationalMergeBranchBasisDenial};

type RelationalMergeBranchBasisBoundarySurface =
    FoundationalBoundaryArtifactSurface<RelationalMergeBranchBasis>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationalMergeBranchBasisFoundationalLoweringDenial {
    CurrentBasisUnavailable(RelationalMergeBranchBasisDenial),
    CurrentBasisDrift {
        retained_digest: String,
        current_digest: String,
    },
    BoundaryMaterialization(forge_foundational::FoundationalBoundaryMaterializationDenial),
    CanonicalBasis(forge_foundational::CanonicalBasisConstructionDenial),
}

pub struct RelationalFoundationalCurrentMergeBranchBasisArtifact {
    pub(crate) inner: CurrentBasisBoundaryArtifact<RelationalMergeBranchBasisBoundarySurface>,
}

impl RelationalFoundationalCurrentMergeBranchBasisArtifact {
    pub(crate) fn new(
        inner: CurrentBasisBoundaryArtifact<RelationalMergeBranchBasisBoundarySurface>,
    ) -> Self {
        Self { inner }
    }

    pub fn basis(&self) -> &RelationalMergeBranchBasis {
        self.inner.materialized().surface().payload()
    }

    pub fn materialized(
        &self,
    ) -> &FoundationalMaterializedBoundaryArtifact<RelationalMergeBranchBasisBoundarySurface> {
        self.inner.materialized()
    }

    pub fn strong_basis(&self) -> &CanonicalBasisReadyArtifact {
        self.inner.strong_basis()
    }

    pub fn proofs(
        &self,
    ) -> &Proof<FoundationalBoundaryCurrentBasisCertified, FoundationalBoundaryCurrentBasisAuthority>
    {
        self.inner.proofs()
    }

    pub fn bridge_trust_boundary(
        self,
    ) -> BoundaryBridgedRelationalFoundationalCurrentMergeBranchBasisArtifact {
        BoundaryBridgedRelationalFoundationalCurrentMergeBranchBasisArtifact::new(
            bridge_current_basis_boundary_artifact_trust_boundary(self.inner),
        )
    }
}

pub struct BoundaryBridgedRelationalFoundationalCurrentMergeBranchBasisArtifact {
    inner: BoundaryBridgedCurrentBasisBoundaryArtifact<RelationalMergeBranchBasisBoundarySurface>,
}

impl BoundaryBridgedRelationalFoundationalCurrentMergeBranchBasisArtifact {
    fn new(
        inner: BoundaryBridgedCurrentBasisBoundaryArtifact<
            RelationalMergeBranchBasisBoundarySurface,
        >,
    ) -> Self {
        Self { inner }
    }

    pub fn basis(&self) -> &RelationalMergeBranchBasis {
        self.inner.materialized().surface().payload()
    }

    pub fn materialized(
        &self,
    ) -> &FoundationalMaterializedBoundaryArtifact<RelationalMergeBranchBasisBoundarySurface> {
        self.inner.materialized()
    }

    pub fn readmit_with_authority(
        self,
        basis: CanonicalBasisReadyArtifact,
        authority: AuthorityWitness<FoundationalBoundaryCurrentBasisReadmissionAuthority>,
    ) -> RelationalFoundationalCurrentMergeBranchBasisArtifact {
        RelationalFoundationalCurrentMergeBranchBasisArtifact::new(
            readmit_current_basis_boundary_artifact_after_boundary(self.inner, basis, authority),
        )
    }
}
