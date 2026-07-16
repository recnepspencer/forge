use crate::{ModelActionFamily, OwnerBoundaryBinding, ProtocolFamily};

use super::{CanonicalProtocolTrace, ProtocolCounterexample};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbstractionFunctionIdentity {
    DurabilityOwnerMapping,
    RecoverySourceTraceMapping,
    CompactionVisibilityOwnerMapping,
    LeaseReclaimOwnerMapping,
    QuarantineReadmissionOwnerMapping,
    ImportPublicationOwnerMapping,
    ReplicationAdmissionOwnerMapping,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificationLaneIdentity(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CounterexampleLocalization {
    counterexample: ProtocolCounterexample,
    owner_binding: OwnerBoundaryBinding,
    abstraction_function: AbstractionFunctionIdentity,
    failing_lane: CertificationLaneIdentity,
    trace_excerpt: CanonicalProtocolTrace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CounterexampleLocalizationDenial {
    EmptyLaneIdentity,
    ProtocolMismatch,
    AbstractionFunctionMismatch,
}

impl CertificationLaneIdentity {
    pub fn admit(raw: impl Into<String>) -> Result<Self, CounterexampleLocalizationDenial> {
        let raw = raw.into();
        if raw.is_empty() {
            Err(CounterexampleLocalizationDenial::EmptyLaneIdentity)
        } else {
            Ok(Self(raw))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl CounterexampleLocalization {
    pub fn localize(
        counterexample: ProtocolCounterexample,
        owner_binding: OwnerBoundaryBinding,
        abstraction_function: AbstractionFunctionIdentity,
        failing_lane: CertificationLaneIdentity,
        trace_excerpt: CanonicalProtocolTrace,
    ) -> Result<Self, CounterexampleLocalizationDenial> {
        let protocol = counterexample.protocol();
        if owner_binding.protocol() != protocol || trace_excerpt.protocol() != protocol {
            return Err(CounterexampleLocalizationDenial::ProtocolMismatch);
        }
        if abstraction_function.protocol() != protocol {
            return Err(CounterexampleLocalizationDenial::AbstractionFunctionMismatch);
        }
        if !abstraction_function.admits(owner_binding.model_action_family()) {
            return Err(CounterexampleLocalizationDenial::AbstractionFunctionMismatch);
        }
        Ok(Self {
            counterexample,
            owner_binding,
            abstraction_function,
            failing_lane,
            trace_excerpt,
        })
    }

    pub const fn counterexample(&self) -> &ProtocolCounterexample {
        &self.counterexample
    }

    pub const fn owner_binding(&self) -> OwnerBoundaryBinding {
        self.owner_binding
    }

    pub const fn abstraction_function(&self) -> AbstractionFunctionIdentity {
        self.abstraction_function
    }

    pub const fn failing_lane(&self) -> &CertificationLaneIdentity {
        &self.failing_lane
    }

    pub const fn trace_excerpt(&self) -> &CanonicalProtocolTrace {
        &self.trace_excerpt
    }
}

impl AbstractionFunctionIdentity {
    const fn admits(self, family: ModelActionFamily) -> bool {
        match self {
            Self::DurabilityOwnerMapping => matches!(
                family,
                ModelActionFamily::DurabilityAdmission | ModelActionFamily::DurabilityFrontier
            ),
            Self::RecoverySourceTraceMapping => matches!(
                family,
                ModelActionFamily::RecoverySourcePrecedence | ModelActionFamily::RecoveryRedo
            ),
            Self::CompactionVisibilityOwnerMapping => matches!(
                family,
                ModelActionFamily::LsmMembership
                    | ModelActionFamily::LsmExecution
                    | ModelActionFamily::LsmMaintenance
                    | ModelActionFamily::PhysicalCompaction
            ),
            Self::LeaseReclaimOwnerMapping => matches!(
                family,
                ModelActionFamily::LeaseReclaim | ModelActionFamily::GenerationReuse
            ),
            Self::QuarantineReadmissionOwnerMapping => {
                matches!(family, ModelActionFamily::QuarantineReadmission)
            }
            Self::ImportPublicationOwnerMapping => matches!(
                family,
                ModelActionFamily::ImportPublication | ModelActionFamily::TrustBoundaryReadmission
            ),
            Self::ReplicationAdmissionOwnerMapping => {
                matches!(family, ModelActionFamily::ReplicationAdmission)
            }
        }
    }

    pub const fn protocol(self) -> ProtocolFamily {
        match self {
            Self::DurabilityOwnerMapping => ProtocolFamily::DurabilityRecovery,
            Self::RecoverySourceTraceMapping => ProtocolFamily::RecoverySourcePrecedence,
            Self::CompactionVisibilityOwnerMapping => ProtocolFamily::CompactionVisibility,
            Self::LeaseReclaimOwnerMapping => ProtocolFamily::LeaseReclaim,
            Self::QuarantineReadmissionOwnerMapping => ProtocolFamily::QuarantineReadmission,
            Self::ImportPublicationOwnerMapping => ProtocolFamily::ImportPublication,
            Self::ReplicationAdmissionOwnerMapping => ProtocolFamily::ReplicationAdmission,
        }
    }
}
