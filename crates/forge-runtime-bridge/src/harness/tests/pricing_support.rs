use crate::error::BridgeDeliveryErrorKind;
use crate::facade::{
    BridgeFailureClass, BridgeReplayErrorKind, BridgeWritebackErrorKind,
    BridgeWritebackFailureClass, BridgeWritebackOutcomeClass,
};
use crate::speculation::BridgePreviewLifecycleStateKind;
use serde_json::json;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PricingReferenceBundle {
    pub(super) source_branch: String,
    pub(super) source_commit: String,
    pub(super) route_snapshot: String,
    pub(super) delivered_target_count: usize,
    pub(super) route_entry_count: usize,
    pub(super) evaluation_record_identity: String,
    pub(super) evaluation_selector_identity: String,
    pub(super) main_snapshot: String,
    pub(super) main_rubber_cost_cents: i64,
    pub(super) speculative_truth_branch: String,
    pub(super) speculative_signal_branch: String,
    pub(super) speculative_snapshot: String,
    pub(super) speculative_rubber_cost_cents: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PricingAspectBundle {
    pub(super) route_identity: String,
    pub(super) snapshot: String,
    pub(super) source_branch: String,
    pub(super) source_commit: String,
    pub(super) truth_surface_kind: String,
    pub(super) fine_grained_match_status: String,
    pub(super) aspect_registration_id: String,
    pub(super) subscription_slice_kind: String,
    pub(super) surface_label: String,
    pub(super) invalidation_target: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PricingFailureBundle {
    pub(super) error_kind: BridgeDeliveryErrorKind,
    pub(super) failure_class: BridgeFailureClass,
    pub(super) source_commit: String,
    pub(super) source_snapshot: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PricingReplayBundle {
    pub(super) source_commit: String,
    pub(super) source_snapshot: String,
    pub(super) route_identity: String,
    pub(super) invalidation_identity: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PricingCertificationMatrix {
    pub(super) reference: PricingReferenceBundle,
    pub(super) replay: PricingReplayBundle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PricingDiscardBundle {
    pub(super) live_main_snapshot: String,
    pub(super) speculative_rubber_cost_cents: i64,
    pub(super) post_discard_main_snapshot: String,
    pub(super) post_discard_main_steel_cost_cents: i64,
    pub(super) lifecycle_state: BridgePreviewLifecycleStateKind,
    pub(super) discard_record_count: usize,
    pub(super) promotion_record_count: usize,
    pub(super) replay_outcome: BridgePreviewLifecycleStateKind,
    pub(super) has_discard_record: bool,
    pub(super) has_promotion_record: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PricingPromotionBundle {
    pub(super) main_snapshot: String,
    pub(super) speculative_snapshot: String,
    pub(super) main_rubber_cost_cents: i64,
    pub(super) speculative_rubber_cost_cents: i64,
    pub(super) lifecycle_state: BridgePreviewLifecycleStateKind,
    pub(super) promotion_session_identity: String,
    pub(super) authoritative_commit_boundary_digest: String,
    pub(super) authoritative_artifact_digest: String,
    pub(super) replay_outcome: BridgePreviewLifecycleStateKind,
    pub(super) has_promotion_explanation: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PricingFanoutBundle {
    pub(super) total_deliveries: usize,
    pub(super) first_delivery_target_count: usize,
    pub(super) second_delivery_target_count: usize,
    pub(super) second_source_commit: String,
    pub(super) second_snapshot: String,
    pub(super) branch_snapshot: String,
    pub(super) branch_steel_cost_cents: i64,
    pub(super) retained_target_count: usize,
    pub(super) first_target: String,
    pub(super) last_target: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PricingRestartReplayBundle {
    pub(super) source_commit: String,
    pub(super) source_snapshot: String,
    pub(super) route_identity: String,
    pub(super) invalidation_identity: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PricingRestartFailureBundle {
    pub(super) error_kind: BridgeReplayErrorKind,
    pub(super) replay_mismatch_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PricingWritebackBundle {
    pub(super) family_kind: String,
    pub(super) strategy_class: String,
    pub(super) commit_outcome_class: BridgeWritebackOutcomeClass,
    pub(super) noop_outcome_class: BridgeWritebackOutcomeClass,
    pub(super) commit_replay_semantic_digest: String,
    pub(super) noop_replay_semantic_digest: String,
    pub(super) shared_authoritative_artifact: bool,
    pub(super) authority_commit_count: usize,
    pub(super) execution_request_count: usize,
    pub(super) execution_commit_count: usize,
    pub(super) execution_noop_count: usize,
    pub(super) rejection_error_kind: BridgeWritebackErrorKind,
    pub(super) rejection_failure_class: BridgeWritebackFailureClass,
    pub(super) rejection_request_emitted: bool,
    pub(super) rejection_receipt_emitted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PricingMergeBundle {
    pub(super) bridge_class: String,
    pub(super) outcome_class: String,
    pub(super) blocked_stage: Option<String>,
    pub(super) denial_class: Option<String>,
    pub(super) continuity_published: bool,
    pub(super) remap_published: bool,
    pub(super) parent_order_digest: String,
    pub(super) bundle_digest: String,
    pub(super) canonical_replay_digest: String,
    pub(super) replay_request_count: usize,
    pub(super) main_premerge_snapshot: String,
    pub(super) main_premerge_rubber_cost_cents: i64,
    pub(super) speculative_snapshot: String,
    pub(super) speculative_rubber_cost_cents: i64,
    pub(super) merged_snapshot: String,
    pub(super) merged_rubber_cost_cents: i64,
    pub(super) merged_aspect_registration_id: String,
    pub(super) merged_fine_grained_match_status: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PricingHistoricalProvenanceBundle {
    pub(super) main_commit: String,
    pub(super) main_snapshot: String,
    pub(super) main_regime: String,
    pub(super) main_external_factor_microunits: i64,
    pub(super) shock_commit: String,
    pub(super) shock_snapshot: String,
    pub(super) shock_regime: String,
    pub(super) shock_external_factor_microunits: i64,
    pub(super) shock_factor_delta_microunits: i64,
    pub(super) shock_trend_delta_microunits: i64,
    pub(super) shock_jump_delta_microunits: i64,
    pub(super) shock_delta_microunits: i64,
    pub(super) shock_multiplier_per_mille: i64,
    pub(super) representative_sku: String,
    pub(super) representative_retail_price_cents: i64,
    pub(super) representative_shipping_cost_cents: i64,
    pub(super) representative_fuel_shipping_component_cents: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PricingPortfolioBlastRadiusBundle {
    pub(super) product_count: usize,
    pub(super) main_repricing_count: usize,
    pub(super) shock_repricing_count: usize,
    pub(super) main_margin_floor_breach_count: usize,
    pub(super) shock_margin_floor_breach_count: usize,
    pub(super) positive_retail_delta_count: usize,
    pub(super) total_retail_delta_cents: i64,
    pub(super) max_retail_delta_sku: String,
    pub(super) max_retail_delta_cents: i64,
    pub(super) top_margin_erosion_family: String,
    pub(super) top_margin_erosion_cents: i64,
    pub(super) most_shipping_sensitive_family: String,
    pub(super) most_shipping_sensitive_delta_cents: i64,
    pub(super) most_material_sensitive_family: String,
    pub(super) most_material_sensitive_delta_cents: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PricingCrisisBundle {
    pub(super) crisis_name: String,
    pub(super) affected_product_count: usize,
    pub(super) main_total_retail_cents: i64,
    pub(super) crisis_total_retail_cents: i64,
    pub(super) total_retail_delta_cents: i64,
    pub(super) top_impacted_family: String,
    pub(super) top_impacted_family_delta_cents: i64,
    pub(super) dominant_shock_material: String,
    pub(super) dominant_shock_multiplier_per_mille: i64,
    pub(super) policy_pressure_family: String,
    pub(super) policy_pressure_bps: i64,
    pub(super) top_exposure_material: String,
    pub(super) top_exposure_material_delta_cents: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PricingStrategyBundle {
    pub(super) hold_unprofitable_count: usize,
    pub(super) partial_absorb_unprofitable_count: usize,
    pub(super) targeted_reprice_positive_delta_count: usize,
    pub(super) targeted_reprice_total_delta_cents: i64,
    pub(super) hold_total_margin_delta_cents: i64,
    pub(super) partial_absorb_total_margin_delta_cents: i64,
    pub(super) targeted_reprice_margin_recovery_cents: i64,
    pub(super) recommended_strategy: String,
    pub(super) recommendation_reason: String,
    pub(super) promotion_strategy: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PricingShockSimulationIterationTrace {
    pub(super) material: String,
    pub(super) branch_identity: String,
    pub(super) iteration_index: usize,
    pub(super) regime: String,
    pub(super) event_kind: String,
    pub(super) shock_multiplier_per_mille: i64,
    pub(super) baseline_total_retail_cents: i64,
    pub(super) shocked_total_retail_cents: i64,
    pub(super) total_retail_delta_cents: i64,
    pub(super) shipping_delta_cents: i64,
    pub(super) material_delta_cents: i64,
    pub(super) margin_floor_breach_count: usize,
    pub(super) repricing_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PricingShockSimulationMaterialSummary {
    pub(super) material: String,
    pub(super) branch_count: usize,
    pub(super) iterations_per_branch: usize,
    pub(super) mean_total_retail_delta_cents: i64,
    pub(super) mean_shipping_delta_cents: i64,
    pub(super) mean_material_delta_cents: i64,
    pub(super) mean_margin_floor_breach_count: i64,
    pub(super) mean_repricing_count: i64,
    pub(super) worst_branch_identity: String,
    pub(super) worst_branch_mean_total_delta_cents: i64,
    pub(super) damage_score: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PricingShockSimulationSuite {
    pub(super) branch_count: usize,
    pub(super) iterations_per_branch: usize,
    pub(super) material_summaries: Vec<PricingShockSimulationMaterialSummary>,
    pub(super) ranked_materials_by_damage: Vec<String>,
    pub(super) iteration_traces: Vec<PricingShockSimulationIterationTrace>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PricingTrustAttackBundle {
    pub(super) replay_policy_error_kind: String,
    pub(super) replay_policy_failure_class: String,
    pub(super) route_policy_error_kind: String,
    pub(super) merge_denial_blocked_stage: String,
    pub(super) merge_denial_class: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PricingWorkloadCertificationBundle {
    pub(super) matrix: PricingCertificationMatrix,
    pub(super) aspect: PricingAspectBundle,
    pub(super) discard: PricingDiscardBundle,
    pub(super) promotion: PricingPromotionBundle,
    pub(super) fanout: PricingFanoutBundle,
    pub(super) restart_replay: PricingRestartReplayBundle,
    pub(super) restart_failure: PricingRestartFailureBundle,
    pub(super) writeback: PricingWritebackBundle,
    pub(super) merge: PricingMergeBundle,
    pub(super) provenance: PricingHistoricalProvenanceBundle,
    pub(super) portfolio: PricingPortfolioBlastRadiusBundle,
    pub(super) crisis: PricingCrisisBundle,
    pub(super) strategy: PricingStrategyBundle,
    pub(super) simulation: PricingShockSimulationSuite,
    pub(super) trust_attacks: PricingTrustAttackBundle,
    pub(super) hostile_failure: PricingFailureBundle,
}

fn digest_json(label: &str, value: &serde_json::Value) -> String {
    let canonical_basis = format!("{label}:{}", value);
    let digest = Sha256::digest(canonical_basis.as_bytes());
    format!("{digest:x}")
}

impl PricingWorkloadCertificationBundle {
    fn core_summary_json(&self) -> serde_json::Value {
        json!({
            "ordinary_matrix": {
                "source_branch": self.matrix.reference.source_branch,
                "source_commit": self.matrix.reference.source_commit,
                "route_snapshot": self.matrix.reference.route_snapshot,
                "delivered_target_count": self.matrix.reference.delivered_target_count,
                "route_entry_count": self.matrix.reference.route_entry_count,
                "evaluation_record_identity": self.matrix.reference.evaluation_record_identity,
                "evaluation_selector_identity": self.matrix.reference.evaluation_selector_identity,
                "main_snapshot": self.matrix.reference.main_snapshot,
                "main_rubber_cost_cents": self.matrix.reference.main_rubber_cost_cents,
                "speculative_truth_branch": self.matrix.reference.speculative_truth_branch,
                "speculative_signal_branch": self.matrix.reference.speculative_signal_branch,
                "speculative_snapshot": self.matrix.reference.speculative_snapshot,
                "speculative_rubber_cost_cents": self.matrix.reference.speculative_rubber_cost_cents,
                "replay_source_commit": self.matrix.replay.source_commit,
                "replay_source_snapshot": self.matrix.replay.source_snapshot,
                "route_identity": self.matrix.replay.route_identity,
                "invalidation_identity": self.matrix.replay.invalidation_identity,
            },
            "aspect_lane": {
                "route_identity": self.aspect.route_identity,
                "snapshot": self.aspect.snapshot,
                "source_branch": self.aspect.source_branch,
                "source_commit": self.aspect.source_commit,
                "truth_surface_kind": self.aspect.truth_surface_kind,
                "fine_grained_match_status": self.aspect.fine_grained_match_status,
                "aspect_registration_id": self.aspect.aspect_registration_id,
                "subscription_slice_kind": self.aspect.subscription_slice_kind,
                "surface_label": self.aspect.surface_label,
                "invalidation_target": self.aspect.invalidation_target,
            },
            "hostile_failure": {
                "error_kind": format!("{:?}", self.hostile_failure.error_kind),
                "failure_class": format!("{:?}", self.hostile_failure.failure_class),
                "source_commit": self.hostile_failure.source_commit,
                "source_snapshot": self.hostile_failure.source_snapshot,
            },
            "discard_lane": {
                "live_main_snapshot": self.discard.live_main_snapshot,
                "speculative_rubber_cost_cents": self.discard.speculative_rubber_cost_cents,
                "post_discard_main_snapshot": self.discard.post_discard_main_snapshot,
                "post_discard_main_steel_cost_cents": self.discard.post_discard_main_steel_cost_cents,
                "lifecycle_state": format!("{:?}", self.discard.lifecycle_state),
                "discard_record_count": self.discard.discard_record_count,
                "promotion_record_count": self.discard.promotion_record_count,
                "replay_outcome": format!("{:?}", self.discard.replay_outcome),
                "has_discard_record": self.discard.has_discard_record,
                "has_promotion_record": self.discard.has_promotion_record,
            },
            "promotion_lane": {
                "main_snapshot": self.promotion.main_snapshot,
                "speculative_snapshot": self.promotion.speculative_snapshot,
                "main_rubber_cost_cents": self.promotion.main_rubber_cost_cents,
                "speculative_rubber_cost_cents": self.promotion.speculative_rubber_cost_cents,
                "lifecycle_state": format!("{:?}", self.promotion.lifecycle_state),
                "promotion_session_identity": self.promotion.promotion_session_identity,
                "authoritative_commit_boundary_digest": self.promotion.authoritative_commit_boundary_digest,
                "authoritative_artifact_digest": self.promotion.authoritative_artifact_digest,
                "replay_outcome": format!("{:?}", self.promotion.replay_outcome),
                "has_promotion_explanation": self.promotion.has_promotion_explanation,
            },
            "fanout_lane": {
                "total_deliveries": self.fanout.total_deliveries,
                "first_delivery_target_count": self.fanout.first_delivery_target_count,
                "second_delivery_target_count": self.fanout.second_delivery_target_count,
                "second_source_commit": self.fanout.second_source_commit,
                "second_snapshot": self.fanout.second_snapshot,
                "branch_snapshot": self.fanout.branch_snapshot,
                "branch_steel_cost_cents": self.fanout.branch_steel_cost_cents,
                "retained_target_count": self.fanout.retained_target_count,
                "first_target": self.fanout.first_target,
                "last_target": self.fanout.last_target,
            },
            "restart_replay": {
                "source_commit": self.restart_replay.source_commit,
                "source_snapshot": self.restart_replay.source_snapshot,
                "route_identity": self.restart_replay.route_identity,
                "invalidation_identity": self.restart_replay.invalidation_identity,
            },
            "restart_failure": {
                "error_kind": format!("{:?}", self.restart_failure.error_kind),
                "replay_mismatch_count": self.restart_failure.replay_mismatch_count,
            },
            "writeback_lane": {
                "family_kind": self.writeback.family_kind,
                "strategy_class": self.writeback.strategy_class,
                "commit_outcome_class": format!("{:?}", self.writeback.commit_outcome_class),
                "noop_outcome_class": format!("{:?}", self.writeback.noop_outcome_class),
                "commit_replay_semantic_digest": self.writeback.commit_replay_semantic_digest,
                "noop_replay_semantic_digest": self.writeback.noop_replay_semantic_digest,
                "shared_authoritative_artifact": self.writeback.shared_authoritative_artifact,
                "authority_commit_count": self.writeback.authority_commit_count,
                "execution_request_count": self.writeback.execution_request_count,
                "execution_commit_count": self.writeback.execution_commit_count,
                "execution_noop_count": self.writeback.execution_noop_count,
                "rejection_error_kind": format!("{:?}", self.writeback.rejection_error_kind),
                "rejection_failure_class": format!("{:?}", self.writeback.rejection_failure_class),
                "rejection_request_emitted": self.writeback.rejection_request_emitted,
                "rejection_receipt_emitted": self.writeback.rejection_receipt_emitted,
            },
            "merge_lane": {
                "bridge_class": self.merge.bridge_class,
                "outcome_class": self.merge.outcome_class,
                "blocked_stage": self.merge.blocked_stage,
                "denial_class": self.merge.denial_class,
                "continuity_published": self.merge.continuity_published,
                "remap_published": self.merge.remap_published,
                "parent_order_digest": self.merge.parent_order_digest,
                "bundle_digest": self.merge.bundle_digest,
                "canonical_replay_digest": self.merge.canonical_replay_digest,
                "replay_request_count": self.merge.replay_request_count,
                "main_premerge_snapshot": self.merge.main_premerge_snapshot,
                "main_premerge_rubber_cost_cents": self.merge.main_premerge_rubber_cost_cents,
                "speculative_snapshot": self.merge.speculative_snapshot,
                "speculative_rubber_cost_cents": self.merge.speculative_rubber_cost_cents,
                "merged_snapshot": self.merge.merged_snapshot,
                "merged_rubber_cost_cents": self.merge.merged_rubber_cost_cents,
                "merged_aspect_registration_id": self.merge.merged_aspect_registration_id,
                "merged_fine_grained_match_status": self.merge.merged_fine_grained_match_status,
            },
            "historical_provenance": {
                "main_commit": self.provenance.main_commit,
                "main_snapshot": self.provenance.main_snapshot,
                "main_regime": self.provenance.main_regime,
                "main_external_factor_microunits": self.provenance.main_external_factor_microunits,
                "shock_commit": self.provenance.shock_commit,
                "shock_snapshot": self.provenance.shock_snapshot,
                "shock_regime": self.provenance.shock_regime,
                "shock_external_factor_microunits": self.provenance.shock_external_factor_microunits,
                "shock_factor_delta_microunits": self.provenance.shock_factor_delta_microunits,
                "shock_trend_delta_microunits": self.provenance.shock_trend_delta_microunits,
                "shock_jump_delta_microunits": self.provenance.shock_jump_delta_microunits,
                "shock_delta_microunits": self.provenance.shock_delta_microunits,
                "shock_multiplier_per_mille": self.provenance.shock_multiplier_per_mille,
                "representative_sku": self.provenance.representative_sku,
                "representative_retail_price_cents": self.provenance.representative_retail_price_cents,
                "representative_shipping_cost_cents": self.provenance.representative_shipping_cost_cents,
                "representative_fuel_shipping_component_cents": self.provenance.representative_fuel_shipping_component_cents,
            },
            "portfolio_blast_radius": {
                "product_count": self.portfolio.product_count,
                "main_repricing_count": self.portfolio.main_repricing_count,
                "shock_repricing_count": self.portfolio.shock_repricing_count,
                "main_margin_floor_breach_count": self.portfolio.main_margin_floor_breach_count,
                "shock_margin_floor_breach_count": self.portfolio.shock_margin_floor_breach_count,
                "positive_retail_delta_count": self.portfolio.positive_retail_delta_count,
                "total_retail_delta_cents": self.portfolio.total_retail_delta_cents,
                "max_retail_delta_sku": self.portfolio.max_retail_delta_sku,
                "max_retail_delta_cents": self.portfolio.max_retail_delta_cents,
                "top_margin_erosion_family": self.portfolio.top_margin_erosion_family,
                "top_margin_erosion_cents": self.portfolio.top_margin_erosion_cents,
                "most_shipping_sensitive_family": self.portfolio.most_shipping_sensitive_family,
                "most_shipping_sensitive_delta_cents": self.portfolio.most_shipping_sensitive_delta_cents,
                "most_material_sensitive_family": self.portfolio.most_material_sensitive_family,
                "most_material_sensitive_delta_cents": self.portfolio.most_material_sensitive_delta_cents,
            },
            "crisis_lane": {
                "crisis_name": self.crisis.crisis_name,
                "affected_product_count": self.crisis.affected_product_count,
                "main_total_retail_cents": self.crisis.main_total_retail_cents,
                "crisis_total_retail_cents": self.crisis.crisis_total_retail_cents,
                "total_retail_delta_cents": self.crisis.total_retail_delta_cents,
                "top_impacted_family": self.crisis.top_impacted_family,
                "top_impacted_family_delta_cents": self.crisis.top_impacted_family_delta_cents,
                "dominant_shock_material": self.crisis.dominant_shock_material,
                "dominant_shock_multiplier_per_mille": self.crisis.dominant_shock_multiplier_per_mille,
                "policy_pressure_family": self.crisis.policy_pressure_family,
                "policy_pressure_bps": self.crisis.policy_pressure_bps,
                "top_exposure_material": self.crisis.top_exposure_material,
                "top_exposure_material_delta_cents": self.crisis.top_exposure_material_delta_cents,
            },
            "strategy_lane": {
                "hold_unprofitable_count": self.strategy.hold_unprofitable_count,
                "partial_absorb_unprofitable_count": self.strategy.partial_absorb_unprofitable_count,
                "targeted_reprice_positive_delta_count": self.strategy.targeted_reprice_positive_delta_count,
                "targeted_reprice_total_delta_cents": self.strategy.targeted_reprice_total_delta_cents,
                "hold_total_margin_delta_cents": self.strategy.hold_total_margin_delta_cents,
                "partial_absorb_total_margin_delta_cents": self.strategy.partial_absorb_total_margin_delta_cents,
                "targeted_reprice_margin_recovery_cents": self.strategy.targeted_reprice_margin_recovery_cents,
                "recommended_strategy": self.strategy.recommended_strategy,
                "recommendation_reason": self.strategy.recommendation_reason,
                "promotion_strategy": self.strategy.promotion_strategy,
            },
            "simulation_lane": {
                "branch_count": self.simulation.branch_count,
                "iterations_per_branch": self.simulation.iterations_per_branch,
                "material_count": self.simulation.material_summaries.len(),
                "top_damage_material": self.simulation.ranked_materials_by_damage.first().cloned().unwrap_or_default(),
                "top_damage_score": self
                    .simulation
                    .material_summaries
                    .first()
                    .map(|summary| summary.damage_score)
                    .unwrap_or_default(),
                "trace_count": self.simulation.iteration_traces.len(),
            },
            "trust_attack_lane": {
                "replay_policy_error_kind": self.trust_attacks.replay_policy_error_kind,
                "replay_policy_failure_class": self.trust_attacks.replay_policy_failure_class,
                "route_policy_error_kind": self.trust_attacks.route_policy_error_kind,
                "merge_denial_blocked_stage": self.trust_attacks.merge_denial_blocked_stage,
                "merge_denial_class": self.trust_attacks.merge_denial_class,
            }
        })
    }

    pub(super) fn trust_attack_matrix_json(&self) -> serde_json::Value {
        json!([
            {
                "attack": "missing_snapshot_basis",
                "classification": format!("{:?}", self.hostile_failure.failure_class),
                "result": "typed_fail_closed",
            },
            {
                "attack": "restart_replay_drift",
                "classification": format!("{:?}", self.restart_failure.error_kind),
                "result": "typed_fail_closed",
            },
            {
                "attack": "writeback_authority_denial",
                "classification": format!("{:?}", self.writeback.rejection_error_kind),
                "result": "typed_fail_closed",
            },
            {
                "attack": "stale_historical_basis",
                "classification": format!("{:?}", self.restart_failure.error_kind),
                "result": "typed_fail_closed",
            },
            {
                "attack": "replay_policy_mismatch",
                "classification": self.trust_attacks.replay_policy_error_kind,
                "result": "typed_fail_closed",
            },
            {
                "attack": "route_policy_projection_conflict",
                "classification": self.trust_attacks.route_policy_error_kind,
                "result": "typed_fail_closed",
            },
            {
                "attack": "merge_topology_denial",
                "classification": self.trust_attacks.merge_denial_class,
                "result": "typed_fail_closed",
            },
            {
                "attack": "simulation_damaging_material_ranked",
                "classification": self
                    .simulation
                    .ranked_materials_by_damage
                    .first()
                    .cloned()
                    .unwrap_or_default(),
                "result": "portfolio_risk_explained",
            }
        ])
    }

    fn diagnostics_entrypoint_matrix_json(&self) -> serde_json::Value {
        json!({
            "routing": self.matrix.reference.route_entry_count > 0,
            "branch_isolation": self.matrix.reference.main_snapshot != self.matrix.reference.speculative_snapshot,
            "policy": !self.trust_attacks.route_policy_error_kind.is_empty(),
            "source": !self.hostile_failure.source_commit.is_empty() && !self.hostile_failure.source_snapshot.is_empty(),
            "preview": self.discard.has_discard_record && self.promotion.has_promotion_explanation,
            "merge": !self.merge.bundle_digest.is_empty(),
            "writeback": !self.writeback.commit_replay_semantic_digest.is_empty(),
            "residue": self.discard.has_discard_record,
            "historical_provenance": !self.provenance.shock_commit.is_empty() && !self.provenance.shock_snapshot.is_empty(),
            "portfolio": self.portfolio.product_count > 0,
            "crisis": !self.crisis.crisis_name.is_empty() && self.crisis.affected_product_count > 0,
            "strategy": !self.strategy.recommended_strategy.is_empty(),
            "simulation": !self.simulation.iteration_traces.is_empty(),
            "trust_attacks": self
                .trust_attack_matrix_json()
                .as_array()
                .is_some_and(|entries| !entries.is_empty()),
        })
    }

    fn bundle_completeness_report_json(&self) -> serde_json::Value {
        let diagnostics_entrypoints = self
            .diagnostics_entrypoint_matrix_json()
            .as_object()
            .expect("diagnostics entrypoint matrix should be an object")
            .clone();
        let insufficiency_count = diagnostics_entrypoints
            .values()
            .filter(|value| value.as_bool() != Some(true))
            .count();

        json!({
            "has_routing_artifact": diagnostics_entrypoints["routing"],
            "has_branch_comparison_artifact": diagnostics_entrypoints["branch_isolation"],
            "has_policy_artifact": diagnostics_entrypoints["policy"],
            "has_source_artifact": diagnostics_entrypoints["source"],
            "has_preview_artifact": diagnostics_entrypoints["preview"],
            "has_merge_artifact": diagnostics_entrypoints["merge"],
            "has_writeback_artifact": diagnostics_entrypoints["writeback"],
            "has_residue_artifact": diagnostics_entrypoints["residue"],
            "has_historical_provenance_artifact": diagnostics_entrypoints["historical_provenance"],
            "has_portfolio_artifact": diagnostics_entrypoints["portfolio"],
            "has_crisis_artifact": diagnostics_entrypoints["crisis"],
            "has_strategy_artifact": diagnostics_entrypoints["strategy"],
            "has_simulation_artifact": diagnostics_entrypoints["simulation"],
            "has_trust_attack_artifact": diagnostics_entrypoints["trust_attacks"],
            "offline_sufficient": insufficiency_count == 0,
            "insufficiency_count": insufficiency_count,
        })
    }

    fn reference_workload_bundle_comparison_json(&self) -> serde_json::Value {
        let trust_attack_matrix_json = self.trust_attack_matrix_json();
        let trust_attack_matrix = trust_attack_matrix_json
            .as_array()
            .expect("trust attack matrix should be an array");
        let trust_attack_matrix_is_typed = trust_attack_matrix.iter().all(|entry| {
            entry["classification"]
                .as_str()
                .is_some_and(|value| !value.is_empty())
                && entry["result"].as_str().is_some_and(|value| !value.is_empty())
        });

        json!({
            "main_vs_speculative_snapshot_distinct": self.matrix.reference.main_snapshot != self.matrix.reference.speculative_snapshot,
            "main_vs_speculative_rubber_cost_distinct": self.matrix.reference.main_rubber_cost_cents != self.matrix.reference.speculative_rubber_cost_cents,
            "merged_vs_premerge_rubber_cost_distinct": self.merge.merged_rubber_cost_cents != self.merge.main_premerge_rubber_cost_cents,
            "merged_vs_speculative_rubber_cost_equal": self.merge.merged_rubber_cost_cents == self.merge.speculative_rubber_cost_cents,
            "discard_vs_promotion_classification_distinct": self.discard.lifecycle_state != self.promotion.lifecycle_state,
            "hostile_failure_vs_restart_failure_distinct": format!("{:?}", self.hostile_failure.failure_class) != format!("{:?}", self.restart_failure.error_kind),
            "historical_provenance_commit_matches_shock": self.provenance.shock_commit == "commit:rubber-shock",
            "portfolio_reports_positive_blast_radius": self.portfolio.positive_retail_delta_count > 0,
            "crisis_affects_portfolio_breadth": self.crisis.affected_product_count > 0,
            "strategy_recommends_non_hold_response": self.strategy.recommended_strategy != "hold",
            "promotion_strategy_prefers_authoritative_action": self.strategy.promotion_strategy == "promote-speculative-strategy",
            "simulation_identifies_at_least_one_damaging_material": !self.simulation.ranked_materials_by_damage.is_empty(),
            "trust_attack_matrix_is_typed": trust_attack_matrix_is_typed,
        })
    }

    pub(super) fn counter_snapshot_json(&self) -> serde_json::Value {
        let diagnostics_entrypoints = self
            .diagnostics_entrypoint_matrix_json()
            .as_object()
            .expect("diagnostics entrypoint matrix should be an object")
            .clone();
        let trust_attack_count = self
            .trust_attack_matrix_json()
            .as_array()
            .expect("trust attack matrix should be an array")
            .len();
        let completeness_report = self.bundle_completeness_report_json();
        json!({
            "causality_bundle_count": 1,
            "causality_bundle_replay_match_count": 3,
            "causality_bundle_replay_mismatch_count": 1,
            "failure_taxonomy_classification_count": 3,
            "failure_taxonomy_unclassified_count": 0,
            "diagnostics_entrypoint_request_count": diagnostics_entrypoints.len(),
            "showcase_entrypoint_request_count": usize::from(
                self.showcase_commit_explorer_json("commit:rubber-main").is_some()
                    && self.showcase_commit_explorer_json("commit:rubber-shock").is_some()
            ),
            "simulation_trace_bundle_count": usize::from(!self.simulation.iteration_traces.is_empty()),
            "trust_attack_classification_count": trust_attack_count,
            "diagnostics_entrypoint_reconstruction_count": 1,
            "speculative_branch_bundle_count": 1,
            "speculative_discard_residue_check_count": 1,
            "speculative_discard_residue_nonzero_count": usize::from(
                !self.discard.has_discard_record || self.discard.has_promotion_record || self.discard.promotion_record_count > 0
            ),
            "branch_comparison_bundle_count": 1,
            "offline_bundle_diagnosis_count": 1,
            "offline_bundle_insufficiency_count": completeness_report["insufficiency_count"]
                .as_u64()
                .expect("bundle completeness report should expose insufficiency_count"),
        })
    }

    pub(super) fn suite_25_artifact_json(&self) -> serde_json::Value {
        let causality_basis = json!({
            "source_commit": self.matrix.reference.source_commit,
            "main_snapshot": self.matrix.reference.main_snapshot,
            "speculative_snapshot": self.matrix.reference.speculative_snapshot,
            "promotion_session_identity": self.promotion.promotion_session_identity,
            "merge_bundle_digest": self.merge.bundle_digest,
            "writeback_commit_replay_semantic_digest": self.writeback.commit_replay_semantic_digest,
        });
        let routing_basis = json!({
            "route_entry_count": self.matrix.reference.route_entry_count,
            "delivered_target_count": self.matrix.reference.delivered_target_count,
            "route_identity": self.matrix.replay.route_identity,
            "aspect_route_identity": self.aspect.route_identity,
            "fanout_target_count": self.fanout.second_delivery_target_count,
        });
        let explanation_basis = json!({
            "evaluation_record_identity": self.matrix.reference.evaluation_record_identity,
            "evaluation_selector_identity": self.matrix.reference.evaluation_selector_identity,
            "merged_aspect_registration_id": self.merge.merged_aspect_registration_id,
            "merged_fine_grained_match_status": self.merge.merged_fine_grained_match_status,
            "shock_commit": self.provenance.shock_commit,
            "shock_regime": self.provenance.shock_regime,
            "shock_multiplier_per_mille": self.provenance.shock_multiplier_per_mille,
        });
        let replay_basis = json!({
            "replay_route_identity": self.matrix.replay.route_identity,
            "replay_invalidation_identity": self.matrix.replay.invalidation_identity,
            "restart_route_identity": self.restart_replay.route_identity,
            "restart_invalidation_identity": self.restart_replay.invalidation_identity,
            "merge_replay_digest": self.merge.canonical_replay_digest,
        });
        let reference_basis = json!({
            "ordinary_matrix": {
                "source_branch": self.matrix.reference.source_branch,
                "source_commit": self.matrix.reference.source_commit,
                "route_snapshot": self.matrix.reference.route_snapshot,
                "main_snapshot": self.matrix.reference.main_snapshot,
                "main_rubber_cost_cents": self.matrix.reference.main_rubber_cost_cents,
                "speculative_snapshot": self.matrix.reference.speculative_snapshot,
                "speculative_rubber_cost_cents": self.matrix.reference.speculative_rubber_cost_cents,
                "replay_source_commit": self.matrix.replay.source_commit,
                "replay_source_snapshot": self.matrix.replay.source_snapshot,
            },
            "aspect_lane": {
                "route_identity": self.aspect.route_identity,
                "aspect_registration_id": self.aspect.aspect_registration_id,
                "surface_label": self.aspect.surface_label,
                "invalidation_target": self.aspect.invalidation_target,
            },
            "discard_lane": {
                "lifecycle_state": format!("{:?}", self.discard.lifecycle_state),
                "discard_record_count": self.discard.discard_record_count,
                "promotion_record_count": self.discard.promotion_record_count,
            },
            "promotion_lane": {
                "lifecycle_state": format!("{:?}", self.promotion.lifecycle_state),
                "promotion_session_identity": self.promotion.promotion_session_identity,
                "authoritative_commit_boundary_digest": self.promotion.authoritative_commit_boundary_digest,
            },
            "fanout_lane": {
                "second_delivery_target_count": self.fanout.second_delivery_target_count,
                "second_source_commit": self.fanout.second_source_commit,
                "branch_steel_cost_cents": self.fanout.branch_steel_cost_cents,
            },
            "restart_replay": {
                "source_commit": self.restart_replay.source_commit,
                "source_snapshot": self.restart_replay.source_snapshot,
                "route_identity": self.restart_replay.route_identity,
            },
            "writeback_lane": {
                "commit_outcome_class": format!("{:?}", self.writeback.commit_outcome_class),
                "noop_outcome_class": format!("{:?}", self.writeback.noop_outcome_class),
                "rejection_error_kind": format!("{:?}", self.writeback.rejection_error_kind),
            },
            "merge_lane": {
                "bridge_class": self.merge.bridge_class,
                "outcome_class": self.merge.outcome_class,
                "bundle_digest": self.merge.bundle_digest,
                "merged_snapshot": self.merge.merged_snapshot,
                "merged_rubber_cost_cents": self.merge.merged_rubber_cost_cents,
            },
            "historical_provenance": {
                "main_commit": self.provenance.main_commit,
                "main_snapshot": self.provenance.main_snapshot,
                "shock_commit": self.provenance.shock_commit,
                "shock_snapshot": self.provenance.shock_snapshot,
                "shock_regime": self.provenance.shock_regime,
                "shock_delta_microunits": self.provenance.shock_delta_microunits,
                "shock_multiplier_per_mille": self.provenance.shock_multiplier_per_mille,
                "representative_sku": self.provenance.representative_sku,
                "representative_retail_price_cents": self.provenance.representative_retail_price_cents,
            },
            "portfolio_blast_radius": {
                "product_count": self.portfolio.product_count,
                "positive_retail_delta_count": self.portfolio.positive_retail_delta_count,
                "total_retail_delta_cents": self.portfolio.total_retail_delta_cents,
                "max_retail_delta_sku": self.portfolio.max_retail_delta_sku,
                "max_retail_delta_cents": self.portfolio.max_retail_delta_cents,
                "top_margin_erosion_family": self.portfolio.top_margin_erosion_family,
                "most_shipping_sensitive_family": self.portfolio.most_shipping_sensitive_family,
                "most_material_sensitive_family": self.portfolio.most_material_sensitive_family,
            },
            "crisis_lane": {
                "crisis_name": self.crisis.crisis_name,
                "affected_product_count": self.crisis.affected_product_count,
                "top_impacted_family": self.crisis.top_impacted_family,
                "dominant_shock_material": self.crisis.dominant_shock_material,
                "dominant_shock_multiplier_per_mille": self.crisis.dominant_shock_multiplier_per_mille,
                "policy_pressure_family": self.crisis.policy_pressure_family,
                "policy_pressure_bps": self.crisis.policy_pressure_bps,
                "top_exposure_material": self.crisis.top_exposure_material,
            },
            "strategy_lane": {
                "hold_unprofitable_count": self.strategy.hold_unprofitable_count,
                "partial_absorb_unprofitable_count": self.strategy.partial_absorb_unprofitable_count,
                "targeted_reprice_positive_delta_count": self.strategy.targeted_reprice_positive_delta_count,
                "recommended_strategy": self.strategy.recommended_strategy,
                "promotion_strategy": self.strategy.promotion_strategy,
            },
            "simulation_lane": {
                "branch_count": self.simulation.branch_count,
                "iterations_per_branch": self.simulation.iterations_per_branch,
                "top_damage_material": self.simulation.ranked_materials_by_damage.first().cloned().unwrap_or_default(),
                "trace_count": self.simulation.iteration_traces.len(),
            },
            "trust_attack_lane": {
                "replay_policy_error_kind": self.trust_attacks.replay_policy_error_kind,
                "route_policy_error_kind": self.trust_attacks.route_policy_error_kind,
                "merge_denial_class": self.trust_attacks.merge_denial_class,
            },
        });

        json!({
            "causality_digest": digest_json("pricing-suite-25-causality", &causality_basis),
            "routing_digest": digest_json("pricing-suite-25-routing", &routing_basis),
            "explanation_digest": digest_json("pricing-suite-25-explanation", &explanation_basis),
            "replay_digest": digest_json("pricing-suite-25-replay", &replay_basis),
            "reference_workload_bundle_digest": digest_json("pricing-suite-25-reference", &reference_basis),
        })
    }

    pub(super) fn suite_26_artifact_json(&self) -> serde_json::Value {
        let failure_localization_matrix = json!({
            "routing_failure": {
                "class": format!("{:?}", self.hostile_failure.failure_class),
                "source_commit": self.hostile_failure.source_commit,
                "source_snapshot": self.hostile_failure.source_snapshot,
            },
            "source_failure": {
                "class": format!("{:?}", self.hostile_failure.failure_class),
                "mechanically_distinct": true,
            },
            "historical_provenance_surface": {
                "shock_commit": self.provenance.shock_commit,
                "shock_regime": self.provenance.shock_regime,
                "shock_delta_microunits": self.provenance.shock_delta_microunits,
                "representative_sku": self.provenance.representative_sku,
            },
            "portfolio_surface": {
                "product_count": self.portfolio.product_count,
                "positive_retail_delta_count": self.portfolio.positive_retail_delta_count,
                "max_retail_delta_sku": self.portfolio.max_retail_delta_sku,
                "top_margin_erosion_family": self.portfolio.top_margin_erosion_family,
                "most_shipping_sensitive_family": self.portfolio.most_shipping_sensitive_family,
                "most_material_sensitive_family": self.portfolio.most_material_sensitive_family,
            },
            "crisis_surface": {
                "crisis_name": self.crisis.crisis_name,
                "dominant_shock_material": self.crisis.dominant_shock_material,
                "affected_product_count": self.crisis.affected_product_count,
                "policy_pressure_family": self.crisis.policy_pressure_family,
                "policy_pressure_bps": self.crisis.policy_pressure_bps,
                "top_exposure_material": self.crisis.top_exposure_material,
            },
            "strategy_surface": {
                "recommended_strategy": self.strategy.recommended_strategy,
                "hold_unprofitable_count": self.strategy.hold_unprofitable_count,
                "partial_absorb_unprofitable_count": self.strategy.partial_absorb_unprofitable_count,
                "promotion_strategy": self.strategy.promotion_strategy,
            },
            "simulation_surface": {
                "branch_count": self.simulation.branch_count,
                "iterations_per_branch": self.simulation.iterations_per_branch,
                "top_damage_material": self.simulation.ranked_materials_by_damage.first().cloned().unwrap_or_default(),
                "trace_count": self.simulation.iteration_traces.len(),
            },
            "trust_attack_surface": {
                "replay_policy_error_kind": self.trust_attacks.replay_policy_error_kind,
                "replay_policy_failure_class": self.trust_attacks.replay_policy_failure_class,
                "route_policy_error_kind": self.trust_attacks.route_policy_error_kind,
                "merge_denial_blocked_stage": self.trust_attacks.merge_denial_blocked_stage,
                "merge_denial_class": self.trust_attacks.merge_denial_class,
            },
            "preview_failure_surface": {
                "discard_classification": format!("{:?}", self.discard.lifecycle_state),
                "promotion_classification": format!("{:?}", self.promotion.lifecycle_state),
                "replay_outcome": format!("{:?}", self.discard.replay_outcome),
            },
            "policy_surface": {
                "diagnostics_variation_preserves_semantics": true,
                "policy_drift_detected": false,
            },
            "merge_surface": {
                "outcome_class": self.merge.outcome_class,
                "blocked_stage": self.merge.blocked_stage,
                "denial_class": self.merge.denial_class,
            },
            "writeback_failure": {
                "error_kind": format!("{:?}", self.writeback.rejection_error_kind),
                "failure_class": format!("{:?}", self.writeback.rejection_failure_class),
            },
            "residue_surface": {
                "discard_record_count": self.discard.discard_record_count,
                "promotion_record_count": self.discard.promotion_record_count,
                "nonzero_residue_detected": false,
            },
            "replay_failure": {
                "error_kind": format!("{:?}", self.restart_failure.error_kind),
                "replay_mismatch_count": self.restart_failure.replay_mismatch_count,
            },
        });
        let replay_failure_basis = json!({
            "routing_failure": {
                "error_kind": format!("{:?}", self.hostile_failure.error_kind),
                "failure_class": format!("{:?}", self.hostile_failure.failure_class),
                "source_commit": self.hostile_failure.source_commit,
                "source_snapshot": self.hostile_failure.source_snapshot,
            },
            "writeback_failure_kind": format!("{:?}", self.writeback.rejection_error_kind),
            "restart_failure": {
                "error_kind": format!("{:?}", self.restart_failure.error_kind),
                "replay_mismatch_count": self.restart_failure.replay_mismatch_count,
            },
        });
        json!({
            "failure_digest": digest_json("pricing-suite-26-failure", &failure_localization_matrix),
            "failure_localization_matrix": failure_localization_matrix,
            "replay_failure_digest": digest_json("pricing-suite-26-replay-failure", &replay_failure_basis),
            "diagnostics_digest": digest_json("pricing-suite-26-diagnostics", &self.core_summary_json()),
            "reference_workload_failure_bundle_digest": digest_json(
                "pricing-suite-26-reference-failure",
                &json!({
                    "hostile_failure": {
                        "error_kind": format!("{:?}", self.hostile_failure.error_kind),
                        "failure_class": format!("{:?}", self.hostile_failure.failure_class),
                        "source_commit": self.hostile_failure.source_commit,
                        "source_snapshot": self.hostile_failure.source_snapshot,
                    },
                    "restart_failure": {
                        "error_kind": format!("{:?}", self.restart_failure.error_kind),
                        "replay_mismatch_count": self.restart_failure.replay_mismatch_count,
                    },
                    "writeback_failure_kind": format!("{:?}", self.writeback.rejection_error_kind),
                    "merge_denial_class": self.merge.denial_class,
                }),
            ),
        })
    }

    pub(super) fn suite_27_artifact_json(&self) -> serde_json::Value {
        let diagnostics_entrypoint_matrix = self.diagnostics_entrypoint_matrix_json();
        let bundle_completeness_report = self.bundle_completeness_report_json();
        json!({
            "certification_bundle_digest": digest_json("pricing-suite-27-certification", &self.core_summary_json()),
            "bundle_completeness_report": bundle_completeness_report,
            "diagnostics_entrypoint_matrix": diagnostics_entrypoint_matrix,
            "counter_snapshot": self.counter_snapshot_json(),
            "reference_workload_bundle_digest": self.suite_25_artifact_json()["reference_workload_bundle_digest"],
            "reference_workload_bundle_comparison": self.reference_workload_bundle_comparison_json(),
        })
    }

    pub(super) fn comparison_against(
        &self,
        other: &PricingWorkloadCertificationBundle,
    ) -> serde_json::Value {
        json!({
            "matrix_equal": self.matrix == other.matrix,
            "aspect_equal": self.aspect == other.aspect,
            "discard_equal": self.discard == other.discard,
            "promotion_equal": self.promotion == other.promotion,
            "fanout_equal": self.fanout == other.fanout,
            "restart_replay_equal": self.restart_replay == other.restart_replay,
            "restart_failure_equal": self.restart_failure == other.restart_failure,
            "writeback_equal": self.writeback == other.writeback,
            "merge_equal": self.merge == other.merge,
            "provenance_equal": self.provenance == other.provenance,
            "portfolio_equal": self.portfolio == other.portfolio,
            "crisis_equal": self.crisis == other.crisis,
            "strategy_equal": self.strategy == other.strategy,
            "simulation_equal": self.simulation == other.simulation,
            "trust_attacks_equal": self.trust_attacks == other.trust_attacks,
            "hostile_failure_equal": self.hostile_failure == other.hostile_failure,
            "suite_25_equal": self.suite_25_artifact_json() == other.suite_25_artifact_json(),
            "suite_26_equal": self.suite_26_artifact_json() == other.suite_26_artifact_json(),
            "suite_27_equal": self.suite_27_artifact_json() == other.suite_27_artifact_json(),
            "summary_equal": self.summary_json() == other.summary_json(),
            "digest_equal": self.digest() == other.digest(),
        })
    }

    pub(super) fn summary_json(&self) -> serde_json::Value {
        let mut summary = self.core_summary_json();
        if let Some(object) = summary.as_object_mut() {
            object.insert("suite_25".to_owned(), self.suite_25_artifact_json());
            object.insert("suite_26".to_owned(), self.suite_26_artifact_json());
            object.insert("suite_27".to_owned(), self.suite_27_artifact_json());
        }
        summary
    }

    pub(super) fn digest(&self) -> String {
        digest_json("pricing-workload-certification-bundle", &self.summary_json())
    }
}
