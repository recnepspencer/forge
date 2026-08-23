use worth_proof::{Artifact, AuthorityMarker, AuthorityWitness, PhaseMarker, TransitionOutcome};

use super::progression_resolution::{
    admit_requested_foundational_profile_with_resolutions,
    materialize_admitted_foundational_profile_with_resolutions,
};
use super::resolution::{
    changed_resolution_families, FoundationalProfileResolutionFamily,
    FoundationalProfileResolutionLedger,
};
use super::FoundationalProfileSet;

mod narrowing;
use narrowing::classify_profile_narrowing;
pub(super) use narrowing::classify_profile_narrowing_for_resolution;
pub use narrowing::{FoundationalProfileNarrowingKind, FoundationalProfileNarrowingRecord};

pub type FoundationalProfileProgressionOutcome<S> = TransitionOutcome<
    S,
    FoundationalProfileProgressionDenial,
    FoundationalProfileProgressionDeferred,
    FoundationalProfileProgressionStale,
    FoundationalProfileProgressionRebindRequired,
    FoundationalProfileProgressionFailure,
>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequestedFoundationalProfilePhase;
impl PhaseMarker for RequestedFoundationalProfilePhase {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedFoundationalProfilePhase;
impl PhaseMarker for AdmittedFoundationalProfilePhase {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaterializedFoundationalProfilePhase;
impl PhaseMarker for MaterializedFoundationalProfilePhase {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequestedFoundationalProfileSet {
    requested: FoundationalProfileSet,
}

impl RequestedFoundationalProfileSet {
    pub const fn requested(&self) -> &FoundationalProfileSet {
        &self.requested
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedFoundationalProfileSet {
    pub(super) requested: FoundationalProfileSet,
    pub(super) admitted: FoundationalProfileSet,
    pub(super) requested_to_admitted_resolutions: FoundationalProfileResolutionLedger,
}

impl AdmittedFoundationalProfileSet {
    pub const fn requested(&self) -> &FoundationalProfileSet {
        &self.requested
    }

    pub const fn admitted(&self) -> &FoundationalProfileSet {
        &self.admitted
    }

    pub fn requested_to_admitted_narrowing(&self) -> Option<FoundationalProfileNarrowingRecord> {
        narrowing::legacy_narrowing_projection(self.requested_to_admitted_resolutions)
    }

    pub const fn requested_to_admitted_resolutions(&self) -> FoundationalProfileResolutionLedger {
        self.requested_to_admitted_resolutions
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaterializedFoundationalProfileSet {
    pub(super) requested: FoundationalProfileSet,
    pub(super) admitted: FoundationalProfileSet,
    pub(super) materialized: FoundationalProfileSet,
    pub(super) requested_to_admitted_resolutions: FoundationalProfileResolutionLedger,
    pub(super) admitted_to_materialized_resolutions: FoundationalProfileResolutionLedger,
}

impl MaterializedFoundationalProfileSet {
    pub const fn requested(&self) -> &FoundationalProfileSet {
        &self.requested
    }

    pub const fn admitted(&self) -> &FoundationalProfileSet {
        &self.admitted
    }

    pub const fn materialized(&self) -> &FoundationalProfileSet {
        &self.materialized
    }

    pub fn requested_to_admitted_narrowing(&self) -> Option<FoundationalProfileNarrowingRecord> {
        narrowing::legacy_narrowing_projection(self.requested_to_admitted_resolutions)
    }

    pub fn admitted_to_materialized_narrowing(&self) -> Option<FoundationalProfileNarrowingRecord> {
        narrowing::legacy_narrowing_projection(self.admitted_to_materialized_resolutions)
    }

    pub const fn requested_to_admitted_resolutions(&self) -> FoundationalProfileResolutionLedger {
        self.requested_to_admitted_resolutions
    }

    pub const fn admitted_to_materialized_resolutions(
        &self,
    ) -> FoundationalProfileResolutionLedger {
        self.admitted_to_materialized_resolutions
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalProfileProgressionAuthority(());

impl FoundationalProfileProgressionAuthority {
    pub(crate) const fn milestone_3_phase_3() -> Self {
        Self(())
    }
}

impl AuthorityMarker for FoundationalProfileProgressionAuthority {}

pub fn foundational_profile_progression_authority(
) -> AuthorityWitness<FoundationalProfileProgressionAuthority> {
    AuthorityWitness::from_authority_marker(
        FoundationalProfileProgressionAuthority::milestone_3_phase_3(),
    )
}

pub type RequestedFoundationalProfileArtifact =
    Artifact<RequestedFoundationalProfilePhase, RequestedFoundationalProfileSet>;
pub type AdmittedFoundationalProfileArtifact =
    Artifact<AdmittedFoundationalProfilePhase, AdmittedFoundationalProfileSet>;
pub type MaterializedFoundationalProfileArtifact =
    Artifact<MaterializedFoundationalProfilePhase, MaterializedFoundationalProfileSet>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalProfileProgressionDenial {
    MissingExplicitNarrowingRecord,
    NarrowingRecordKindMismatch,
    RequestedAndAdmittedProfilesMayDifferInOnlyOneFamily,
    RequestedAndAdmittedProfilesMayOnlyNarrow,
    AdmissionReadinessCannotChangeAcrossProfileProgression,
    ResolutionLedgerDoesNotMatchProfileChange,
    ResolutionRelationMismatch(FoundationalProfileResolutionFamily),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalProfileProgressionDeferred {
    reason: &'static str,
}

impl FoundationalProfileProgressionDeferred {
    pub const fn new(reason: &'static str) -> Self {
        Self { reason }
    }

    pub const fn reason(&self) -> &'static str {
        self.reason
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalProfileProgressionStale {
    reason: &'static str,
}

impl FoundationalProfileProgressionStale {
    pub const fn new(reason: &'static str) -> Self {
        Self { reason }
    }

    pub const fn reason(&self) -> &'static str {
        self.reason
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalProfileProgressionRebindRequired {
    reason: &'static str,
}

impl FoundationalProfileProgressionRebindRequired {
    pub const fn new(reason: &'static str) -> Self {
        Self { reason }
    }

    pub const fn reason(&self) -> &'static str {
        self.reason
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalProfileProgressionFailure {
    reason: &'static str,
}

impl FoundationalProfileProgressionFailure {
    pub const fn new(reason: &'static str) -> Self {
        Self { reason }
    }

    pub const fn reason(&self) -> &'static str {
        self.reason
    }
}

pub fn request_foundational_profile_set(
    requested: FoundationalProfileSet,
) -> RequestedFoundationalProfileArtifact {
    Artifact::new(RequestedFoundationalProfileSet { requested })
}

pub fn admit_requested_foundational_profile(
    requested: RequestedFoundationalProfileArtifact,
    admitted: FoundationalProfileSet,
    narrowing: Option<FoundationalProfileNarrowingRecord>,
    authority: AuthorityWitness<FoundationalProfileProgressionAuthority>,
) -> FoundationalProfileProgressionOutcome<AdmittedFoundationalProfileArtifact> {
    let requested_profile = *requested.payload().requested();
    if let Err(denial) = classify_profile_narrowing(requested_profile, admitted, narrowing) {
        return TransitionOutcome::denied(denial);
    }
    admit_requested_foundational_profile_with_resolutions(
        requested,
        admitted,
        legacy_resolution_ledger(requested_profile, admitted, narrowing),
        authority,
    )
}

pub fn materialize_admitted_foundational_profile(
    admitted: AdmittedFoundationalProfileArtifact,
    materialized: FoundationalProfileSet,
    narrowing: Option<FoundationalProfileNarrowingRecord>,
    authority: AuthorityWitness<FoundationalProfileProgressionAuthority>,
) -> FoundationalProfileProgressionOutcome<MaterializedFoundationalProfileArtifact> {
    let admitted_profile = *admitted.payload().admitted();
    if let Err(denial) = classify_profile_narrowing(admitted_profile, materialized, narrowing) {
        return TransitionOutcome::denied(denial);
    }
    materialize_admitted_foundational_profile_with_resolutions(
        admitted,
        materialized,
        legacy_resolution_ledger(admitted_profile, materialized, narrowing),
        authority,
    )
}

fn legacy_resolution_ledger(
    stronger: FoundationalProfileSet,
    weaker: FoundationalProfileSet,
    narrowing: Option<FoundationalProfileNarrowingRecord>,
) -> FoundationalProfileResolutionLedger {
    let mut ledger = changed_resolution_families(stronger, weaker);
    if let Some(narrowing) = narrowing {
        let family = match narrowing.kind() {
            FoundationalProfileNarrowingKind::RichnessReduced => {
                FoundationalProfileResolutionFamily::DiagnosticRichness
            }
            FoundationalProfileNarrowingKind::RetentionNarrowed => {
                FoundationalProfileResolutionFamily::RetentionDelivery
            }
            FoundationalProfileNarrowingKind::SupportPostureReduced => {
                FoundationalProfileResolutionFamily::SupportPosture
            }
            FoundationalProfileNarrowingKind::CertificationPostureReduced => {
                FoundationalProfileResolutionFamily::CertificationPosture
            }
            FoundationalProfileNarrowingKind::CompatibilityRestricted => {
                FoundationalProfileResolutionFamily::CompatibilityPosture
            }
        };
        ledger.replace_descriptive_reason(family, narrowing.reason());
    }
    ledger
}
