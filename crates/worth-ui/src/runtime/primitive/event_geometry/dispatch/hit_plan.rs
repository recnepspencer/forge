use super::super::super::WorthUiPrimitiveResolvedCursorPosture;
use super::super::digest::event_plan_digest;
use super::super::receipt::WorthUiPrimitiveEventContainment;
use super::dispatch_receipt::{
    WorthUiPrimitiveEventDispatchCandidateReceipt, WorthUiPrimitiveEventDispatchCounters,
    WorthUiPrimitiveEventDispatchOutcome, WorthUiPrimitiveEventDispatchReceipt,
};
use super::region_receipt::{
    WorthUiPrimitiveEventHitTestPoint, WorthUiPrimitiveEventRegionReceipt,
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

    pub fn hit_test(
        &self,
        point: WorthUiPrimitiveEventHitTestPoint,
    ) -> Option<&WorthUiPrimitiveEventRegionReceipt> {
        self.regions
            .iter()
            .filter(|region| region.contains(point))
            .max_by_key(|region| (region.order().depth(), region.order().order()))
    }

    pub fn dispatch_primary_click(
        &self,
        point: WorthUiPrimitiveEventHitTestPoint,
    ) -> WorthUiPrimitiveEventDispatchReceipt {
        let hit_primary = self.hit_test(point);
        let candidates = self.dispatch_candidates(point, hit_primary);
        let Some(primary) = hit_primary else {
            return WorthUiPrimitiveEventDispatchReceipt::empty(self.regions.len(), candidates);
        };
        let mut emitted = Vec::new();
        if primary.can_activate() {
            emitted.push(primary.surface_id().to_owned());
            if primary.containment() == WorthUiPrimitiveEventContainment::Bubble {
                for region in self.parent_regions(primary) {
                    if region.can_activate() {
                        emitted.push(region.surface_id().to_owned());
                    }
                }
            }
        }
        let outcome = dispatch_outcome(primary, &emitted);
        let parent_chain_count = if primary.can_activate()
            && primary.containment() == WorthUiPrimitiveEventContainment::Bubble
        {
            self.parent_regions(primary).count()
        } else {
            0
        };
        WorthUiPrimitiveEventDispatchReceipt::new(
            Some(primary.surface_id().to_owned()),
            emitted.clone(),
            primary.cursor(),
            Some(primary.containment()),
            outcome,
            candidates,
            WorthUiPrimitiveEventDispatchCounters::new(
                self.regions.len(),
                candidates_hit_count(self, point),
                0,
                parent_chain_count,
                emitted.len(),
            ),
        )
    }

    pub fn dispatch_captured_drag(&self, surface_id: &str) -> WorthUiPrimitiveEventDispatchReceipt {
        let Some(primary) = self
            .regions
            .iter()
            .find(|region| region.surface_id() == surface_id)
        else {
            return WorthUiPrimitiveEventDispatchReceipt::empty(self.regions.len(), Vec::new());
        };
        let emitted = if primary.can_activate() {
            vec![primary.surface_id().to_owned()]
        } else {
            Vec::new()
        };
        let candidates = self
            .regions
            .iter()
            .map(|region| {
                let selected = region.surface_id() == primary.surface_id();
                WorthUiPrimitiveEventDispatchCandidateReceipt::from_region(
                    region,
                    selected,
                    selected,
                    selected && region.can_activate(),
                )
            })
            .collect::<Vec<_>>();
        let outcome = dispatch_outcome(primary, &emitted);
        WorthUiPrimitiveEventDispatchReceipt::new(
            Some(primary.surface_id().to_owned()),
            emitted.clone(),
            primary.cursor(),
            Some(primary.containment()),
            outcome,
            candidates,
            WorthUiPrimitiveEventDispatchCounters::new(self.regions.len(), 1, 0, 0, emitted.len()),
        )
    }

    pub fn cursor_receipt_at(
        &self,
        point: WorthUiPrimitiveEventHitTestPoint,
    ) -> WorthUiPrimitiveEventDispatchReceipt {
        let hit_primary = self.hit_test(point);
        let candidates = self.dispatch_candidates(point, hit_primary);
        let cursor = hit_primary
            .map(|region| region.cursor())
            .unwrap_or(WorthUiPrimitiveResolvedCursorPosture::Default);
        WorthUiPrimitiveEventDispatchReceipt::new(
            hit_primary.map(|region| region.surface_id().to_owned()),
            Vec::new(),
            cursor,
            hit_primary.map(|region| region.containment()),
            if hit_primary.is_some() {
                WorthUiPrimitiveEventDispatchOutcome::HitNoActivation
            } else {
                WorthUiPrimitiveEventDispatchOutcome::NoHit
            },
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

    pub fn cursor_at(
        &self,
        point: WorthUiPrimitiveEventHitTestPoint,
    ) -> WorthUiPrimitiveResolvedCursorPosture {
        self.hit_test(point)
            .map(|region| region.cursor())
            .unwrap_or(WorthUiPrimitiveResolvedCursorPosture::Default)
    }

    pub fn plan_digest(&self) -> u64 {
        self.plan_digest
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

    fn dispatch_candidates(
        &self,
        point: WorthUiPrimitiveEventHitTestPoint,
        selected: Option<&WorthUiPrimitiveEventRegionReceipt>,
    ) -> Vec<WorthUiPrimitiveEventDispatchCandidateReceipt> {
        self.regions
            .iter()
            .map(|region| {
                let hit = region.contains(point);
                let selected_region = selected
                    .is_some_and(|selected| selected.receipt_digest() == region.receipt_digest());
                WorthUiPrimitiveEventDispatchCandidateReceipt::from_region(
                    region,
                    hit,
                    selected_region,
                    selected_region && region.can_activate(),
                )
            })
            .collect()
    }
}

fn dispatch_outcome(
    primary: &WorthUiPrimitiveEventRegionReceipt,
    emitted: &[String],
) -> WorthUiPrimitiveEventDispatchOutcome {
    if !primary.can_activate() {
        return WorthUiPrimitiveEventDispatchOutcome::HitDisabled;
    }
    if emitted.is_empty() {
        return WorthUiPrimitiveEventDispatchOutcome::HitNoActivation;
    }
    if emitted.len() > 1 {
        WorthUiPrimitiveEventDispatchOutcome::Bubbled
    } else {
        WorthUiPrimitiveEventDispatchOutcome::Emitted
    }
}

fn candidates_hit_count(
    plan: &WorthUiPrimitiveEventDispatchPlan,
    point: WorthUiPrimitiveEventHitTestPoint,
) -> usize {
    plan.regions()
        .iter()
        .filter(|region| region.contains(point))
        .count()
}
