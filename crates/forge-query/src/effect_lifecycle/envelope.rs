use crate::identity::hash_parts;

use super::inventory::EffectReceiptArtifactKind;
use super::planning::EffectAuthorityOwner;
use super::receipt::{EffectExecutionReceipt, EffectReceiptTargetEvidence};
use super::support_contract::EffectDeferredNeighborFamily;
use super::taxonomy::EffectAuthorityLane;
use super::taxonomy::EffectFamily;
use crate::basis_lifecycle::BasisFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectEnvelopePrimaryResult {
    MutationCommitted,
    MergeCommitted,
    WritebackCommitted,
    BatchMutationCommitted,
}

impl EffectEnvelopePrimaryResult {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MutationCommitted => "mutation_committed",
            Self::MergeCommitted => "merge_committed",
            Self::WritebackCommitted => "writeback_committed",
            Self::BatchMutationCommitted => "batch_mutation_committed",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectEnvelopeSourceRefs {
    receipt_digest: String,
    lowered_digest: String,
    authority_artifact_digest: String,
    counter_snapshot_digest: String,
    sources_digest: String,
}

impl EffectEnvelopeSourceRefs {
    fn from_receipt(receipt: &EffectExecutionReceipt) -> Self {
        let receipt_digest = receipt.receipt_digest().to_string();
        let lowered_digest = receipt.lowered_digest().to_string();
        let authority_artifact_digest = receipt
            .integrity_markers()
            .authority_artifact_digest()
            .to_string();
        let counter_snapshot_digest = receipt
            .integrity_markers()
            .counter_snapshot_digest()
            .to_string();
        let sources_digest = hash_parts(&[
            "effect_envelope_source_refs_v1".to_string(),
            format!("receipt:{receipt_digest}"),
            format!("lowered:{lowered_digest}"),
            format!("authority_artifact:{authority_artifact_digest}"),
            format!("counters:{counter_snapshot_digest}"),
        ]);
        Self {
            receipt_digest,
            lowered_digest,
            authority_artifact_digest,
            counter_snapshot_digest,
            sources_digest,
        }
    }

    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }

    pub fn lowered_digest(&self) -> &str {
        &self.lowered_digest
    }

    pub fn authority_artifact_digest(&self) -> &str {
        &self.authority_artifact_digest
    }

    pub fn counter_snapshot_digest(&self) -> &str {
        &self.counter_snapshot_digest
    }

    pub fn sources_digest(&self) -> &str {
        &self.sources_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelfDescribingEffectEnvelope {
    declared_effect_family: EffectFamily,
    authority_lane: EffectAuthorityLane,
    authority_owner: EffectAuthorityOwner,
    basis_lane: BasisFamily,
    receipt_family: EffectReceiptArtifactKind,
    primary_result: EffectEnvelopePrimaryResult,
    warnings: Vec<String>,
    trace_digest: String,
    structural_deltas: Vec<String>,
    integrity_digest: String,
    performance_digest: String,
    boundary_digest: String,
    transition_rules_digest: String,
    deferred_neighbors: Vec<EffectDeferredNeighborFamily>,
    sources: EffectEnvelopeSourceRefs,
    envelope_digest: String,
}

impl SelfDescribingEffectEnvelope {
    pub(super) fn from_receipt(receipt: &EffectExecutionReceipt) -> Self {
        let primary_result = match receipt.target_evidence() {
            EffectReceiptTargetEvidence::MutationCommit { .. } => {
                EffectEnvelopePrimaryResult::MutationCommitted
            }
            EffectReceiptTargetEvidence::MergeCommit { .. } => {
                EffectEnvelopePrimaryResult::MergeCommitted
            }
            EffectReceiptTargetEvidence::Writeback { .. } => {
                EffectEnvelopePrimaryResult::WritebackCommitted
            }
            EffectReceiptTargetEvidence::BatchMutation { .. } => {
                EffectEnvelopePrimaryResult::BatchMutationCommitted
            }
        };
        let structural_deltas = structural_deltas(receipt);
        let integrity_digest = receipt.integrity_markers().integrity_digest().to_string();
        let sources = EffectEnvelopeSourceRefs::from_receipt(receipt);
        let transition_rules = receipt.transition_rules();
        let performance_digest = hash_parts(&[
            "effect_envelope_performance_v1".to_string(),
            format!("receipt:{}", receipt.receipt_digest()),
            format!("counters:{}", receipt.delivery_counters().digest()),
        ]);
        let boundary_digest = hash_parts(&[
            "effect_envelope_boundary_v1".to_string(),
            format!("authority_lane:{}", receipt.authority_lane().as_str()),
            format!("basis_lane:{}", receipt.basis_lane().as_str()),
            format!("family:{}", receipt.declared_effect_family().as_str()),
        ]);
        let deferred_neighbors = transition_rules
            .rules()
            .iter()
            .filter_map(|rule| rule.deferred_neighbor())
            .collect::<Vec<_>>();
        let transition_rules_digest = transition_rules.rules_digest().to_string();
        let envelope_digest = hash_parts(
            &std::iter::once("self_describing_effect_envelope_v1".to_string())
                .chain(std::iter::once(format!(
                    "receipt:{}",
                    receipt.receipt_digest()
                )))
                .chain(std::iter::once(format!(
                    "primary:{}",
                    primary_result.as_str()
                )))
                .chain(
                    structural_deltas
                        .iter()
                        .map(|delta| format!("delta:{delta}")),
                )
                .chain(std::iter::once(format!("integrity:{integrity_digest}")))
                .chain(std::iter::once(format!("performance:{performance_digest}")))
                .chain(std::iter::once(format!("boundary:{boundary_digest}")))
                .chain(std::iter::once(format!(
                    "transitions:{transition_rules_digest}"
                )))
                .chain(std::iter::once(format!(
                    "sources:{}",
                    sources.sources_digest()
                )))
                .collect::<Vec<_>>(),
        );
        Self {
            declared_effect_family: receipt.declared_effect_family(),
            authority_lane: receipt.authority_lane(),
            authority_owner: receipt.authority_owner(),
            basis_lane: receipt.basis_lane(),
            receipt_family: receipt.receipt_family(),
            primary_result,
            warnings: Vec::new(),
            trace_digest: receipt.decision_trace().decision_trace_digest().to_string(),
            structural_deltas,
            integrity_digest,
            performance_digest,
            boundary_digest,
            transition_rules_digest,
            deferred_neighbors,
            sources,
            envelope_digest,
        }
    }

    pub fn declared_effect_family(&self) -> EffectFamily {
        self.declared_effect_family
    }

    pub fn authority_lane(&self) -> EffectAuthorityLane {
        self.authority_lane
    }

    pub fn authority_owner(&self) -> EffectAuthorityOwner {
        self.authority_owner
    }

    pub fn basis_lane(&self) -> BasisFamily {
        self.basis_lane
    }

    pub fn receipt_family(&self) -> EffectReceiptArtifactKind {
        self.receipt_family
    }

    pub fn primary_result(&self) -> EffectEnvelopePrimaryResult {
        self.primary_result
    }

    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }

    pub fn trace_digest(&self) -> &str {
        &self.trace_digest
    }

    pub fn structural_deltas(&self) -> &[String] {
        &self.structural_deltas
    }

    pub fn integrity_digest(&self) -> &str {
        &self.integrity_digest
    }

    pub fn performance_digest(&self) -> &str {
        &self.performance_digest
    }

    pub fn boundary_digest(&self) -> &str {
        &self.boundary_digest
    }

    pub fn transition_rules_digest(&self) -> &str {
        &self.transition_rules_digest
    }

    pub fn deferred_neighbors(&self) -> &[EffectDeferredNeighborFamily] {
        &self.deferred_neighbors
    }

    pub fn sources(&self) -> &EffectEnvelopeSourceRefs {
        &self.sources
    }

    pub fn envelope_digest(&self) -> &str {
        &self.envelope_digest
    }
}

fn structural_deltas(receipt: &EffectExecutionReceipt) -> Vec<String> {
    match receipt.target_evidence() {
        EffectReceiptTargetEvidence::MutationCommit {
            commit_id,
            version_id,
        } => vec![format!("mutation_commit:{commit_id}:{version_id}")],
        EffectReceiptTargetEvidence::MergeCommit {
            commit_id,
            version_id,
        } => vec![format!("merge_commit:{commit_id}:{version_id}")],
        EffectReceiptTargetEvidence::Writeback {
            outcome_digest,
            receipt_digest,
        } => vec![format!("writeback:{outcome_digest}:{receipt_digest}")],
        EffectReceiptTargetEvidence::BatchMutation {
            commit_id,
            version_id,
            component_count,
        } => vec![format!(
            "batch_commit:{commit_id}:{version_id}:components:{component_count}"
        )],
    }
}
