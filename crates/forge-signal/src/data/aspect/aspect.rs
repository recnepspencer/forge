use serde::{Deserialize, Serialize};

/// Which aspect of a feature output a downstream node subscribes to.
///
/// This enables the topology change firewall: a geometry-only change
/// (e.g., dragging an extrude depth) won't trigger re-evaluation of
/// nodes that only subscribe to the topology aspect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Aspect {
    /// Subscribes to topology changes (connectivity, face count, etc.).
    Topology,
    /// Subscribes to geometry changes (positions, dimensions, etc.).
    Geometry,
}
