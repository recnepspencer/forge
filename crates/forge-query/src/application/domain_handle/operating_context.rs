use crate::application::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainEntrySupportSnapshot,
};
use crate::basis_lifecycle::LowerRuntimeEvidenceAuthority;
use crate::continuation_pipeline::{
    ForgeQueryPreparedContinuationAuthorityWitness,
    ForgeQueryPreparedContinuationExecutionReadmission,
    ForgeQueryPreparedContinuationFreshnessPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryContinuationExecutionReadmissionObservation {
    authority: LowerRuntimeEvidenceAuthority,
    basis_identity_digest: String,
    lower_runtime_binding_digest: Option<String>,
    freshness_posture: ForgeQueryPreparedContinuationFreshnessPosture,
}

impl ForgeQueryContinuationExecutionReadmissionObservation {
    pub(crate) fn new(
        authority: LowerRuntimeEvidenceAuthority,
        basis_identity_digest: String,
        lower_runtime_binding_digest: Option<String>,
        freshness_posture: ForgeQueryPreparedContinuationFreshnessPosture,
    ) -> Self {
        Self {
            authority,
            basis_identity_digest,
            lower_runtime_binding_digest,
            freshness_posture,
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
}

pub trait ForgeQueryDomainOperatingContext<D: ForgeQueryDomainEntryMarker>: Clone + Eq {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily];

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily];

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
