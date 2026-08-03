pub(crate) struct UiPointAdjudicationInput<'snapshot> {
    pub(crate) point: worth_ui_inspection::UiClientPhysicalPixel,
    pub(crate) visible_index: &'snapshot super::UiVisibleRegionIndex,
    pub(crate) hit_test_index: &'snapshot super::UiHitTestRegionIndex,
    pub(crate) trace_basis: &'snapshot crate::mounting::UiMountedIdentityTraceBasis,
    pub(crate) budget: worth_ui_inspection::UiVisualQueryBudget,
}

#[derive(Default)]
struct UiPointQueryCost {
    spatial_probes: usize,
    candidates: usize,
    trace_probes: usize,
}

pub(crate) fn adjudicate_point(
    input: UiPointAdjudicationInput<'_>,
) -> worth_ui_inspection::UiVisualPointAdjudication {
    let maximum_candidates = usize::from(input.budget.maximum_candidates());
    let mut cost = UiPointQueryCost::default();
    let visible = visible_outcome(&input, maximum_candidates, &mut cost);
    let remaining = maximum_candidates.saturating_sub(cost.candidates);
    let hit_test = hit_test_outcome(&input, remaining, &mut cost);
    worth_ui_inspection::UiVisualPointAdjudication::from_runtime_projection(
        visible,
        hit_test,
        input.budget,
        worth_ui_inspection::UiVisualInspectionCostReceipt::from_runtime_projection([
            0,
            cost.spatial_probes as u64,
            cost.candidates as u64,
            cost.trace_probes as u64,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ]),
    )
}

fn visible_outcome(
    input: &UiPointAdjudicationInput<'_>,
    maximum_candidates: usize,
    cost: &mut UiPointQueryCost,
) -> worth_ui_inspection::UiVisualVisibleOutcome {
    let (mut candidates, probes, exhausted) = input
        .visible_index
        .point_candidates(input.point, maximum_candidates)
        .into_parts();
    cost.spatial_probes = cost.spatial_probes.saturating_add(probes);
    cost.candidates = cost.candidates.saturating_add(candidates.len());
    if exhausted {
        return worth_ui_inspection::UiVisualVisibleOutcome::Incomplete(input.budget);
    }
    candidates.sort_unstable_by_key(|record| {
        (
            std::cmp::Reverse(record.layer_order()),
            std::cmp::Reverse(record.paint_order()),
        )
    });
    let maximum_results = usize::from(input.budget.maximum_results());
    let mut contributors = Vec::new();
    for record in candidates {
        if contributors.len() == maximum_results {
            return worth_ui_inspection::UiVisualVisibleOutcome::Incomplete(input.budget);
        }
        let resolved = super::resolve_identity_trace(input.trace_basis, record.node_receipt())
            .expect("sealed visible records retain their exact mounted trace basis");
        let (trace, trace_cost) = resolved.into_parts();
        cost.trace_probes = cost.trace_probes.saturating_add(trace_cost.index_probes());
        contributors.push(
            worth_ui_inspection::UiVisualVisibleContributor::from_runtime_projection(
                record.inspection_region(),
                record.layer_order(),
                record.paint_order(),
                trace,
            ),
        );
        if record.opacity() == super::UiVisibleOpacity::Opaque {
            break;
        }
    }
    if contributors.is_empty() {
        worth_ui_inspection::UiVisualVisibleOutcome::None
    } else {
        worth_ui_inspection::UiVisualVisibleOutcome::Contributors(
            worth_ui_inspection::UiVisualContributorStack::from_runtime_projection(contributors),
        )
    }
}

fn hit_test_outcome(
    input: &UiPointAdjudicationInput<'_>,
    maximum_candidates: usize,
    cost: &mut UiPointQueryCost,
) -> worth_ui_inspection::UiVisualHitTestOutcome {
    let (mut candidates, probes, exhausted) = input
        .hit_test_index
        .point_candidates(input.point, maximum_candidates)
        .into_parts();
    cost.spatial_probes = cost.spatial_probes.saturating_add(probes);
    cost.candidates = cost.candidates.saturating_add(candidates.len());
    if exhausted {
        return worth_ui_inspection::UiVisualHitTestOutcome::Incomplete(input.budget);
    }
    candidates.sort_unstable_by_key(|record| record.total_order().rank());
    let Some(target) = candidates.first() else {
        return worth_ui_inspection::UiVisualHitTestOutcome::None;
    };
    let resolved = super::resolve_identity_trace(input.trace_basis, target.node_receipt())
        .expect("sealed hit-test records retain their exact mounted trace basis");
    let (trace, trace_cost) = resolved.into_parts();
    cost.trace_probes = cost.trace_probes.saturating_add(trace_cost.index_probes());
    worth_ui_inspection::UiVisualHitTestOutcome::Target(
        worth_ui_inspection::UiVisualHitTestTarget::from_runtime_projection(
            target.total_order().rank(),
            trace,
        ),
    )
}
