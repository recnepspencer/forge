use forge_proof::{Artifact, AuthorityMarker, AuthorityWitness, PhaseMarker, TransitionOutcome};

use super::FoundationalProfileSet;

pub type FoundationalProfileProgressionOutcome<S> = TransitionOutcome<
    S,
    FoundationalProfileProgressionDenial,
    FoundationalProfileProgressionDeferred,
    FoundationalProfileProgressionStale,
    FoundationalProfileProgressionRebindRequired,
    FoundationalProfileProgressionFailure,
>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalProfileNarrowingKind {
    RichnessReduced,
    RetentionNarrowed,
    SupportPostureReduced,
    CertificationPostureReduced,
    CompatibilityRestricted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalProfileNarrowingRecord {
    kind: FoundationalProfileNarrowingKind,
    reason: &'static str,
}

impl FoundationalProfileNarrowingRecord {
    pub const fn new(kind: FoundationalProfileNarrowingKind, reason: &'static str) -> Self {
        Self { kind, reason }
    }

    pub const fn kind(&self) -> FoundationalProfileNarrowingKind {
        self.kind
    }

    pub const fn reason(&self) -> &'static str {
        self.reason
    }
}

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
    requested: FoundationalProfileSet,
    admitted: FoundationalProfileSet,
    requested_to_admitted_narrowing: Option<FoundationalProfileNarrowingRecord>,
}

impl AdmittedFoundationalProfileSet {
    pub const fn requested(&self) -> &FoundationalProfileSet {
        &self.requested
    }

    pub const fn admitted(&self) -> &FoundationalProfileSet {
        &self.admitted
    }

    pub const fn requested_to_admitted_narrowing(
        &self,
    ) -> Option<FoundationalProfileNarrowingRecord> {
        self.requested_to_admitted_narrowing
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaterializedFoundationalProfileSet {
    requested: FoundationalProfileSet,
    admitted: FoundationalProfileSet,
    materialized: FoundationalProfileSet,
    requested_to_admitted_narrowing: Option<FoundationalProfileNarrowingRecord>,
    admitted_to_materialized_narrowing: Option<FoundationalProfileNarrowingRecord>,
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

    pub const fn requested_to_admitted_narrowing(
        &self,
    ) -> Option<FoundationalProfileNarrowingRecord> {
        self.requested_to_admitted_narrowing
    }

    pub const fn admitted_to_materialized_narrowing(
        &self,
    ) -> Option<FoundationalProfileNarrowingRecord> {
        self.admitted_to_materialized_narrowing
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
    _authority: AuthorityWitness<FoundationalProfileProgressionAuthority>,
) -> FoundationalProfileProgressionOutcome<AdmittedFoundationalProfileArtifact> {
    let requested_profile = *requested.payload().requested();
    let admitted_narrowing =
        match classify_profile_narrowing(requested_profile, admitted, narrowing) {
            Ok(narrowing) => narrowing,
            Err(denial) => return TransitionOutcome::denied(denial),
        };

    TransitionOutcome::success(Artifact::new(AdmittedFoundationalProfileSet {
        requested: requested_profile,
        admitted,
        requested_to_admitted_narrowing: admitted_narrowing,
    }))
}

pub fn materialize_admitted_foundational_profile(
    admitted: AdmittedFoundationalProfileArtifact,
    materialized: FoundationalProfileSet,
    narrowing: Option<FoundationalProfileNarrowingRecord>,
    _authority: AuthorityWitness<FoundationalProfileProgressionAuthority>,
) -> FoundationalProfileProgressionOutcome<MaterializedFoundationalProfileArtifact> {
    let admitted_payload = admitted.payload();
    let materialized_narrowing =
        match classify_profile_narrowing(*admitted_payload.admitted(), materialized, narrowing) {
            Ok(narrowing) => narrowing,
            Err(denial) => return TransitionOutcome::denied(denial),
        };

    TransitionOutcome::success(Artifact::new(MaterializedFoundationalProfileSet {
        requested: *admitted_payload.requested(),
        admitted: *admitted_payload.admitted(),
        materialized,
        requested_to_admitted_narrowing: admitted_payload.requested_to_admitted_narrowing(),
        admitted_to_materialized_narrowing: materialized_narrowing,
    }))
}

fn classify_profile_narrowing(
    stronger: FoundationalProfileSet,
    weaker: FoundationalProfileSet,
    narrowing: Option<FoundationalProfileNarrowingRecord>,
) -> Result<Option<FoundationalProfileNarrowingRecord>, FoundationalProfileProgressionDenial> {
    let Some(expected_kind) = detect_profile_narrowing_kind(stronger, weaker)? else {
        return Ok(None);
    };

    let Some(record) = narrowing else {
        return Err(FoundationalProfileProgressionDenial::MissingExplicitNarrowingRecord);
    };

    if record.kind() != expected_kind {
        return Err(FoundationalProfileProgressionDenial::NarrowingRecordKindMismatch);
    }

    Ok(Some(record))
}

fn detect_profile_narrowing_kind(
    stronger: FoundationalProfileSet,
    weaker: FoundationalProfileSet,
) -> Result<Option<FoundationalProfileNarrowingKind>, FoundationalProfileProgressionDenial> {
    if stronger.admission_readiness() != weaker.admission_readiness() {
        return Err(
            FoundationalProfileProgressionDenial::AdmissionReadinessCannotChangeAcrossProfileProgression,
        );
    }

    let mut changed_kind = None;

    record_family_narrowing(
        stronger.diagnostic_richness(),
        weaker.diagnostic_richness(),
        FoundationalProfileNarrowingKind::RichnessReduced,
        &mut changed_kind,
    )?;
    record_family_narrowing(
        stronger.retention_delivery(),
        weaker.retention_delivery(),
        FoundationalProfileNarrowingKind::RetentionNarrowed,
        &mut changed_kind,
    )?;
    record_family_narrowing(
        stronger.support_posture(),
        weaker.support_posture(),
        FoundationalProfileNarrowingKind::SupportPostureReduced,
        &mut changed_kind,
    )?;
    record_family_narrowing(
        stronger.certification_posture(),
        weaker.certification_posture(),
        FoundationalProfileNarrowingKind::CertificationPostureReduced,
        &mut changed_kind,
    )?;
    record_family_narrowing(
        stronger.compatibility_posture(),
        weaker.compatibility_posture(),
        FoundationalProfileNarrowingKind::CompatibilityRestricted,
        &mut changed_kind,
    )?;

    Ok(changed_kind)
}

fn record_family_narrowing<T>(
    stronger: T,
    weaker: T,
    kind: FoundationalProfileNarrowingKind,
    changed_kind: &mut Option<FoundationalProfileNarrowingKind>,
) -> Result<(), FoundationalProfileProgressionDenial>
where
    T: Copy + Ord,
{
    if stronger == weaker {
        return Ok(());
    }

    if weaker > stronger {
        return Err(
            FoundationalProfileProgressionDenial::RequestedAndAdmittedProfilesMayOnlyNarrow,
        );
    }

    if changed_kind.replace(kind).is_some() {
        return Err(
            FoundationalProfileProgressionDenial::RequestedAndAdmittedProfilesMayDifferInOnlyOneFamily,
        );
    }

    Ok(())
}
