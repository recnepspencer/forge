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
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        pub struct $name {
            index: u32,
            generation: u32,
        }

        impl $name {
            /// Create a handle. Arena-internal only.
            pub(crate) fn new(index: u32, generation: u32) -> Self {
                Self { index, generation }
            }

            /// Reconstruct a handle from raw index and generation values.
            ///
            /// Use this when deserializing handles or reconstructing them
            /// from arena iteration results. Prefer obtaining handles through
            /// Euler operators (`apply_op`) for normal topology construction.
            pub fn from_raw_parts(index: u32, generation: u32) -> Self {
                Self { index, generation }
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
                write!(f, "{}({}:gen{})", stringify!($name), self.index, self.generation)
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
