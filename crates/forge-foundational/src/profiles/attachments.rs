use std::marker::PhantomData;

use forge_proof::{Artifact, AuthorityWitness, TransitionOutcome};

use super::progression::{
    materialize_admitted_foundational_profile, AdmittedFoundationalProfileArtifact,
    FoundationalProfileNarrowingRecord, FoundationalProfileProgressionAuthority,
    FoundationalProfileProgressionDeferred, FoundationalProfileProgressionDenial,
    FoundationalProfileProgressionFailure, FoundationalProfileProgressionOutcome,
    FoundationalProfileProgressionRebindRequired, FoundationalProfileProgressionStale,
    MaterializedFoundationalProfileSet,
};
use super::FoundationalProfileSet;

mod sealed {
    pub trait Sealed {}
}

pub trait FoundationalProfileAttachmentTargetMarker: sealed::Sealed {
    fn kind() -> FoundationalProfileAttachmentTargetKind;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoundaryArtifactTarget;
impl sealed::Sealed for BoundaryArtifactTarget {}
impl FoundationalProfileAttachmentTargetMarker for BoundaryArtifactTarget {
    fn kind() -> FoundationalProfileAttachmentTargetKind {
        FoundationalProfileAttachmentTargetKind::BoundaryArtifact
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SupportArtifactTarget;
impl sealed::Sealed for SupportArtifactTarget {}
impl FoundationalProfileAttachmentTargetMarker for SupportArtifactTarget {
    fn kind() -> FoundationalProfileAttachmentTargetKind {
        FoundationalProfileAttachmentTargetKind::SupportArtifact
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProofBearingArtifactTarget;
impl sealed::Sealed for ProofBearingArtifactTarget {}
impl FoundationalProfileAttachmentTargetMarker for ProofBearingArtifactTarget {
    fn kind() -> FoundationalProfileAttachmentTargetKind {
        FoundationalProfileAttachmentTargetKind::ProofBearingArtifact
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalProfileAttachmentTargetKind {
    BoundaryArtifact,
    SupportArtifact,
    ProofBearingArtifact,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalProfiledArtifact<Target, T> {
    payload: T,
    profile: MaterializedFoundationalProfileSet,
    target_kind: FoundationalProfileAttachmentTargetKind,
    marker: PhantomData<Target>,
}

impl<Target, T> FoundationalProfiledArtifact<Target, T>
where
    Target: FoundationalProfileAttachmentTargetMarker,
{
    fn new(payload: T, profile: MaterializedFoundationalProfileSet) -> Self {
        Self {
            payload,
            profile,
            target_kind: Target::kind(),
            marker: PhantomData,
        }
    }

    pub const fn payload(&self) -> &T {
        &self.payload
    }

    pub const fn profile(&self) -> &MaterializedFoundationalProfileSet {
        &self.profile
    }

    pub const fn target_kind(&self) -> FoundationalProfileAttachmentTargetKind {
        self.target_kind
    }
}

pub type BoundaryProfiledArtifact<T> = Artifact<
    super::progression::MaterializedFoundationalProfilePhase,
    FoundationalProfiledArtifact<BoundaryArtifactTarget, T>,
>;
pub type SupportProfiledArtifact<T> = Artifact<
    super::progression::MaterializedFoundationalProfilePhase,
    FoundationalProfiledArtifact<SupportArtifactTarget, T>,
>;
pub type ProofBearingProfiledArtifact<T> = Artifact<
    super::progression::MaterializedFoundationalProfilePhase,
    FoundationalProfiledArtifact<ProofBearingArtifactTarget, T>,
>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalProfileAttachmentDenial {
    ProgressionDenied(FoundationalProfileProgressionDenial),
    SupportArtifactsCannotCarryInternalOnlySupportPosture,
    ProofBearingArtifactsRequireAdmittedReadiness,
}

pub type FoundationalProfileAttachmentOutcome<S> = TransitionOutcome<
    S,
    FoundationalProfileAttachmentDenial,
    FoundationalProfileProgressionDeferred,
    FoundationalProfileProgressionStale,
    FoundationalProfileProgressionRebindRequired,
    FoundationalProfileProgressionFailure,
>;

pub fn attach_boundary_profiled_artifact<T>(
    admitted: AdmittedFoundationalProfileArtifact,
    materialized: FoundationalProfileSet,
    narrowing: Option<FoundationalProfileNarrowingRecord>,
    payload: T,
    authority: AuthorityWitness<FoundationalProfileProgressionAuthority>,
) -> FoundationalProfileProgressionOutcome<BoundaryProfiledArtifact<T>> {
    attach_profiled_artifact::<BoundaryArtifactTarget, T>(
        admitted,
        materialized,
        narrowing,
        payload,
        authority,
    )
}

pub fn attach_support_profiled_artifact<T>(
    admitted: AdmittedFoundationalProfileArtifact,
    materialized: FoundationalProfileSet,
    narrowing: Option<FoundationalProfileNarrowingRecord>,
    payload: T,
    authority: AuthorityWitness<FoundationalProfileProgressionAuthority>,
) -> FoundationalProfileAttachmentOutcome<SupportProfiledArtifact<T>> {
    if admitted.payload().admitted().support_posture() == super::SupportPostureProfile::InternalOnly
    {
        return TransitionOutcome::denied(
            FoundationalProfileAttachmentDenial::SupportArtifactsCannotCarryInternalOnlySupportPosture,
        );
    }

    map_attachment_outcome(attach_profiled_artifact::<SupportArtifactTarget, T>(
        admitted,
        materialized,
        narrowing,
        payload,
        authority,
    ))
}

pub fn attach_proof_bearing_profiled_artifact<T>(
    admitted: AdmittedFoundationalProfileArtifact,
    materialized: FoundationalProfileSet,
    narrowing: Option<FoundationalProfileNarrowingRecord>,
    payload: T,
    authority: AuthorityWitness<FoundationalProfileProgressionAuthority>,
) -> FoundationalProfileAttachmentOutcome<ProofBearingProfiledArtifact<T>> {
    if admitted.payload().admitted().admission_readiness()
        == super::AdmissionReadinessProfile::CandidateOnly
    {
        return TransitionOutcome::denied(
            FoundationalProfileAttachmentDenial::ProofBearingArtifactsRequireAdmittedReadiness,
        );
    }

    map_attachment_outcome(attach_profiled_artifact::<ProofBearingArtifactTarget, T>(
        admitted,
        materialized,
        narrowing,
        payload,
        authority,
    ))
}

fn attach_profiled_artifact<Target, T>(
    admitted: AdmittedFoundationalProfileArtifact,
    materialized: FoundationalProfileSet,
    narrowing: Option<FoundationalProfileNarrowingRecord>,
    payload: T,
    authority: AuthorityWitness<FoundationalProfileProgressionAuthority>,
) -> FoundationalProfileProgressionOutcome<
    Artifact<
        super::progression::MaterializedFoundationalProfilePhase,
        FoundationalProfiledArtifact<Target, T>,
    >,
>
where
    Target: FoundationalProfileAttachmentTargetMarker,
{
    match materialize_admitted_foundational_profile(admitted, materialized, narrowing, authority) {
        TransitionOutcome::Success(profile) => TransitionOutcome::success(Artifact::new(
            FoundationalProfiledArtifact::<Target, T>::new(payload, *profile.payload()),
        )),
        TransitionOutcome::Denied(denial) => TransitionOutcome::denied(denial),
        TransitionOutcome::Deferred(deferred) => TransitionOutcome::deferred(deferred),
        TransitionOutcome::Stale(stale) => TransitionOutcome::stale(stale),
        TransitionOutcome::RebindRequired(rebind) => TransitionOutcome::rebind_required(rebind),
        TransitionOutcome::Failed(failure) => TransitionOutcome::failed(failure),
    }
}

fn map_attachment_outcome<S>(
    outcome: FoundationalProfileProgressionOutcome<S>,
) -> FoundationalProfileAttachmentOutcome<S> {
    match outcome {
        TransitionOutcome::Success(success) => TransitionOutcome::success(success),
        TransitionOutcome::Denied(denial) => TransitionOutcome::denied(
            FoundationalProfileAttachmentDenial::ProgressionDenied(denial),
        ),
        TransitionOutcome::Deferred(deferred) => TransitionOutcome::deferred(deferred),
        TransitionOutcome::Stale(stale) => TransitionOutcome::stale(stale),
        TransitionOutcome::RebindRequired(rebind) => TransitionOutcome::rebind_required(rebind),
        TransitionOutcome::Failed(failure) => TransitionOutcome::failed(failure),
    }
}
