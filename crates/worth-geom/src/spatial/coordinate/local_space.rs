use crate::primitives::plane::Plane;
use worth_math::arithmetic::Rational;

/// Local coordinate frame for scale-invariant computation.
///
/// Translates a point cloud to the origin and scales to unit range.
/// All predicate evaluations in the local frame benefit from full
/// f64 precision regardless of the original coordinate magnitude.
#[derive(Debug, Clone)]
pub struct LocalCoordinateSpace {
    /// Centroid of the input geometry (translation offset).
    origin: [f64; 3],
    /// Reciprocal of the max half-extent (normalization factor).
    scale: f64,
}

/// Scale analysis for a set of coordinates.
///
/// Captures the condition number and determines whether a local
/// coordinate transform is required for numerical safety.
#[derive(Debug, Clone)]
pub struct ScaleAnalysis {
    /// Maximum absolute coordinate value.
    coordinate_magnitude: f64,
    /// Smallest meaningful distance between features.
    feature_size: f64,
    /// Ratio: magnitude / feature_size (higher = worse conditioned).
    condition_number: f64,
    /// Machine epsilon at the coordinate magnitude.
    machine_epsilon_at_scale: f64,
    /// Whether a local transform is required for numerical safety.
    needs_local_transform: bool,
}

impl LocalCoordinateSpace {
    /// Compute a local frame from a set of 3D points.
    ///
    /// The origin is the centroid of the bounding box, and the scale
    /// normalizes the max half-extent to 1.0.
    pub fn from_points(points: &[[f64; 3]]) -> Self {
        if points.is_empty() {
            return Self::identity();
        }

        let mut min = [f64::INFINITY; 3];
        let mut max = [f64::NEG_INFINITY; 3];

        for p in points {
            for i in 0..3 {
                if p[i] < min[i] {
                    min[i] = p[i];
                }
                if p[i] > max[i] {
                    max[i] = p[i];
                }
            }
        }

        let origin = [
            (min[0] + max[0]) * 0.5,
            (min[1] + max[1]) * 0.5,
            (min[2] + max[2]) * 0.5,
        ];

        let half_extent = [
            (max[0] - min[0]) * 0.5,
            (max[1] - min[1]) * 0.5,
            (max[2] - min[2]) * 0.5,
        ];

        let max_extent = half_extent[0]
            .max(half_extent[1])
            .max(half_extent[2])
            .max(f64::MIN_POSITIVE);

        let scale_log2 = max_extent.log2().ceil();
        let safe_max_extent = scale_log2.exp2();
        let scale = 1.0 / safe_max_extent;

        Self { origin, scale }
    }

    /// Compute a local frame from a set of planes.
    ///
    /// Uses the plane offsets and normals to estimate a representative
    /// origin and scale.
    pub fn from_planes(planes: &[Plane]) -> Self {
        if planes.is_empty() {
            return Self::identity();
        }

        let mut points = Vec::with_capacity(planes.len());
        for plane in planes {
            let [a, b, c] = plane.raw_normal();
            let d = plane.raw_offset();
            let scale = a * a + b * b + c * c;
            if scale > f64::MIN_POSITIVE {
                let inv = -d / scale;
                points.push([a * inv, b * inv, c * inv]);
            }
        }

        if points.is_empty() {
            return Self::identity();
        }

        Self::from_points(&points)
    }

    /// Identity transform (no translation, no scaling).
    pub fn identity() -> Self {
        Self {
            origin: [0.0, 0.0, 0.0],
            scale: 1.0,
        }
    }

    /// Transform a point to local coordinates.
    pub fn to_local(&self, point: [f64; 3]) -> [f64; 3] {
        [
            (point[0] - self.origin[0]) * self.scale,
            (point[1] - self.origin[1]) * self.scale,
            (point[2] - self.origin[2]) * self.scale,
        ]
    }

    /// Transform a point from local coordinates back to world.
    pub fn from_local(&self, local: [f64; 3]) -> [f64; 3] {
        let inv_scale = 1.0 / self.scale;
        [
            local[0] * inv_scale + self.origin[0],
            local[1] * inv_scale + self.origin[1],
            local[2] * inv_scale + self.origin[2],
        ]
    }

    /// Transform a plane to local coordinates using exact Rational arithmetic.
    ///
    /// For plane `ax + by + cz + d = 0`, substituting `x = x'/s + ox`
    /// gives `(a/s)x' + (b/s)y' + (c/s)z' + (a·ox + b·oy + c·oz + d) = 0`.
    ///
    /// All arithmetic is exact (Rational). The power-of-2 scale factor
    /// has an exact rational representation, so no precision is lost.
    pub fn transform_plane_exact(&self, plane: &Plane) -> Plane {
        let (a, b, c, d) = plane.exact_coefficients();
        let inv_scale =
            Rational::try_from_f64(1.0 / self.scale).unwrap_or_else(|_| Rational::one());
        let ox = Rational::try_from_f64(self.origin[0]).unwrap_or_else(|_| Rational::zero());
        let oy = Rational::try_from_f64(self.origin[1]).unwrap_or_else(|_| Rational::zero());
        let oz = Rational::try_from_f64(self.origin[2]).unwrap_or_else(|_| Rational::zero());

        let new_a = a * &inv_scale;
        let new_b = b * &inv_scale;
        let new_c = c * &inv_scale;
        let new_d = &(&(a * &ox) + &(b * &oy)) + &(&(c * &oz) + d);

        Plane::from_rationals(new_a, new_b, new_c, new_d)
            .expect("transformed plane normal cannot be zero")
    }

    /// Transform a plane from local coordinates back to world using exact Rational.
    pub fn inverse_transform_plane_exact(&self, local_plane: &Plane) -> Plane {
        let (al, bl, cl, dl) = local_plane.exact_coefficients();
        let scale_r = Rational::try_from_f64(self.scale).unwrap_or_else(|_| Rational::one());
        let ox = Rational::try_from_f64(self.origin[0]).unwrap_or_else(|_| Rational::zero());
        let oy = Rational::try_from_f64(self.origin[1]).unwrap_or_else(|_| Rational::zero());
        let oz = Rational::try_from_f64(self.origin[2]).unwrap_or_else(|_| Rational::zero());

        let a = al * &scale_r;
        let b = bl * &scale_r;
        let c = cl * &scale_r;
        let d = dl - &(&(&(&a * &ox) + &(&b * &oy)) + &(&c * &oz));

        Plane::from_rationals(a, b, c, d).expect("inverse transformed plane normal cannot be zero")
    }

    /// Transform an exact Rational position to local coordinates.
    pub fn to_local_exact(&self, point: &[Rational; 3]) -> [Rational; 3] {
        let s = Rational::try_from_f64(self.scale).unwrap_or_else(|_| Rational::one());
        let ox = Rational::try_from_f64(self.origin[0]).unwrap_or_else(|_| Rational::zero());
        let oy = Rational::try_from_f64(self.origin[1]).unwrap_or_else(|_| Rational::zero());
        let oz = Rational::try_from_f64(self.origin[2]).unwrap_or_else(|_| Rational::zero());
        let dx = &point[0] - &ox;
        let dy = &point[1] - &oy;
        let dz = &point[2] - &oz;
        [&dx * &s, &dy * &s, &dz * &s]
    }

    /// Transform an exact Rational position from local back to world.
    pub fn from_local_exact(&self, local: &[Rational; 3]) -> [Rational; 3] {
        let inv_s = Rational::try_from_f64(1.0 / self.scale).unwrap_or_else(|_| Rational::one());
        let ox = Rational::try_from_f64(self.origin[0]).unwrap_or_else(|_| Rational::zero());
        let oy = Rational::try_from_f64(self.origin[1]).unwrap_or_else(|_| Rational::zero());
        let oz = Rational::try_from_f64(self.origin[2]).unwrap_or_else(|_| Rational::zero());
        let wx = &local[0] * &inv_s;
        let wy = &local[1] * &inv_s;
        let wz = &local[2] * &inv_s;
        [&wx + &ox, &wy + &oy, &wz + &oz]
    }

    /// Transform a plane to local coordinates (f64 only, legacy).
    pub fn transform_plane(&self, plane: &Plane) -> Plane {
        let [a, b, c] = plane.raw_normal();
        let d = plane.raw_offset();
        let inv_scale = 1.0 / self.scale;
        let new_d = a * self.origin[0] + b * self.origin[1] + c * self.origin[2] + d;
        Plane::try_new([a * inv_scale, b * inv_scale, c * inv_scale], new_d)
            .expect("transformed plane normal cannot be zero")
    }

    /// Transform a plane from local coordinates back to world (f64 only, legacy).
    pub fn inverse_transform_plane(&self, local_plane: &Plane) -> Plane {
        let [a_local, b_local, c_local] = local_plane.raw_normal();
        let d_local = local_plane.raw_offset();
        let a = a_local * self.scale;
        let b = b_local * self.scale;
        let c = c_local * self.scale;
        let d = d_local - (a * self.origin[0] + b * self.origin[1] + c * self.origin[2]);
        Plane::try_new([a, b, c], d).expect("inverse transformed plane normal cannot be zero")
    }

    /// The translation origin.
    pub fn get_origin(&self) -> [f64; 3] {
        self.origin
    }

    /// The normalization scale factor.
    pub fn get_scale(&self) -> f64 {
        self.scale
    }
}

impl ScaleAnalysis {
    /// Analyze a point cloud for scale-related precision risks.
    pub fn compute(points: &[[f64; 3]], feature_tolerance: f64) -> Self {
        if points.is_empty() {
            return Self {
                coordinate_magnitude: 0.0,
                feature_size: feature_tolerance,
                condition_number: 0.0,
                machine_epsilon_at_scale: f64::MIN_POSITIVE,
                needs_local_transform: false,
            };
        }

        let mut max_mag: f64 = 0.0;
        for p in points {
            for v in p {
                let abs = v.abs();
                if abs > max_mag {
                    max_mag = abs;
                }
            }
        }

        let machine_eps = ulp(max_mag);
        let feature_size = feature_tolerance.max(f64::MIN_POSITIVE);
        let condition_number = max_mag / feature_size;

        let needs_transform = machine_eps > feature_size * 0.01;

        Self {
            coordinate_magnitude: max_mag,
            feature_size,
            condition_number,
            machine_epsilon_at_scale: machine_eps,
            needs_local_transform: needs_transform,
        }
    }

    /// Maximum absolute coordinate value.
    pub fn get_coordinate_magnitude(&self) -> f64 {
        self.coordinate_magnitude
    }

    /// Smallest meaningful feature distance.
    pub fn get_feature_size(&self) -> f64 {
        self.feature_size
    }

    /// Condition number (magnitude / feature_size).
    pub fn get_condition_number(&self) -> f64 {
        self.condition_number
    }

    /// Machine epsilon at the coordinate scale.
    pub fn get_machine_epsilon_at_scale(&self) -> f64 {
        self.machine_epsilon_at_scale
    }

    /// Whether a local coordinate transform is needed.
    pub fn get_needs_local_transform(&self) -> bool {
        self.needs_local_transform
    }
}

/// Unit of least precision at a given magnitude.
fn ulp(x: f64) -> f64 {
    let abs = x.abs();
    if abs == 0.0 {
        return f64::MIN_POSITIVE;
    }
    let bits = abs.to_bits();
    let next = f64::from_bits(bits + 1);
    next - abs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_is_no_op() {
        let space = LocalCoordinateSpace::identity();
        let p = [1e12, -2e6, 3.0];
        let local = space.to_local(p);
        assert_eq!(local, p);
        let back = space.from_local(local);
        assert_eq!(back, p);
    }

    #[test]
    fn round_trip_preserves_precision() {
        let points = vec![[1e12, 1e12, 1e12], [1e12 + 1.0, 1e12 + 1.0, 1e12 + 1.0]];
        let space = LocalCoordinateSpace::from_points(&points);

        for p in &points {
            let local = space.to_local(*p);
            let back = space.from_local(local);
            for i in 0..3 {
                let err = (back[i] - p[i]).abs();
                let u = ulp(p[i]);
                assert!(
                    err <= u,
                    "Round-trip error {} exceeds ULP {} at coord {}",
                    err,
                    u,
                    p[i]
                );
            }
        }
    }

    #[test]
    fn scale_analysis_detects_mixed_scale() {
        let points = vec![[1e12, 0.0, 0.0], [1e12 + 1e-9, 0.0, 0.0]];
        let analysis = ScaleAnalysis::compute(&points, 1e-9);
        assert!(analysis.get_needs_local_transform());
        assert!(analysis.get_condition_number() > 1e15);
    }

    #[test]
    fn scale_analysis_unit_scale_is_fine() {
        let points = vec![[0.0, 0.0, 0.0], [1.0, 1.0, 1.0]];
        let analysis = ScaleAnalysis::compute(&points, 1e-6);
        assert!(!analysis.get_needs_local_transform());
    }

    #[test]
    fn plane_round_trip() {
        let plane = Plane::try_new([0.0, 0.0, 1.0], -1e12).unwrap();
        let space = LocalCoordinateSpace::from_points(&[[0.0, 0.0, 1e12], [1.0, 1.0, 1e12 + 1.0]]);
        let local_plane = space.transform_plane(&plane);
        let back = space.inverse_transform_plane(&local_plane);
        let d = back.raw_offset();
        let d2 = plane.raw_offset();
        assert!((d - d2).abs() < 1e-3);
    }
}
