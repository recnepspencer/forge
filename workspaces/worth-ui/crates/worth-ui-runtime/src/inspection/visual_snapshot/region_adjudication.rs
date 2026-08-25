pub(crate) struct UiRegionAdjudicationInput<'snapshot> {
    pub(crate) region: worth_ui_inspection::UiClientPhysicalRect,
    pub(crate) visible_index: &'snapshot super::UiVisibleRegionIndex,
    pub(crate) trace_basis: &'snapshot crate::mounting::UiMountedIdentityTraceBasis,
    pub(crate) budget: worth_ui_inspection::UiVisualQueryBudget,
}

#[derive(Default)]
struct UiRegionQueryCost {
    spatial_probes: usize,
    candidates: usize,
    trace_probes: usize,
}

struct UiRegionCandidateSelection<'index> {
    records: Vec<&'index super::UiVisibleRegionRecord>,
    cost: UiRegionQueryCost,
    exhausted: bool,
}

struct UiRegionProjection {
    intersections: Vec<worth_ui_inspection::UiVisualRegionIntersection>,
    opaque_coverage: Vec<worth_ui_inspection::UiClientPhysicalRect>,
    cost: UiRegionQueryCost,
}

pub(crate) fn adjudicate_region(
    input: UiRegionAdjudicationInput<'_>,
) -> worth_ui_inspection::UiVisualRegionAdjudication {
    let selection = select_candidates(&input);
    if selection.exhausted {
        return result(
            Vec::new(),
            worth_ui_inspection::UiVisualRegionCompleteness::Truncated,
            input.budget,
            selection.cost,
        );
    }
    let mut projection = UiRegionProjection::new(selection.cost);
    for record in selection.records {
        if let Err(completeness) = projection.project_record(&input, *record) {
            return result(
                projection.intersections,
                completeness,
                input.budget,
                projection.cost,
            );
        }
    }
    projection.finish(input.budget)
}

fn select_candidates<'index>(
    input: &'index UiRegionAdjudicationInput<'_>,
) -> UiRegionCandidateSelection<'index> {
    let (mut records, spatial_probes, exhausted) = input
        .visible_index
        .region_candidates(input.region, usize::from(input.budget.maximum_candidates()))
        .into_parts();
    records.sort_unstable_by_key(|record| {
        (
            std::cmp::Reverse(record.layer_order()),
            std::cmp::Reverse(record.paint_order()),
        )
    });
    UiRegionCandidateSelection {
        cost: UiRegionQueryCost {
            spatial_probes,
            candidates: records.len(),
            trace_probes: 0,
        },
        records,
        exhausted,
    }
}

impl UiRegionProjection {
    fn new(cost: UiRegionQueryCost) -> Self {
        Self {
            intersections: Vec::new(),
            opaque_coverage: Vec::new(),
            cost,
        }
    }

    fn project_record(
        &mut self,
        input: &UiRegionAdjudicationInput<'_>,
        record: super::UiVisibleRegionRecord,
    ) -> Result<(), worth_ui_inspection::UiVisualRegionCompleteness> {
        let Some(record_region) =
            super::region_occlusion::intersection(input.region, record.inspection_region())
        else {
            return Ok(());
        };
        let fragments =
            super::region_occlusion::subtract_opaque_coverage(record_region, &self.opaque_coverage);
        if fragments.is_empty() {
            return Ok(());
        }
        if record.opacity() == super::UiVisibleOpacity::Unsupported {
            return Err(worth_ui_inspection::UiVisualRegionCompleteness::Unsupported);
        }
        if self.intersections.len().saturating_add(fragments.len())
            > usize::from(input.budget.maximum_results())
        {
            return Err(worth_ui_inspection::UiVisualRegionCompleteness::Truncated);
        }
        let resolved = super::resolve_identity_trace(input.trace_basis, record.node_receipt())
            .expect("sealed visible records retain their exact mounted trace basis");
        let (trace, trace_cost) = resolved.into_parts();
        self.cost.trace_probes = self
            .cost
            .trace_probes
            .saturating_add(trace_cost.index_probes());
        self.intersections
            .extend(fragments.into_iter().map(|region| {
                worth_ui_inspection::UiVisualRegionIntersection::from_runtime_projection(
                    region,
                    trace.clone(),
                )
            }));
        if record.opacity() == super::UiVisibleOpacity::Opaque {
            self.opaque_coverage.push(record_region);
        }
        Ok(())
    }

    fn finish(
        self,
        budget: worth_ui_inspection::UiVisualQueryBudget,
    ) -> worth_ui_inspection::UiVisualRegionAdjudication {
        let completeness = if self.intersections.is_empty() {
            worth_ui_inspection::UiVisualRegionCompleteness::EmptyAndComplete
        } else {
            worth_ui_inspection::UiVisualRegionCompleteness::Complete
        };
        result(self.intersections, completeness, budget, self.cost)
    }
}

fn result(
    intersections: Vec<worth_ui_inspection::UiVisualRegionIntersection>,
    completeness: worth_ui_inspection::UiVisualRegionCompleteness,
    budget: worth_ui_inspection::UiVisualQueryBudget,
    cost: UiRegionQueryCost,
) -> worth_ui_inspection::UiVisualRegionAdjudication {
    worth_ui_inspection::UiVisualRegionAdjudication::from_runtime_projection(
        intersections,
        completeness,
        budget,
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
