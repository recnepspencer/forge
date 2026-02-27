//! Classification result types and the SpatialAccelerator trait.
//!
//! DOMAIN: Data shapes for point-in-solid and point-on-face results.

use forge_topo::handles::FaceId;
use forge_geom::{Aabb, BvhNode};

/// Result of classifying a point relative to a solid's boundary.
#[derive(Debug, Clone, PartialEq)]
pub enum PointClassification {
    /// Point is strictly inside the solid.
    Inside {
        escalation: Option<forge_math::arithmetic::precision::PrecisionEscalation>,
    },
    /// Point is strictly outside the solid.
    Outside {
        escalation: Option<forge_math::arithmetic::precision::PrecisionEscalation>,
    },
    /// Point lies exactly on a boundary face.
    OnBoundary(FaceId),
}

/// Trait for spatial acceleration structures.
pub trait SpatialAccelerator {
    fn candidates(&self, aabb: &Aabb) -> Vec<FaceId>;
}

impl SpatialAccelerator for BvhNode<FaceId> {
    fn candidates(&self, aabb: &Aabb) -> Vec<FaceId> {
        self.query_aabb(aabb)
    }
}
