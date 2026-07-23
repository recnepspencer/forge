use std::sync::Arc;

use worth_query_installation::facade::{
    WorthQueryConditionalNodeLocation, WorthQueryPortableConditionalNodeDeclaration,
};
use worth_signal::facade::InstalledSignalConditionalContract;

use super::{BridgeConditionalDenial, BridgeConditionalDenialKind, BridgeConditionalProviderSet};
use crate::correspondence::BridgeInstalledSemanticCorrespondence;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BridgeInstalledConditionalLoweringCounters {
    pub declaration_checks: usize,
    pub posture_checks: usize,
    pub correspondence_registrations_inspected: usize,
    pub correspondence_targets_inspected: usize,
    pub signal_graph_checks: usize,
    pub signal_node_ownership_checks: usize,
    pub dependency_registry_compilations: usize,
    pub dependency_registry_existing_key_lookups: usize,
    pub dependency_registry_batch_key_lookups: usize,
    pub dependency_registry_commits: usize,
    pub unrelated_dependency_registry_scans: usize,
    pub semantic_observation_plan_compilations: usize,
    pub signal_node_admissions: usize,
    pub correspondence_batch_preparations: usize,
    pub signal_contract_lowerings: usize,
    pub correspondence_admissions: usize,
    pub signal_targets_joined: usize,
    pub provider_checks: usize,
    pub signal_contract_installations: usize,
}

pub struct BridgeInstalledConditionalLowering {
    pub(crate) bridge_runtime_key: u64,
    pub(super) _authority:
        super::lowering_authority::BridgeInstalledConditionalLoweringAuthorityIdentity,
    pub(crate) projection: super::BridgeConditionalLoweringProjectionIdentity,
    pub(crate) declaration: WorthQueryPortableConditionalNodeDeclaration,
    pub(crate) location: WorthQueryConditionalNodeLocation,
    pub(crate) correspondences: Vec<BridgeInstalledSemanticCorrespondence>,
    pub(super) semantic_observation_plan:
        Option<super::semantic_observation_plan::BridgeConditionalSemanticObservationPlan>,
    pub(crate) signal_contract: InstalledSignalConditionalContract,
    pub(crate) providers: BridgeConditionalProviderSet,
    pub(super) provider_admission: super::provider_admission::BridgeConditionalProviderAdmission,
    pub(super) lease: Arc<super::liveness::BridgeConditionalLoweringLease>,
    pub(crate) counters: BridgeInstalledConditionalLoweringCounters,
}

impl BridgeInstalledConditionalLowering {
    pub fn projection(&self) -> &super::BridgeConditionalLoweringProjectionIdentity {
        &self.projection
    }
    pub fn location(&self) -> &WorthQueryConditionalNodeLocation {
        &self.location
    }
    pub fn declaration(&self) -> &WorthQueryPortableConditionalNodeDeclaration {
        &self.declaration
    }
    pub fn signal_graph_instance_id(&self) -> u64 {
        self.signal_contract.graph_instance_id()
    }
    pub fn signal_node(&self) -> worth_signal::facade::NodeId {
        self.signal_contract.node()
    }
    pub fn counters(&self) -> BridgeInstalledConditionalLoweringCounters {
        self.counters
    }
    pub fn correspondence_count(&self) -> usize {
        self.correspondences.len()
    }

    pub fn validate_query_authority_continuity(
        &self,
        operation_identity: &str,
        runtime_authority: u64,
        installation_generation: u64,
        graph_authorities: &[(
            &str,
            &Arc<worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority>,
        )],
    ) -> Result<(), BridgeConditionalDenial> {
        if self.correspondences.iter().any(|correspondence| {
            let basis = correspondence.basis();
            basis.query_basis.as_ref() != operation_identity
                || basis.query_runtime_authority() != runtime_authority
                || basis.query_installation_generation() != installation_generation
        }) {
            return Err(BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::OperationAuthorityMismatch,
                "conditional lowering no longer joins the bound operation authority basis",
            ));
        }
        if self.correspondences.iter().any(|correspondence| {
            let basis = correspondence.basis();
            !graph_authorities.iter().any(|(role, authority)| {
                *role == basis.declared_graph_role()
                    && Arc::ptr_eq(authority, &basis.graph_authority)
            })
        }) {
            return Err(BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::GraphAuthorityMismatch,
                "conditional lowering graph participation is absent from the bound operation",
            ));
        }
        Ok(())
    }

    pub fn validate_signal_decision_contract(
        &self,
        evidence: &worth_signal::facade::SignalConditionalDecisionEvidence,
    ) -> Result<(), BridgeConditionalDenial> {
        if !self.signal_contract.retains_decision(evidence) {
            return Err(BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::SignalContractMismatch,
                "Signal decision evidence does not retain the installed conditional contract",
            ));
        }
        Ok(())
    }
}
