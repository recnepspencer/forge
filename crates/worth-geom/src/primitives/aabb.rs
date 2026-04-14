//! Axis-Aligned Bounding Box (AABB) for spatial indexing.

/// An axis-aligned bounding box defined by min and max points.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb {
    pub min: [f64; 3],
    pub max: [f64; 3],
}

impl Aabb {
    /// Create a new AABB from min and max points.
    ///
    /// Automatically ensures min <= max per component.
    pub fn new(p1: [f64; 3], p2: [f64; 3]) -> Self {
        let min = [p1[0].min(p2[0]), p1[1].min(p2[1]), p1[2].min(p2[2])];
        let max = [p1[0].max(p2[0]), p1[1].max(p2[1]), p1[2].max(p2[2])];
        Self { min, max }
    }

    /// Create an AABB enclosing a set of points.
    ///
    /// Returns None if the input is empty.
    pub fn from_points(points: &[[f64; 3]]) -> Option<Self> {
        if points.is_empty() {
            return None;
        }
        let mut min = points[0];
        let mut max = points[0];

        for p in &points[1..] {
            for i in 0..3 {
                if p[i] < min[i] {
                    min[i] = p[i];
                }
                if p[i] > max[i] {
                    max[i] = p[i];
                }
            }
        }
        Some(Self { min, max })
    }

    /// Expand the AABB by a margin in all directions.
    pub fn expand(&mut self, margin: f64) {
        for i in 0..3 {
            self.min[i] -= margin;
            self.max[i] += margin;
        }
    }

    /// Check if this AABB intersects another AABB.
    pub fn intersects(&self, other: &Aabb) -> bool {
        self.min[0] <= other.max[0]
            && self.max[0] >= other.min[0]
            && self.min[1] <= other.max[1]
            && self.max[1] >= other.min[1]
            && self.min[2] <= other.max[2]
            && self.max[2] >= other.min[2]
    }

    /// Union of two AABBs.
    pub fn union(&self, other: &Aabb) -> Self {
        let min = [
            self.min[0].min(other.min[0]),
            self.min[1].min(other.min[1]),
            self.min[2].min(other.min[2]),
        ];
        let max = [
            self.max[0].max(other.max[0]),
            self.max[1].max(other.max[1]),
            self.max[2].max(other.max[2]),
        ];
        Self { min, max }
    }

    /// Squared distance from a point to this AABB (0 if the point is inside).
    pub fn distance_to_point_sq(&self, point: &[f64; 3]) -> f64 {
        let mut sum = 0.0;
        for i in 0..3 {
            let delta = if point[i] < self.min[i] {
                self.min[i] - point[i]
            } else if point[i] > self.max[i] {
                point[i] - self.max[i]
            } else {
                0.0
            };
            sum += delta * delta;
        }
        sum
    }

    /// Check if a plane intersects this AABB.
    ///
    /// Plane equation: ax + by + cz + d = 0
    /// Using the "positive/negative vertex" optimization.
    pub fn plane_crosses(&self, plane_eq: &[f64; 4]) -> bool {
        let normal = [plane_eq[0], plane_eq[1], plane_eq[2]];
        let d = plane_eq[3];

        // Find the "positive" vertex (max extents in normal direction)
        // and "negative" vertex (min extents in normal direction)
        let mut v_min = [0.0; 3];
        let mut v_max = [0.0; 3];

        for i in 0..3 {
            if normal[i] >= 0.0 {
                v_min[i] = self.min[i];
                v_max[i] = self.max[i];
            } else {
                v_min[i] = self.max[i];
                v_max[i] = self.min[i];
            }
        }

        let d_max = normal[0] * v_max[0] + normal[1] * v_max[1] + normal[2] * v_max[2] + d;
        let d_min = normal[0] * v_min[0] + normal[1] * v_min[1] + normal[2] * v_min[2] + d;

        // If d_max >= 0 and d_min <= 0, the plane crosses the box.
        d_max >= 0.0 && d_min <= 0.0
    }

    /// Center of the AABB.
    pub fn center(&self) -> [f64; 3] {
        [
            (self.min[0] + self.max[0]) * 0.5,
            (self.min[1] + self.max[1]) * 0.5,
            (self.min[2] + self.max[2]) * 0.5,
        ]
    }

    /// Largest dimension index (0=x, 1=y, 2=z).
    pub fn largest_axis(&self) -> usize {
        let dx = self.max[0] - self.min[0];
        let dy = self.max[1] - self.min[1];
        let dz = self.max[2] - self.min[2];
        if dx >= dy && dx >= dz {
            0
        } else if dy >= dz {
            1
        } else {
            2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Aabb;

    #[test]
    fn distance_to_point_sq_is_zero_inside_box() {
        let aabb = Aabb::new([0.0, 0.0, 0.0], [2.0, 2.0, 2.0]);
        let distance_sq = aabb.distance_to_point_sq(&[1.0, 1.0, 1.0]);
        assert_eq!(distance_sq, 0.0);
    }

    #[test]
    fn distance_to_point_sq_accumulates_outside_axes() {
        let aabb = Aabb::new([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
        let distance_sq = aabb.distance_to_point_sq(&[3.0, -2.0, 0.5]);
        assert_eq!(distance_sq, 8.0);
    }
}
