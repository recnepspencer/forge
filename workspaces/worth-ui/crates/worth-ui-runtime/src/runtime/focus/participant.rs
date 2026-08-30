#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime) struct UiFocusParticipant {
    identity: super::UiFocusParticipantIdentity,
    scope: super::UiFocusScopeIdentity,
    graph_node: crate::graph::UiGraphNodeIdentity,
    incarnation: worth_ui_host_contract::UiMountIncarnation,
    node_receipt: worth_ui_host_contract::UiMountedNodeReceiptIdentity,
    mounted_order: u32,
    container: Option<super::UiFocusParticipantIdentity>,
    container_policy: Option<crate::capability::ComponentFocusContainerPolicy>,
}

impl UiFocusParticipant {
    pub(super) fn from_mounted(
        mounted: crate::mounting::UiMountedFocusParticipant,
    ) -> Option<Self> {
        if mounted.support() == crate::capability::ComponentFocusSupport::NotFocusable {
            return None;
        }
        Some(Self {
            identity: super::UiFocusParticipantIdentity::for_mounted_instance(
                mounted.mounted_instance(),
            ),
            scope: super::UiFocusScopeIdentity::from_mounted(
                mounted.semantic_surface(),
                mounted.scope(),
            ),
            graph_node: mounted.graph_node(),
            incarnation: mounted.incarnation(),
            node_receipt: mounted.node_receipt(),
            mounted_order: mounted.mounted_order(),
            container: mounted
                .container()
                .map(super::UiFocusParticipantIdentity::for_mounted_instance),
            container_policy: mounted.support().container_policy(),
        })
    }

    pub(in crate::runtime) const fn identity(self) -> super::UiFocusParticipantIdentity {
        self.identity
    }

    pub(in crate::runtime) const fn scope(self) -> super::UiFocusScopeIdentity {
        self.scope
    }

    pub(in crate::runtime) const fn graph_node(self) -> crate::graph::UiGraphNodeIdentity {
        self.graph_node
    }

    pub(in crate::runtime) const fn incarnation(
        self,
    ) -> worth_ui_host_contract::UiMountIncarnation {
        self.incarnation
    }

    pub(in crate::runtime) const fn node_receipt(
        self,
    ) -> worth_ui_host_contract::UiMountedNodeReceiptIdentity {
        self.node_receipt
    }

    pub(in crate::runtime) const fn mounted_order(self) -> u32 {
        self.mounted_order
    }

    pub(super) const fn container(self) -> Option<super::UiFocusParticipantIdentity> {
        self.container
    }

    pub(super) const fn container_policy(
        self,
    ) -> Option<crate::capability::ComponentFocusContainerPolicy> {
        self.container_policy
    }
}
