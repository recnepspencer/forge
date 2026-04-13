use super::pricing_support::PricingWorkloadCertificationBundle;
use serde_json::json;

impl PricingWorkloadCertificationBundle {
    pub(super) fn showcase_artifact_json(&self) -> serde_json::Value {
        json!({
            "executive_summary": {
                "main_branch": self.matrix.reference.source_branch,
                "speculative_truth_branch": self.matrix.reference.speculative_truth_branch,
                "main_snapshot": self.matrix.reference.main_snapshot,
                "speculative_snapshot": self.matrix.reference.speculative_snapshot,
                "same_fork_basis_preserved": true,
                "shock_commit": self.provenance.shock_commit,
                "shock_regime": self.provenance.shock_regime,
                "shock_multiplier_per_mille": self.provenance.shock_multiplier_per_mille,
                "main_vs_speculative_rubber_delta_cents":
                    self.matrix.reference.speculative_rubber_cost_cents
                    - self.matrix.reference.main_rubber_cost_cents,
                "fanout_target_count": self.fanout.second_delivery_target_count,
                "discard_zero_residue": self.discard.has_discard_record
                    && !self.discard.has_promotion_record
                    && self.discard.promotion_record_count == 0,
                "promotion_outcome": format!("{:?}", self.promotion.lifecycle_state),
                "writeback_commit_outcome": format!("{:?}", self.writeback.commit_outcome_class),
                "writeback_noop_outcome": format!("{:?}", self.writeback.noop_outcome_class),
            },
            "timeline": [
                {
                    "phase": "main_basis",
                    "branch": self.matrix.reference.source_branch,
                    "commit": self.matrix.reference.source_commit,
                    "snapshot": self.matrix.reference.main_snapshot,
                    "meaning": "ordinary live route basis",
                },
                {
                    "phase": "historical_main_provenance",
                    "branch": self.matrix.reference.source_branch,
                    "commit": self.provenance.main_commit,
                    "snapshot": self.provenance.main_snapshot,
                    "meaning": "retained main-branch pricing provenance",
                },
                {
                    "phase": "speculative_shock",
                    "branch": self.matrix.reference.speculative_truth_branch,
                    "commit": self.provenance.shock_commit,
                    "snapshot": self.provenance.shock_snapshot,
                    "meaning": "branch-local crisis shock",
                },
                {
                    "phase": "main_interleaved_fanout",
                    "branch": self.matrix.reference.source_branch,
                    "commit": self.fanout.second_source_commit,
                    "snapshot": self.fanout.second_snapshot,
                    "meaning": "live high-fanout main-branch continuation",
                },
                {
                    "phase": "merged_authority",
                    "branch": self.matrix.reference.source_branch,
                    "commit": "merge:pricing-shock",
                    "snapshot": self.merge.merged_snapshot,
                    "meaning": "authoritative merged pricing truth",
                }
            ],
            "branch_comparison": {
                "main_snapshot": self.matrix.reference.main_snapshot,
                "speculative_snapshot": self.matrix.reference.speculative_snapshot,
                "main_rubber_cost_cents": self.matrix.reference.main_rubber_cost_cents,
                "speculative_rubber_cost_cents": self.matrix.reference.speculative_rubber_cost_cents,
                "delta_rubber_cost_cents":
                    self.matrix.reference.speculative_rubber_cost_cents
                    - self.matrix.reference.main_rubber_cost_cents,
                "main_and_speculative_distinct": self.matrix.reference.main_snapshot
                    != self.matrix.reference.speculative_snapshot,
                "merged_snapshot": self.merge.merged_snapshot,
                "merged_matches_speculative_rubber_cost":
                    self.merge.merged_rubber_cost_cents == self.merge.speculative_rubber_cost_cents,
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
            "multi_factor_crisis": {
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
            "strategy_comparison": {
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
            "shock_simulation": {
                "branch_count": self.simulation.branch_count,
                "iterations_per_branch": self.simulation.iterations_per_branch,
                "ranked_materials_by_damage": self.simulation.ranked_materials_by_damage,
                "top_damage_summary": self.simulation.material_summaries.first().map(|summary| json!({
                    "material": summary.material,
                    "mean_total_retail_delta_cents": summary.mean_total_retail_delta_cents,
                    "mean_shipping_delta_cents": summary.mean_shipping_delta_cents,
                    "mean_material_delta_cents": summary.mean_material_delta_cents,
                    "damage_score": summary.damage_score,
                    "worst_branch_identity": summary.worst_branch_identity,
                    "worst_branch_mean_total_delta_cents": summary.worst_branch_mean_total_delta_cents,
                })).unwrap_or_else(|| json!({})),
            },
            "retained_commit_explorer": {
                "commit:rubber-main": self.showcase_commit_explorer_json("commit:rubber-main")
                    .expect("main provenance commit should be present"),
                "commit:rubber-shock": self.showcase_commit_explorer_json("commit:rubber-shock")
                    .expect("shock provenance commit should be present"),
            },
            "trust_proof": {
                "replay_route_identity": self.matrix.replay.route_identity,
                "restart_replay_route_identity": self.restart_replay.route_identity,
                "hostile_failure_class": format!("{:?}", self.hostile_failure.failure_class),
                "restart_failure_kind": format!("{:?}", self.restart_failure.error_kind),
                "merge_bundle_digest": self.merge.bundle_digest,
                "writeback_commit_replay_semantic_digest": self.writeback.commit_replay_semantic_digest,
                "bundle_digest": self.digest(),
                "suite_25": self.suite_25_artifact_json(),
                "suite_26": self.suite_26_artifact_json(),
                "suite_27": self.suite_27_artifact_json(),
            },
            "trust_attack_matrix": [
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
                    "classification": self.simulation.ranked_materials_by_damage.first().cloned().unwrap_or_default(),
                    "result": "portfolio_risk_explained",
                }
            ],
            "demo_flow": [
                "stabilize main live pricing world",
                "fork speculative crisis branch",
                "inspect split reality and retained shock lineage",
                "measure portfolio blast radius",
                "choose discard or promotion path",
                "replay and trust-check canonical evidence",
            ],
            "demo_artifact_family": {
                "control_digest": self.suite_25_artifact_json()["reference_workload_bundle_digest"],
                "hostile_digest": self.suite_26_artifact_json()["reference_workload_failure_bundle_digest"],
                "certification_digest": self.suite_27_artifact_json()["certification_bundle_digest"],
                "showcase_digest": self.digest(),
            }
        })
    }

    pub(super) fn showcase_commit_explorer_json(
        &self,
        commit_identity: &str,
    ) -> Option<serde_json::Value> {
        match commit_identity {
            "commit:rubber-main" => Some(json!({
                "branch": self.matrix.reference.source_branch,
                "snapshot": self.provenance.main_snapshot,
                "regime": self.provenance.main_regime,
                "external_factor_microunits": self.provenance.main_external_factor_microunits,
                "shock_delta_microunits": 0,
                "shock_multiplier_per_mille": 1000,
                "representative_sku": self.provenance.representative_sku,
                "representative_retail_price_cents": self.provenance.representative_retail_price_cents,
                "representative_shipping_cost_cents": self.provenance.representative_shipping_cost_cents,
                "representative_fuel_shipping_component_cents":
                    self.provenance.representative_fuel_shipping_component_cents,
            })),
            "commit:rubber-shock" => Some(json!({
                "branch": self.matrix.reference.speculative_truth_branch,
                "snapshot": self.provenance.shock_snapshot,
                "regime": self.provenance.shock_regime,
                "external_factor_microunits": self.provenance.shock_external_factor_microunits,
                "factor_delta_microunits": self.provenance.shock_factor_delta_microunits,
                "trend_delta_microunits": self.provenance.shock_trend_delta_microunits,
                "jump_delta_microunits": self.provenance.shock_jump_delta_microunits,
                "shock_delta_microunits": self.provenance.shock_delta_microunits,
                "shock_multiplier_per_mille": self.provenance.shock_multiplier_per_mille,
                "representative_sku": self.provenance.representative_sku,
                "representative_retail_price_cents": self.provenance.representative_retail_price_cents,
                "representative_shipping_cost_cents": self.provenance.representative_shipping_cost_cents,
                "representative_fuel_shipping_component_cents":
                    self.provenance.representative_fuel_shipping_component_cents,
            })),
            _ => None,
        }
    }

    pub(super) fn showcase_markdown_report(&self) -> String {
        let main_vs_speculative_delta = self.matrix.reference.speculative_rubber_cost_cents
            - self.matrix.reference.main_rubber_cost_cents;
        let top_damage_material = self
            .simulation
            .ranked_materials_by_damage
            .first()
            .cloned()
            .unwrap_or_default();
        format!(
            concat!(
                "# Pricing Shock Showcase Report\n\n",
                "## Executive Summary\n\n",
                "- Main branch: `{}`\n",
                "- Speculative branch: `{}`\n",
                "- Shock commit: `{}`\n",
                "- Shock regime: `{}`\n",
                "- Shock multiplier (per mille): `{}`\n",
                "- Main vs speculative rubber delta (cents): `{}`\n",
                "- High-fanout target count: `{}`\n",
                "- Products with positive retail delta: `{}`\n",
                "- Total retail delta (cents): `{}`\n",
                "- Largest retail delta SKU: `{}` (`{}` cents)\n",
                "- Top margin erosion family: `{}` (`{}` cents)\n",
                "- Most shipping-sensitive family: `{}` (`{}` cents)\n",
                "- Most material-sensitive family: `{}` (`{}` cents)\n",
                "- Crisis name: `{}`\n",
                "- Affected products under crisis: `{}`\n",
                "- Top impacted family: `{}` (`{}` cents)\n",
                "- Policy pressure family: `{}` (`{}` bps)\n",
                "- Top exposure material: `{}` (`{}` microunits)\n",
                "- Recommended strategy: `{}`\n",
                "- Promotion strategy: `{}`\n",
                "- Top damaging shock material across simulation: `{}`\n",
                "- Discard proved zero residue: `{}`\n",
                "- Promotion outcome: `{}`\n",
                "- Writeback commit outcome: `{}`\n\n",
                "## Trust Attacks\n\n",
                "- Missing snapshot basis: `{}`\n",
                "- Restart replay drift: `{}`\n",
                "- Writeback authority denial: `{}`\n\n",
                "## Retained Commit Explorer\n\n",
                "- Main historical provenance commit: `{}` on `{}`\n",
                "- Shock historical provenance commit: `{}` on `{}`\n",
                "- Representative SKU: `{}`\n",
                "- Representative retail price (cents): `{}`\n",
                "- Representative shipping cost (cents): `{}`\n",
                "- Representative fuel shipping component (cents): `{}`\n\n",
                "## Demo Flow\n\n",
                "1. Stabilize main live pricing world\n",
                "2. Fork speculative crisis branch\n",
                "3. Inspect split reality and retained shock lineage\n",
                "4. Measure portfolio blast radius\n",
                "5. Choose discard or promotion path\n",
                "6. Replay and trust-check canonical evidence\n\n",
                "## Trust Proof\n\n",
                "- Bundle digest: `{}`\n",
                "- Suite 25 digest: `{}`\n",
                "- Suite 26 digest: `{}`\n",
                "- Suite 27 digest: `{}`\n"
            ),
            self.matrix.reference.source_branch,
            self.matrix.reference.speculative_truth_branch,
            self.provenance.shock_commit,
            self.provenance.shock_regime,
            self.provenance.shock_multiplier_per_mille,
            main_vs_speculative_delta,
            self.fanout.second_delivery_target_count,
            self.portfolio.positive_retail_delta_count,
            self.portfolio.total_retail_delta_cents,
            self.portfolio.max_retail_delta_sku,
            self.portfolio.max_retail_delta_cents,
            self.portfolio.top_margin_erosion_family,
            self.portfolio.top_margin_erosion_cents,
            self.portfolio.most_shipping_sensitive_family,
            self.portfolio.most_shipping_sensitive_delta_cents,
            self.portfolio.most_material_sensitive_family,
            self.portfolio.most_material_sensitive_delta_cents,
            self.crisis.crisis_name,
            self.crisis.affected_product_count,
            self.crisis.top_impacted_family,
            self.crisis.top_impacted_family_delta_cents,
            self.crisis.policy_pressure_family,
            self.crisis.policy_pressure_bps,
            self.crisis.top_exposure_material,
            self.crisis.top_exposure_material_delta_cents,
            self.strategy.recommended_strategy,
            self.strategy.promotion_strategy,
            top_damage_material,
            self.discard.has_discard_record
                && !self.discard.has_promotion_record
                && self.discard.promotion_record_count == 0,
            format!("{:?}", self.promotion.lifecycle_state),
            format!("{:?}", self.writeback.commit_outcome_class),
            format!("{:?}", self.hostile_failure.failure_class),
            format!("{:?}", self.restart_failure.error_kind),
            format!("{:?}", self.writeback.rejection_error_kind),
            self.provenance.main_commit,
            self.provenance.main_snapshot,
            self.provenance.shock_commit,
            self.provenance.shock_snapshot,
            self.provenance.representative_sku,
            self.provenance.representative_retail_price_cents,
            self.provenance.representative_shipping_cost_cents,
            self.provenance.representative_fuel_shipping_component_cents,
            self.digest(),
            self.suite_25_artifact_json()["reference_workload_bundle_digest"]
                .as_str()
                .unwrap_or_default(),
            self.suite_26_artifact_json()["reference_workload_failure_bundle_digest"]
                .as_str()
                .unwrap_or_default(),
            self.suite_27_artifact_json()["certification_bundle_digest"]
                .as_str()
                .unwrap_or_default(),
        )
    }

    pub(super) fn ml_pipeline_export_json(&self) -> serde_json::Value {
        json!({
            "schema": "forge-runtime-bridge.pricing-showcase.ml-pipeline.v1",
            "bundle_digest": self.digest(),
            "showcase_artifact": self.showcase_artifact_json(),
            "counter_snapshot": self.counter_snapshot_json(),
            "suite_25": self.suite_25_artifact_json(),
            "suite_26": self.suite_26_artifact_json(),
            "suite_27": self.suite_27_artifact_json(),
            "lineage_provenance": {
                "reference_lineage": {
                    "source_branch": self.matrix.reference.source_branch,
                    "source_commit": self.matrix.reference.source_commit,
                    "route_snapshot": self.matrix.reference.route_snapshot,
                    "main_snapshot": self.matrix.reference.main_snapshot,
                    "speculative_truth_branch": self.matrix.reference.speculative_truth_branch,
                    "speculative_signal_branch": self.matrix.reference.speculative_signal_branch,
                    "speculative_snapshot": self.matrix.reference.speculative_snapshot,
                    "evaluation_record_identity": self.matrix.reference.evaluation_record_identity,
                    "evaluation_selector_identity": self.matrix.reference.evaluation_selector_identity,
                },
                "route_and_aspect_lineage": {
                    "replay_source_commit": self.matrix.replay.source_commit,
                    "replay_source_snapshot": self.matrix.replay.source_snapshot,
                    "replay_route_identity": self.matrix.replay.route_identity,
                    "replay_invalidation_identity": self.matrix.replay.invalidation_identity,
                    "aspect_route_identity": self.aspect.route_identity,
                    "aspect_source_branch": self.aspect.source_branch,
                    "aspect_source_commit": self.aspect.source_commit,
                    "aspect_registration_id": self.aspect.aspect_registration_id,
                    "aspect_subscription_slice_kind": self.aspect.subscription_slice_kind,
                    "aspect_surface_label": self.aspect.surface_label,
                    "aspect_invalidation_target": self.aspect.invalidation_target,
                    "aspect_match_status": self.aspect.fine_grained_match_status,
                },
                "speculation_lifecycle_lineage": {
                    "discard_live_main_snapshot": self.discard.live_main_snapshot,
                    "discard_post_discard_main_snapshot": self.discard.post_discard_main_snapshot,
                    "discard_replay_outcome": format!("{:?}", self.discard.replay_outcome),
                    "promotion_main_snapshot": self.promotion.main_snapshot,
                    "promotion_speculative_snapshot": self.promotion.speculative_snapshot,
                    "promotion_session_identity": self.promotion.promotion_session_identity,
                    "promotion_lifecycle_state": format!("{:?}", self.promotion.lifecycle_state),
                    "promotion_authoritative_commit_boundary_digest":
                        self.promotion.authoritative_commit_boundary_digest,
                    "promotion_authoritative_artifact_digest":
                        self.promotion.authoritative_artifact_digest,
                    "promotion_replay_outcome": format!("{:?}", self.promotion.replay_outcome),
                },
                "fanout_and_restart_lineage": {
                    "fanout_second_source_commit": self.fanout.second_source_commit,
                    "fanout_second_snapshot": self.fanout.second_snapshot,
                    "fanout_branch_snapshot": self.fanout.branch_snapshot,
                    "restart_source_commit": self.restart_replay.source_commit,
                    "restart_source_snapshot": self.restart_replay.source_snapshot,
                    "restart_route_identity": self.restart_replay.route_identity,
                    "restart_invalidation_identity": self.restart_replay.invalidation_identity,
                    "restart_failure_error_kind": format!("{:?}", self.restart_failure.error_kind),
                    "restart_replay_mismatch_count": self.restart_failure.replay_mismatch_count,
                },
                "writeback_and_merge_lineage": {
                    "writeback_family_kind": self.writeback.family_kind,
                    "writeback_strategy_class": self.writeback.strategy_class,
                    "writeback_commit_outcome_class": format!("{:?}", self.writeback.commit_outcome_class),
                    "writeback_noop_outcome_class": format!("{:?}", self.writeback.noop_outcome_class),
                    "writeback_commit_replay_semantic_digest": self.writeback.commit_replay_semantic_digest,
                    "writeback_noop_replay_semantic_digest": self.writeback.noop_replay_semantic_digest,
                    "writeback_rejection_error_kind": format!("{:?}", self.writeback.rejection_error_kind),
                    "writeback_rejection_failure_class": format!("{:?}", self.writeback.rejection_failure_class),
                    "merge_bridge_class": self.merge.bridge_class,
                    "merge_outcome_class": self.merge.outcome_class,
                    "merge_blocked_stage": self.merge.blocked_stage,
                    "merge_denial_class": self.merge.denial_class,
                    "merge_parent_order_digest": self.merge.parent_order_digest,
                    "merge_bundle_digest": self.merge.bundle_digest,
                    "merge_canonical_replay_digest": self.merge.canonical_replay_digest,
                    "merge_main_premerge_snapshot": self.merge.main_premerge_snapshot,
                    "merge_speculative_snapshot": self.merge.speculative_snapshot,
                    "merge_merged_snapshot": self.merge.merged_snapshot,
                    "merge_aspect_registration_id": self.merge.merged_aspect_registration_id,
                    "merge_fine_grained_match_status": self.merge.merged_fine_grained_match_status,
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
                    "representative_fuel_shipping_component_cents":
                        self.provenance.representative_fuel_shipping_component_cents,
                },
                "hostile_and_trust_lineage": {
                    "hostile_error_kind": format!("{:?}", self.hostile_failure.error_kind),
                    "hostile_failure_class": format!("{:?}", self.hostile_failure.failure_class),
                    "hostile_source_commit": self.hostile_failure.source_commit,
                    "hostile_source_snapshot": self.hostile_failure.source_snapshot,
                    "replay_policy_error_kind": self.trust_attacks.replay_policy_error_kind,
                    "replay_policy_failure_class": self.trust_attacks.replay_policy_failure_class,
                    "route_policy_error_kind": self.trust_attacks.route_policy_error_kind,
                    "merge_denial_blocked_stage": self.trust_attacks.merge_denial_blocked_stage,
                    "merge_denial_class": self.trust_attacks.merge_denial_class,
                },
                "causality": {
                    "suite_25_causality_digest": self.suite_25_artifact_json()["causality_digest"],
                    "suite_25_routing_digest": self.suite_25_artifact_json()["routing_digest"],
                    "suite_25_replay_digest": self.suite_25_artifact_json()["replay_digest"],
                    "suite_25_discard_digest": self.suite_25_artifact_json()["discard_digest"],
                    "suite_25_promotion_digest": self.suite_25_artifact_json()["promotion_digest"],
                    "suite_25_fanout_digest": self.suite_25_artifact_json()["fanout_digest"],
                    "suite_25_writeback_digest": self.suite_25_artifact_json()["writeback_digest"],
                    "suite_25_merge_digest": self.suite_25_artifact_json()["merge_digest"],
                    "suite_25_historical_provenance_digest":
                        self.suite_25_artifact_json()["historical_provenance_digest"],
                    "bundle_digest": self.digest(),
                },
            },
            "simulation": {
                "branch_count": self.simulation.branch_count,
                "iterations_per_branch": self.simulation.iterations_per_branch,
                "ranked_materials_by_damage": self.simulation.ranked_materials_by_damage,
                "material_summaries": self
                    .simulation
                    .material_summaries
                    .iter()
                    .map(|summary| json!({
                        "material": summary.material,
                        "branch_count": summary.branch_count,
                        "iterations_per_branch": summary.iterations_per_branch,
                        "mean_total_retail_delta_cents": summary.mean_total_retail_delta_cents,
                        "mean_shipping_delta_cents": summary.mean_shipping_delta_cents,
                        "mean_material_delta_cents": summary.mean_material_delta_cents,
                        "mean_margin_floor_breach_count": summary.mean_margin_floor_breach_count,
                        "mean_repricing_count": summary.mean_repricing_count,
                        "worst_branch_identity": summary.worst_branch_identity,
                        "worst_branch_mean_total_delta_cents": summary.worst_branch_mean_total_delta_cents,
                        "damage_score": summary.damage_score,
                    }))
                    .collect::<Vec<_>>(),
                "iteration_traces": self
                    .simulation
                    .iteration_traces
                    .iter()
                    .map(|trace| json!({
                        "material": trace.material,
                        "branch_identity": trace.branch_identity,
                        "iteration_index": trace.iteration_index,
                        "regime": trace.regime,
                        "event_kind": trace.event_kind,
                        "shock_multiplier_per_mille": trace.shock_multiplier_per_mille,
                        "baseline_total_retail_cents": trace.baseline_total_retail_cents,
                        "shocked_total_retail_cents": trace.shocked_total_retail_cents,
                        "total_retail_delta_cents": trace.total_retail_delta_cents,
                        "shipping_delta_cents": trace.shipping_delta_cents,
                        "material_delta_cents": trace.material_delta_cents,
                        "margin_floor_breach_count": trace.margin_floor_breach_count,
                        "repricing_count": trace.repricing_count,
                    }))
                    .collect::<Vec<_>>(),
            },
        })
    }
}
