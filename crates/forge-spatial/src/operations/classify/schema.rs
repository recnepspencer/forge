//! Classification result types and the SpatialAccelerator trait.
//!
//! DOMAIN: Data shapes for point-in-solid and point-on-face results.

use worth_geom::{Aabb, BvhNode};
use forge_topo::handles::FaceId;

/// Result of classifying a point relative to a solid's boundary.
#[derive(Debug, Clone, PartialEq)]
pub enum PointClassification {
    /// Point is strictly inside the solid.
    Inside {
        escalation: Option<worth_math::arithmetic::precision::PrecisionEscalation>,
    },
    /// Point is strictly outside the solid.
    Outside {
        escalation: Option<worth_math::arithmetic::precision::PrecisionEscalation>,
    },
    /// Point lies exactly on a boundary face.
    OnBoundary(FaceId),
}

/// Result of classifying whether a face normal points outward.
///
/// Determined by probing both sides of the face with `classify_point_in_solid`.
/// No centroid-of-solid heuristic — works for any solid topology.
#[derive(Debug, Clone, PartialEq)]
pub enum NormalClassification {
    /// `p + εn` is outside AND `p - εn` is inside.
    OutwardConfirmed,
    /// `p + εn` is inside AND `p - εn` is outside (normal is inverted).
    InwardDetected,
    /// Classification was ambiguous (boundary hit, missing data, etc.)
    Degenerate { reason: &'static str },
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
