use crate::application::{
    WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily, WorthQueryDomainEntryMarker,
    WorthQueryDomainEntrySupportSnapshot,
};
use crate::basis_lifecycle::LowerRuntimeEvidenceAuthority;
use crate::continuation_pipeline::{
    WorthQueryPreparedContinuationAuthorityWitness, WorthQueryPreparedContinuationDriftKind,
    WorthQueryPreparedContinuationExecutionReadmission,
    WorthQueryPreparedContinuationFreshnessPosture,
};
use crate::evidence_identity::WorthQueryEvidenceIdentity;
use crate::runtime::WorthQueryRuntimeFacadeFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryDomainOperatingRequirement {
    TemporalQuery,
    AsyncResourceQuery,
    MixedCauseDelivery,
}

impl WorthQueryDomainOperatingRequirement {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TemporalQuery => "temporal-query",
            Self::AsyncResourceQuery => "async-resource-query",
            Self::MixedCauseDelivery => "mixed-cause-delivery",
        }
    }

    pub fn runtime_facade_family(self) -> WorthQueryRuntimeFacadeFamily {
        match self {
            Self::TemporalQuery => WorthQueryRuntimeFacadeFamily::Temporal,
            Self::AsyncResourceQuery => WorthQueryRuntimeFacadeFamily::AsyncResource,
            Self::MixedCauseDelivery => WorthQueryRuntimeFacadeFamily::MixedCauseDelivery,
        }
    }
}

impl std::fmt::Display for WorthQueryDomainOperatingRequirement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryContinuationExecutionReadmissionObservation {
    authority: LowerRuntimeEvidenceAuthority,
    basis_identity: WorthQueryEvidenceIdentity,
    lower_runtime_binding_identity: Option<WorthQueryEvidenceIdentity>,
    freshness_posture: WorthQueryPreparedContinuationFreshnessPosture,
    drift_kind: Option<WorthQueryPreparedContinuationDriftKind>,
}

impl WorthQueryContinuationExecutionReadmissionObservation {
    pub(crate) fn new(
        authority: LowerRuntimeEvidenceAuthority,
        basis_identity: WorthQueryEvidenceIdentity,
        lower_runtime_binding_identity: Option<WorthQueryEvidenceIdentity>,
        freshness_posture: WorthQueryPreparedContinuationFreshnessPosture,
        drift_kind: Option<WorthQueryPreparedContinuationDriftKind>,
    ) -> Self {
        Self {
            authority,
            basis_identity,
            lower_runtime_binding_identity,
            freshness_posture,
            drift_kind,
        }
    }

    pub fn from_retained(retained: &WorthQueryPreparedContinuationExecutionReadmission) -> Self {
        let witness = retained.basis_witness();
        Self::new(
            lower_runtime_authority_from_witness(retained.authority_witness()),
            witness.basis_identity().clone(),
            witness.expected_lower_runtime_binding_identity().cloned(),
            retained.freshness_posture(),
            retained.drift_kind(),
        )
    }

    pub fn authority(&self) -> LowerRuntimeEvidenceAuthority {
        self.authority
    }

    pub fn basis_identity_digest(&self) -> &str {
        self.basis_identity.as_str()
    }

    pub fn basis_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.basis_identity
    }

    pub fn lower_runtime_binding_digest(&self) -> Option<&str> {
        self.lower_runtime_binding_identity
            .as_ref()
            .map(WorthQueryEvidenceIdentity::as_str)
    }

    pub fn lower_runtime_binding_identity(&self) -> Option<&WorthQueryEvidenceIdentity> {
        self.lower_runtime_binding_identity.as_ref()
    }

    pub fn freshness_posture(&self) -> WorthQueryPreparedContinuationFreshnessPosture {
        self.freshness_posture
    }

    pub fn drift_kind(&self) -> Option<WorthQueryPreparedContinuationDriftKind> {
        self.drift_kind
    }
}

pub trait WorthQueryDomainOperatingContext<D: WorthQueryDomainEntryMarker>: Clone + Eq {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily];

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily];

    fn required_operating_requirements(&self) -> &'static [WorthQueryDomainOperatingRequirement] {
        &[]
    }

    fn context_identity_digest(&self) -> String;

    fn continuation_execution_readmission_observation(
        &self,
        retained: &WorthQueryPreparedContinuationExecutionReadmission,
        _support_snapshot: &WorthQueryDomainEntrySupportSnapshot,
    ) -> WorthQueryContinuationExecutionReadmissionObservation {
        WorthQueryContinuationExecutionReadmissionObservation::from_retained(retained)
    }
}

fn lower_runtime_authority_from_witness(
    witness: WorthQueryPreparedContinuationAuthorityWitness,
) -> LowerRuntimeEvidenceAuthority {
    match witness {
        WorthQueryPreparedContinuationAuthorityWitness::Runtime => {
            LowerRuntimeEvidenceAuthority::Runtime
        }
        WorthQueryPreparedContinuationAuthorityWitness::RuntimeBridgeFacade => {
            LowerRuntimeEvidenceAuthority::RuntimeBridgeFacade
        }
        WorthQueryPreparedContinuationAuthorityWitness::RelationalFacade => {
            LowerRuntimeEvidenceAuthority::RelationalFacade
        }
        WorthQueryPreparedContinuationAuthorityWitness::SignalFacade => {
            LowerRuntimeEvidenceAuthority::SignalFacade
        }
    }
}
