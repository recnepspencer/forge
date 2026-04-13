use std::sync::Arc;

use super::*;
use crate::policy::{self, BridgePolicyAuthorityInputs};

impl RuntimeBridge {
    /// Specialist admission entry for policy declarations.
    ///
    /// Most callers should use the builder or the standard path instead of
    /// performing policy declaration validation directly.
    pub fn validate_policy_declaration(
        &self,
        declaration: BridgePolicyDeclaration,
    ) -> Result<ValidatedBridgePolicyDeclaration, BridgePolicyRejection> {
        ValidatedBridgePolicyDeclaration::new(declaration)
    }

    /// Admits a policy declaration against this runtime's frozen capabilities.
    ///
    /// This is an advanced control surface for callers that need explicit
    /// policy artifacts rather than the bridge's default execution policy.
    ///
    /// ```no_run
    /// use forge_runtime_bridge::facade::{
    ///     BridgePolicyDeclaration, RuntimeBridge,
    /// };
    ///
    /// fn admit_policy(
    ///     bridge: &RuntimeBridge,
    ///     declaration: BridgePolicyDeclaration,
    /// ) {
    ///     let Ok(contract) = bridge.admit_policy_declaration(declaration) else {
    ///         return;
    ///     };
    ///     let lowered = bridge.lower_admitted_policy(&contract);
    ///     let _provenance = bridge.canonicalize_policy_provenance(&contract, &lowered);
    /// }
    /// ```
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

    /// Lowers an admitted policy contract into executable runtime policy state.
    ///
    /// This remains public for advanced and specialist workflows, but ordinary
    /// callers should not need to lower policy artifacts manually.
    pub fn lower_admitted_policy(
        &self,
        contract: &AdmittedBridgePolicyContract,
    ) -> LoweredBridgeExecutionPolicy {
        LoweredBridgeExecutionPolicy::from_contract(contract)
    }

    /// Produces the canonical provenance record for an admitted policy bundle.
    pub fn canonicalize_policy_provenance(
        &self,
        contract: &AdmittedBridgePolicyContract,
        lowered: &LoweredBridgeExecutionPolicy,
    ) -> BridgePolicyProvenanceRecord {
        BridgePolicyProvenanceRecord::from_contract_and_lowered(contract, lowered)
    }

    /// Produces the replay bundle for a canonical policy provenance set.
    pub fn replay_policy_bundle(
        &self,
        contract: &AdmittedBridgePolicyContract,
        lowered: &LoweredBridgeExecutionPolicy,
        provenance: &BridgePolicyProvenanceRecord,
    ) -> BridgePolicyReplayBundle {
        BridgePolicyReplayBundle::from_canonical_records(contract, lowered, provenance)
    }

    /// Summarizes one policy bundle into a report row for comparison or audit.
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

    /// Builds a policy provenance report from precomputed report rows.
    pub fn summarize_policy_provenance_report(
        &self,
        rows: Vec<BridgePolicyProvenanceReportRow>,
    ) -> BridgePolicyProvenanceReport {
        BridgePolicyProvenanceReport::new(rows)
    }
}
