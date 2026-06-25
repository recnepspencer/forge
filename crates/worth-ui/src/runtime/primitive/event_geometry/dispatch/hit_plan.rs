use super::super::digest::event_plan_digest;
use super::candidate_receipt::WorthUiPrimitiveEventDispatchCandidateReceipt;
use super::dispatch_receipt::{
    WorthUiPrimitiveEventDispatchCounters, WorthUiPrimitiveEventDispatchReceipt,
};
use super::graph_binding::primitive_event_dispatch_execution;
use super::outcome_receipt::WorthUiPrimitiveEventDispatchOutcome;
use super::plan_resolution::{
    candidate_for_region, candidates_hit_count, dispatch_outcome, primary_activation_bubbles,
    region_activation_is_eligible, CandidateSelectionMode,
};
use super::region_receipt::{
    WorthUiPrimitiveEventHitTestPoint, WorthUiPrimitiveEventRegionReceipt,
};
use crate::runtime::{
    WorthUiEventDispatchTargetBinding, WorthUiPrimitiveEventContainment,
    WorthUiPrimitiveHostAppearanceObservation,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveEventDispatchPlan {
    regions: Vec<WorthUiPrimitiveEventRegionReceipt>,
    plan_digest: u64,
}

impl WorthUiPrimitiveEventDispatchPlan {
    pub fn from_regions(
        regions: impl IntoIterator<Item = WorthUiPrimitiveEventRegionReceipt>,
    ) -> Self {
        let mut regions = regions.into_iter().collect::<Vec<_>>();
        regions.sort_by_key(|region| (region.order().depth(), region.order().order()));
        let plan_digest = event_plan_digest(&regions);
        Self {
            regions,
            plan_digest,
        }
    }

    pub fn regions(&self) -> &[WorthUiPrimitiveEventRegionReceipt] {
        &self.regions
    }

    fn hit_test(
        &self,
        point: WorthUiPrimitiveEventHitTestPoint,
    ) -> Option<&WorthUiPrimitiveEventRegionReceipt> {
        self.regions
            .iter()
            .filter(|region| region.contains(point))
            .max_by_key(|region| (region.order().depth(), region.order().order()))
    }

    pub(crate) fn dispatch_primary_click(
        &self,
        point: WorthUiPrimitiveEventHitTestPoint,
    ) -> WorthUiPrimitiveEventDispatchReceipt {
        let primary = self.hit_test(point);
        self.dispatch_primary_click_for_region(point, primary)
    }

    pub fn dispatch_primary_click_for_target(
        &self,
        target: &WorthUiEventDispatchTargetBinding,
        point: WorthUiPrimitiveEventHitTestPoint,
    ) -> WorthUiPrimitiveEventDispatchReceipt {
        let primary = self.hit_test(point);
        if let Some(primary) = primary {
            if primary.surface_id() != target.surface_id().as_str() {
                let candidates =
                    self.dispatch_candidates(point, Some(primary), CandidateSelectionMode::Press);
                return self.dispatch_receipt(
                    WorthUiPrimitiveEventDispatchOutcome::denied(primary, "target-mismatch"),
                    candidates,
                    WorthUiPrimitiveEventDispatchCounters::new(
                        self.regions.len(),
                        candidates_hit_count(self.regions(), point),
                        0,
                        0,
                        0,
                    ),
                );
            }
        }
        self.dispatch_primary_click_for_region(point, primary)
    }

    fn dispatch_primary_click_for_region(
        &self,
        point: WorthUiPrimitiveEventHitTestPoint,
        primary: Option<&WorthUiPrimitiveEventRegionReceipt>,
    ) -> WorthUiPrimitiveEventDispatchReceipt {
        let candidates = self.dispatch_candidates(point, primary, CandidateSelectionMode::Press);
        let Some(primary) = primary else {
            return self.dispatch_receipt(
                WorthUiPrimitiveEventDispatchOutcome::no_hit(),
                candidates,
                WorthUiPrimitiveEventDispatchCounters::new(self.regions.len(), 0, 0, 0, 0),
            );
        };
        let emitted = self.emitted_surfaces_for_primary(primary);
        let parent_chain_count = if primary_activation_bubbles(primary) {
            self.parent_regions(primary).count()
        } else {
            0
        };
        let outcome = dispatch_outcome(primary, emitted.clone());
        self.dispatch_receipt(
            outcome,
            candidates,
            WorthUiPrimitiveEventDispatchCounters::new(
                self.regions.len(),
                candidates_hit_count(self.regions(), point),
                0,
                parent_chain_count,
                emitted.len(),
            ),
        )
    }

    pub(crate) fn primary_region_at(
        &self,
        point: WorthUiPrimitiveEventHitTestPoint,
    ) -> Option<&WorthUiPrimitiveEventRegionReceipt> {
        self.hit_test(point)
    }

    pub(in crate::runtime::primitive::event_geometry::dispatch) fn dispatch_captured_drag(
        &self,
        surface_id: &str,
    ) -> WorthUiPrimitiveEventDispatchReceipt {
        let Some(primary) = self
            .regions
            .iter()
            .find(|region| region.surface_id() == surface_id)
        else {
            return self.dispatch_receipt(
                WorthUiPrimitiveEventDispatchOutcome::no_hit(),
                Vec::new(),
                WorthUiPrimitiveEventDispatchCounters::new(self.regions.len(), 0, 0, 0, 0),
            );
        };
        let candidates = self
            .regions
            .iter()
            .map(|region| {
                if region.surface_id() == primary.surface_id() {
                    WorthUiPrimitiveEventDispatchCandidateReceipt::captured_target(region)
                } else {
                    WorthUiPrimitiveEventDispatchCandidateReceipt::pass_through(region)
                }
            })
            .collect::<Vec<_>>();
        let outcome = if region_activation_is_eligible(primary) {
            WorthUiPrimitiveEventDispatchOutcome::captured(primary)
        } else {
            WorthUiPrimitiveEventDispatchOutcome::disabled(primary)
        };
        self.dispatch_receipt(
            outcome,
            candidates,
            WorthUiPrimitiveEventDispatchCounters::new(
                self.regions.len(),
                1,
                0,
                0,
                usize::from(region_activation_is_eligible(primary)),
            ),
        )
    }

    pub(crate) fn cursor_receipt_at(
        &self,
        point: WorthUiPrimitiveEventHitTestPoint,
    ) -> WorthUiPrimitiveEventDispatchReceipt {
        let primary = self.hit_test(point);
        let candidates = self.dispatch_candidates(point, primary, CandidateSelectionMode::Hover);
        let outcome = primary.map_or_else(WorthUiPrimitiveEventDispatchOutcome::no_hit, |region| {
            WorthUiPrimitiveEventDispatchOutcome::denied(region, "cursor-observation")
        });
        self.dispatch_receipt(
            outcome,
            candidates,
            WorthUiPrimitiveEventDispatchCounters::new(
                self.regions.len(),
                0,
                self.regions.len(),
                0,
                0,
            ),
        )
    }

    pub fn cursor_receipt_for_target(
        &self,
        target: &WorthUiEventDispatchTargetBinding,
        point: WorthUiPrimitiveEventHitTestPoint,
    ) -> WorthUiPrimitiveEventDispatchReceipt {
        let primary = self.hit_test(point);
        if let Some(primary) = primary {
            if primary.surface_id() != target.surface_id().as_str() {
                let candidates =
                    self.dispatch_candidates(point, Some(primary), CandidateSelectionMode::Hover);
                return self.dispatch_receipt(
                    WorthUiPrimitiveEventDispatchOutcome::denied(primary, "target-mismatch"),
                    candidates,
                    WorthUiPrimitiveEventDispatchCounters::new(
                        self.regions.len(),
                        candidates_hit_count(self.regions(), point),
                        self.regions.len(),
                        0,
                        0,
                    ),
                );
            }
        }
        self.cursor_receipt_at(point)
    }

    fn appearance_observation_at(
        &self,
        point: WorthUiPrimitiveEventHitTestPoint,
        primary_down: bool,
        surface_id: &str,
    ) -> WorthUiPrimitiveHostAppearanceObservation {
        let hover_receipt = self.cursor_receipt_at(point);
        let hovered = hover_receipt
            .candidates()
            .iter()
            .any(|candidate| candidate.surface_id() == surface_id && candidate.selected());
        let pressed = primary_down
            && self.hit_test(point).is_some_and(|region| {
                region.surface_id() == surface_id && region_activation_is_eligible(region)
            });
        WorthUiPrimitiveHostAppearanceObservation::new(hovered, pressed, false)
    }

    pub fn appearance_observation_for_target(
        &self,
        point: WorthUiPrimitiveEventHitTestPoint,
        primary_down: bool,
        target: &WorthUiEventDispatchTargetBinding,
    ) -> WorthUiPrimitiveHostAppearanceObservation {
        self.appearance_observation_at(point, primary_down, target.surface_id().as_str())
    }

    pub fn plan_digest(&self) -> u64 {
        self.plan_digest
    }

    fn dispatch_receipt(
        &self,
        outcome: WorthUiPrimitiveEventDispatchOutcome,
        candidates: Vec<WorthUiPrimitiveEventDispatchCandidateReceipt>,
        counters: WorthUiPrimitiveEventDispatchCounters,
    ) -> WorthUiPrimitiveEventDispatchReceipt {
        let graph_execution = primitive_event_dispatch_execution(&outcome, self.regions());
        WorthUiPrimitiveEventDispatchReceipt::new(outcome, candidates, counters, graph_execution)
    }

    fn parent_regions<'a>(
        &'a self,
        child: &'a WorthUiPrimitiveEventRegionReceipt,
    ) -> impl Iterator<Item = &'a WorthUiPrimitiveEventRegionReceipt> + 'a {
        self.regions
            .iter()
            .rev()
            .filter(move |region| child.parent_surface_id() == Some(region.surface_id()))
            .filter(move |region| region.order().depth() < child.order().depth())
    }

    fn emitted_surfaces_for_primary(
        &self,
        primary: &WorthUiPrimitiveEventRegionReceipt,
    ) -> Vec<String> {
        if !region_activation_is_eligible(primary) {
            return Vec::new();
        }
        let mut emitted = vec![primary.surface_id().to_owned()];
        if primary.containment() == WorthUiPrimitiveEventContainment::Bubble {
            emitted.extend(
                self.parent_regions(primary)
                    .filter(|region| region_activation_is_eligible(region))
                    .map(|region| region.surface_id().to_owned()),
            );
        }
        emitted
    }

    fn dispatch_candidates(
        &self,
        point: WorthUiPrimitiveEventHitTestPoint,
        selected: Option<&WorthUiPrimitiveEventRegionReceipt>,
        mode: CandidateSelectionMode,
    ) -> Vec<WorthUiPrimitiveEventDispatchCandidateReceipt> {
        self.regions
            .iter()
            .map(|region| {
                candidate_for_region(
                    region,
                    region.contains(point),
                    selected,
                    mode,
                    self.parent_regions(selected.unwrap_or(region))
                        .any(|parent| parent.surface_id() == region.surface_id()),
                )
            })
            .collect()
    }
}
