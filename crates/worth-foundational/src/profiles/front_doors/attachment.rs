use super::super::{
    attach_boundary_profiled_artifact, attach_proof_bearing_profiled_artifact,
    attach_support_profiled_artifact, foundational_profile_progression_authority,
    AdmittedFoundationalProfileArtifact, BoundaryProfiledArtifact,
    FoundationalProfileAttachmentOutcome, FoundationalProfileNarrowingRecord,
    FoundationalProfileProgressionOutcome, FoundationalProfileSet,
    MaterializedFoundationalProfileSet, ProofBearingProfiledArtifact, SupportProfiledArtifact,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FoundationalProfileAttachmentFrontDoor;

impl FoundationalProfileAttachmentFrontDoor {
    pub fn to_boundary_artifact<T>(
        self,
        admitted: AdmittedFoundationalProfileArtifact,
        materialized: FoundationalProfileSet,
        narrowing: Option<FoundationalProfileNarrowingRecord>,
        payload: T,
    ) -> FoundationalProfileProgressionOutcome<BoundaryProfiledArtifact<T>> {
        attach_boundary_profiled_artifact(
            admitted,
            materialized,
            narrowing,
            payload,
            foundational_profile_progression_authority(),
        )
    }

    pub fn to_support_artifact<T>(
        self,
        admitted: AdmittedFoundationalProfileArtifact,
        materialized: FoundationalProfileSet,
        narrowing: Option<FoundationalProfileNarrowingRecord>,
        payload: T,
    ) -> FoundationalProfileAttachmentOutcome<SupportProfiledArtifact<T>> {
        attach_support_profiled_artifact(
            admitted,
            materialized,
            narrowing,
            payload,
            foundational_profile_progression_authority(),
        )
    }

    pub fn to_proof_bearing_artifact<T>(
        self,
        admitted: AdmittedFoundationalProfileArtifact,
        materialized: FoundationalProfileSet,
        narrowing: Option<FoundationalProfileNarrowingRecord>,
        payload: T,
    ) -> FoundationalProfileAttachmentOutcome<ProofBearingProfiledArtifact<T>> {
        attach_proof_bearing_profiled_artifact(
            admitted,
            materialized,
            narrowing,
            payload,
            foundational_profile_progression_authority(),
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MaterializedBoundaryArtifactStep<'a, T> {
    profile: &'a MaterializedFoundationalProfileSet,
    _payload: &'a T,
}

impl<'a, T> MaterializedBoundaryArtifactStep<'a, T> {
    pub fn new(artifact: &'a BoundaryProfiledArtifact<T>) -> Self {
        Self {
            profile: artifact.payload().profile(),
            _payload: artifact.payload().payload(),
        }
    }

    pub const fn profile(&self) -> &'a MaterializedFoundationalProfileSet {
        self.profile
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MaterializedSupportArtifactStep<'a, T> {
    profile: &'a MaterializedFoundationalProfileSet,
    _payload: &'a T,
}

impl<'a, T> MaterializedSupportArtifactStep<'a, T> {
    pub fn new(artifact: &'a SupportProfiledArtifact<T>) -> Self {
        Self {
            profile: artifact.payload().profile(),
            _payload: artifact.payload().payload(),
        }
    }

    pub const fn profile(&self) -> &'a MaterializedFoundationalProfileSet {
        self.profile
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MaterializedProofBearingArtifactStep<'a, T> {
    profile: &'a MaterializedFoundationalProfileSet,
    _payload: &'a T,
}

impl<'a, T> MaterializedProofBearingArtifactStep<'a, T> {
    pub fn new(artifact: &'a ProofBearingProfiledArtifact<T>) -> Self {
        Self {
            profile: artifact.payload().profile(),
            _payload: artifact.payload().payload(),
        }
    }

    pub const fn profile(&self) -> &'a MaterializedFoundationalProfileSet {
        self.profile
    }
}
