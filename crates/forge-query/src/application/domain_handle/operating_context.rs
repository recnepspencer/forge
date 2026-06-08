use crate::application::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainEntrySupportSnapshot,
};
use crate::basis_lifecycle::LowerRuntimeEvidenceAuthority;
use crate::continuation_pipeline::{
    ForgeQueryPreparedContinuationAuthorityWitness, ForgeQueryPreparedContinuationDriftKind,
    ForgeQueryPreparedContinuationExecutionReadmission,
    ForgeQueryPreparedContinuationFreshnessPosture,
};
use crate::runtime::ForgeQueryRuntimeFacadeFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryDomainOperatingRequirement {
    TemporalQuery,
    AsyncResourceQuery,
    MixedCauseDelivery,
}

impl ForgeQueryDomainOperatingRequirement {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TemporalQuery => "temporal-query",
            Self::AsyncResourceQuery => "async-resource-query",
            Self::MixedCauseDelivery => "mixed-cause-delivery",
        }
    }

    pub fn runtime_facade_family(self) -> ForgeQueryRuntimeFacadeFamily {
        match self {
            Self::TemporalQuery => ForgeQueryRuntimeFacadeFamily::Temporal,
            Self::AsyncResourceQuery => ForgeQueryRuntimeFacadeFamily::AsyncResource,
            Self::MixedCauseDelivery => ForgeQueryRuntimeFacadeFamily::MixedCauseDelivery,
        }
    }
}

impl std::fmt::Display for ForgeQueryDomainOperatingRequirement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryContinuationExecutionReadmissionObservation {
    authority: LowerRuntimeEvidenceAuthority,
    basis_identity_digest: String,
    lower_runtime_binding_digest: Option<String>,
    freshness_posture: ForgeQueryPreparedContinuationFreshnessPosture,
    drift_kind: Option<ForgeQueryPreparedContinuationDriftKind>,
}

impl ForgeQueryContinuationExecutionReadmissionObservation {
    pub(crate) fn new(
        authority: LowerRuntimeEvidenceAuthority,
        basis_identity_digest: String,
        lower_runtime_binding_digest: Option<String>,
        freshness_posture: ForgeQueryPreparedContinuationFreshnessPosture,
        drift_kind: Option<ForgeQueryPreparedContinuationDriftKind>,
    ) -> Self {
        Self {
            authority,
            basis_identity_digest,
            lower_runtime_binding_digest,
            freshness_posture,
            drift_kind,
        }
    }

    pub fn from_retained(retained: &ForgeQueryPreparedContinuationExecutionReadmission) -> Self {
        let witness = retained.basis_witness();
        Self::new(
            lower_runtime_authority_from_witness(retained.authority_witness()),
            witness.basis_identity_digest().to_string(),
            witness
                .expected_lower_runtime_binding_digest()
                .map(str::to_string),
            retained.freshness_posture(),
            retained.drift_kind(),
        )
    }

    pub fn authority(&self) -> LowerRuntimeEvidenceAuthority {
        self.authority
    }

    pub fn basis_identity_digest(&self) -> &str {
        &self.basis_identity_digest
    }

    pub fn lower_runtime_binding_digest(&self) -> Option<&str> {
        self.lower_runtime_binding_digest.as_deref()
    }

    pub fn freshness_posture(&self) -> ForgeQueryPreparedContinuationFreshnessPosture {
        self.freshness_posture
    }

    pub fn drift_kind(&self) -> Option<ForgeQueryPreparedContinuationDriftKind> {
        self.drift_kind
    }
}

pub trait ForgeQueryDomainOperatingContext<D: ForgeQueryDomainEntryMarker>: Clone + Eq {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily];

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily];

    fn required_operating_requirements(&self) -> &'static [ForgeQueryDomainOperatingRequirement] {
        &[]
    }

    fn context_identity_digest(&self) -> String;

    fn continuation_execution_readmission_observation(
        &self,
        retained: &ForgeQueryPreparedContinuationExecutionReadmission,
        _support_snapshot: &ForgeQueryDomainEntrySupportSnapshot,
    ) -> ForgeQueryContinuationExecutionReadmissionObservation {
        ForgeQueryContinuationExecutionReadmissionObservation::from_retained(retained)
    }
}

fn lower_runtime_authority_from_witness(
    witness: ForgeQueryPreparedContinuationAuthorityWitness,
) -> LowerRuntimeEvidenceAuthority {
    match witness {
        ForgeQueryPreparedContinuationAuthorityWitness::Runtime => {
            LowerRuntimeEvidenceAuthority::Runtime
        }
        ForgeQueryPreparedContinuationAuthorityWitness::RuntimeBridgeFacade => {
            LowerRuntimeEvidenceAuthority::RuntimeBridgeFacade
        }
        ForgeQueryPreparedContinuationAuthorityWitness::RelationalFacade => {
            LowerRuntimeEvidenceAuthority::RelationalFacade
        }
        ForgeQueryPreparedContinuationAuthorityWitness::SignalFacade => {
            LowerRuntimeEvidenceAuthority::SignalFacade
        }
    }
}
