use super::alignment_summary::{
    current_topology_public_closeout_alignment_summary,
    current_topology_public_closeout_alignment_summary_with_cutover_loader,
    TopologyPublicCloseoutAlignmentSummary, TopologyPublicCloseoutFreshnessRequirementPosture,
    TopologyPublicCloseoutRenderedOutputComparisonPosture, TopologyPublicCloseoutSeedSupportError,
};
use crate::projection::query_backed_consumer_cutover::current_topology_query_backed_consumer_cutover_with_hostile_selected_basis_overrides;
use std::sync::OnceLock;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyMilestoneFifteenPlannerSeedSupport {
    selected_equivalence_family_identity: String,
    selected_equivalence_basis_identity_digest: String,
    selected_compatibility_basis_identity_digest: String,
    selected_reuse_basis_identity_digest: String,
    reuse_decision_identity_digest: Option<String>,
    rebuild_denial_identity_digest: Option<String>,
    freshness_requirement_posture: TopologyPublicCloseoutFreshnessRequirementPosture,
    rendered_output_comparison_posture: TopologyPublicCloseoutRenderedOutputComparisonPosture,
    query_execution_count: usize,
    row_scan_fallback_count: usize,
    whole_view_fallback_count: usize,
    repeated_rediscovery_denied_count: usize,
}

pub fn current_topology_milestone_fifteen_planner_seed_support(
) -> Result<TopologyMilestoneFifteenPlannerSeedSupport, TopologyPublicCloseoutSeedSupportError> {
    static CACHE: OnceLock<TopologyMilestoneFifteenPlannerSeedSupport> = OnceLock::new();
    if let Some(cached) = CACHE.get() {
        return Ok(cached.clone());
    }

    let support = current_topology_milestone_fifteen_planner_seed_support_with_summary_loader(
        current_topology_public_closeout_alignment_summary,
    )?;
    let _ = CACHE.set(support.clone());
    Ok(support)
}

pub fn current_topology_milestone_fifteen_planner_seed_support_with_hostile_selected_reuse_basis_identity_digest(
    selected_reuse_basis_identity_digest: &str,
) -> Result<TopologyMilestoneFifteenPlannerSeedSupport, TopologyPublicCloseoutSeedSupportError> {
    current_topology_milestone_fifteen_planner_seed_support_with_summary_loader(|| {
        current_topology_public_closeout_alignment_summary_with_cutover_loader(|| {
            current_topology_query_backed_consumer_cutover_with_hostile_selected_basis_overrides(
                None,
                Some(selected_reuse_basis_identity_digest),
            )
        })
    })
}

pub fn current_topology_query_backed_consumer_cutover_with_hostile_loop_cycle_selected_compatibility_basis(
    identity_digest: &str,
) -> Result<
    crate::facade::TopologyQueryBackedConsumerCutover,
    crate::facade::TopologyQueryBackedConsumerCutoverCurrentError,
> {
    current_topology_query_backed_consumer_cutover_with_hostile_selected_basis_overrides(
        Some(identity_digest),
        None,
    )
}

pub(crate) fn current_topology_milestone_fifteen_planner_seed_support_with_summary_loader<F>(
    load_summary: F,
) -> Result<TopologyMilestoneFifteenPlannerSeedSupport, TopologyPublicCloseoutSeedSupportError>
where
    F: FnOnce() -> Result<
        TopologyPublicCloseoutAlignmentSummary,
        TopologyPublicCloseoutSeedSupportError,
    >,
{
    let summary = load_summary()?;
    Ok(TopologyMilestoneFifteenPlannerSeedSupport::from_alignment_summary(&summary))
}

impl TopologyMilestoneFifteenPlannerSeedSupport {
    fn from_alignment_summary(summary: &TopologyPublicCloseoutAlignmentSummary) -> Self {
        Self {
            selected_equivalence_family_identity: summary
                .selected_equivalence_family_identity()
                .to_string(),
            selected_equivalence_basis_identity_digest: summary
                .selected_equivalence_basis_identity_digest()
                .to_string(),
            selected_compatibility_basis_identity_digest: summary
                .selected_compatibility_basis_identity_digest()
                .to_string(),
            selected_reuse_basis_identity_digest: summary
                .selected_reuse_basis_identity_digest()
                .to_string(),
            reuse_decision_identity_digest: summary
                .reuse_decision_identity_digest()
                .map(str::to_string),
            rebuild_denial_identity_digest: summary
                .rebuild_denial_identity_digest()
                .map(str::to_string),
            freshness_requirement_posture: summary.freshness_requirement_posture(),
            rendered_output_comparison_posture: summary.rendered_output_comparison_posture(),
            query_execution_count: summary.query_execution_count(),
            row_scan_fallback_count: summary.row_scan_fallback_count(),
            whole_view_fallback_count: summary.whole_view_fallback_count(),
            repeated_rediscovery_denied_count: summary.repeated_rediscovery_denied_count(),
        }
    }

    pub fn selected_equivalence_family_identity(&self) -> &str {
        &self.selected_equivalence_family_identity
    }

    pub fn selected_equivalence_basis_identity_digest(&self) -> &str {
        &self.selected_equivalence_basis_identity_digest
    }

    pub fn selected_compatibility_basis_identity_digest(&self) -> &str {
        &self.selected_compatibility_basis_identity_digest
    }

    pub fn selected_reuse_basis_identity_digest(&self) -> &str {
        &self.selected_reuse_basis_identity_digest
    }

    pub fn reuse_decision_identity_digest(&self) -> Option<&str> {
        self.reuse_decision_identity_digest.as_deref()
    }

    pub fn rebuild_denial_identity_digest(&self) -> Option<&str> {
        self.rebuild_denial_identity_digest.as_deref()
    }

    pub const fn freshness_requirement_posture(
        &self,
    ) -> TopologyPublicCloseoutFreshnessRequirementPosture {
        self.freshness_requirement_posture
    }

    pub const fn rendered_output_comparison_posture(
        &self,
    ) -> TopologyPublicCloseoutRenderedOutputComparisonPosture {
        self.rendered_output_comparison_posture
    }

    pub const fn query_execution_count(&self) -> usize {
        self.query_execution_count
    }

    pub const fn row_scan_fallback_count(&self) -> usize {
        self.row_scan_fallback_count
    }

    pub const fn whole_view_fallback_count(&self) -> usize {
        self.whole_view_fallback_count
    }

    pub const fn repeated_rediscovery_denied_count(&self) -> usize {
        self.repeated_rediscovery_denied_count
    }
}
