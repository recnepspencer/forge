use std::collections::BTreeMap;

use crate::graph::UiGraphNodeIdentity;

const EMPTY_NODE_SET: [UiGraphNodeIdentity; 0] = [];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphPageMembershipIndex {
    members_by_page: BTreeMap<UiGraphNodeIdentity, Vec<UiGraphNodeIdentity>>,
}

impl UiGraphPageMembershipIndex {
    pub(crate) fn new(
        members_by_page: BTreeMap<UiGraphNodeIdentity, Vec<UiGraphNodeIdentity>>,
    ) -> Self {
        Self { members_by_page }
    }

    pub fn page_members(&self, page_node_identity: UiGraphNodeIdentity) -> &[UiGraphNodeIdentity] {
        self.members_by_page
            .get(&page_node_identity)
            .map(Vec::as_slice)
            .unwrap_or(&EMPTY_NODE_SET)
    }
}
