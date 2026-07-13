use super::{ProjectionSourceBasisAuthority, ProjectionSourceBasisAuthorityKind};

impl ProjectionSourceBasisAuthority {
    pub(crate) fn binds_resolved_basis(&self, basis: &crate::basis::ResolvedSnapshotBasis) -> bool {
        match &self.kind {
            ProjectionSourceBasisAuthorityKind::RuntimeSnapshot(identity) => {
                identity.evidence_identity() == *basis.identity().snapshot_identity()
            }
            ProjectionSourceBasisAuthorityKind::QueryContext { basis_digest, .. } => {
                basis_digest == basis.proof().digest().as_str()
            }
            ProjectionSourceBasisAuthorityKind::Certification(_) => false,
        }
    }
}
