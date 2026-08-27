#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiMountedFocusParticipant {
    graph_node: crate::graph::UiGraphNodeIdentity,
    semantic_surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
    mounted_instance: worth_ui_host_contract::UiMountedInstanceIdentity,
    incarnation: worth_ui_host_contract::UiMountIncarnation,
    node_receipt: worth_ui_host_contract::UiMountedNodeReceiptIdentity,
    support: crate::capability::ComponentFocusSupport,
    scope: super::projection::UiMountedFocusScope,
    mounted_order: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiMountedFocusParticipationSnapshot {
    frame: worth_ui_host_contract::UiMountedFrameIdentity,
    participants: Box<[UiMountedFocusParticipant]>,
    nodes_visited: u32,
}

impl UiMountedFocusParticipant {
    pub(crate) const fn new(
        graph_node: crate::graph::UiGraphNodeIdentity,
        semantic_surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
        mounted_instance: worth_ui_host_contract::UiMountedInstanceIdentity,
        incarnation: worth_ui_host_contract::UiMountIncarnation,
        node_receipt: worth_ui_host_contract::UiMountedNodeReceiptIdentity,
        support: crate::capability::ComponentFocusSupport,
        scope: super::projection::UiMountedFocusScope,
        mounted_order: u32,
    ) -> Self {
        Self {
            graph_node,
            semantic_surface,
            mounted_instance,
            incarnation,
            node_receipt,
            support,
            scope,
            mounted_order,
        }
    }

    pub(crate) const fn graph_node(self) -> crate::graph::UiGraphNodeIdentity {
        self.graph_node
    }
    pub(crate) const fn semantic_surface(
        self,
    ) -> worth_ui_host_contract::UiSemanticSurfaceIdentity {
        self.semantic_surface
    }
    pub(crate) const fn mounted_instance(
        self,
    ) -> worth_ui_host_contract::UiMountedInstanceIdentity {
        self.mounted_instance
    }
    pub(crate) const fn incarnation(self) -> worth_ui_host_contract::UiMountIncarnation {
        self.incarnation
    }
    pub(crate) const fn node_receipt(self) -> worth_ui_host_contract::UiMountedNodeReceiptIdentity {
        self.node_receipt
    }
    pub(crate) const fn support(self) -> crate::capability::ComponentFocusSupport {
        self.support
    }
    pub(crate) const fn scope(self) -> super::projection::UiMountedFocusScope {
        self.scope
    }
    pub(crate) const fn mounted_order(self) -> u32 {
        self.mounted_order
    }
}

impl UiMountedFocusParticipationSnapshot {
    pub(in crate::mounting) fn from_projection(
        projection: &super::UiMountedProjectionFrame,
        receipts: &super::UiMountedNodeReceiptBasis,
    ) -> Self {
        let mut nodes_visited = 0_u32;
        let participants = projection
            .semantic_projection()
            .nodes_in_mounted_order()
            .enumerate()
            .filter_map(|(order, node)| {
                nodes_visited = nodes_visited.checked_add(1)?;
                if node.focus_support == crate::capability::ComponentFocusSupport::NotFocusable {
                    return None;
                }
                if !projection.participates_in_focus(node.receipt().mounted_instance()) {
                    return None;
                }
                let scope = node.focus_scope?;
                let mounted_instance = node.receipt().mounted_instance();
                Some(UiMountedFocusParticipant::new(
                    node.receipt().graph_node(),
                    node.receipt().semantic_surface(),
                    mounted_instance,
                    node.receipt().incarnation(),
                    receipts.receipt_for(mounted_instance)?,
                    node.focus_support,
                    scope,
                    u32::try_from(order).ok()?,
                ))
            })
            .collect::<Vec<_>>();
        Self::new(receipts.frame(), participants, nodes_visited)
    }

    pub(crate) fn new(
        frame: worth_ui_host_contract::UiMountedFrameIdentity,
        participants: Vec<UiMountedFocusParticipant>,
        nodes_visited: u32,
    ) -> Self {
        Self {
            frame,
            participants: participants.into_boxed_slice(),
            nodes_visited,
        }
    }

    pub(crate) const fn frame(&self) -> worth_ui_host_contract::UiMountedFrameIdentity {
        self.frame
    }
    pub(crate) fn participants(&self) -> &[UiMountedFocusParticipant] {
        &self.participants
    }
    pub(crate) const fn nodes_visited(&self) -> u32 {
        self.nodes_visited
    }
}
