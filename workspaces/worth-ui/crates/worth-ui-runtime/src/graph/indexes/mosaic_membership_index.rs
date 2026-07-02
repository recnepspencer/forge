use std::collections::BTreeMap;

use crate::graph::UiGraphNodeIdentity;

const EMPTY_NODE_SET: [UiGraphNodeIdentity; 0] = [];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphMosaicMembershipIndex {
    members_by_mosaic: BTreeMap<Box<str>, Vec<UiGraphNodeIdentity>>,
}

impl UiGraphMosaicMembershipIndex {
    pub(crate) fn new(members_by_mosaic: BTreeMap<Box<str>, Vec<UiGraphNodeIdentity>>) -> Self {
        Self { members_by_mosaic }
    }

    pub fn mosaic_members(&self, mosaic_name: &str) -> &[UiGraphNodeIdentity] {
        self.members_by_mosaic
            .get(mosaic_name)
            .map(Vec::as_slice)
            .unwrap_or(&EMPTY_NODE_SET)
    }
}
