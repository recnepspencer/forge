use crate::identity::hash_parts;
use crate::runtime::{
    ForgeQueryAuthorityLane, ForgeQueryBatchMutationEvidence, ForgeQueryRuntimeError,
};

use forge_runtime_bridge::facade::BridgeBatchMutationAuthorityBundle;

use super::ForgeQueryWriteReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBatchWriteReceipt {
    write_receipts: Vec<ForgeQueryWriteReceipt>,
    authority_lane: ForgeQueryAuthorityLane,
    basis_lane: ForgeQueryAuthorityLane,
    batch_mutation_evidence: ForgeQueryBatchMutationEvidence,
    batch_digest: String,
    touched_aspect_paths: Vec<String>,
    affected_live_view_ids: Vec<String>,
    affected_derived_view_ids: Vec<String>,
    considered_computed_view_count: usize,
    considered_effect_count: usize,
    delivered_effect_count: usize,
    pending_write_intent_count: usize,
    suppressed_effect_count: usize,
    meaningful_effect_suppression_count: usize,
    effect_expression_failure_count: usize,
    refresh_fallback: bool,
}

impl ForgeQueryBatchWriteReceipt {
    pub(in crate::runtime) fn new(
        write_receipts: Vec<ForgeQueryWriteReceipt>,
        authority_lane: ForgeQueryAuthorityLane,
        basis_lane: ForgeQueryAuthorityLane,
        touched_aspect_paths: Vec<String>,
        affected_live_view_ids: Vec<String>,
        affected_derived_view_ids: Vec<String>,
        considered_computed_view_count: usize,
        considered_effect_count: usize,
        delivered_effect_count: usize,
        pending_write_intent_count: usize,
        suppressed_effect_count: usize,
        meaningful_effect_suppression_count: usize,
        effect_expression_failure_count: usize,
        refresh_fallback: bool,
    ) -> Result<Self, ForgeQueryRuntimeError> {
        if write_receipts.is_empty() {
            return Err(ForgeQueryRuntimeError::Workspace(
                crate::memory_workspace::ForgeQueryWorkspaceError::new(
                    "mutation batch must produce at least one write receipt",
                ),
            ));
        }
        if write_receipts
            .iter()
            .any(|receipt| receipt.authority_lane() != authority_lane)
        {
            return Err(ForgeQueryRuntimeError::Workspace(
                crate::memory_workspace::ForgeQueryWorkspaceError::new(
                    "mutation batch may not mix authority lanes",
                ),
            ));
        }

        let batch_digest = hash_parts(
            &std::iter::once("forge_query_batch_write_receipt_v1".to_string())
                .chain(
                    write_receipts
                        .iter()
                        .map(|receipt| format!("commit:{}", receipt.commit_identity())),
                )
                .chain(
                    touched_aspect_paths
                        .iter()
                        .map(|path| format!("aspect:{path}")),
                )
                .chain(
                    affected_live_view_ids
                        .iter()
                        .map(|view| format!("live:{view}")),
                )
                .chain(
                    affected_derived_view_ids
                        .iter()
                        .map(|view| format!("derived:{view}")),
                )
                .collect::<Vec<_>>(),
        );

        let target_evidence = write_receipts
            .iter()
            .map(|receipt| receipt.target_evidence().clone())
            .collect::<Vec<_>>();
        let mutation_families = write_receipts
            .iter()
            .map(ForgeQueryWriteReceipt::mutation_family)
            .collect::<Vec<_>>();
        let existing_truth_assertions = write_receipts
            .iter()
            .map(|receipt| receipt.existing_truth_assertion_evidence().cloned())
            .collect::<Vec<_>>();
        let existing_truth_bindings = write_receipts
            .iter()
            .map(|receipt| receipt.existing_truth_binding_evidence().cloned())
            .collect::<Vec<_>>();
        let symbolic_target_references = write_receipts
            .iter()
            .map(|receipt| receipt.symbolic_target_reference_evidence().cloned())
            .collect::<Vec<_>>();
        let naming_mutations = write_receipts
            .iter()
            .map(|receipt| receipt.naming_mutation_evidence().cloned())
            .collect::<Vec<_>>();
        let continuity_mutations = write_receipts
            .iter()
            .map(|receipt| receipt.continuity_mutation_evidence().cloned())
            .collect::<Vec<_>>();
        let aggregate_bridge = batch_bridge_evidence_from_receipts(&write_receipts);
        let batch_mutation_evidence = ForgeQueryBatchMutationEvidence::from_components(
            &mutation_families,
            &target_evidence,
            &existing_truth_assertions,
            &existing_truth_bindings,
            &symbolic_target_references,
            &naming_mutations,
            &continuity_mutations,
            aggregate_bridge.as_ref(),
        )
        .expect("non-empty mutation batch must produce batch evidence");

        Ok(Self {
            write_receipts,
            authority_lane,
            basis_lane,
            batch_mutation_evidence,
            batch_digest,
            touched_aspect_paths,
            affected_live_view_ids,
            affected_derived_view_ids,
            considered_computed_view_count,
            considered_effect_count,
            delivered_effect_count,
            pending_write_intent_count,
            suppressed_effect_count,
            meaningful_effect_suppression_count,
            effect_expression_failure_count,
            refresh_fallback,
        })
    }

    pub fn from_write_receipts(
        write_receipts: Vec<ForgeQueryWriteReceipt>,
    ) -> Result<Self, ForgeQueryRuntimeError> {
        if write_receipts.is_empty() {
            return Err(ForgeQueryRuntimeError::Workspace(
                crate::memory_workspace::ForgeQueryWorkspaceError::new(
                    "mutation batch must produce at least one write receipt",
                ),
            ));
        }
        let authority_lane = write_receipts[0].authority_lane();
        let basis_lane = write_receipts[0].basis_lane();
        if write_receipts
            .iter()
            .any(|receipt| receipt.authority_lane() != authority_lane)
        {
            return Err(ForgeQueryRuntimeError::Workspace(
                crate::memory_workspace::ForgeQueryWorkspaceError::new(
                    "mutation batch may not mix authority lanes",
                ),
            ));
        }
        if write_receipts
            .iter()
            .any(|receipt| receipt.basis_lane() != basis_lane)
        {
            return Err(ForgeQueryRuntimeError::Workspace(
                crate::memory_workspace::ForgeQueryWorkspaceError::new(
                    "mutation batch may not mix basis lanes",
                ),
            ));
        }

        let mut touched_aspect_paths = write_receipts
            .iter()
            .flat_map(|receipt| {
                receipt
                    .deltas()
                    .iter()
                    .flat_map(|delta| delta.aspect_paths.iter().cloned())
            })
            .collect::<Vec<_>>();
        touched_aspect_paths.sort();
        touched_aspect_paths.dedup();

        let mut affected_live_view_ids = write_receipts
            .iter()
            .flat_map(|receipt| receipt.affected_live_view_ids().iter().cloned())
            .collect::<Vec<_>>();
        affected_live_view_ids.sort();
        affected_live_view_ids.dedup();

        let mut affected_derived_view_ids = write_receipts
            .iter()
            .flat_map(|receipt| receipt.affected_derived_view_ids().iter().cloned())
            .collect::<Vec<_>>();
        affected_derived_view_ids.sort();
        affected_derived_view_ids.dedup();

        let considered_computed_view_count = write_receipts
            .iter()
            .map(ForgeQueryWriteReceipt::considered_computed_view_count)
            .sum();
        let considered_effect_count = write_receipts
            .iter()
            .map(ForgeQueryWriteReceipt::considered_effect_count)
            .sum();
        let delivered_effect_count = write_receipts
            .iter()
            .map(ForgeQueryWriteReceipt::delivered_effect_count)
            .sum();
        let pending_write_intent_count = write_receipts
            .iter()
            .map(ForgeQueryWriteReceipt::pending_write_intent_count)
            .sum();
        let suppressed_effect_count = write_receipts
            .iter()
            .map(ForgeQueryWriteReceipt::suppressed_effect_count)
            .sum();
        let meaningful_effect_suppression_count = write_receipts
            .iter()
            .map(ForgeQueryWriteReceipt::meaningful_effect_suppression_count)
            .sum();
        let effect_expression_failure_count = write_receipts
            .iter()
            .map(ForgeQueryWriteReceipt::effect_expression_failure_count)
            .sum();
        let refresh_fallback = write_receipts
            .iter()
            .any(ForgeQueryWriteReceipt::refresh_fallback);

        Self::new(
            write_receipts,
            authority_lane,
            basis_lane,
            touched_aspect_paths,
            affected_live_view_ids,
            affected_derived_view_ids,
            considered_computed_view_count,
            considered_effect_count,
            delivered_effect_count,
            pending_write_intent_count,
            suppressed_effect_count,
            meaningful_effect_suppression_count,
            effect_expression_failure_count,
            refresh_fallback,
        )
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }

    pub fn basis_lane(&self) -> ForgeQueryAuthorityLane {
        self.basis_lane
    }

    pub fn batch_digest(&self) -> &str {
        &self.batch_digest
    }

    pub fn batch_mutation_evidence(&self) -> &ForgeQueryBatchMutationEvidence {
        &self.batch_mutation_evidence
    }

    pub fn write_count(&self) -> usize {
        self.write_receipts.len()
    }

    pub fn write_receipts(&self) -> &[ForgeQueryWriteReceipt] {
        &self.write_receipts
    }

    pub fn touched_aspect_paths(&self) -> &[String] {
        &self.touched_aspect_paths
    }

    pub fn affected_live_view_ids(&self) -> &[String] {
        &self.affected_live_view_ids
    }

    pub fn affected_derived_view_ids(&self) -> &[String] {
        &self.affected_derived_view_ids
    }

    pub fn considered_computed_view_count(&self) -> usize {
        self.considered_computed_view_count
    }

    pub fn considered_effect_count(&self) -> usize {
        self.considered_effect_count
    }

    pub fn delivered_effect_count(&self) -> usize {
        self.delivered_effect_count
    }

    pub fn pending_write_intent_count(&self) -> usize {
        self.pending_write_intent_count
    }

    pub fn suppressed_effect_count(&self) -> usize {
        self.suppressed_effect_count
    }

    pub fn meaningful_effect_suppression_count(&self) -> usize {
        self.meaningful_effect_suppression_count
    }

    pub fn effect_expression_failure_count(&self) -> usize {
        self.effect_expression_failure_count
    }

    pub fn refresh_fallback(&self) -> bool {
        self.refresh_fallback
    }
}

fn batch_bridge_evidence_from_receipts(
    write_receipts: &[ForgeQueryWriteReceipt],
) -> Option<BridgeBatchMutationAuthorityBundle> {
    let components = write_receipts
        .iter()
        .filter_map(|receipt| receipt.inner.bridge_authority.clone())
        .collect::<Vec<_>>();
    if components.len() != write_receipts.len() {
        return None;
    }
    BridgeBatchMutationAuthorityBundle::from_components(&components)
}
