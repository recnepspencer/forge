//! Semantic tag data shapes.
//!
//! DOMAIN: Entity key discriminant, tag value types, and type alias.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::handles::{EdgeId, FaceId, ShellId, VertexId};

/// The key into the attribute store — identifies which entity owns a tag set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityKey {
    Shell(ShellId),
    Face(FaceId),
    Edge(EdgeId),
    Vertex(VertexId),
}

impl From<ShellId> for EntityKey {
    fn from(id: ShellId) -> Self { Self::Shell(id) }
}
impl From<FaceId> for EntityKey {
    fn from(id: FaceId) -> Self { Self::Face(id) }
}
impl From<EdgeId> for EntityKey {
    fn from(id: EdgeId) -> Self { Self::Edge(id) }
}
impl From<VertexId> for EntityKey {
    fn from(id: VertexId) -> Self { Self::Vertex(id) }
}

/// A single attribute value attached to an entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TagValue {
    Text(String),
    Number(f64),
    Flag(bool),
}

/// A collection of key-value tags for a single entity.
pub type SemanticTag = HashMap<String, TagValue>;
