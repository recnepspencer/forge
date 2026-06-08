use forge_foundational::{
    admit_authoritative_current_boundary_surface, admit_current_basis_boundary_artifact,
    foundational_boundary_authority_admission, foundational_boundary_current_basis_authority,
    materialize_authoritative_boundary_surface, CanonicalizationRuleVersion,
    FoundationalBoundaryArtifactSurface, FoundationalBoundaryMaterializationSeam,
    FoundationalBoundaryMaterializationSource, MaterializedFoundationalProfileSet,
};
use forge_proof::TransitionOutcome;

use crate::history::data::{
    RelationalFoundationalCurrentMergeBranchBasisArtifact, RelationalMergeBranchBasis,
    RelationalMergeBranchBasisFoundationalLoweringDenial,
};

use super::HistoryAccess;

impl<'runtime> HistoryAccess<'runtime> {
    pub fn lower_merge_branch_basis_to_foundational_current_basis(
        &self,
        basis: &RelationalMergeBranchBasis,
        version: CanonicalizationRuleVersion,
        profile: MaterializedFoundationalProfileSet,
    ) -> Result<
        RelationalFoundationalCurrentMergeBranchBasisArtifact,
        RelationalMergeBranchBasisFoundationalLoweringDenial,
    > {
        let current_basis = self
            .resolve_merge_branch_basis(basis.source_branch(), basis.target_branch())
            .map_err(
                RelationalMergeBranchBasisFoundationalLoweringDenial::CurrentBasisUnavailable,
            )?;
        if current_basis != *basis {
            return Err(
                RelationalMergeBranchBasisFoundationalLoweringDenial::CurrentBasisDrift {
                    retained_digest: basis.basis_digest(),
                    current_digest: current_basis.basis_digest(),
                },
            );
        }

        let claim = admit_authoritative_current_boundary_surface(
            FoundationalBoundaryArtifactSurface::new(basis.clone(), 0),
            foundational_boundary_authority_admission(),
        );
        let materialized = materialize_authoritative_boundary_surface(
            claim,
            FoundationalBoundaryMaterializationSource::NativeAuthority,
            FoundationalBoundaryMaterializationSeam::BoundaryExchange,
            profile,
        )
        .map_err(RelationalMergeBranchBasisFoundationalLoweringDenial::BoundaryMaterialization)?;

        match admit_current_basis_boundary_artifact(
            version,
            materialized,
            foundational_boundary_current_basis_authority(),
        ) {
            TransitionOutcome::Success(artifact) => Ok(
                RelationalFoundationalCurrentMergeBranchBasisArtifact::new(artifact),
            ),
            TransitionOutcome::Denied(denial) => {
                Err(RelationalMergeBranchBasisFoundationalLoweringDenial::CanonicalBasis(denial))
            }
            TransitionOutcome::Deferred(_)
            | TransitionOutcome::Stale(_)
            | TransitionOutcome::RebindRequired(_)
            | TransitionOutcome::Failed(_) => {
                unreachable!("foundational current-basis admission only denies on lowering")
            }
        }
    }
}
