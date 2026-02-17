//! # forge-schema
//!
//! Versioned declarative command schema for the Forge agent API.
//!
//! This crate defines the "language" that both human UI and AI agents
//! speak. Commands are pure data — they declare intent without executing
//! anything. The kernel interprets and executes them.
//!
//! ## Design Goals
//!
//! - **Token efficiency**: An agent specifies a modeling operation in
//!   ~100-200 tokens instead of ~2,000 (the SolidWorks COM API problem)
//! - **Versioned**: `SCHEMA_VERSION` tracks breaking changes
//! - **Serializable**: All types derive `Serialize`/`Deserialize` for
//!   JSON/Protobuf transport
//! - **Declarative**: Commands describe what to build, not how

#![forbid(unsafe_code)]

use serde::{Serialize, Deserialize};

/// The current schema version. Bump on breaking changes.
pub const SCHEMA_VERSION: &str = "0.1.0";

/// A modeling command in the Forge declarative API.
///
/// Each variant maps to a high-level modeling operation. The kernel
/// resolves commands into topology mutations via the feature pipeline.
///
/// # Token Budget
///
/// Commands are designed for minimal token cost:
/// - `AddBlock` ≈ 30 tokens
/// - `AddHole` ≈ 25 tokens
/// - `SetAttribute` ≈ 20 tokens
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Command {
    /// Create an axis-aligned block (rectangular prism).
    AddBlock {
        /// Origin corner of the block.
        origin: [f64; 3],
        /// Width, height, depth extents.
        dimensions: [f64; 3],
    },
    /// Create a cylindrical hole.
    AddHole {
        /// Center point of the hole on the target face.
        center: [f64; 3],
        /// Hole diameter.
        diameter: f64,
        /// Hole depth (positive = into material).
        depth: f64,
    },
    /// Apply a fillet (rounded edge).
    AddFillet {
        /// Which edge(s) to fillet.
        edge_selector: EdgeSelector,
        /// Fillet radius.
        radius: f64,
    },
    /// Boolean union of two entities.
    BooleanUnion {
        /// The base solid.
        target: EntityRef,
        /// The tool solid to add.
        tool: EntityRef,
    },
    /// Boolean subtraction.
    BooleanSubtract {
        /// The base solid.
        target: EntityRef,
        /// The tool solid to subtract.
        tool: EntityRef,
    },
    /// Set a semantic attribute on an entity.
    SetAttribute {
        /// The entity to tag.
        entity: EntityRef,
        /// Attribute key.
        key: String,
        /// Attribute value.
        value: TagValue,
    },
}

/// A reference to a topological entity in the model.
///
/// Entity references are stable across parameter changes
/// (they use lineage-based identity, not integer IDs).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "ref_type")]
pub enum EntityRef {
    /// Reference by feature name and optional sub-selector.
    ByFeature {
        /// The feature that created this entity.
        feature_name: String,
        /// Optional sub-entity selector within the feature.
        selector: Option<String>,
    },
    /// Reference by sequential operation index (simple cases).
    ByIndex {
        /// Zero-based index into the command sequence.
        index: usize,
    },
}

/// Selector for edges in fillet/chamfer operations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "selector_type")]
pub enum EdgeSelector {
    /// All edges of an entity.
    AllEdges {
        /// The entity whose edges to select.
        entity: EntityRef,
    },
    /// Edges where two features meet.
    IntersectionEdges {
        /// First feature.
        feature_a: EntityRef,
        /// Second feature.
        feature_b: EntityRef,
    },
}

/// Attribute values for the SetAttribute command.
///
/// Mirrors `forge_topo::attributes::TagValue` for serialization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "value_type", content = "data")]
pub enum TagValue {
    /// Free-form text.
    Text(String),
    /// Numeric value.
    Number(f64),
    /// Boolean flag.
    Flag(bool),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_version_is_set() {
        assert_eq!(SCHEMA_VERSION, "0.1.0");
    }

    #[test]
    fn add_block_round_trip() {
        let cmd = Command::AddBlock {
            origin: [0.0, 0.0, 0.0],
            dimensions: [10.0, 5.0, 3.0],
        };
        let json = serde_json::to_string(&cmd).unwrap();
        let parsed: Command = serde_json::from_str(&json).unwrap();
        assert_eq!(cmd, parsed);
    }

    #[test]
    fn add_hole_round_trip() {
        let cmd = Command::AddHole {
            center: [5.0, 2.5, 3.0],
            diameter: 4.0,
            depth: 3.0,
        };
        let json = serde_json::to_string(&cmd).unwrap();
        let parsed: Command = serde_json::from_str(&json).unwrap();
        assert_eq!(cmd, parsed);
    }

    #[test]
    fn set_attribute_round_trip() {
        let cmd = Command::SetAttribute {
            entity: EntityRef::ByFeature {
                feature_name: "Extrude-1".to_string(),
                selector: Some("top_face".to_string()),
            },
            key: "material".to_string(),
            value: TagValue::Text("AL_6061".to_string()),
        };
        let json = serde_json::to_string(&cmd).unwrap();
        let parsed: Command = serde_json::from_str(&json).unwrap();
        assert_eq!(cmd, parsed);
    }

    #[test]
    fn boolean_subtract_round_trip() {
        let cmd = Command::BooleanSubtract {
            target: EntityRef::ByIndex { index: 0 },
            tool: EntityRef::ByIndex { index: 1 },
        };
        let json = serde_json::to_string(&cmd).unwrap();
        let parsed: Command = serde_json::from_str(&json).unwrap();
        assert_eq!(cmd, parsed);
    }

    #[test]
    fn fillet_with_intersection_selector_round_trip() {
        let cmd = Command::AddFillet {
            edge_selector: EdgeSelector::IntersectionEdges {
                feature_a: EntityRef::ByFeature {
                    feature_name: "Extrude-1".to_string(),
                    selector: None,
                },
                feature_b: EntityRef::ByFeature {
                    feature_name: "Cut-1".to_string(),
                    selector: None,
                },
            },
            radius: 2.0,
        };
        let json = serde_json::to_string(&cmd).unwrap();
        let parsed: Command = serde_json::from_str(&json).unwrap();
        assert_eq!(cmd, parsed);
    }

    #[test]
    fn agent_token_efficiency() {
        let cmd = Command::AddHole {
            center: [5.0, 2.5, 3.0],
            diameter: 8.0,
            depth: 10.0,
        };
        let json = serde_json::to_string(&cmd).unwrap();
        assert!(json.len() < 200);
    }
}
