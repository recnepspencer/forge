use super::error::{EvidenceLookupInventoryError, EvidenceLookupInventoryErrorKind};
use super::row::{EvidenceLookupAuthorityKind, EvidenceLookupQuerySurface};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupQuerySurfaceContext {
    NotQuery,
    SupportAdmission,
    SupportPinning,
    ProjectionConsumption,
    LowerRuntimeBoundaryEnvelope,
    TypedArtifactIdentity,
    ConsumerKitProof,
}

pub const fn classify_evidence_lookup_query_surface(
    authority_kind: EvidenceLookupAuthorityKind,
    context: EvidenceLookupQuerySurfaceContext,
) -> Result<EvidenceLookupQuerySurface, EvidenceLookupInventoryError> {
    match (authority_kind, context) {
        (
            EvidenceLookupAuthorityKind::QueryLookingLocalProof,
            EvidenceLookupQuerySurfaceContext::NotQuery,
        ) => Err(error(
            EvidenceLookupInventoryErrorKind::QuerySurfaceRequired,
        )),
        (
            EvidenceLookupAuthorityKind::QueryLookingLocalProof,
            EvidenceLookupQuerySurfaceContext::SupportAdmission,
        ) => Ok(EvidenceLookupQuerySurface::SupportAdmission),
        (
            EvidenceLookupAuthorityKind::QueryLookingLocalProof,
            EvidenceLookupQuerySurfaceContext::SupportPinning,
        ) => Ok(EvidenceLookupQuerySurface::SupportPinning),
        (
            EvidenceLookupAuthorityKind::QueryLookingLocalProof,
            EvidenceLookupQuerySurfaceContext::ProjectionConsumption,
        ) => Ok(EvidenceLookupQuerySurface::ProjectionConsumption),
        (
            EvidenceLookupAuthorityKind::QueryLookingLocalProof,
            EvidenceLookupQuerySurfaceContext::LowerRuntimeBoundaryEnvelope,
        ) => Ok(EvidenceLookupQuerySurface::LowerRuntimeBoundaryEnvelope),
        (
            EvidenceLookupAuthorityKind::QueryLookingLocalProof,
            EvidenceLookupQuerySurfaceContext::TypedArtifactIdentity,
        ) => Ok(EvidenceLookupQuerySurface::TypedArtifactIdentity),
        (
            EvidenceLookupAuthorityKind::QueryLookingLocalProof,
            EvidenceLookupQuerySurfaceContext::ConsumerKitProof,
        ) => Ok(EvidenceLookupQuerySurface::ConsumerKitProof),
        (_, EvidenceLookupQuerySurfaceContext::NotQuery) => {
            Ok(EvidenceLookupQuerySurface::NotQuery)
        }
        (_, _) => Err(error(
            EvidenceLookupInventoryErrorKind::QuerySurfaceCannotMintLookupAuthority,
        )),
    }
}

const fn error(kind: EvidenceLookupInventoryErrorKind) -> EvidenceLookupInventoryError {
    EvidenceLookupInventoryError::new(kind)
}
