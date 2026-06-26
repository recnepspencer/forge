use super::firewall_region::WorthGraphReadAccessHardDeletionSourceRegion;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct HardDeletionForbiddenPattern {
    label: &'static str,
    needle: &'static str,
    regions: &'static [WorthGraphReadAccessHardDeletionSourceRegion],
}

impl HardDeletionForbiddenPattern {
    const fn new(
        label: &'static str,
        needle: &'static str,
        regions: &'static [WorthGraphReadAccessHardDeletionSourceRegion],
    ) -> Self {
        Self {
            label,
            needle,
            regions,
        }
    }

    pub(crate) const fn label(&self) -> &'static str {
        self.label
    }

    pub(crate) const fn needle(&self) -> &'static str {
        self.needle
    }

    pub(crate) fn applies_to(&self, region: WorthGraphReadAccessHardDeletionSourceRegion) -> bool {
        self.regions.contains(&region)
    }

    #[cfg(test)]
    pub(crate) fn regions(&self) -> &[WorthGraphReadAccessHardDeletionSourceRegion] {
        self.regions
    }
}

const ALL_HARD_DELETION_REGIONS: &[WorthGraphReadAccessHardDeletionSourceRegion] = &[
    WorthGraphReadAccessHardDeletionSourceRegion::PlanAdoptionAuthority,
    WorthGraphReadAccessHardDeletionSourceRegion::TopologyReadConsumers,
    WorthGraphReadAccessHardDeletionSourceRegion::SpatialReadConsumers,
    WorthGraphReadAccessHardDeletionSourceRegion::KernelGraphReadHelpers,
    WorthGraphReadAccessHardDeletionSourceRegion::StandaloneTestInput,
];

pub(crate) const HARD_DELETION_FORBIDDEN_PATTERNS: &[HardDeletionForbiddenPattern] = &[
    HardDeletionForbiddenPattern::new(
        "local_graph_read_loop",
        "local_graph_read_loop",
        ALL_HARD_DELETION_REGIONS,
    ),
    HardDeletionForbiddenPattern::new(
        "local_graph_traversal_call",
        "local_graph_traversal()",
        ALL_HARD_DELETION_REGIONS,
    ),
    HardDeletionForbiddenPattern::new(
        "local_adjacency_map",
        "local_adjacency_map",
        ALL_HARD_DELETION_REGIONS,
    ),
    HardDeletionForbiddenPattern::new(
        "local_graph_cache",
        "local_graph_cache",
        ALL_HARD_DELETION_REGIONS,
    ),
    HardDeletionForbiddenPattern::new("local_cache", "local_cache", ALL_HARD_DELETION_REGIONS),
    HardDeletionForbiddenPattern::new(
        "local_spatial_evidence_graph_read_fallback",
        "local_spatial_evidence_graph_read_fallback",
        ALL_HARD_DELETION_REGIONS,
    ),
    HardDeletionForbiddenPattern::new(
        "dense_frontier_local_loop",
        "dense_frontier_local_loop",
        ALL_HARD_DELETION_REGIONS,
    ),
    HardDeletionForbiddenPattern::new(
        "call_query_after_local_traversal",
        "call_query_after_local_traversal",
        ALL_HARD_DELETION_REGIONS,
    ),
    HardDeletionForbiddenPattern::new(
        "fabricated_graph_read_receipt",
        "fabricated_graph_read_receipt",
        ALL_HARD_DELETION_REGIONS,
    ),
    HardDeletionForbiddenPattern::new(
        "fabricated_access_plan_receipt",
        "fabricated_access_plan_receipt",
        ALL_HARD_DELETION_REGIONS,
    ),
    HardDeletionForbiddenPattern::new(
        "old_helper_to_query_receipt_adapter",
        "old_helper_to_query_receipt_adapter",
        ALL_HARD_DELETION_REGIONS,
    ),
    HardDeletionForbiddenPattern::new(
        "operator_read_plan_hint",
        "operator_read_plan_hint",
        ALL_HARD_DELETION_REGIONS,
    ),
    HardDeletionForbiddenPattern::new(
        "local_access_mode_switch",
        "local_access_mode_switch",
        ALL_HARD_DELETION_REGIONS,
    ),
    HardDeletionForbiddenPattern::new(
        "execution_strategy_hint_enum",
        "execution_strategy_hint_enum",
        ALL_HARD_DELETION_REGIONS,
    ),
    HardDeletionForbiddenPattern::new(
        "manual_read_plan_list",
        "manual_read_plan_list",
        ALL_HARD_DELETION_REGIONS,
    ),
    HardDeletionForbiddenPattern::new(
        "manual_read_plan_call",
        "manual_read_plan()",
        ALL_HARD_DELETION_REGIONS,
    ),
];

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct HardDeletionForbiddenPatternAuditRow {
    label: &'static str,
    needle: &'static str,
    region: WorthGraphReadAccessHardDeletionSourceRegion,
}

#[cfg(test)]
impl HardDeletionForbiddenPatternAuditRow {
    pub(in crate::graph_read_access_plan_adoption::phase_seven_hard_deletion) const fn label(
        &self,
    ) -> &'static str {
        self.label
    }

    pub(in crate::graph_read_access_plan_adoption::phase_seven_hard_deletion) const fn needle(
        &self,
    ) -> &'static str {
        self.needle
    }

    pub(in crate::graph_read_access_plan_adoption::phase_seven_hard_deletion) const fn region(
        &self,
    ) -> WorthGraphReadAccessHardDeletionSourceRegion {
        self.region
    }
}

#[cfg(test)]
pub(crate) fn forbidden_pattern_audit_rows() -> Vec<HardDeletionForbiddenPatternAuditRow> {
    HARD_DELETION_FORBIDDEN_PATTERNS
        .iter()
        .flat_map(|pattern| {
            pattern.regions().iter().copied().map(move |region| {
                HardDeletionForbiddenPatternAuditRow {
                    label: pattern.label(),
                    needle: pattern.needle(),
                    region,
                }
            })
        })
        .collect()
}
