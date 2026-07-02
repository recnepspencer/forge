use std::collections::BTreeMap;

use crate::graph::UiGraphNodeIdentity;

const EMPTY_NODE_SET: [UiGraphNodeIdentity; 0] = [];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphRegionMembershipIndex {
    members_by_region: BTreeMap<Box<str>, Vec<UiGraphNodeIdentity>>,
}

impl UiGraphRegionMembershipIndex {
    pub(crate) fn new(members_by_region: BTreeMap<Box<str>, Vec<UiGraphNodeIdentity>>) -> Self {
        Self { members_by_region }
    }

    pub fn region_members(&self, region_name: &str) -> &[UiGraphNodeIdentity] {
        self.members_by_region
            .get(region_name)
            .map(Vec::as_slice)
            .unwrap_or(&EMPTY_NODE_SET)
    }
}
