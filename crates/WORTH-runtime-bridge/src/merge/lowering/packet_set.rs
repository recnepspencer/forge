use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::merge::{
    AdmittedMergeHistoryContract, BridgeMergeAuthoritativeLineageDisposition,
    BridgeMergeCausalFrontierDisposition, BridgeMergeConsumptionClass, BridgeMergeCounters,
    BridgeMergeDenialClass, BridgeMergePrecedenceStage, BridgeMergeSchemaPolicyDisposition,
    BridgeMergeStageDecisionClass, BridgeMergeStructuralAdvisoryDisposition,
};

use super::parent_order::BridgeMergeParentOrderDigestBasis;
use super::stages::{
    MergeAuthoritativeLineageStage, MergeCausalFrontierStage, MergeClassAdmissionStage,
    MergeDecisionLogEntry, MergeDeletionTopologyGateStage, MergePrecedenceStageOutput,
    MergePublicationStage, MergeSchemaPolicyStage, MergeStructuralAdvisoryStage,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoweredMergeHistoryPacketSet {
    contract: AdmittedMergeHistoryContract,
    parent_order_digest_basis: BridgeMergeParentOrderDigestBasis,
    stage_outputs: Arc<[MergePrecedenceStageOutput]>,
    decision_log: Arc<[MergeDecisionLogEntry]>,
    blocked_stage: Option<BridgeMergePrecedenceStage>,
    denial_class: Option<BridgeMergeDenialClass>,
    structural_contradiction: bool,
    counters: BridgeMergeCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl LoweredMergeHistoryPacketSet {
    pub(crate) fn from_contract(contract: &AdmittedMergeHistoryContract) -> Self {
        let declaration = contract.validated_declaration().declaration();
        let parent_order_digest_basis = BridgeMergeParentOrderDigestBasis::from_contract(contract);
        let parent_count = declaration
            .authority_basis()
            .parent_order_proof()
            .parents()
            .len();
        let mut counters = BridgeMergeCounters::for_contract(parent_count, parent_count)
            .with_supported_class()
            .with_lineage_resolution_width(1)
            .with_packet();
        let mut stage_outputs = Vec::new();
        let mut decision_log = Vec::new();
        let mut blocked_stage = None;
        let mut denial_class = None;
        let mut structural_contradiction = false;

        stage_outputs.push(MergePrecedenceStageOutput::MergeClassAdmission(
            MergeClassAdmissionStage::new(declaration.bridge_class()),
        ));
        decision_log.push(MergeDecisionLogEntry::new(
            BridgeMergePrecedenceStage::MergeClassAdmission,
            BridgeMergeStageDecisionClass::Admitted,
            format!(
                "merge class `{:?}` remained admitted through canonical ontology mapping.",
                declaration.bridge_class()
            ),
        ));

        stage_outputs.push(MergePrecedenceStageOutput::AuthoritativeLineage(
            MergeAuthoritativeLineageStage::new(declaration.authoritative_lineage()),
        ));
        match declaration.authoritative_lineage() {
            BridgeMergeAuthoritativeLineageDisposition::CanonicalSuccessor => {
                decision_log.push(MergeDecisionLogEntry::new(
                    BridgeMergePrecedenceStage::AuthoritativeLineage,
                    BridgeMergeStageDecisionClass::Admitted,
                    "authoritative lineage exported a canonical successor surface.",
                ));
            }
            BridgeMergeAuthoritativeLineageDisposition::NoAuthoritativeSuccessor => {
                blocked_stage = Some(BridgeMergePrecedenceStage::AuthoritativeLineage);
                denial_class = Some(BridgeMergeDenialClass::NoAuthoritativeSuccessor);
                counters = counters.with_continuity_denial();
                decision_log.push(MergeDecisionLogEntry::new(
                    BridgeMergePrecedenceStage::AuthoritativeLineage,
                    BridgeMergeStageDecisionClass::Denied,
                    "authoritative lineage denied continuation because no canonical successor was exported.",
                ));
            }
        }

        if blocked_stage.is_none() {
            let gate_denial = match declaration.bridge_class() {
                BridgeMergeConsumptionClass::DeletionMerge => {
                    counters = counters.with_deletion_class();
                    Some(BridgeMergeDenialClass::DeletionGate)
                }
                BridgeMergeConsumptionClass::TopologyRewireMerge => {
                    counters = counters.with_topology_rewire_class();
                    Some(BridgeMergeDenialClass::TopologyRewireGate)
                }
                BridgeMergeConsumptionClass::AspectReconciliationMerge
                | BridgeMergeConsumptionClass::PolicyResolvedConflictMerge => None,
            };
            stage_outputs.push(MergePrecedenceStageOutput::DeletionTopologyGate(
                MergeDeletionTopologyGateStage::new(declaration.bridge_class(), gate_denial),
            ));
            if let Some(gate_denial) = gate_denial {
                blocked_stage = Some(BridgeMergePrecedenceStage::DeletionTopologyGate);
                denial_class = Some(gate_denial);
                counters = counters.with_continuity_denial();
                decision_log.push(MergeDecisionLogEntry::new(
                    BridgeMergePrecedenceStage::DeletionTopologyGate,
                    BridgeMergeStageDecisionClass::Denied,
                    format!(
                        "merge class `{:?}` was denied at the deletion/topology gate.",
                        declaration.bridge_class()
                    ),
                ));
            } else {
                decision_log.push(MergeDecisionLogEntry::new(
                    BridgeMergePrecedenceStage::DeletionTopologyGate,
                    BridgeMergeStageDecisionClass::Admitted,
                    "merge class passed deletion/topology denial gating.",
                ));
            }
        }

        if blocked_stage.is_none() {
            stage_outputs.push(MergePrecedenceStageOutput::CausalFrontierAdmissibility(
                MergeCausalFrontierStage::new(declaration.causal_frontier()),
            ));
            counters = counters.with_causal_frontier_lookup();
            match declaration.causal_frontier() {
                BridgeMergeCausalFrontierDisposition::Admitted => {
                    decision_log.push(MergeDecisionLogEntry::new(
                        BridgeMergePrecedenceStage::CausalFrontierAdmissibility,
                        BridgeMergeStageDecisionClass::Admitted,
                        "causal frontier remained admissible for merge routing.",
                    ));
                }
                BridgeMergeCausalFrontierDisposition::Truncated => {
                    blocked_stage = Some(BridgeMergePrecedenceStage::CausalFrontierAdmissibility);
                    denial_class = Some(BridgeMergeDenialClass::CausalFrontierTruncated);
                    counters = counters.with_continuity_denial();
                    decision_log.push(MergeDecisionLogEntry::new(
                        BridgeMergePrecedenceStage::CausalFrontierAdmissibility,
                        BridgeMergeStageDecisionClass::Denied,
                        "causal frontier was truncated before continuity publication.",
                    ));
                }
            }
        }

        if blocked_stage.is_none() {
            stage_outputs.push(
                MergePrecedenceStageOutput::SchemaPolicyOutcomeAdmissibility(
                    MergeSchemaPolicyStage::new(declaration.schema_policy()),
                ),
            );
            counters = counters.with_policy_outcome();
            match declaration.schema_policy() {
                BridgeMergeSchemaPolicyDisposition::Admitted => {
                    decision_log.push(MergeDecisionLogEntry::new(
                        BridgeMergePrecedenceStage::SchemaPolicyOutcomeAdmissibility,
                        BridgeMergeStageDecisionClass::Admitted,
                        "schema policy outcome admitted merge-aware continuation.",
                    ));
                }
                BridgeMergeSchemaPolicyDisposition::Rejected => {
                    blocked_stage =
                        Some(BridgeMergePrecedenceStage::SchemaPolicyOutcomeAdmissibility);
                    denial_class = Some(BridgeMergeDenialClass::SchemaPolicyRejected);
                    counters = counters.with_continuity_denial();
                    decision_log.push(MergeDecisionLogEntry::new(
                        BridgeMergePrecedenceStage::SchemaPolicyOutcomeAdmissibility,
                        BridgeMergeStageDecisionClass::Denied,
                        "schema policy outcome rejected merge-aware continuation.",
                    ));
                }
            }
        }

        if blocked_stage.is_none() {
            stage_outputs.push(MergePrecedenceStageOutput::StructuralAdvisoryRefinement(
                MergeStructuralAdvisoryStage::new(declaration.structural_advisory()),
            ));
            match declaration.structural_advisory() {
                BridgeMergeStructuralAdvisoryDisposition::NotConsulted => {
                    decision_log.push(MergeDecisionLogEntry::new(
                        BridgeMergePrecedenceStage::StructuralAdvisoryRefinement,
                        BridgeMergeStageDecisionClass::Admitted,
                        "structural advisory evidence was not consulted for this merge route.",
                    ));
                }
                BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent => {
                    counters = counters.with_structural_consult_width(1);
                    decision_log.push(MergeDecisionLogEntry::new(
                        BridgeMergePrecedenceStage::StructuralAdvisoryRefinement,
                        BridgeMergeStageDecisionClass::Refined,
                        "structural advisory evidence remained consistent with merge authority.",
                    ));
                }
                BridgeMergeStructuralAdvisoryDisposition::AdvisoryContradiction => {
                    counters = counters
                        .with_structural_consult_width(1)
                        .with_structural_contradiction();
                    structural_contradiction = true;
                    decision_log.push(MergeDecisionLogEntry::new(
                        BridgeMergePrecedenceStage::StructuralAdvisoryRefinement,
                        BridgeMergeStageDecisionClass::Denied,
                        "structural advisory evidence contradicted admitted merge authority and was localized as a typed contradiction.",
                    ));
                }
            }
        }

        if blocked_stage.is_none() && !structural_contradiction {
            stage_outputs.push(MergePrecedenceStageOutput::ContinuityOrRemapPublication(
                MergePublicationStage,
            ));
            counters = counters.with_continuity();
            decision_log.push(MergeDecisionLogEntry::new(
                BridgeMergePrecedenceStage::ContinuityOrRemapPublication,
                BridgeMergeStageDecisionClass::Admitted,
                "merge route remained admissible for continuity/remap publication.",
            ));
        }

        let canonical_basis = Arc::<str>::from(format!(
            "lowered-merge-history-packet-set|contract={}|parent-order-basis={}|blocked-stage={:?}|denial={:?}|structural-contradiction={}|stages={}|decisions={}",
            contract.digest(),
            parent_order_digest_basis.digest(),
            blocked_stage,
            denial_class,
            structural_contradiction,
            stage_outputs
                .iter()
                .map(|stage| format!("{stage:?}"))
                .collect::<Vec<_>>()
                .join(","),
            decision_log
                .iter()
                .map(|entry| format!("{:?}:{:?}:{}", entry.stage(), entry.decision_class(), entry.detail()))
                .collect::<Vec<_>>()
                .join("|"),
        ));
        counters = counters.with_digest(canonical_basis.len());
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            contract: contract.clone(),
            parent_order_digest_basis,
            stage_outputs: Arc::from(stage_outputs),
            decision_log: Arc::from(decision_log),
            blocked_stage,
            denial_class,
            structural_contradiction,
            counters,
            canonical_basis,
            digest: Arc::from(format!(
                "lowered-merge-history-packet-set:sha256:{digest:x}"
            )),
        }
    }

    pub fn contract(&self) -> &AdmittedMergeHistoryContract {
        &self.contract
    }

    pub fn parent_order_digest_basis(&self) -> &BridgeMergeParentOrderDigestBasis {
        &self.parent_order_digest_basis
    }

    pub fn stage_outputs(&self) -> &[MergePrecedenceStageOutput] {
        &self.stage_outputs
    }

    pub fn decision_log(&self) -> &[MergeDecisionLogEntry] {
        &self.decision_log
    }

    pub fn blocked_stage(&self) -> Option<BridgeMergePrecedenceStage> {
        self.blocked_stage
    }

    pub fn denial_class(&self) -> Option<BridgeMergeDenialClass> {
        self.denial_class
    }

    pub fn structural_contradiction(&self) -> bool {
        self.structural_contradiction
    }

    pub fn counters(&self) -> &BridgeMergeCounters {
        &self.counters
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
