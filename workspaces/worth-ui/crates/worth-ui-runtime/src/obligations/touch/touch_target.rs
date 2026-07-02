use crate::declaration::stable_text_digest;
use crate::graph::{UiGraphNodeIdentity, UiMountedReceiptIdentity};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiGraphTouchTargetClass {
    Node,
    SlotOccupancy,
    PageMembership,
    RegionMembership,
    MosaicMembership,
    AttachmentLane,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiGraphTouchAttachmentLane {
    MountedReceiptSlot,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiGraphTouchTarget {
    Node {
        graph_node_identity: UiGraphNodeIdentity,
    },
    SlotOccupancy {
        graph_node_identity: UiGraphNodeIdentity,
        parent_node_identity: UiGraphNodeIdentity,
        slot_name: Box<str>,
    },
    PageMembership {
        graph_node_identity: UiGraphNodeIdentity,
        page_node_identity: UiGraphNodeIdentity,
    },
    RegionMembership {
        graph_node_identity: UiGraphNodeIdentity,
        region_name: Box<str>,
    },
    MosaicMembership {
        graph_node_identity: UiGraphNodeIdentity,
        mosaic_name: Box<str>,
    },
    AttachmentLane {
        graph_node_identity: UiGraphNodeIdentity,
        attachment_lane: UiGraphTouchAttachmentLane,
        mounted_receipt_identity: UiMountedReceiptIdentity,
    },
}

impl UiGraphTouchTarget {
    pub(crate) const fn node(graph_node_identity: UiGraphNodeIdentity) -> Self {
        Self::Node {
            graph_node_identity,
        }
    }

    pub(crate) fn slot_occupancy(
        graph_node_identity: UiGraphNodeIdentity,
        parent_node_identity: UiGraphNodeIdentity,
        slot_name: Box<str>,
    ) -> Self {
        Self::SlotOccupancy {
            graph_node_identity,
            parent_node_identity,
            slot_name,
        }
    }

    pub(crate) const fn page_membership(
        graph_node_identity: UiGraphNodeIdentity,
        page_node_identity: UiGraphNodeIdentity,
    ) -> Self {
        Self::PageMembership {
            graph_node_identity,
            page_node_identity,
        }
    }

    pub(crate) fn region_membership(
        graph_node_identity: UiGraphNodeIdentity,
        region_name: Box<str>,
    ) -> Self {
        Self::RegionMembership {
            graph_node_identity,
            region_name,
        }
    }

    pub(crate) fn mosaic_membership(
        graph_node_identity: UiGraphNodeIdentity,
        mosaic_name: Box<str>,
    ) -> Self {
        Self::MosaicMembership {
            graph_node_identity,
            mosaic_name,
        }
    }

    pub(crate) fn mounted_receipt_slot(
        graph_node_identity: UiGraphNodeIdentity,
        mounted_receipt_identity: UiMountedReceiptIdentity,
    ) -> Self {
        Self::AttachmentLane {
            graph_node_identity,
            attachment_lane: UiGraphTouchAttachmentLane::MountedReceiptSlot,
            mounted_receipt_identity,
        }
    }

    pub fn class(&self) -> UiGraphTouchTargetClass {
        match self {
            Self::Node { .. } => UiGraphTouchTargetClass::Node,
            Self::SlotOccupancy { .. } => UiGraphTouchTargetClass::SlotOccupancy,
            Self::PageMembership { .. } => UiGraphTouchTargetClass::PageMembership,
            Self::RegionMembership { .. } => UiGraphTouchTargetClass::RegionMembership,
            Self::MosaicMembership { .. } => UiGraphTouchTargetClass::MosaicMembership,
            Self::AttachmentLane { .. } => UiGraphTouchTargetClass::AttachmentLane,
        }
    }

    pub fn graph_node_identity(&self) -> UiGraphNodeIdentity {
        match self {
            Self::Node {
                graph_node_identity,
            }
            | Self::SlotOccupancy {
                graph_node_identity,
                ..
            }
            | Self::PageMembership {
                graph_node_identity,
                ..
            }
            | Self::RegionMembership {
                graph_node_identity,
                ..
            }
            | Self::MosaicMembership {
                graph_node_identity,
                ..
            }
            | Self::AttachmentLane {
                graph_node_identity,
                ..
            } => *graph_node_identity,
        }
    }

    pub fn parent_node_identity(&self) -> Option<UiGraphNodeIdentity> {
        match self {
            Self::SlotOccupancy {
                parent_node_identity,
                ..
            } => Some(*parent_node_identity),
            _ => None,
        }
    }

    pub fn slot_name(&self) -> Option<&str> {
        match self {
            Self::SlotOccupancy { slot_name, .. } => Some(slot_name),
            _ => None,
        }
    }

    pub fn page_node_identity(&self) -> Option<UiGraphNodeIdentity> {
        match self {
            Self::PageMembership {
                page_node_identity, ..
            } => Some(*page_node_identity),
            _ => None,
        }
    }

    pub fn region_name(&self) -> Option<&str> {
        match self {
            Self::RegionMembership { region_name, .. } => Some(region_name),
            _ => None,
        }
    }

    pub fn mosaic_name(&self) -> Option<&str> {
        match self {
            Self::MosaicMembership { mosaic_name, .. } => Some(mosaic_name),
            _ => None,
        }
    }

    pub fn attachment_lane(&self) -> Option<UiGraphTouchAttachmentLane> {
        match self {
            Self::AttachmentLane {
                attachment_lane, ..
            } => Some(*attachment_lane),
            _ => None,
        }
    }

    pub fn mounted_receipt_identity(&self) -> Option<UiMountedReceiptIdentity> {
        match self {
            Self::AttachmentLane {
                mounted_receipt_identity,
                ..
            } => Some(*mounted_receipt_identity),
            _ => None,
        }
    }

    pub(crate) fn identity_digest(&self) -> u64 {
        match self {
            Self::Node {
                graph_node_identity,
            } => {
                stable_text_digest("graph-touch-target:node")
                    ^ graph_node_identity.digest().rotate_left(7)
            }
            Self::SlotOccupancy {
                graph_node_identity,
                parent_node_identity,
                slot_name,
            } => {
                stable_text_digest("graph-touch-target:slot-occupancy")
                    ^ graph_node_identity.digest().rotate_left(7)
                    ^ parent_node_identity.digest().rotate_left(13)
                    ^ stable_text_digest(slot_name).rotate_left(19)
            }
            Self::PageMembership {
                graph_node_identity,
                page_node_identity,
            } => {
                stable_text_digest("graph-touch-target:page-membership")
                    ^ graph_node_identity.digest().rotate_left(7)
                    ^ page_node_identity.digest().rotate_left(17)
            }
            Self::RegionMembership {
                graph_node_identity,
                region_name,
            } => {
                stable_text_digest("graph-touch-target:region-membership")
                    ^ graph_node_identity.digest().rotate_left(7)
                    ^ stable_text_digest(region_name).rotate_left(23)
            }
            Self::MosaicMembership {
                graph_node_identity,
                mosaic_name,
            } => {
                stable_text_digest("graph-touch-target:mosaic-membership")
                    ^ graph_node_identity.digest().rotate_left(7)
                    ^ stable_text_digest(mosaic_name).rotate_left(29)
            }
            Self::AttachmentLane {
                graph_node_identity,
                attachment_lane,
                mounted_receipt_identity,
            } => {
                stable_text_digest("graph-touch-target:attachment-lane")
                    ^ graph_node_identity.digest().rotate_left(7)
                    ^ (*attachment_lane as u64).rotate_left(13)
                    ^ mounted_receipt_identity.digest().rotate_left(19)
            }
        }
    }
}
