//! Aspect-aware dependency declarations for feature graph nodes.
//!
//! DOMAIN: Expresses which semantic outputs a feature reads from each upstream
//! feature without leaking signal-graph wiring details into every caller.

use serde::{Deserialize, Serialize};

use forge_signal::facade::{Aspect, AspectMask, NodeId};

/// Kernel-level semantic aspects currently exposed through `forge-signal`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeatureAspect {
    Topology,
    Geometry,
}

impl FeatureAspect {
    /// Convert one semantic feature aspect into its signal bit.
    pub const fn index(self) -> u8 {
        match self {
            Self::Topology => 0,
            Self::Geometry => 1,
        }
    }

    /// Convert this semantic feature aspect into the signal aspect handle.
    pub const fn signal_aspect(self) -> Aspect {
        Aspect::new(self.index())
    }
}

/// One static dependency declaration for a feature node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeatureDependency {
    node_id: NodeId,
    aspects: AspectMask,
}

impl FeatureDependency {
    /// Subscribe to one upstream node with an explicit semantic aspect mask.
    pub const fn new(node_id: NodeId, aspects: AspectMask) -> Self {
        Self { node_id, aspects }
    }

    /// Topology-only dependency.
    pub const fn topology(node_id: NodeId) -> Self {
        Self::new(
            node_id,
            AspectMask::from_aspect(FeatureAspect::Topology.signal_aspect()),
        )
    }

    /// Geometry-only dependency.
    pub const fn geometry(node_id: NodeId) -> Self {
        Self::new(
            node_id,
            AspectMask::from_aspect(FeatureAspect::Geometry.signal_aspect()),
        )
    }

    /// Dependency on both currently-defined kernel aspects.
    pub const fn topology_and_geometry(node_id: NodeId) -> Self {
        Self::new(
            node_id,
            AspectMask::from_aspect(FeatureAspect::Topology.signal_aspect()).union(
                AspectMask::from_aspect(FeatureAspect::Geometry.signal_aspect()),
            ),
        )
    }

    /// Upstream node identifier.
    pub const fn node_id(self) -> NodeId {
        self.node_id
    }

    /// Signal-facing aspect mask.
    pub const fn aspects(self) -> AspectMask {
        self.aspects
    }
}
