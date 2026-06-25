use crate::runtime::{
    WorthUiAdmittedHostFrameObservationReceipt, WorthUiClipPosture, WorthUiCompositionPolicyKind,
    WorthUiLayoutAllocationReceipt, WorthUiMeasuredProductViewReceipt,
    WorthUiResolvedViewportBoundaryReceipt, WorthUiRuntimeFactId, WorthUiRuntimeHost,
    WorthUiScrollOwner, WorthUiViewportBoundaryDenial, WorthUiViewportBoundaryDenialReason,
    WorthUiViewportBoundaryPolicyReceipt, WorthUiViewportBoundaryReceipt,
    WorthUiViewportDescendantParticipationReceipt, WorthUiViewportRect,
};

impl WorthUiRuntimeHost {
    pub fn resolve_viewport_boundaries(
        &self,
        measured: &WorthUiMeasuredProductViewReceipt,
        allocation: &WorthUiLayoutAllocationReceipt,
    ) -> Result<WorthUiViewportBoundaryReceipt, Vec<WorthUiViewportBoundaryDenial>> {
        WorthUiViewportBoundaryPlan::new(measured, allocation).resolve()
    }
}

struct WorthUiViewportBoundaryPlan<'a> {
    measured: &'a WorthUiMeasuredProductViewReceipt,
    allocation: &'a WorthUiLayoutAllocationReceipt,
}

impl<'a> WorthUiViewportBoundaryPlan<'a> {
    fn new(
        measured: &'a WorthUiMeasuredProductViewReceipt,
        allocation: &'a WorthUiLayoutAllocationReceipt,
    ) -> Self {
        Self {
            measured,
            allocation,
        }
    }

    fn resolve(self) -> Result<WorthUiViewportBoundaryReceipt, Vec<WorthUiViewportBoundaryDenial>> {
        let policies = self.viewport_policies();
        let mut denials = self.policy_denials(&policies);
        denials.extend(self.nested_scroll_denials(&policies));
        if !denials.is_empty() {
            denials.sort_by(|left, right| {
                left.reason()
                    .token()
                    .cmp(right.reason().token())
                    .then_with(|| left.subject().cmp(right.subject()))
            });
            return Err(denials);
        }

        let mut boundaries = Vec::new();
        for selected_policy in policies
            .into_iter()
            .filter_map(WorthUiSelectedViewportBoundaryPolicy::admitted)
        {
            let node_id = selected_policy.node_id;
            let policy = selected_policy
                .policy
                .expect("admitted viewport policy selection carries policy receipt");
            let frame = self.node_frame(&node_id).ok_or_else(|| {
                vec![WorthUiViewportBoundaryDenial::new(
                    WorthUiViewportBoundaryDenialReason::MissingAllocatedFrame,
                    node_id.clone(),
                )]
            })?;
            let (scroll_x, scroll_y, viewport) = self
                .viewport_for_node(&node_id, frame, &policy)
                .ok_or_else(|| {
                vec![WorthUiViewportBoundaryDenial::new(
                    WorthUiViewportBoundaryDenialReason::MissingHostViewportObservation,
                    node_id.clone(),
                )]
            })?;
            let descendants = self.descendant_participation(&node_id, viewport, &policy);
            boundaries.push(WorthUiResolvedViewportBoundaryReceipt::new(
                node_id,
                policy,
                viewport,
                scroll_x,
                scroll_y,
                descendants,
                self.boundary_consumed_facts(selected_policy.policy_fact),
            ));
        }

        Ok(WorthUiViewportBoundaryReceipt::new(
            boundaries,
            self.shared_consumed_facts(),
            self.measured
                .mounted_product_view()
                .composition_tree()
                .graph_access()
                .plan()
                .query_graph_execution()
                .selected_obligation_count(),
        ))
    }

    fn viewport_policies(&self) -> Vec<WorthUiSelectedViewportBoundaryPolicy> {
        self.measured
            .mounted_product_view()
            .composition_tree()
            .policy_attachments()
            .iter()
            .filter(|policy| policy.policy_kind() == WorthUiCompositionPolicyKind::ViewportBoundary)
            .map(|policy| WorthUiSelectedViewportBoundaryPolicy {
                node_id: policy.node_id().as_str().to_owned(),
                policy_identity: policy.policy_identity().to_owned(),
                policy_fact: policy.fact_id().clone(),
                policy: WorthUiViewportBoundaryPolicyReceipt::admit(policy.policy_identity()),
            })
            .collect()
    }

    fn policy_denials(
        &self,
        policies: &[WorthUiSelectedViewportBoundaryPolicy],
    ) -> Vec<WorthUiViewportBoundaryDenial> {
        policies
            .iter()
            .filter(|selected_policy| selected_policy.policy.is_none())
            .map(|selected_policy| {
                WorthUiViewportBoundaryDenial::new(
                    WorthUiViewportBoundaryDenialReason::UnsupportedPolicyIdentity,
                    format!(
                        "{}:{}",
                        selected_policy.node_id, selected_policy.policy_identity
                    ),
                )
            })
            .collect()
    }

    fn nested_scroll_denials(
        &self,
        policies: &[WorthUiSelectedViewportBoundaryPolicy],
    ) -> Vec<WorthUiViewportBoundaryDenial> {
        let scroll_nodes = policies
            .iter()
            .filter_map(|selected_policy| {
                selected_policy
                    .policy
                    .as_ref()
                    .filter(|policy| policy.scroll_owner() == WorthUiScrollOwner::Composition)
                    .map(|_| selected_policy.node_id.as_str())
            })
            .collect::<Vec<_>>();
        let graph_access = self
            .measured
            .mounted_product_view()
            .composition_tree()
            .graph_access();
        scroll_nodes
            .iter()
            .filter(|node_id| {
                graph_access
                    .ancestors_of(node_id)
                    .iter()
                    .any(|ancestor| scroll_nodes.contains(&ancestor.ancestor_id()))
            })
            .map(|node_id| {
                WorthUiViewportBoundaryDenial::new(
                    WorthUiViewportBoundaryDenialReason::NestedCompositionScrollOwner,
                    *node_id,
                )
            })
            .collect()
    }

    fn viewport_for_node(
        &self,
        node_id: &str,
        frame: WorthUiViewportRect,
        policy: &WorthUiViewportBoundaryPolicyReceipt,
    ) -> Option<(f32, f32, WorthUiViewportRect)> {
        match policy.scroll_owner() {
            WorthUiScrollOwner::Composition => self.scroll_viewport(node_id),
            WorthUiScrollOwner::None => Some((0.0, 0.0, frame)),
        }
    }

    fn scroll_viewport(&self, node_id: &str) -> Option<(f32, f32, WorthUiViewportRect)> {
        self.observations()
            .scroll_viewports()
            .iter()
            .find(|row| row.node_id() == node_id)
            .map(|row| {
                (
                    row.scroll_x_points(),
                    row.scroll_y_points(),
                    WorthUiViewportRect::new(
                        row.scroll_x_points(),
                        row.scroll_y_points(),
                        row.width_points(),
                        row.height_points(),
                    ),
                )
            })
    }

    fn node_frame(&self, node_id: &str) -> Option<WorthUiViewportRect> {
        self.allocation
            .child_frame(node_id)
            .map(|frame| {
                WorthUiViewportRect::new(frame.x(), frame.y(), frame.width(), frame.height())
            })
            .or_else(|| {
                self.observations()
                    .available_bounds()
                    .iter()
                    .find(|row| row.node_id() == node_id)
                    .map(|row| {
                        WorthUiViewportRect::new(0.0, 0.0, row.width_points(), row.height_points())
                    })
            })
    }

    fn descendant_participation(
        &self,
        node_id: &str,
        viewport: WorthUiViewportRect,
        policy: &WorthUiViewportBoundaryPolicyReceipt,
    ) -> Vec<WorthUiViewportDescendantParticipationReceipt> {
        let graph_access = self
            .measured
            .mounted_product_view()
            .composition_tree()
            .graph_access();
        self.allocation
            .children()
            .iter()
            .filter(|child| {
                child.child_node_id() != node_id
                    && graph_access
                        .ancestors_of(child.child_node_id())
                        .iter()
                        .any(|ancestor| ancestor.ancestor_id() == node_id)
            })
            .map(|child| {
                let frame = child.frame();
                let rect =
                    WorthUiViewportRect::new(frame.x(), frame.y(), frame.width(), frame.height());
                let visible = policy.clip_posture() != WorthUiClipPosture::ClipToViewport
                    || viewport.intersects(rect);
                WorthUiViewportDescendantParticipationReceipt::new(
                    child.child_node_id(),
                    rect,
                    visible,
                    policy.hit_policy(),
                    policy.focus_policy(),
                    policy.accessibility_policy(),
                    policy.measurement_policy(),
                )
            })
            .collect()
    }

    fn observations(&self) -> &WorthUiAdmittedHostFrameObservationReceipt {
        self.measured.host_observations()
    }

    fn boundary_consumed_facts(
        &self,
        policy_fact: WorthUiRuntimeFactId,
    ) -> Vec<WorthUiRuntimeFactId> {
        let mut facts = vec![policy_fact];
        facts.extend(self.shared_consumed_facts());
        facts.sort();
        facts.dedup();
        facts
    }

    fn shared_consumed_facts(&self) -> Vec<WorthUiRuntimeFactId> {
        let mut facts = Vec::new();
        facts.extend(
            self.measured
                .mounted_product_view()
                .root_entries()
                .iter()
                .flat_map(|entry| entry.root_mount().consumed_facts().iter().cloned()),
        );
        facts.extend(self.allocation.consumed_facts().iter().cloned());
        facts.extend(self.observations().consumed_facts().iter().cloned());
        facts.sort();
        facts.dedup();
        facts
    }
}

struct WorthUiSelectedViewportBoundaryPolicy {
    node_id: String,
    policy_identity: String,
    policy_fact: WorthUiRuntimeFactId,
    policy: Option<WorthUiViewportBoundaryPolicyReceipt>,
}

impl WorthUiSelectedViewportBoundaryPolicy {
    fn admitted(self) -> Option<Self> {
        self.policy.as_ref()?;
        Some(self)
    }
}
