//! Data shapes for the EMBER integer grid boolean pipeline.
//!
//! DOMAIN: Quantization space mapping continuous f64 coordinates to
//! a discrete integer grid with ~1 billion points per axis.

/// Integer grid quantization space for exact boolean operations.
///
/// Maps the combined bounding box of two input solids to a fixed
/// 30-bit integer grid (`±2^30` per axis). Every f64 vertex coordinate
/// is snapped to the nearest grid point, eliminating floating-point
/// drift entirely.
///
/// After quantization:
/// - Two vertices either share the exact same `[i64; 3]` coordinate
///   (they are the same point) or differ by at least 1 grid unit.
/// - Orientation predicates use `i128` arithmetic (zero epsilon).
/// - The spatial NNS weld tolerance becomes obsolete.
pub struct QuantizedSpace {
    origin_offset: [f64; 3],
    scale_factor: f64,
    inverse_scale: f64,
}

impl QuantizedSpace {
    /// Build from two geometry stores — computes combined AABB and derives
    /// the integer grid scaling.
    ///
    /// Target: 30-bit grid (`±2^30` ≈ 1 billion points per axis).
    /// For meter-scale objects, this gives ~1 nanometer resolution.
    pub fn build(
        target_geom: &crate::geometry_state::GeometryState,
        tool_geom: &crate::geometry_state::GeometryState,
    ) -> Self {
        let mut min_pos = [f64::INFINITY; 3];
        let mut max_pos = [f64::NEG_INFINITY; 3];

        for pos in target_geom.iter_vertex_positions().chain(tool_geom.iter_vertex_positions()) {
            for i in 0..3 {
                min_pos[i] = min_pos[i].min(pos[i]);
                max_pos[i] = max_pos[i].max(pos[i]);
            }
        }

        let center = [
            (min_pos[0] + max_pos[0]) * 0.5,
            (min_pos[1] + max_pos[1]) * 0.5,
            (min_pos[2] + max_pos[2]) * 0.5,
        ];

        let max_dim = (max_pos[0] - min_pos[0])
            .max(max_pos[1] - min_pos[1])
            .max(max_pos[2] - min_pos[2]);

        let grid_max = (1i64 << 30) as f64;
        let scale_factor = if max_dim > 1e-15 { grid_max / max_dim } else { 1.0 };

        Self {
            origin_offset: center,
            scale_factor,
            inverse_scale: 1.0 / scale_factor,
        }
    }

    /// Snap an f64 coordinate to the nearest integer grid point.
    pub fn quantize(&self, point: &[f64; 3]) -> [i64; 3] {
        [
            ((point[0] - self.origin_offset[0]) * self.scale_factor).round() as i64,
            ((point[1] - self.origin_offset[1]) * self.scale_factor).round() as i64,
            ((point[2] - self.origin_offset[2]) * self.scale_factor).round() as i64,
        ]
    }

    /// Restore an integer grid coordinate back to f64 world space.
    pub fn restore(&self, grid_point: &[i64; 3]) -> [f64; 3] {
        [
            (grid_point[0] as f64 * self.inverse_scale) + self.origin_offset[0],
            (grid_point[1] as f64 * self.inverse_scale) + self.origin_offset[1],
            (grid_point[2] as f64 * self.inverse_scale) + self.origin_offset[2],
        ]
    }

    /// The scaling factor used for quantization.
    pub fn get_scale_factor(&self) -> f64 {
        self.scale_factor
    }

    /// The origin offset (center of the combined AABB).
    pub fn get_origin_offset(&self) -> &[f64; 3] {
        &self.origin_offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quantize_and_restore_round_trip() {
        let space = QuantizedSpace {
            origin_offset: [0.0, 0.0, 0.0],
            scale_factor: (1i64 << 30) as f64,
            inverse_scale: 1.0 / (1i64 << 30) as f64,
        };

        let original = [0.5, -0.25, 0.125];
        let grid = space.quantize(&original);
        let restored = space.restore(&grid);

        for i in 0..3 {
            assert!((original[i] - restored[i]).abs() < 1e-9,
                "axis {}: {} vs {}", i, original[i], restored[i]);
        }
    }

    #[test]
    fn identical_points_snap_to_same_grid() {
        let space = QuantizedSpace {
            origin_offset: [0.0, 0.0, 0.0],
            scale_factor: (1i64 << 30) as f64,
            inverse_scale: 1.0 / (1i64 << 30) as f64,
        };

        let a = [0.1, 0.2, 0.3];
        let b = [0.1 + 1e-15, 0.2 - 1e-15, 0.3 + 1e-15];

        assert_eq!(space.quantize(&a), space.quantize(&b),
            "Points 1e-15 apart should snap to the same grid coordinate");
    }

    #[test]
    fn distinct_points_snap_to_different_grid() {
        let space = QuantizedSpace {
            origin_offset: [0.0, 0.0, 0.0],
            scale_factor: (1i64 << 30) as f64,
            inverse_scale: 1.0 / (1i64 << 30) as f64,
        };

        let a = [0.0, 0.0, 0.0];
        let b = [1.0, 0.0, 0.0];

        assert_ne!(space.quantize(&a), space.quantize(&b));
    }
}
