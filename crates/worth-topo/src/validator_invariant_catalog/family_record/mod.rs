mod family_record_counters;
mod family_record_input;
mod invariant_family_record;
mod validator_family_record;

pub(super) use family_record_counters::WorthTopologyLegalityFamilyRecordCounters;
pub(super) use family_record_input::WorthTopologyLegalityFamilyRecordInput;
pub use invariant_family_record::WorthTopologyInvariantFamilyRecord;
pub use validator_family_record::WorthTopologyValidatorFamilyRecord;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorthTopologyLegalityFamilyRecord {
    Validator(WorthTopologyValidatorFamilyRecord),
    Invariant(WorthTopologyInvariantFamilyRecord),
}

impl WorthTopologyLegalityFamilyRecord {
    pub fn identity(
        &self,
    ) -> crate::validator_invariant_catalog::WorthTopologyLegalityFamilyIdentity {
        match self {
            Self::Validator(record) => {
                crate::validator_invariant_catalog::WorthTopologyLegalityFamilyIdentity::Validator(
                    record.identity().clone(),
                )
            }
            Self::Invariant(record) => {
                crate::validator_invariant_catalog::WorthTopologyLegalityFamilyIdentity::Invariant(
                    record.identity().clone(),
                )
            }
        }
    }

    pub fn family_digest(&self) -> &str {
        match self {
            Self::Validator(record) => record.family_digest(),
            Self::Invariant(record) => record.family_digest(),
        }
    }

    pub fn query_obligation_kind(&self) -> forge_query::facade::ForgeQueryGraphObligationKind {
        match self {
            Self::Validator(record) => record.query_obligation_kind(),
            Self::Invariant(record) => record.query_obligation_kind(),
        }
    }

    pub fn touched_applicability(
        &self,
    ) -> &crate::validator_invariant_catalog::WorthTopologyTouchedApplicability {
        match self {
            Self::Validator(record) => record.touched_applicability(),
            Self::Invariant(record) => record.touched_applicability(),
        }
    }

    pub fn query_support_posture(
        &self,
    ) -> &forge_query::facade::ForgeQueryGraphObligationSupportPosture {
        match self {
            Self::Validator(record) => record.query_support_posture(),
            Self::Invariant(record) => record.query_support_posture(),
        }
    }

    pub fn required_access_posture(
        &self,
    ) -> &crate::validator_invariant_catalog::WorthTopologyRequiredAccessPosture {
        match self {
            Self::Validator(record) => record.required_access_posture(),
            Self::Invariant(record) => record.required_access_posture(),
        }
    }

    pub fn enforcement_phase(
        &self,
    ) -> crate::validator_invariant_catalog::WorthTopologyEnforcementPhase {
        match self {
            Self::Validator(record) => record.enforcement_phase(),
            Self::Invariant(record) => record.enforcement_phase(),
        }
    }

    pub fn witness_posture(
        &self,
    ) -> crate::validator_invariant_catalog::WorthTopologyWitnessPosture {
        match self {
            Self::Validator(record) => record.witness_posture(),
            Self::Invariant(record) => record.witness_posture(),
        }
    }

    pub fn diagnostic_projection(
        &self,
    ) -> crate::validator_invariant_catalog::WorthTopologyDiagnosticProjectionPosture {
        match self {
            Self::Validator(record) => record.diagnostic_projection(),
            Self::Invariant(record) => record.diagnostic_projection(),
        }
    }
}
