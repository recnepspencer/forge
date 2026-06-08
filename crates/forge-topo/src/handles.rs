//! Typed, generational handles for topology entities.
//!
//! # Why Handles Instead of References
//!
//! A B-Rep mesh is a cyclic graph: faces → edges → vertices → edges → faces.
//! Rust's ownership model (borrow checker) cannot express cyclic references
//! without `Rc<RefCell<T>>` (runtime panics) or `unsafe` (we forbid that).
//!
//! Instead, we use **generational handles**: lightweight IDs that refer to
//! entities stored in a central arena. The generation counter prevents
//! use-after-free: if a face is deleted and a new one takes its slot,
//! the old handle's generation won't match, and lookups fail safely.
//!
//! # Type Safety
//!
//! Each entity type gets its own handle type via the `define_handle!` macro.
//! You cannot accidentally pass a `VertexId` where a `FaceId` is expected —
//! the compiler catches it.

use serde::{Deserialize, Serialize};

/// Generates a strongly-typed generational handle.
///
/// Each handle is a `(index, generation)` pair. The index identifies the
/// slot in the arena; the generation prevents stale references.
///
/// Serializes as `"index:generation"` strings for JSON map-key compatibility.
///
/// # Example
/// ```ignore
/// define_handle!(FaceId);
/// define_handle!(VertexId);
///
/// let face: FaceId = /* from arena */;
/// let vertex: VertexId = /* from arena */;
/// // face == vertex;  // Compile error! Different types.
/// ```
macro_rules! define_handle {
    ($name:ident) => {
        /// A typed, generational handle for a topology entity.
        ///
        /// - `index`: slot position in the arena
        /// - `generation`: incremented when the slot is reused after deletion
        ///
        /// Handles are `Copy` (cheap to pass around) and safe (stale handles
        /// are detected by generation mismatch).
        ///
        /// Serializes as `"index:generation"` (e.g. `"5:2"`) for JSON
        /// map-key compatibility.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name {
            index: u32,
            generation: u32,
        }

        impl $name {
            /// A sentinel handle that does not refer to any valid entity.
            ///
            /// Used as a placeholder during multi-step insertions where
            /// the real target isn't known yet. Code that encounters a
            /// `DANGLING` handle must treat it as uninitialized — never
            /// dereference it via `get_*` accessors.
            pub const DANGLING: Self = Self {
                index: u32::MAX,
                generation: 0,
            };

            /// Create a handle from an index and generation pair.
            pub fn new(index: u32, generation: u32) -> Self {
                Self { index, generation }
            }

            /// Returns `true` if this handle is the `DANGLING` sentinel.
            pub fn is_dangling(self) -> bool {
                self.index == u32::MAX && self.generation == 0
            }

            /// The slot index in the arena.
            pub fn index(self) -> u32 {
                self.index
            }

            /// The generation counter (for stale-handle detection).
            pub fn generation(self) -> u32 {
                self.generation
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}({}:gen{})",
                    stringify!($name),
                    self.index,
                    self.generation
                )
            }
        }

        impl Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                let s = format!("{}:{}", self.index, self.generation);
                serializer.serialize_str(&s)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let s = String::deserialize(deserializer)?;
                let parts: Vec<&str> = s.split(':').collect();
                if parts.len() != 2 {
                    return Err(serde::de::Error::custom(format!(
                        "expected 'index:generation', got '{}'",
                        s
                    )));
                }
                let index = parts[0].parse::<u32>().map_err(serde::de::Error::custom)?;
                let generation = parts[1].parse::<u32>().map_err(serde::de::Error::custom)?;
                Ok(Self { index, generation })
            }
        }
    };
}

// Core topology entity handles
define_handle!(FaceId);
define_handle!(HalfEdgeId);
define_handle!(VertexId);
define_handle!(LoopId);
define_handle!(BodyId);
define_handle!(LumpId);
define_handle!(RegionId);
define_handle!(ShellId);
define_handle!(EdgeId);

// ── Handle → EntityRef conversions ───────────────────────────────────────
// Enables `recorder.stamp(store, face_id)` without manual conversion.

macro_rules! impl_entity_ref_from_handle {
    ($handle:ty, $kind:expr) => {
        impl From<$handle> for forge_core::EntityRef {
            fn from(id: $handle) -> Self {
                forge_core::EntityRef::new($kind, id.index(), id.generation())
            }
        }
    };
}

impl_entity_ref_from_handle!(FaceId, forge_core::EntityKind::Face);
impl_entity_ref_from_handle!(HalfEdgeId, forge_core::EntityKind::HalfEdge);
impl_entity_ref_from_handle!(VertexId, forge_core::EntityKind::Vertex);
impl_entity_ref_from_handle!(LoopId, forge_core::EntityKind::Loop);
impl_entity_ref_from_handle!(BodyId, forge_core::EntityKind::Body);
impl_entity_ref_from_handle!(LumpId, forge_core::EntityKind::Lump);
impl_entity_ref_from_handle!(RegionId, forge_core::EntityKind::Region);
impl_entity_ref_from_handle!(ShellId, forge_core::EntityKind::Shell);
impl_entity_ref_from_handle!(EdgeId, forge_core::EntityKind::Edge);

// Opaque cross-crate reference to a curve in `worth-geom::CurveGeom`.
//
// This handle is stored in `EdgeData.curve` so that the topology arena can
// hold a typed reference to edge-curve geometry without owning or inspecting
// any `f64` values (Doctrine D3). The actual `CurveGeom` arena lives in
// `worth-geom` and is managed by the kernel's `GeometryStore`.
//
// During Phase 1–2 (planar-only), this is always `None`. Phase 4+ populates
// it when curved surfaces are introduced.
define_handle!(CurveRef);

// Opaque cross-crate reference to a surface in `worth-geom::SurfaceData`.
//
// Stored in `FaceData.surface` so that the topology arena can hold a typed
// reference to face-surface geometry without owning any `f64` values
// (Doctrine D3). The actual `SurfaceData` arena lives in the kernel's
// `GeometryStore`.
//
// `None` for planar faces where the surface is an implicit plane defined
// by face-plane association in the `GeometryStore`. `Some` for curved
// surfaces (cylinders, cones, spheres, tori, NURBS).
define_handle!(SurfaceRef);

// Opaque cross-crate reference to a coedge (UV trim curve) in
// `worth-geom::Coedge`.
//
// Stored in `HalfEdgeData.coedge` so that each directed use of an edge
// can reference its 2D trim curve in the adjacent face's parameter space.
// This is the mechanism that prevents 3D positional drift in deep boolean
// chains: `surface.point_at(coedge.uv_at(t))` is on the surface by
// construction.
//
// `None` for planar halfedges (the coedge is a trivial straight line in UV).
define_handle!(CoedgeRef);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_are_distinct_types() {
        let face = FaceId::new(0, 1);
        let vertex = VertexId::new(0, 1);

        // Same index and generation, but different types
        assert_eq!(face.index(), vertex.index());
        assert_eq!(face.generation(), vertex.generation());
        // face == vertex would be a compile error — that's the point
    }

    #[test]
    fn handle_display() {
        let face = FaceId::new(5, 2);
        assert_eq!(format!("{}", face), "FaceId(5:gen2)");
    }

    #[test]
    fn handles_are_orderable() {
        let a = FaceId::new(1, 1);
        let b = FaceId::new(2, 1);
        let c = FaceId::new(1, 2);

        // Ordered by index first, then generation
        assert!(a < b);
        assert!(a < c);
    }

    #[test]
    fn handles_are_copy() {
        let original = VertexId::new(3, 1);
        let copied = original; // Copy, not move
        assert_eq!(original, copied); // original is still valid
    }
}
