use crate::workload_platform::evidence_lookup_plan_selection::{
    EvidenceLookupPlanTopologyPosture, EvidenceLookupPlanTopologyPostureState,
};

use super::error::{EvidenceLookupStageCutoverError, EvidenceLookupStageCutoverErrorKind};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupTopologyDerivedReceiptRef {
    seed_digest: String,
    receipt_ref_digest: String,
    family_identity: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EvidenceLookupTopologyDerivedReceiptState {
    NotRequired,
    ReceiptRef(EvidenceLookupTopologyDerivedReceiptRef),
}

impl EvidenceLookupTopologyDerivedReceiptRef {
    pub fn seed_digest(&self) -> &str {
        &self.seed_digest
    }

    pub fn receipt_ref_digest(&self) -> &str {
        &self.receipt_ref_digest
    }

    pub fn family_identity(&self) -> &str {
        &self.family_identity
    }
}

impl EvidenceLookupTopologyDerivedReceiptState {
    pub(crate) fn from_plan_topology_posture(
        posture: &EvidenceLookupPlanTopologyPosture,
        family_identity: &str,
    ) -> Result<Self, EvidenceLookupStageCutoverError> {
        match posture.state() {
            EvidenceLookupPlanTopologyPostureState::NotRequired => Ok(Self::NotRequired),
            EvidenceLookupPlanTopologyPostureState::Satisfied {
                seed_digest,
                receipt_ref_digest,
                family_identity: topology_family_identity,
            } => Ok(Self::ReceiptRef(EvidenceLookupTopologyDerivedReceiptRef {
                seed_digest: seed_digest.clone(),
                receipt_ref_digest: receipt_ref_digest.clone(),
                family_identity: (*topology_family_identity).to_string(),
            })),
            EvidenceLookupPlanTopologyPostureState::RequiredButMissing { .. } => Err(
                EvidenceLookupStageCutoverError::new(
                    EvidenceLookupStageCutoverErrorKind::MissingTopologyDerivedReceipt,
                    format!(
                        "covered lookup family `{family_identity}` requires topology-derived receipt refs"
                    ),
                ),
            ),
            EvidenceLookupPlanTopologyPostureState::NotEvaluatedForUnaffectedFamily => Err(
                EvidenceLookupStageCutoverError::new(
                    EvidenceLookupStageCutoverErrorKind::ScopeExpansionDenied,
                    format!(
                        "covered lookup family `{family_identity}` cannot use unaffected-family topology posture"
                    ),
                ),
            ),
        }
    }
}
