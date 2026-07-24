use crate::declaration::UiDeclarationIdentity;
use crate::graph::UiGraphNodeIdentity;
use crate::obligations::touch::{
    UiGraphTouchAspectPosture, UiGraphTouchOriginClass, UiGraphTouchRuntimeLane,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiGraphTouchDenial {
    MissingAspectPosture,
    DeclarationChangeOutsideGraphAuthority {
        declaration_identity: UiDeclarationIdentity,
    },
    QueryFactChangeUnavailableInCurrentWorld,
    QueryBindingChangeUnavailableInCurrentWorld,
    OriginAuthorityUnavailable {
        origin_class: UiGraphTouchOriginClass,
    },
    OriginOwnerMismatch {
        origin_class: UiGraphTouchOriginClass,
    },
    UnknownGraphNode {
        graph_node_identity: UiGraphNodeIdentity,
    },
    ForeignMountEligibilityTransition {
        graph_node_identity: UiGraphNodeIdentity,
    },
    OriginDoesNotAuthorizeGraphNode {
        origin_class: UiGraphTouchOriginClass,
        graph_node_identity: UiGraphNodeIdentity,
    },
    SlotOccupancyUnavailable {
        graph_node_identity: UiGraphNodeIdentity,
    },
    PageMembershipUnavailable {
        graph_node_identity: UiGraphNodeIdentity,
    },
    RegionMembershipUnavailable {
        graph_node_identity: UiGraphNodeIdentity,
    },
    MosaicMembershipUnavailable {
        graph_node_identity: UiGraphNodeIdentity,
    },
    ContradictoryAspectPosture {
        lane: UiGraphTouchRuntimeLane,
        first: UiGraphTouchAspectPosture,
        second: UiGraphTouchAspectPosture,
    },
}
