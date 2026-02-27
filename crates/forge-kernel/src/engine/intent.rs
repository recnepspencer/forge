//! Mathematical intent specifications for dual SDF/B-Rep consumption.
//!
//! DOMAIN: Declarative feature primitives that both the SDF engine
//! and the B-Rep solver can consume independently.
//!
//! INVARIANTS:
//! - Specs are pure data — no eval logic or topology mutation
//! - Each variant maps 1:1 to a modeling operation
//!
//! DEPENDENCIES: None (data-only)

/// A declarative specification of a modeling primitive.
///
/// Instead of features directly calling B-Rep functions, they produce
/// a `PrimitiveSpec` that both the SDF preview engine (60fps real-time)
/// and the B-Rep solver (exact topology) can consume independently.
///
/// This eliminates the need to write every feature twice.
#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveSpec {
    /// An extruded 2D profile along a direction vector.
    ExtrudedProfile {
        /// The 2D profile to extrude.
        profile: ProfileSpec,
        /// Extrusion direction (unit vector).
        direction: [f64; 3],
        /// Extrusion depth (positive = along direction).
        depth: f64,
    },
    /// A revolved 2D profile around an axis.
    RevolvedProfile {
        /// The 2D profile to revolve.
        profile: ProfileSpec,
        /// Origin point of the revolution axis.
        axis_origin: [f64; 3],
        /// Direction of the revolution axis (unit vector).
        axis_dir: [f64; 3],
        /// Angle of revolution in radians (2π = full revolution).
        angle: f64,
    },
    /// A CSG Boolean combination of primitives.
    BooleanOp {
        /// The Boolean operation type.
        op_type: BooleanType,
        /// The target (base) solid.
        target: Box<PrimitiveSpec>,
        /// The tool solid.
        tool: Box<PrimitiveSpec>,
    },
}

/// A 2D profile specification for extrude/revolve operations.
#[derive(Debug, Clone, PartialEq)]
pub enum ProfileSpec {
    /// Axis-aligned rectangle centered at origin.
    Rectangle {
        /// Width (X extent).
        width: f64,
        /// Height (Y extent).
        height: f64,
    },
    /// Circle centered at origin.
    Circle {
        /// Circle radius.
        radius: f64,
    },
    /// Arbitrary polygon defined by ordered vertices.
    Polygon {
        /// Ordered 2D vertices (implicitly closed).
        vertices: Vec<[f64; 2]>,
    },
}

/// Boolean operation types for CSG combinations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BooleanType {
    /// Material addition (A ∪ B).
    Union,
    /// Material removal (A − B).
    Subtraction,
    /// Common material (A ∩ B).
    Intersection,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extrude_spec_construction() {
        let spec = PrimitiveSpec::ExtrudedProfile {
            profile: ProfileSpec::Rectangle {
                width: 10.0,
                height: 5.0,
            },
            direction: [0.0, 0.0, 1.0],
            depth: 20.0,
        };
        assert_eq!(spec, spec.clone());
    }

    #[test]
    fn revolve_spec_construction() {
        let spec = PrimitiveSpec::RevolvedProfile {
            profile: ProfileSpec::Circle { radius: 5.0 },
            axis_origin: [0.0, 0.0, 0.0],
            axis_dir: [0.0, 1.0, 0.0],
            angle: std::f64::consts::TAU,
        };
        assert_eq!(spec, spec.clone());
    }

    #[test]
    fn boolean_spec_nesting() {
        let block = PrimitiveSpec::ExtrudedProfile {
            profile: ProfileSpec::Rectangle {
                width: 10.0,
                height: 10.0,
            },
            direction: [0.0, 0.0, 1.0],
            depth: 5.0,
        };
        let hole = PrimitiveSpec::ExtrudedProfile {
            profile: ProfileSpec::Circle { radius: 2.0 },
            direction: [0.0, 0.0, 1.0],
            depth: 5.0,
        };
        let result = PrimitiveSpec::BooleanOp {
            op_type: BooleanType::Subtraction,
            target: Box::new(block),
            tool: Box::new(hole),
        };
        assert_eq!(result, result.clone());
    }

    #[test]
    fn polygon_profile_construction() {
        let triangle = ProfileSpec::Polygon {
            vertices: vec![[0.0, 0.0], [10.0, 0.0], [5.0, 8.66]],
        };
        assert_eq!(triangle, triangle.clone());
    }
}
