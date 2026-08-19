use worth_foundational::facade::{
    profiles, AdmissionReadinessProfile, CertificationPostureProfile, CompatibilityPostureProfile,
    DiagnosticRichnessProfile, ExecutionObjectiveProfile, FoundationalProfileSet,
    ObservationActivationProfile, RetentionDeliveryProfile, SupportPostureProfile,
    SupportProfiledArtifact,
};
use worth_proof::TransitionOutcome;

use super::{WorthQueryConsumerSupportDimension, WorthQueryConsumerSupportPosture};
use crate::domain_installation::{
    WorthQueryOperationSupportRequirements, WorthQuerySupportRequirement,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryConsumerSupportBoundaryFreshness {
    Current,
    Stale,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryConsumerSupportBoundaryAvailability {
    FullyAvailable,
    RequiredDimensionsAvailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryConsumerSupportBoundaryDegradation {
    None,
    UnrequiredDimensionsDeferred,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryConsumerSupportBoundaryRow {
    pub dimension: WorthQueryConsumerSupportDimension,
    pub requirement: WorthQuerySupportRequirement,
    pub posture: WorthQueryConsumerSupportPosture,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryFoundationalConsumerSupportProjection {
    binding_identity: String,
    basis_identity: String,
    installation_generation: u64,
    freshness: WorthQueryConsumerSupportBoundaryFreshness,
    availability: WorthQueryConsumerSupportBoundaryAvailability,
    degradation: WorthQueryConsumerSupportBoundaryDegradation,
    rows: [WorthQueryConsumerSupportBoundaryRow; WorthQueryConsumerSupportDimension::COUNT],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryFoundationalSupportExportDenial {
    ProfileConstruction,
    ProfileAdmissionDenied,
    ProfileAdmissionDeferred,
    ProfileAdmissionStale,
    ProfileAdmissionRebindRequired,
    ProfileAdmissionFailed,
    SupportAttachmentDenied,
    SupportAttachmentDeferred,
    SupportAttachmentStale,
    SupportAttachmentRebindRequired,
    SupportAttachmentFailed,
}

impl WorthQueryFoundationalConsumerSupportProjection {
    pub fn binding_identity(&self) -> &str {
        &self.binding_identity
    }

    pub fn basis_identity(&self) -> &str {
        &self.basis_identity
    }

    pub fn installation_generation(&self) -> u64 {
        self.installation_generation
    }

    pub fn freshness(&self) -> WorthQueryConsumerSupportBoundaryFreshness {
        self.freshness
    }

    pub fn availability(&self) -> WorthQueryConsumerSupportBoundaryAvailability {
        self.availability
    }

    pub fn degradation(&self) -> WorthQueryConsumerSupportBoundaryDegradation {
        self.degradation
    }

    pub fn rows(
        &self,
    ) -> &[WorthQueryConsumerSupportBoundaryRow; WorthQueryConsumerSupportDimension::COUNT] {
        &self.rows
    }
}

pub(super) fn materialize_foundational_support_projection(
    binding_identity: &str,
    basis_identity: &str,
    installation_generation: super::super::WorthQueryDomainInstallationGeneration,
    is_current: bool,
    requirements: WorthQueryOperationSupportRequirements,
    postures: [WorthQueryConsumerSupportPosture; WorthQueryConsumerSupportDimension::COUNT],
) -> Result<
    SupportProfiledArtifact<WorthQueryFoundationalConsumerSupportProjection>,
    WorthQueryFoundationalSupportExportDenial,
> {
    let rows = WorthQueryConsumerSupportDimension::ALL.map(|dimension| {
        WorthQueryConsumerSupportBoundaryRow {
            dimension,
            requirement: super::contract::requirement(requirements, dimension),
            posture: postures[dimension.index()],
        }
    });
    let payload = consumer_support_export_payload(
        binding_identity,
        basis_identity,
        installation_generation,
        is_current,
        rows,
    );
    let profile = consumer_support_export_profile()?;
    attach_consumer_support_export(profile, payload)
}

fn consumer_support_export_profile(
) -> Result<FoundationalProfileSet, WorthQueryFoundationalSupportExportDenial> {
    profiles()
        .set()
        .diagnostic_richness(DiagnosticRichnessProfile::Standard)
        .support_posture(SupportPostureProfile::SupportReady)
        .compatibility_posture(CompatibilityPostureProfile::NativeOnly)
        .admission_readiness(AdmissionReadinessProfile::Admitted)
        .retention_delivery(RetentionDeliveryProfile::Retained)
        .certification_posture(CertificationPostureProfile::EvidenceBacked)
        .execution_objective(ExecutionObjectiveProfile::Balanced)
        .observation_activation(ObservationActivationProfile::Continuous)
        .compose()
        .map_err(|_| WorthQueryFoundationalSupportExportDenial::ProfileConstruction)
}

fn consumer_support_export_payload(
    binding_identity: &str,
    basis_identity: &str,
    installation_generation: super::super::WorthQueryDomainInstallationGeneration,
    is_current: bool,
    rows: [WorthQueryConsumerSupportBoundaryRow; WorthQueryConsumerSupportDimension::COUNT],
) -> WorthQueryFoundationalConsumerSupportProjection {
    WorthQueryFoundationalConsumerSupportProjection {
        binding_identity: binding_identity.into(),
        basis_identity: basis_identity.into(),
        installation_generation: installation_generation.ordinal(),
        freshness: consumer_support_export_freshness(is_current),
        availability: consumer_support_export_availability(&rows),
        degradation: consumer_support_export_degradation(&rows),
        rows,
    }
}

fn consumer_support_export_freshness(
    is_current: bool,
) -> WorthQueryConsumerSupportBoundaryFreshness {
    if is_current {
        WorthQueryConsumerSupportBoundaryFreshness::Current
    } else {
        WorthQueryConsumerSupportBoundaryFreshness::Stale
    }
}

fn consumer_support_export_availability(
    rows: &[WorthQueryConsumerSupportBoundaryRow; WorthQueryConsumerSupportDimension::COUNT],
) -> WorthQueryConsumerSupportBoundaryAvailability {
    if rows
        .iter()
        .all(|row| row.posture == WorthQueryConsumerSupportPosture::Supported)
    {
        WorthQueryConsumerSupportBoundaryAvailability::FullyAvailable
    } else {
        WorthQueryConsumerSupportBoundaryAvailability::RequiredDimensionsAvailable
    }
}

fn consumer_support_export_degradation(
    rows: &[WorthQueryConsumerSupportBoundaryRow; WorthQueryConsumerSupportDimension::COUNT],
) -> WorthQueryConsumerSupportBoundaryDegradation {
    if rows
        .iter()
        .any(|row| row.posture == WorthQueryConsumerSupportPosture::Deferred)
    {
        WorthQueryConsumerSupportBoundaryDegradation::UnrequiredDimensionsDeferred
    } else {
        WorthQueryConsumerSupportBoundaryDegradation::None
    }
}

fn attach_consumer_support_export(
    profile: FoundationalProfileSet,
    payload: WorthQueryFoundationalConsumerSupportProjection,
) -> Result<
    SupportProfiledArtifact<WorthQueryFoundationalConsumerSupportProjection>,
    WorthQueryFoundationalSupportExportDenial,
> {
    let requested = worth_foundational::facade::request_foundational_profile_set(profile);
    let admitted = match profiles().progression().admit_same(requested) {
        TransitionOutcome::Success(admitted) => admitted,
        TransitionOutcome::Denied(_) => {
            return Err(WorthQueryFoundationalSupportExportDenial::ProfileAdmissionDenied)
        }
        TransitionOutcome::Deferred(_) => {
            return Err(WorthQueryFoundationalSupportExportDenial::ProfileAdmissionDeferred)
        }
        TransitionOutcome::Stale(_) => {
            return Err(WorthQueryFoundationalSupportExportDenial::ProfileAdmissionStale)
        }
        TransitionOutcome::RebindRequired(_) => {
            return Err(WorthQueryFoundationalSupportExportDenial::ProfileAdmissionRebindRequired)
        }
        TransitionOutcome::Failed(_) => {
            return Err(WorthQueryFoundationalSupportExportDenial::ProfileAdmissionFailed)
        }
    };
    match profiles()
        .attach()
        .to_support_artifact(admitted, profile, None, payload)
    {
        TransitionOutcome::Success(artifact) => Ok(artifact),
        TransitionOutcome::Denied(_) => {
            Err(WorthQueryFoundationalSupportExportDenial::SupportAttachmentDenied)
        }
        TransitionOutcome::Deferred(_) => {
            Err(WorthQueryFoundationalSupportExportDenial::SupportAttachmentDeferred)
        }
        TransitionOutcome::Stale(_) => {
            Err(WorthQueryFoundationalSupportExportDenial::SupportAttachmentStale)
        }
        TransitionOutcome::RebindRequired(_) => {
            Err(WorthQueryFoundationalSupportExportDenial::SupportAttachmentRebindRequired)
        }
        TransitionOutcome::Failed(_) => {
            Err(WorthQueryFoundationalSupportExportDenial::SupportAttachmentFailed)
        }
    }
}
