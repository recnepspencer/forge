use worth_foundational::facade::{
    admit_requested_foundational_profile, attach_boundary_profiled_artifact,
    foundational_profile_progression_authority, request_foundational_profile_set,
    AdmittedFoundationalProfileArtifact, BoundaryProfiledArtifact,
    FoundationalBoundaryArtifactSurface, FoundationalBoundaryReceiptSurface,
    FoundationalProfileNarrowingRecord, FoundationalProfileSet,
};
use worth_proof::TransitionOutcome;

use super::{
    WorthQueryApplicationAuthorizationPublicationDenial,
    WorthQueryPublishedApplicationAuthorizationKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationAuthorizationProfileStage {
    profile: FoundationalProfileSet,
    narrowing_from_previous: Option<FoundationalProfileNarrowingRecord>,
}

impl WorthQueryApplicationAuthorizationProfileStage {
    pub const fn new(
        profile: FoundationalProfileSet,
        narrowing_from_previous: Option<FoundationalProfileNarrowingRecord>,
    ) -> Self {
        Self {
            profile,
            narrowing_from_previous,
        }
    }

    pub const fn profile(&self) -> FoundationalProfileSet {
        self.profile
    }

    pub const fn narrowing_from_previous(&self) -> Option<FoundationalProfileNarrowingRecord> {
        self.narrowing_from_previous
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationAuthorizationPublicationProfile {
    requested: FoundationalProfileSet,
    admitted: FoundationalProfileSet,
    materialized: FoundationalProfileSet,
    requested_to_admitted: Option<FoundationalProfileNarrowingRecord>,
    admitted_to_materialized: Option<FoundationalProfileNarrowingRecord>,
}

impl WorthQueryApplicationAuthorizationPublicationProfile {
    pub const fn exact(profile: FoundationalProfileSet) -> Self {
        Self {
            requested: profile,
            admitted: profile,
            materialized: profile,
            requested_to_admitted: None,
            admitted_to_materialized: None,
        }
    }

    pub const fn with_progression(
        requested: FoundationalProfileSet,
        admitted: WorthQueryApplicationAuthorizationProfileStage,
        materialized: WorthQueryApplicationAuthorizationProfileStage,
    ) -> Self {
        Self {
            requested,
            admitted: admitted.profile,
            materialized: materialized.profile,
            requested_to_admitted: admitted.narrowing_from_previous,
            admitted_to_materialized: materialized.narrowing_from_previous,
        }
    }

    pub const fn requested(&self) -> FoundationalProfileSet {
        self.requested
    }

    pub const fn admitted(&self) -> FoundationalProfileSet {
        self.admitted
    }

    pub const fn materialized(&self) -> FoundationalProfileSet {
        self.materialized
    }
}

pub(super) fn profile_boundary(
    kind: WorthQueryPublishedApplicationAuthorizationKind,
    attested_effect_count: usize,
    profile: WorthQueryApplicationAuthorizationPublicationProfile,
) -> Result<
    BoundaryProfiledArtifact<FoundationalBoundaryReceiptSurface>,
    WorthQueryApplicationAuthorizationPublicationDenial,
> {
    let boundary =
        FoundationalBoundaryReceiptSurface::new(kind.completed_boundary(), attested_effect_count)
            .map_err(WorthQueryApplicationAuthorizationPublicationDenial::BoundaryCategory)?;
    match attach_boundary_profiled_artifact(
        admit_profile(profile)?,
        profile.materialized,
        profile.admitted_to_materialized,
        boundary,
        foundational_profile_progression_authority(),
    ) {
        TransitionOutcome::Success(boundary) => Ok(boundary),
        TransitionOutcome::Denied(denial) => {
            Err(WorthQueryApplicationAuthorizationPublicationDenial::ProfileMaterialization(denial))
        }
        _ => unreachable!("Foundational profile materialization has no nonterminal outcome"),
    }
}

pub(in crate::application_authorization) fn profile_boundary_artifact<T>(
    artifact: FoundationalBoundaryArtifactSurface<T>,
    profile: WorthQueryApplicationAuthorizationPublicationProfile,
) -> Result<
    BoundaryProfiledArtifact<FoundationalBoundaryArtifactSurface<T>>,
    WorthQueryApplicationAuthorizationPublicationDenial,
> {
    match attach_boundary_profiled_artifact(
        admit_profile(profile)?,
        profile.materialized,
        profile.admitted_to_materialized,
        artifact,
        foundational_profile_progression_authority(),
    ) {
        TransitionOutcome::Success(boundary) => Ok(boundary),
        TransitionOutcome::Denied(denial) => {
            Err(WorthQueryApplicationAuthorizationPublicationDenial::ProfileMaterialization(denial))
        }
        _ => unreachable!("Foundational profile materialization has no nonterminal outcome"),
    }
}

pub(super) fn admit_profile(
    profile: WorthQueryApplicationAuthorizationPublicationProfile,
) -> Result<AdmittedFoundationalProfileArtifact, WorthQueryApplicationAuthorizationPublicationDenial>
{
    match admit_requested_foundational_profile(
        request_foundational_profile_set(profile.requested),
        profile.admitted,
        profile.requested_to_admitted,
        foundational_profile_progression_authority(),
    ) {
        TransitionOutcome::Success(admitted) => Ok(admitted),
        TransitionOutcome::Denied(denial) => {
            Err(WorthQueryApplicationAuthorizationPublicationDenial::ProfileAdmission(denial))
        }
        _ => unreachable!("Foundational profile admission has no nonterminal outcome"),
    }
}
