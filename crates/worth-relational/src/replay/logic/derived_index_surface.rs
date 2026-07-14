use crate::indexes::data::DerivedIndexArtifacts;
use crate::replay::data::{
    digest_derived_index_summary, digest_derived_index_surface, CanonicalCommitEnvelope,
    ReplayObservableSurface, ReplaySurfaceAuthorityKind, ReplaySurfaceComparisonBasis,
    VerifiedReplaySurfaceDigest,
};

pub(super) fn derived_index_surface_is_promised(
    original: &CanonicalCommitEnvelope,
    recovered: Option<&CanonicalCommitEnvelope>,
) -> bool {
    !original.derived_index_artifacts().is_empty()
        || recovered.is_some_and(|envelope| !envelope.derived_index_artifacts().is_empty())
}

pub(super) fn surface_basis_for_derived_index_artifacts(
    artifacts: &DerivedIndexArtifacts,
) -> ReplaySurfaceComparisonBasis {
    ReplaySurfaceComparisonBasis::new(
        ReplaySurfaceAuthorityKind::DerivedIndexes,
        Some(VerifiedReplaySurfaceDigest::from_digest(
            ReplaySurfaceAuthorityKind::DerivedIndexes,
            digest_derived_index_surface(artifacts),
        )),
        Some(digest_derived_index_summary(artifacts)),
    )
}

pub(super) const DERIVED_INDEX_SURFACE: ReplayObservableSurface =
    ReplayObservableSurface::DerivedIndexes;
