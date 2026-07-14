use crate::{WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag};

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
    receipt_identity: WorthQueryEvidenceIdentity,
    lowered_identity: WorthQueryEvidenceIdentity,
    authority_artifact_identity: WorthQueryEvidenceIdentity,
    counter_snapshot_identity: WorthQueryEvidenceIdentity,
    sources_identity: WorthQueryEvidenceIdentity,
}

impl EffectEnvelopeSourceRefs {
    fn from_receipt(receipt: &EffectExecutionReceipt) -> Self {
        let receipt_identity = receipt.receipt_identity().clone();
        let lowered_identity = receipt.decision_trace().lowered_identity().clone();
        let authority_artifact_identity = receipt
            .integrity_markers()
            .authority_artifact_identity()
            .clone();
        let counter_snapshot_identity = receipt
            .integrity_markers()
            .counter_snapshot_identity()
            .clone();
        let sources_identity =
            WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::EffectIntentReceipt)
                .field_shape(
                    WorthQueryEvidenceTag::new("identity_family"),
                    "effect_envelope_source_refs_v1",
                )
                .field_evidence_identity(WorthQueryEvidenceTag::new("receipt"), &receipt_identity)
                .field_evidence_identity(WorthQueryEvidenceTag::new("lowered"), &lowered_identity)
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("authority_artifact"),
                    &authority_artifact_identity,
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("counters"),
                    &counter_snapshot_identity,
                )
                .seal();
        Self {
            receipt_identity,
            lowered_identity,
            authority_artifact_identity,
            counter_snapshot_identity,
            sources_identity,
        }
    }

    pub fn receipt_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.receipt_identity
    }

    pub fn receipt_for_reporting(&self) -> &str {
        self.receipt_identity.as_str()
    }

    pub fn lowered_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.lowered_identity
    }

    pub fn lowered_for_reporting(&self) -> &str {
        self.lowered_identity.as_str()
    }

    pub fn authority_artifact_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.authority_artifact_identity
    }

    pub fn authority_artifact_for_reporting(&self) -> &str {
        self.authority_artifact_identity.as_str()
    }

    pub fn counter_snapshot_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.counter_snapshot_identity
    }

    pub fn counter_snapshot_for_reporting(&self) -> &str {
        self.counter_snapshot_identity.as_str()
    }

    pub fn sources_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.sources_identity
    }

    pub fn sources_for_reporting(&self) -> &str {
        self.sources_identity.as_str()
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
    trace_identity: WorthQueryEvidenceIdentity,
    structural_delta_identities: Vec<WorthQueryEvidenceIdentity>,
    integrity_identity: WorthQueryEvidenceIdentity,
    performance_identity: WorthQueryEvidenceIdentity,
    boundary_identity: WorthQueryEvidenceIdentity,
    transition_rules_identity: WorthQueryEvidenceIdentity,
    deferred_neighbors: Vec<EffectDeferredNeighborFamily>,
    sources: EffectEnvelopeSourceRefs,
    envelope_identity: WorthQueryEvidenceIdentity,
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
        let structural_delta_identities = structural_delta_identities(receipt);
        let trace_identity = receipt.decision_trace().decision_trace_identity().clone();
        let integrity_identity = receipt.integrity_markers().integrity_identity().clone();
        let sources = EffectEnvelopeSourceRefs::from_receipt(receipt);
        let transition_rules = receipt.transition_rules();
        let performance_identity =
            WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::EffectIntentReceipt)
                .field_shape(
                    WorthQueryEvidenceTag::new("identity_family"),
                    "effect_envelope_performance_v1",
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("receipt"),
                    receipt.receipt_identity(),
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("counters"),
                    &receipt.delivery_counters().evidence_identity(),
                )
                .seal();
        let boundary_identity =
            WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::EffectIntentReceipt)
                .field_shape(
                    WorthQueryEvidenceTag::new("identity_family"),
                    "effect_envelope_boundary_v1",
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("authority_lane"),
                    receipt.authority_lane().as_str(),
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("basis_lane"),
                    receipt.basis_lane().as_str(),
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("family"),
                    receipt.declared_effect_family().as_str(),
                )
                .seal();
        let deferred_neighbors = transition_rules
            .rules()
            .iter()
            .filter_map(|rule| rule.deferred_neighbor())
            .collect::<Vec<_>>();
        let transition_rules_identity = transition_rules.rules_identity().clone();
        let envelope_identity =
            WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::EffectIntentReceipt)
                .field_shape(
                    WorthQueryEvidenceTag::new("identity_family"),
                    "self_describing_effect_envelope_v1",
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("receipt"),
                    receipt.receipt_identity(),
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("primary"),
                    primary_result.as_str(),
                )
                .field_evidence_identity_sequence(
                    WorthQueryEvidenceTag::new("delta"),
                    structural_delta_identities.iter(),
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("integrity"),
                    &integrity_identity,
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("performance"),
                    &performance_identity,
                )
                .field_evidence_identity(WorthQueryEvidenceTag::new("boundary"), &boundary_identity)
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("transitions"),
                    &transition_rules_identity,
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("sources"),
                    &sources.sources_identity,
                )
                .seal();
        Self {
            declared_effect_family: receipt.declared_effect_family(),
            authority_lane: receipt.authority_lane(),
            authority_owner: receipt.authority_owner(),
            basis_lane: receipt.basis_lane(),
            receipt_family: receipt.receipt_family(),
            primary_result,
            warnings: Vec::new(),
            trace_identity,
            structural_delta_identities,
            integrity_identity,
            performance_identity,
            boundary_identity,
            transition_rules_identity,
            deferred_neighbors,
            sources,
            envelope_identity,
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

    pub fn trace_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.trace_identity
    }

    pub fn trace_for_reporting(&self) -> &str {
        self.trace_identity.as_str()
    }

    pub fn structural_delta_identities(&self) -> &[WorthQueryEvidenceIdentity] {
        &self.structural_delta_identities
    }

    pub fn integrity_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.integrity_identity
    }

    pub fn integrity_for_reporting(&self) -> &str {
        self.integrity_identity.as_str()
    }

    pub fn performance_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.performance_identity
    }

    pub fn performance_for_reporting(&self) -> &str {
        self.performance_identity.as_str()
    }

    pub fn boundary_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.boundary_identity
    }

    pub fn boundary_for_reporting(&self) -> &str {
        self.boundary_identity.as_str()
    }

    pub fn transition_rules_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.transition_rules_identity
    }

    pub fn transition_rules_for_reporting(&self) -> &str {
        self.transition_rules_identity.as_str()
    }

    pub fn deferred_neighbors(&self) -> &[EffectDeferredNeighborFamily] {
        &self.deferred_neighbors
    }

    pub fn sources(&self) -> &EffectEnvelopeSourceRefs {
        &self.sources
    }

    pub fn envelope_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.envelope_identity
    }

    pub fn envelope_for_reporting(&self) -> &str {
        self.envelope_identity.as_str()
    }
}

fn structural_delta_identities(
    receipt: &EffectExecutionReceipt,
) -> Vec<WorthQueryEvidenceIdentity> {
    match receipt.target_evidence() {
        EffectReceiptTargetEvidence::MutationCommit {
            commit_id,
            version_id,
        } => {
            vec![
                WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::EffectIntentReceipt)
                    .field_shape(
                        WorthQueryEvidenceTag::new("identity_family"),
                        "effect_envelope_structural_delta_v1",
                    )
                    .field_shape(WorthQueryEvidenceTag::new("kind"), "mutation_commit")
                    .field_usize(WorthQueryEvidenceTag::new("commit_id"), commit_id as usize)
                    .field_usize(
                        WorthQueryEvidenceTag::new("version_id"),
                        version_id as usize,
                    )
                    .seal(),
            ]
        }
        EffectReceiptTargetEvidence::MergeCommit {
            commit_id,
            version_id,
        } => {
            vec![
                WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::EffectIntentReceipt)
                    .field_shape(
                        WorthQueryEvidenceTag::new("identity_family"),
                        "effect_envelope_structural_delta_v1",
                    )
                    .field_shape(WorthQueryEvidenceTag::new("kind"), "merge_commit")
                    .field_usize(WorthQueryEvidenceTag::new("commit_id"), commit_id as usize)
                    .field_usize(
                        WorthQueryEvidenceTag::new("version_id"),
                        version_id as usize,
                    )
                    .seal(),
            ]
        }
        EffectReceiptTargetEvidence::Writeback {
            outcome_identity,
            authority_receipt_identity,
            execution_receipt_identity,
        } => {
            vec![
                WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::EffectIntentReceipt)
                    .field_shape(
                        WorthQueryEvidenceTag::new("identity_family"),
                        "effect_envelope_structural_delta_v1",
                    )
                    .field_shape(WorthQueryEvidenceTag::new("kind"), "writeback")
                    .field_evidence_identity(
                        WorthQueryEvidenceTag::new("outcome"),
                        &outcome_identity,
                    )
                    .field_evidence_identity(
                        WorthQueryEvidenceTag::new("authority_receipt"),
                        &authority_receipt_identity,
                    )
                    .field_evidence_identity(
                        WorthQueryEvidenceTag::new("execution_receipt"),
                        &execution_receipt_identity,
                    )
                    .seal(),
            ]
        }
        EffectReceiptTargetEvidence::BatchMutation {
            commit_id,
            version_id,
            component_count,
        } => {
            vec![
                WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::EffectIntentReceipt)
                    .field_shape(
                        WorthQueryEvidenceTag::new("identity_family"),
                        "effect_envelope_structural_delta_v1",
                    )
                    .field_shape(WorthQueryEvidenceTag::new("kind"), "batch_commit")
                    .field_usize(WorthQueryEvidenceTag::new("commit_id"), commit_id as usize)
                    .field_usize(
                        WorthQueryEvidenceTag::new("version_id"),
                        version_id as usize,
                    )
                    .field_usize(
                        WorthQueryEvidenceTag::new("component_count"),
                        component_count,
                    )
                    .seal(),
            ]
        }
    }
}
