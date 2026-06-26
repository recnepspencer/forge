use super::source_roots::WorthGraphReadAccessSpatialDenseSourceRegion;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct WorthGraphReadAccessSpatialDenseForbiddenPattern {
    pub label: &'static str,
    pub needle: &'static str,
    pub regions: &'static [WorthGraphReadAccessSpatialDenseSourceRegion],
}

impl WorthGraphReadAccessSpatialDenseForbiddenPattern {
    pub(crate) fn applies_to(&self, region: WorthGraphReadAccessSpatialDenseSourceRegion) -> bool {
        self.regions.contains(&region)
    }
}

const ALL_PHASE_FIVE_REGIONS: &[WorthGraphReadAccessSpatialDenseSourceRegion] = &[
    WorthGraphReadAccessSpatialDenseSourceRegion::PlanAdoptionAuthority,
    WorthGraphReadAccessSpatialDenseSourceRegion::SpatialReadConsumers,
    WorthGraphReadAccessSpatialDenseSourceRegion::TopologyReadConsumers,
    WorthGraphReadAccessSpatialDenseSourceRegion::StandaloneTestInput,
];

pub(crate) const FORBIDDEN_PATTERNS: [WorthGraphReadAccessSpatialDenseForbiddenPattern; 8] = [
    WorthGraphReadAccessSpatialDenseForbiddenPattern {
        label: "local_spatial_evidence_graph_read_fallback",
        needle: "local_spatial_evidence_graph_read_fallback",
        regions: ALL_PHASE_FIVE_REGIONS,
    },
    WorthGraphReadAccessSpatialDenseForbiddenPattern {
        label: "broad_boolean_whole_graph_scan",
        needle: "broad_boolean_whole_graph_scan",
        regions: ALL_PHASE_FIVE_REGIONS,
    },
    WorthGraphReadAccessSpatialDenseForbiddenPattern {
        label: "dense_frontier_local_loop",
        needle: "dense_frontier_local_loop",
        regions: ALL_PHASE_FIVE_REGIONS,
    },
    WorthGraphReadAccessSpatialDenseForbiddenPattern {
        label: "unbounded_ephemeral_graph_index",
        needle: "unbounded_ephemeral_graph_index",
        regions: ALL_PHASE_FIVE_REGIONS,
    },
    WorthGraphReadAccessSpatialDenseForbiddenPattern {
        label: "operator_read_plan_hint",
        needle: "operator_read_plan_hint",
        regions: ALL_PHASE_FIVE_REGIONS,
    },
    WorthGraphReadAccessSpatialDenseForbiddenPattern {
        label: "local_access_mode_switch",
        needle: "local_access_mode_switch",
        regions: ALL_PHASE_FIVE_REGIONS,
    },
    WorthGraphReadAccessSpatialDenseForbiddenPattern {
        label: "required_posture_to_receipt_adapter",
        needle: "required_posture_to_receipt_adapter",
        regions: ALL_PHASE_FIVE_REGIONS,
    },
    WorthGraphReadAccessSpatialDenseForbiddenPattern {
        label: "scalarized_grouped_graph_read_loop",
        needle: "scalarized_grouped_graph_read_loop",
        regions: ALL_PHASE_FIVE_REGIONS,
    },
];
