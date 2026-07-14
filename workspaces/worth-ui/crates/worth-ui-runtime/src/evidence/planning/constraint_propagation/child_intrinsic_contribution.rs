use crate::declaration::stable_text_digest;
use crate::graph::UiGraphNodeIdentity;

use crate::evidence::{
    UiConstraintAxisScope, UiMeasurementCoordinateSpace, UiMeasurementRoundingPosture,
    UiMeasurementUnitPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiConstraintIntrinsicSourcePosture {
    QueryOnly,
    HostOnly,
    QueryAndHost,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiConstraintHostIntrinsicKind {
    None,
    Text,
    NativeControl,
    Mixed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiConstraintChildIntrinsicContribution {
    contributor_graph_node_identity: UiGraphNodeIdentity,
    axis_scope: UiConstraintAxisScope,
    primary_extent_bits: u32,
    cross_extent_bits: Option<u32>,
    source_posture: UiConstraintIntrinsicSourcePosture,
    host_kind: UiConstraintHostIntrinsicKind,
    unit_posture: UiMeasurementUnitPosture,
    coordinate_space: UiMeasurementCoordinateSpace,
    rounding_posture: UiMeasurementRoundingPosture,
    identity_digest: u64,
}

impl UiConstraintChildIntrinsicContribution {
    pub fn new(
        contributor_graph_node_identity: UiGraphNodeIdentity,
        axis_scope: UiConstraintAxisScope,
        primary_extent_bits: u32,
        cross_extent_bits: Option<u32>,
        source_posture: UiConstraintIntrinsicSourcePosture,
        host_kind: UiConstraintHostIntrinsicKind,
        unit_posture: UiMeasurementUnitPosture,
        coordinate_space: UiMeasurementCoordinateSpace,
        rounding_posture: UiMeasurementRoundingPosture,
    ) -> Self {
        let identity_digest = stable_text_digest("worth-ui.constraint-child-intrinsic")
            ^ contributor_graph_node_identity.digest().rotate_left(7)
            ^ axis_scope_digest(axis_scope).rotate_left(13)
            ^ (primary_extent_bits as u64).rotate_left(19)
            ^ cross_extent_bits
                .map(|value| value as u64)
                .unwrap_or_default()
                .rotate_left(23)
            ^ source_digest(source_posture).rotate_left(29)
            ^ host_kind_digest(host_kind).rotate_left(31)
            ^ stable_text_digest(unit_posture.as_str()).rotate_left(37)
            ^ stable_text_digest(coordinate_space.as_str()).rotate_left(41)
            ^ stable_text_digest(rounding_posture.as_str()).rotate_left(47);
        Self {
            contributor_graph_node_identity,
            axis_scope,
            primary_extent_bits,
            cross_extent_bits,
            source_posture,
            host_kind,
            unit_posture,
            coordinate_space,
            rounding_posture,
            identity_digest,
        }
    }

    pub fn contributor_graph_node_identity(&self) -> UiGraphNodeIdentity {
        self.contributor_graph_node_identity
    }

    pub fn axis_scope(&self) -> UiConstraintAxisScope {
        self.axis_scope
    }

    pub fn primary_extent(&self) -> f32 {
        f32::from_bits(self.primary_extent_bits)
    }

    pub fn cross_extent(&self) -> Option<f32> {
        self.cross_extent_bits.map(f32::from_bits)
    }

    pub fn source_posture(&self) -> UiConstraintIntrinsicSourcePosture {
        self.source_posture
    }

    pub fn host_kind(&self) -> UiConstraintHostIntrinsicKind {
        self.host_kind
    }

    pub fn unit_posture(&self) -> UiMeasurementUnitPosture {
        self.unit_posture
    }

    pub fn coordinate_space(&self) -> UiMeasurementCoordinateSpace {
        self.coordinate_space
    }

    pub fn rounding_posture(&self) -> UiMeasurementRoundingPosture {
        self.rounding_posture
    }

    pub fn identity_digest(&self) -> u64 {
        self.identity_digest
    }
}

fn axis_scope_digest(axis_scope: UiConstraintAxisScope) -> u64 {
    match axis_scope {
        UiConstraintAxisScope::Primary => stable_text_digest("worth-ui.constraint-axis.primary"),
        UiConstraintAxisScope::Cross => stable_text_digest("worth-ui.constraint-axis.cross"),
        UiConstraintAxisScope::Both => stable_text_digest("worth-ui.constraint-axis.both"),
    }
}

fn source_digest(source_posture: UiConstraintIntrinsicSourcePosture) -> u64 {
    match source_posture {
        UiConstraintIntrinsicSourcePosture::QueryOnly => {
            stable_text_digest("worth-ui.constraint-child-intrinsic.query-only")
        }
        UiConstraintIntrinsicSourcePosture::HostOnly => {
            stable_text_digest("worth-ui.constraint-child-intrinsic.host-only")
        }
        UiConstraintIntrinsicSourcePosture::QueryAndHost => {
            stable_text_digest("worth-ui.constraint-child-intrinsic.query-and-host")
        }
    }
}

fn host_kind_digest(host_kind: UiConstraintHostIntrinsicKind) -> u64 {
    match host_kind {
        UiConstraintHostIntrinsicKind::None => {
            stable_text_digest("worth-ui.constraint-child-intrinsic.host.none")
        }
        UiConstraintHostIntrinsicKind::Text => {
            stable_text_digest("worth-ui.constraint-child-intrinsic.host.text")
        }
        UiConstraintHostIntrinsicKind::NativeControl => {
            stable_text_digest("worth-ui.constraint-child-intrinsic.host.native-control")
        }
        UiConstraintHostIntrinsicKind::Mixed => {
            stable_text_digest("worth-ui.constraint-child-intrinsic.host.mixed")
        }
    }
}
