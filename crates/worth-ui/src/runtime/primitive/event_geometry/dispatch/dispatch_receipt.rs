use super::super::super::{WorthUiBoxEdges, WorthUiPrimitiveResolvedCursorPosture};
use super::super::digest::event_dispatch_digest;
use super::super::receipt::WorthUiPrimitiveEventContainment;
use super::region_receipt::{WorthUiPrimitiveEventRegionOrder, WorthUiPrimitiveEventRegionReceipt};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveHitFrameDerivationBasis {
    VisualBounds,
    FlowPadding,
    ExplicitHitSlop,
    DisabledNone,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WorthUiPrimitiveHitFrameDerivationReceipt {
    basis: WorthUiPrimitiveHitFrameDerivationBasis,
    edges: WorthUiBoxEdges,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveEventDispatchOutcome {
    NoHit,
    HitDisabled,
    HitNoActivation,
    Emitted,
    Bubbled,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveEventDispatchCandidateReceipt {
    surface_id: String,
    parent_surface_id: Option<String>,
    order: WorthUiPrimitiveEventRegionOrder,
    hit: bool,
    selected: bool,
    emitted: bool,
    can_activate: bool,
    cursor: WorthUiPrimitiveResolvedCursorPosture,
    containment: WorthUiPrimitiveEventContainment,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiPrimitiveEventDispatchCounters {
    region_count: usize,
    hit_candidate_count: usize,
    cursor_candidate_count: usize,
    parent_chain_count: usize,
    emitted_surface_count: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveEventDispatchReceipt {
    primary_surface_id: Option<String>,
    emitted_surface_ids: Vec<String>,
    cursor: WorthUiPrimitiveResolvedCursorPosture,
    containment: Option<WorthUiPrimitiveEventContainment>,
    outcome: WorthUiPrimitiveEventDispatchOutcome,
    candidates: Vec<WorthUiPrimitiveEventDispatchCandidateReceipt>,
    counters: WorthUiPrimitiveEventDispatchCounters,
    dispatch_digest: u64,
}

impl WorthUiPrimitiveHitFrameDerivationReceipt {
    pub(super) fn new(
        basis: WorthUiPrimitiveHitFrameDerivationBasis,
        edges: WorthUiBoxEdges,
    ) -> Self {
        Self { basis, edges }
    }

    pub fn basis(self) -> WorthUiPrimitiveHitFrameDerivationBasis {
        self.basis
    }

    pub fn edges(self) -> WorthUiBoxEdges {
        self.edges
    }
}

impl WorthUiPrimitiveEventDispatchCandidateReceipt {
    pub(super) fn from_region(
        region: &WorthUiPrimitiveEventRegionReceipt,
        hit: bool,
        selected: bool,
        emitted: bool,
    ) -> Self {
        Self {
            surface_id: region.surface_id().to_owned(),
            parent_surface_id: region.parent_surface_id().map(str::to_owned),
            order: region.order(),
            hit,
            selected,
            emitted,
            can_activate: region.can_activate(),
            cursor: region.cursor(),
            containment: region.containment(),
        }
    }

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn parent_surface_id(&self) -> Option<&str> {
        self.parent_surface_id.as_deref()
    }

    pub fn order(&self) -> WorthUiPrimitiveEventRegionOrder {
        self.order
    }

    pub fn hit(&self) -> bool {
        self.hit
    }

    pub fn selected(&self) -> bool {
        self.selected
    }

    pub fn emitted(&self) -> bool {
        self.emitted
    }

    pub fn can_activate(&self) -> bool {
        self.can_activate
    }

    pub fn cursor(&self) -> WorthUiPrimitiveResolvedCursorPosture {
        self.cursor
    }

    pub fn containment(&self) -> WorthUiPrimitiveEventContainment {
        self.containment
    }
}

impl WorthUiPrimitiveEventDispatchCounters {
    pub(super) fn new(
        region_count: usize,
        hit_candidate_count: usize,
        cursor_candidate_count: usize,
        parent_chain_count: usize,
        emitted_surface_count: usize,
    ) -> Self {
        Self {
            region_count,
            hit_candidate_count,
            cursor_candidate_count,
            parent_chain_count,
            emitted_surface_count,
        }
    }

    pub fn region_count(self) -> usize {
        self.region_count
    }

    pub fn hit_candidate_count(self) -> usize {
        self.hit_candidate_count
    }

    pub fn cursor_candidate_count(self) -> usize {
        self.cursor_candidate_count
    }

    pub fn parent_chain_count(self) -> usize {
        self.parent_chain_count
    }

    pub fn emitted_surface_count(self) -> usize {
        self.emitted_surface_count
    }
}

impl WorthUiPrimitiveEventDispatchReceipt {
    pub(super) fn new(
        primary_surface_id: Option<String>,
        emitted_surface_ids: Vec<String>,
        cursor: WorthUiPrimitiveResolvedCursorPosture,
        containment: Option<WorthUiPrimitiveEventContainment>,
        outcome: WorthUiPrimitiveEventDispatchOutcome,
        candidates: Vec<WorthUiPrimitiveEventDispatchCandidateReceipt>,
        counters: WorthUiPrimitiveEventDispatchCounters,
    ) -> Self {
        let dispatch_digest =
            event_dispatch_digest(primary_surface_id.as_deref(), &emitted_surface_ids, cursor);
        Self {
            primary_surface_id,
            emitted_surface_ids,
            cursor,
            containment,
            outcome,
            candidates,
            counters,
            dispatch_digest,
        }
    }

    pub(super) fn empty(
        region_count: usize,
        candidates: Vec<WorthUiPrimitiveEventDispatchCandidateReceipt>,
    ) -> Self {
        Self::new(
            None,
            Vec::new(),
            WorthUiPrimitiveResolvedCursorPosture::Default,
            None,
            WorthUiPrimitiveEventDispatchOutcome::NoHit,
            candidates,
            WorthUiPrimitiveEventDispatchCounters::new(region_count, 0, 0, 0, 0),
        )
    }

    pub fn primary_surface_id(&self) -> Option<&str> {
        self.primary_surface_id.as_deref()
    }

    pub fn emitted_surface_ids(&self) -> &[String] {
        &self.emitted_surface_ids
    }

    pub fn cursor(&self) -> WorthUiPrimitiveResolvedCursorPosture {
        self.cursor
    }

    pub fn containment(&self) -> Option<WorthUiPrimitiveEventContainment> {
        self.containment
    }

    pub fn outcome(&self) -> WorthUiPrimitiveEventDispatchOutcome {
        self.outcome
    }

    pub fn candidates(&self) -> &[WorthUiPrimitiveEventDispatchCandidateReceipt] {
        &self.candidates
    }

    pub fn counters(&self) -> WorthUiPrimitiveEventDispatchCounters {
        self.counters
    }

    pub fn dispatch_digest(&self) -> u64 {
        self.dispatch_digest
    }
}
