# Persistent Naming & Topological Stability — Engineering Specification

## 1. Problem Statement

The current `PersistentName` implementation relies exclusively on an `ancestry_hash` derived from the Euler operation invocation sequence. When a user edits the feature tree (e.g., inserts or removes an operation early in the timeline), the `invocation_id`s of all subsequent operations shift. This causes the `ancestry_hash` to avalanche, breaking all downstream `PersistentName` references.

This architectural vulnerability is known as the **Topological Naming Problem (TNP)**. A robust CAD kernel must be able to re-attach downstream features (like fillets or sketches) even when the exact operation sequence changes, provided the underlying topology and geometry remain conceptually similar.

## 2. Prerequisite: Completing `EntityBitset`

The solution to TNP requires fast, repeated Breadth-First Search (BFS) traversals over the topology to find stable anchoring entities. Using `std::collections::HashSet` or `BTreeSet` for these traversals is too slow and heap-intensive for a core naming fallback loop.

**Prerequisite Milestone (PN-0): 100% `EntityBitset` Adoption**
The `EntityBitset` has been built and integrated into `forge-topo::integrity`. Before starting TNP, it must replace all remaining topological set allocations:

- `forge-kernel::operations::boolean::postprocess::polygon_extract` (replaces `BTreeSet<FaceId>`)
- `forge-kernel::analysis::region_extractor` (replaces `BTreeSet<FaceId>`)
- `forge-topo::operations::algorithms::extract_shell` (replaces `BTreeSet<FaceId>`)
- `forge-kernel::operations::ember_boolean::mesh` (replaces `HashSet<FaceId>`)

**Acceptance (PN-0):** A full codebase grep for `HashSet<FaceId>` and `BTreeSet<FaceId>` returns zero results.

## 3. The Three-Tier Naming Architecture

To survive tree edits, `PersistentName` resolution will transition from a binary pass/fail to a three-tier fallback architecture, stopping at the first unambiguous match. Fingerprints are captured at _naming time_ and stored in the `PersistentName`.

### Tier 1: Ancestry Hash Match (Existing)

- **Mechanism:** Exact match on the `ancestry_hash`.
- **Confidence:** Highest (`ExactAncestry`).
- **Use Case:** Append-only operations. If the user adds a new feature without editing the past, Tier 1 resolves in $O(1)$ time.
- **Status:** Already implemented. Tiers 2 and 3 are only invoked if Tier 1 yields zero matches.

### Tier 2: Topological Anchoring (Path-based)

- **Mechanism:** Finds an "Anchor Entity" — an entity with a highly stable derivation (e.g., the origin planes or the base solid boundary) that survives tree edits.
- **Fingerprint:** Stores the anchor's `ancestry_hash`, the creation operation name of the target entity, its `EntityKind`, and the BFS distance (steps) to the anchor.
- **Resolution:** Executes a BFS (using `EntityBitset`) from known stable anchors out to candidate entities. If a candidate matches the creation op name and step distance, it's a match.
- **Confidence:** Medium (`TopologicalAnchor`).
- **Why not just neighbor counts?** Simple neighbor counts (e.g., "Face with 4 edges") fail catastrophically on symmetric parts (like a block with 4 identical holes). Topological pathing breaks symmetry.

### Tier 3: Intrinsic Geometric Signature

- **Mechanism:** Fallback when topological structure changes too radically for Tier 2 to confidently anchor. Uses scale-invariant and transform-invariant geometric properties.
- **Fingerprint:** Stores the `SurfaceKindTag` (Plane, Cylinder, Sphere) and an ordered list of dihedral angles to adjacent faces.
- **Resolution:** Queries `GeometryStore` via a read-only `GeometricSignatureProvider` trait.
- **Confidence:** Lowest (`IntrinsicGeometry`).
- **Why not absolute coordinates?** Storing a bounding box centroid `[x, y, z]` violates transform invariance (`MB-S6`). A parametric scale or translation would break absolute coordinate signatures. Dihedral angles and surface types are intrinsic.

---

## 4. Proposed Data Structures

### `forge-topo::topology::naming::schema.rs`

```rust
pub enum NameConfidence {
    ExactAncestry,
    TopologicalAnchor { distance: u32, score: f64 },
    IntrinsicGeometry { score: f64 },
}

pub struct TopologicalFingerprint {
    pub creation_op_name: String,
    pub anchor_hash: u128,      // Tier 1 hash of the nearest stable anchor
    pub steps_to_anchor: u32,   // BFS distance to the anchor
    pub kind: EntityKind,
}

pub struct GeometricFingerprint {
    pub surface_kind: SurfaceKindTag,
    pub neighbor_dihedral_angles: Vec<f64>, // Ordered float sequence (Intrinsic)
}

pub struct PersistentName {
    ancestry_hash: u128,
    kind: EntityKind,
    ordinal: u32,
    // Captured at naming time
    topo_fingerprint: TopologicalFingerprint,
    geo_fingerprint: Option<GeometricFingerprint>,
}
```

### `forge-topo::topology::naming::eval.rs`

```rust
/// Trait implemented by forge-kernel to provide intrinsic geometry without circular dependencies
pub trait GeometricSignatureProvider {
    fn get_surface_kind(&self, face: FaceId) -> Option<SurfaceKindTag>;
    fn get_dihedral_angles(&self, face: FaceId) -> Option<Vec<f64>>;
}

/// The new primary multi-heuristic resolver
pub fn resolve_name_tiered(
    arena: &TopologyArena,
    name: &PersistentName,
    geometry: Option<&dyn GeometricSignatureProvider>
) -> Vec<(EntityKey, NameConfidence)> {
    // 1. Try Tier 1 (Exact Hash)
    // 2. Try Tier 2 (Topological Anchor via BFS + EntityBitset)
    // 3. Try Tier 3 (Intrinsic Geometry)
}
```

---

## 5. Implementation Milestones

### Milestone PN-1: Fallback Schemas & Traits

Add the new fingerprint structs to `PersistentName`, adjusting `assign_name` to mock or populate empty fingerprints. Introduce the `GeometricSignatureProvider` trait in `forge-topo`. Update all existing pure-topology tests to pass `None` for geometry.

### Milestone PN-2: Tier 2 Topological Anchoring

Implement the `EntityBitset` BFS anchoring logic in `assign_name` (to find an anchor and record distance) and in `resolve_name_tiered` (to walk outward from anchors when Tier 1 fails).

### Milestone PN-3: Proof Integration (MB-C8 TNP Extension)

Add `PV-TNP-1` to the test suite:

1. Build a Solid (e.g., `make_cube` + `extrude`).
2. Assign a `PersistentName` to a specific resultant face.
3. Rebuild the solid, but intentionally insert a dummy Euler operation early in the sequence (simulating a rolled-back tree edit).
4. Assert: `resolve_name_tiered` correctly finds the face, returning `NameConfidence::TopologicalAnchor`.

### Milestone PN-4: Tier 3 Intrinsic Geometry

Implement `GeometricSignatureProvider` on `forge-kernel`'s `GeometryStore`. Implement dihedral angle extraction. Update the resolver to fall back to Tier 3 when Tier 2 yields ambiguous results (e.g., severe topological disruption). Add `PV-TNP-2` where an edge split completely disrupts Tier 2's step count, forcing Tier 3 to successfully resolve via intrinsic geometry.
