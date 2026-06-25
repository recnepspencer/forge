use crate::runtime::{
    WorthUiRuntimeFactId, WorthUiScrollRestorationPolicy, WorthUiViewportBoundaryCounters,
    WorthUiViewportBoundaryPolicyReceipt, WorthUiViewportParticipationPolicy, WorthUiViewportRect,
};

use super::digest::digest_parts;
use super::policy::{WorthUiClipPosture, WorthUiScrollOwner};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiViewportBoundaryReceipt {
    boundaries: Vec<WorthUiResolvedViewportBoundaryReceipt>,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
    counters: WorthUiViewportBoundaryCounters,
    receipt_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiResolvedViewportBoundaryReceipt {
    node_id: String,
    policy: WorthUiViewportBoundaryPolicyReceipt,
    viewport_rect: WorthUiViewportRect,
    scroll_x_points: f32,
    scroll_y_points: f32,
    descendants: Vec<WorthUiViewportDescendantParticipationReceipt>,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
    receipt_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiViewportDescendantParticipationReceipt {
    node_id: String,
    visual_frame: WorthUiViewportRect,
    visible: bool,
    hit_participates: bool,
    focus_participates: bool,
    accessibility_participates: bool,
    measurement_participates: bool,
    receipt_digest: u64,
}

impl WorthUiViewportBoundaryReceipt {
    pub(super) fn new(
        boundaries: Vec<WorthUiResolvedViewportBoundaryReceipt>,
        mut consumed_facts: Vec<WorthUiRuntimeFactId>,
        selected_graph_obligation_count: usize,
    ) -> Self {
        let descendant_count = boundaries
            .iter()
            .map(|boundary| boundary.descendants().len())
            .sum::<usize>();
        let clipped_descendant_count = boundaries
            .iter()
            .flat_map(|boundary| boundary.descendants())
            .filter(|descendant| !descendant.visible())
            .count();
        consumed_facts.extend(
            boundaries
                .iter()
                .flat_map(|boundary| boundary.consumed_facts().iter().cloned()),
        );
        consumed_facts.sort();
        consumed_facts.dedup();
        let counters = WorthUiViewportBoundaryCounters::new(
            boundaries.len(),
            descendant_count,
            clipped_descendant_count,
            selected_graph_obligation_count,
        );
        let receipt_digest = digest_parts(
            ["viewport_boundary_root".to_owned()]
                .into_iter()
                .chain(
                    boundaries
                        .iter()
                        .map(|row| row.receipt_digest().to_string()),
                )
                .chain(consumed_facts.iter().map(|fact| fact.identity().to_owned())),
        );
        Self {
            boundaries,
            consumed_facts,
            counters,
            receipt_digest,
        }
    }

    pub fn boundaries(&self) -> &[WorthUiResolvedViewportBoundaryReceipt] {
        &self.boundaries
    }

    pub fn boundary_for_node(
        &self,
        node_id: &str,
    ) -> Option<&WorthUiResolvedViewportBoundaryReceipt> {
        self.boundaries.iter().find(|row| row.node_id() == node_id)
    }

    pub fn consumed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.consumed_facts
    }

    pub fn counters(&self) -> WorthUiViewportBoundaryCounters {
        self.counters
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiResolvedViewportBoundaryReceipt {
    pub(super) fn new(
        node_id: impl Into<String>,
        policy: WorthUiViewportBoundaryPolicyReceipt,
        viewport_rect: WorthUiViewportRect,
        scroll_x_points: f32,
        scroll_y_points: f32,
        descendants: Vec<WorthUiViewportDescendantParticipationReceipt>,
        mut consumed_facts: Vec<WorthUiRuntimeFactId>,
    ) -> Self {
        let node_id = node_id.into();
        consumed_facts.push(WorthUiRuntimeFactId::viewport_boundary(node_id.clone()));
        if policy.clip_posture() == WorthUiClipPosture::ClipToViewport {
            consumed_facts.push(WorthUiRuntimeFactId::clip_boundary(node_id.clone()));
        }
        if policy.scroll_owner() == WorthUiScrollOwner::Composition
            && policy.restoration_policy() == WorthUiScrollRestorationPolicy::ByCompositionIdentity
        {
            consumed_facts.push(WorthUiRuntimeFactId::scroll_restoration(node_id.clone()));
        }
        consumed_facts.push(WorthUiRuntimeFactId::viewport_event_participation(
            node_id.clone(),
        ));
        consumed_facts.sort();
        consumed_facts.dedup();
        let receipt_digest = digest_parts(
            [
                "viewport_boundary".to_owned(),
                node_id.clone(),
                policy.policy_identity().to_owned(),
                policy.scroll_owner().token().to_owned(),
                policy.clip_posture().token().to_owned(),
                policy.hit_policy().token().to_owned(),
                policy.measurement_policy().token().to_owned(),
                viewport_rect.x().to_string(),
                viewport_rect.y().to_string(),
                viewport_rect.width().to_string(),
                viewport_rect.height().to_string(),
                scroll_x_points.to_string(),
                scroll_y_points.to_string(),
            ]
            .into_iter()
            .chain(
                descendants
                    .iter()
                    .map(|row| row.receipt_digest().to_string()),
            )
            .chain(consumed_facts.iter().map(|fact| fact.identity().to_owned())),
        );
        Self {
            node_id,
            policy,
            viewport_rect,
            scroll_x_points,
            scroll_y_points,
            descendants,
            consumed_facts,
            receipt_digest,
        }
    }

    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    pub fn policy(&self) -> &WorthUiViewportBoundaryPolicyReceipt {
        &self.policy
    }

    pub fn viewport_rect(&self) -> WorthUiViewportRect {
        self.viewport_rect
    }

    pub fn scroll_x_points(&self) -> f32 {
        self.scroll_x_points
    }

    pub fn scroll_y_points(&self) -> f32 {
        self.scroll_y_points
    }

    pub fn descendants(&self) -> &[WorthUiViewportDescendantParticipationReceipt] {
        &self.descendants
    }

    pub fn consumed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.consumed_facts
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiViewportDescendantParticipationReceipt {
    pub(super) fn new(
        node_id: impl Into<String>,
        visual_frame: WorthUiViewportRect,
        visible: bool,
        hit_policy: WorthUiViewportParticipationPolicy,
        focus_policy: WorthUiViewportParticipationPolicy,
        accessibility_policy: WorthUiViewportParticipationPolicy,
        measurement_policy: WorthUiViewportParticipationPolicy,
    ) -> Self {
        let node_id = node_id.into();
        let hit_participates = participates(hit_policy, visible);
        let focus_participates = participates(focus_policy, visible);
        let accessibility_participates = participates(accessibility_policy, visible);
        let measurement_participates = participates(measurement_policy, visible);
        let receipt_digest = digest_parts([
            "viewport_descendant",
            node_id.as_str(),
            &visual_frame.x().to_string(),
            &visual_frame.y().to_string(),
            &visual_frame.width().to_string(),
            &visual_frame.height().to_string(),
            if visible { "visible" } else { "clipped" },
            if hit_participates { "hit" } else { "no_hit" },
            if focus_participates {
                "focus"
            } else {
                "no_focus"
            },
            if accessibility_participates {
                "a11y"
            } else {
                "no_a11y"
            },
            if measurement_participates {
                "measurement"
            } else {
                "no_measurement"
            },
        ]);
        Self {
            node_id,
            visual_frame,
            visible,
            hit_participates,
            focus_participates,
            accessibility_participates,
            measurement_participates,
            receipt_digest,
        }
    }

    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    pub fn visual_frame(&self) -> WorthUiViewportRect {
        self.visual_frame
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn hit_participates(&self) -> bool {
        self.hit_participates
    }

    pub fn focus_participates(&self) -> bool {
        self.focus_participates
    }

    pub fn accessibility_participates(&self) -> bool {
        self.accessibility_participates
    }

    pub fn measurement_participates(&self) -> bool {
        self.measurement_participates
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

fn participates(policy: WorthUiViewportParticipationPolicy, visible: bool) -> bool {
    match policy {
        WorthUiViewportParticipationPolicy::AllDescendants => true,
        WorthUiViewportParticipationPolicy::VisibleDescendantsOnly => visible,
    }
}
