use std::sync::Arc;

use super::*;
use crate::policy::{self, BridgePolicyAuthorityInputs};

impl RuntimeBridge {
    pub fn validate_policy_declaration(
        &self,
        declaration: BridgePolicyDeclaration,
    ) -> Result<ValidatedBridgePolicyDeclaration, BridgePolicyRejection> {
        ValidatedBridgePolicyDeclaration::new(declaration)
    }

    pub fn admit_policy_declaration(
        &self,
        declaration: BridgePolicyDeclaration,
    ) -> Result<AdmittedBridgePolicyContract, BridgePolicyRejection> {
        let validated = self.validate_policy_declaration(declaration)?;
        let authority_inputs = BridgePolicyAuthorityInputs::new(
            self.policy.diagnostics_tier(),
            self.policy.allow_replay_artifacts(),
            self.policy.record_route_artifacts(),
        );
        policy::admission::admit_policy_declaration(validated, authority_inputs)
    }

    pub fn lower_admitted_policy(
        &self,
        contract: &AdmittedBridgePolicyContract,
    ) -> LoweredBridgeExecutionPolicy {
        LoweredBridgeExecutionPolicy::from_contract(contract)
    }

    pub fn canonicalize_policy_provenance(
        &self,
        contract: &AdmittedBridgePolicyContract,
        lowered: &LoweredBridgeExecutionPolicy,
    ) -> BridgePolicyProvenanceRecord {
        BridgePolicyProvenanceRecord::from_contract_and_lowered(contract, lowered)
    }

    pub fn replay_policy_bundle(
        &self,
        contract: &AdmittedBridgePolicyContract,
        lowered: &LoweredBridgeExecutionPolicy,
        provenance: &BridgePolicyProvenanceRecord,
    ) -> BridgePolicyReplayBundle {
        BridgePolicyReplayBundle::from_canonical_records(contract, lowered, provenance)
    }

    pub fn summarize_policy_provenance_row(
        &self,
        label: impl Into<Arc<str>>,
        contract: &AdmittedBridgePolicyContract,
        lowered: &LoweredBridgeExecutionPolicy,
        provenance: &BridgePolicyProvenanceRecord,
        replay_bundle: &BridgePolicyReplayBundle,
    ) -> BridgePolicyProvenanceReportRow {
        BridgePolicyProvenanceReportRow::from_policy_bundle(
            label,
            contract,
            lowered,
            provenance,
            replay_bundle,
        )
    }

    pub fn summarize_policy_provenance_report(
        &self,
        rows: Vec<BridgePolicyProvenanceReportRow>,
    ) -> BridgePolicyProvenanceReport {
        BridgePolicyProvenanceReport::new(rows)
    }
}
