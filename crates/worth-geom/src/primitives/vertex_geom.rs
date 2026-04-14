//! `VertexGeom` — geometric data for a topology vertex.
//!
//! DOMAIN: Holds the best-known 3D position for a vertex and its certified
//! error bound (tolerance sphere). This is the geometry-layer mirror of
//! the topology-layer `VertexData`, which contains only connectivity.
//!
//! DEPENDENCIES: `worth-math` precision certificate types (future — currently
//! the provenance records which operation created this vertex).

use serde::{Deserialize, Serialize};

/// How a vertex's position and tolerance were established.
///
/// Used for audit trails, counterfactual replay, and debugging near-boundary
/// decisions. Phase 4+ will add `SsiSolver { residual_error: f64, ... }`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VertexProvenance {
    /// Position was computed as the exact intersection of three planes.
    ///
    /// Tolerance is effectively zero for topology decisions (exact arithmetic
    /// used). `residual` will carry the SSI solver residual once the SSI
    /// milestone lands; until then it is `None` (pre-SSI conservative fallback).
    ThreePlaneIntersection {
        /// Indices into the kernel's plane list.
        plane_indices: [usize; 3],
        /// Residual from the SSI solver, if available.
        ///
        /// `None` = pre-SSI estimate — use `global_default()` as a certified-
        /// conservative tolerance for this vertex (D3 compliant: no silent
        /// heuristic, explicitly documented interim fallback).
        #[serde(default)]
        residual: Option<f64>,
    },

    /// Position was split from an existing edge at a parametric location `t ∈ [0,1]`.
    /// Tolerance is inherited from the parent edge's tube radius.
    EdgeSplit {
        /// Raw index of the parent edge (topology EdgeId.index()).
        parent_edge_index: u32,
        /// Parameter along the parent edge (0.0 = start vertex, 1.0 = end vertex).
        parameter: f64,
    },

    /// Position was imported from an external source (STEP, IGES, OBJ, f64 literal).
    /// Tolerance reflects the import gap tolerance used during healing.
    Imported {
        /// The healing tolerance used to close gaps around this vertex.
        healing_tolerance: f64,
    },

    /// Position was produced as a coalescence of two or more near-coincident vertices.
    /// Tolerance is the sphere that encompasses all original positions.
    Coalesced {
        /// Number of source vertices merged into this one.
        source_count: u32,
        /// Maximum distance between any two source vertices.
        max_gap: f64,
    },
}
/// Geometric properties attached to a topological vertex.
///
/// Contains the spatial position and its certified uncertainty bound.
/// This acts as the geometry layer's representation of a vertex,
/// completely isolated from topological pointers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VertexGeom {
    /// Spatial position (X, Y, Z).
    pub position: [f64; 3],

    /// For planar vertices computed via exact arithmetic this is a tiny
    /// constant (`ToleranceConfig::planar_vertex_tolerance`, typically 1e-10).
    /// For curved vertices this is `max(parent_tolerances) + solver_residual`.
    ///
    /// INVARIANT: always strictly positive. The `ToleranceProvider` implementation
    /// must never return `0.0` — use `global_default()` for unbound vertices.
    pub tolerance: f64,

    /// How this vertex's position and tolerance were derived.
    pub provenance: VertexProvenance,

    /// The regime controlling how this vertex treats threshold comparisons.
    #[serde(default)]
    pub regime: ToleranceRegime,
}

/// The precision regime under which this geometry was constructed.
///
/// Determines how the kernel interprets this vertex's tolerance during
/// boolean boundary checks and classification logic.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ToleranceRegime {
    /// Vertex geometry is exact; tolerance is just machine-epsilon or algorithmic residual.
    Exact,
    /// Vertex geometry was explicitly modeled to a specific precision.
    Modeled,
    /// Vertex geometry was patched over a gap during import/healing.
    Healed { healing_tolerance_mm: f64 },
    /// Vertex geometry came from a polyhedral or lossy mesh import.
    Imported,
}

impl Default for ToleranceRegime {
    fn default() -> Self {
        Self::Exact
    }
}

impl VertexGeom {
    /// Create a planar vertex from its exact f64 position and the plane indices
    /// that define it. Tolerance is the given `planar_tolerance` (typically 1e-7 × scale).
    pub fn from_plane_intersection(
        position: [f64; 3],
        plane_indices: [usize; 3],
        planar_tolerance: f64,
    ) -> Self {
        debug_assert!(planar_tolerance > 0.0, "planar_tolerance must be > 0.0");
        Self {
            position,
            tolerance: planar_tolerance,
            provenance: VertexProvenance::ThreePlaneIntersection {
                plane_indices,
                residual: None,
            },
            regime: ToleranceRegime::Exact,
        }
    }

    /// Create a vertex from an edge split at parameter `t`.
    ///
    /// Tolerance is the conservative maximum of both endpoint tolerances,
    /// because the new vertex position error is bounded by whichever endpoint
    /// is least certain.
    pub fn from_edge_split(
        position: [f64; 3],
        parent_edge_index: u32,
        parameter: f64,
        inherited_tolerance: f64,
    ) -> Self {
        debug_assert!(
            inherited_tolerance > 0.0,
            "inherited_tolerance must be > 0.0"
        );
        Self {
            position,
            tolerance: inherited_tolerance,
            provenance: VertexProvenance::EdgeSplit {
                parent_edge_index,
                parameter,
            },
            regime: ToleranceRegime::Exact,
        }
    }

    /// Create a vertex from an external import source.
    pub fn from_import(position: [f64; 3], healing_tolerance: f64) -> Self {
        debug_assert!(healing_tolerance > 0.0, "healing_tolerance must be > 0.0");
        Self {
            position,
            tolerance: healing_tolerance,
            provenance: VertexProvenance::Imported { healing_tolerance },
            regime: ToleranceRegime::Imported,
        }
    }

    /// Create a coalesced vertex from n merged near-coincident vertices.
    pub fn from_coalescence(
        position: [f64; 3],
        merged_tolerance: f64,
        source_count: u32,
        max_gap: f64,
    ) -> Self {
        debug_assert!(merged_tolerance > 0.0, "merged_tolerance must be > 0.0");
        Self {
            position,
            tolerance: merged_tolerance,
            provenance: VertexProvenance::Coalesced {
                source_count,
                max_gap,
            },
            regime: ToleranceRegime::Exact,
        }
    }

    /// Certified tolerance for a vertex coalesced from two source vertices.
    ///
    /// Uses RSS (root-sum-of-squares) combination, correct for statistically
    /// independent error sources. The result is always ≥ `max(a, b)`.
    pub fn coalesced_tolerance(a: f64, b: f64) -> f64 {
        (a * a + b * b).sqrt()
    }

    /// Conservative tolerance for an edge-split vertex.
    ///
    /// Returns the larger of the two endpoint tolerances — the new vertex
    /// inherits no worse error than the worst of its parents.
    pub fn split_tolerance(origin_tol: f64, target_tol: f64) -> f64 {
        origin_tol.max(target_tol)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn planar_vertex_has_positive_tolerance_and_exact_regime() {
        let vg = VertexGeom::from_plane_intersection([0.0, 0.0, 0.0], [0, 1, 2], 1e-10);
        assert!(vg.tolerance > 0.0);
        assert_eq!(vg.position, [0.0, 0.0, 0.0]);
        assert_eq!(vg.regime, ToleranceRegime::Exact);
    }

    #[test]
    fn coalesced_vertex_has_correct_provenance() {
        let vg = VertexGeom::from_coalescence([1.0, 2.0, 3.0], 5e-7, 2, 3e-7);
        assert!(matches!(
            vg.provenance,
            VertexProvenance::Coalesced {
                source_count: 2,
                ..
            }
        ));
    }

    // ── Phase B — Propagation arithmetic ─────────────────────────────────────

    #[test]
    fn coalesced_tolerance_is_rss_combination() {
        let a = 1e-9_f64;
        let b = 2e-9_f64;
        let combined = VertexGeom::coalesced_tolerance(a, b);
        let expected = (a * a + b * b).sqrt(); // ≈ 2.236e-9
        assert!((combined - expected).abs() < 1e-25);
        assert!(combined >= a.max(b));
    }

    #[test]
    fn split_tolerance_returns_max_of_endpoints() {
        let origin = 1e-8_f64;
        let target = 3e-8_f64;
        assert_eq!(VertexGeom::split_tolerance(origin, target), 3e-8);
        assert_eq!(VertexGeom::split_tolerance(target, origin), 3e-8);
    }

    #[test]
    fn three_plane_intersection_has_none_residual_pre_ssi() {
        let vg = VertexGeom::from_plane_intersection([0.0, 0.0, 0.0], [0, 1, 2], 1e-7);
        assert!(matches!(
            vg.provenance,
            VertexProvenance::ThreePlaneIntersection { residual: None, .. }
        ));
    }

    #[test]
    fn coalesced_tolerance_exceeds_each_input() {
        let a = 5e-9_f64;
        let b = 5e-9_f64;
        let combined = VertexGeom::coalesced_tolerance(a, b);
        // RSS of equal values = a * sqrt(2) > a.
        assert!(combined > a);
    }

    // ── Phase D — Regime Enforcement ─────────────────────────────────────────

    #[test]
    fn regime_serialization_round_trips() {
        let r1 = ToleranceRegime::Exact;
        let s = serde_json::to_string(&r1).unwrap();
        let r2: ToleranceRegime = serde_json::from_str(&s).unwrap();
        assert_eq!(r1, r2);

        let r3 = ToleranceRegime::Healed {
            healing_tolerance_mm: 0.1,
        };
        let s2 = serde_json::to_string(&r3).unwrap();
        let r4: ToleranceRegime = serde_json::from_str(&s2).unwrap();
        assert_eq!(r3, r4);
    }

    #[test]
    fn vertex_geom_deserializes_missing_regime_as_exact() {
        // Raw JSON omitting the new `regime` field.
        let json = r#"{
            "position": [0.0, 0.0, 0.0],
            "tolerance": 1e-10,
            "provenance": {
                "ThreePlaneIntersection": {
                    "plane_indices": [0, 1, 2]
                }
            }
        }"#;
        let vg: VertexGeom = serde_json::from_str(json).unwrap();
        assert_eq!(vg.regime, ToleranceRegime::Exact); // Caught by #[serde(default)]
    }
}
