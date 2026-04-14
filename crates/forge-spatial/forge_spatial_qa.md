# forge-spatial — QA Checklist

Bridge crate: needs **both** topology handles and geometry math. Anything requiring a `FaceId` and a floating-point calculation lives here.

**Architecture refs:** KERNEL_ARCHITECTURE §2 (layering), §4.1 (TopologyState — no geometry), Doctrines D1–D3

---

## Boundary Rules (every file, every PR)

- [ ] **No upward imports.** Depends on `forge-topo`, `worth-geom`, `worth-math`, `forge-core`. Never `forge-kernel`.
- [ ] **Read-only topology.** Every function takes `&TopologyArena`. Zero `&mut` refs — this crate classifies and measures, never mutates.
- [ ] **Positions via callbacks.** All vertex coordinates come through `Fn(VertexId) -> Option<[f64; 3]>` closures. Never import `GeometryState` or `ExactPosition`.
- [ ] **Tolerances via `&dyn ToleranceProvider`.** No `const EPS`, no `1e-8`, no hardcoded thresholds. If a function needs a tolerance, it accepts it as a parameter.
- [ ] **No `ToleranceConfig` import.** That's a `forge-kernel` type. This crate receives individual `f64` values.
- [ ] **Deterministic for identical inputs (D1).** No `HashMap` driving output order. Use `BTreeSet`/`EntityBitset` for visited-set tracking.
- [ ] **No `unwrap()`/`panic!()` outside `#[cfg(test)]`.**

---

## Classification — Invariants

- [ ] **Boundary proximity is checked BEFORE ray casting.** Point-on-face pre-pass must catch boundary-contact cases before SoS perturbation can corrupt them.
- [ ] **Orientation predicates come from `worth-math`.** All [orient2d](file:///Users/spenstar/Documents/programming/Forge/crates/forge-spatial/src/classify/sos.rs#17-40)/[orient3d](file:///Users/spenstar/Documents/programming/Forge/crates/forge-spatial/src/classify/sos.rs#41-85) calls use the certified predicates. No raw `f64` cross products or `> 0.0` orientation checks (D3).
- [ ] **SoS is purely symbolic.** Simulation of Simplicity resolves exact-zero orientations via sign-of-coefficient analysis. No floating-point epsilon is ever added to coordinates.
- [ ] **SoS never produces `Zero`.** The cascade must always terminate with a definite sign. If a code path can return `Zero` from SoS, it's broken.
- [ ] **Per-entity tolerances.** Vertex proximity uses [vertex_tolerance(v)](file:///Users/spenstar/Documents/programming/Forge/crates/forge-core/src/tolerance.rs#101-104), edge proximity uses [edge_tolerance(e)](file:///Users/spenstar/Documents/programming/Forge/crates/forge-core/src/tolerance.rs#105-108). Never a single global tolerance for both entity types.
- [ ] **Edge closest-point is clamped.** Projection parameter `t` is clamped to `[0, 1]`. Unbounded projection gives wrong distance-to-segment.
- [ ] **Face samples are validated before use.** Interior sample points generated for face classification must be confirmed as `OnFace` (not on boundary) before being trusted.

**Known defect (D8):** [classify_point_in_solid](file:///Users/spenstar/Documents/programming/Forge/crates/forge-spatial/src/classify/point_in_solid.rs#28-74) has no multi-direction ray retry. If the +X ray hits a degenerate configuration, there's no fallback to Y or Z direction. This is documented with a `DEFECT(D8)` tag — do NOT silently remove it.

---

## Bounds — Invariants

- [ ] **Hierarchy follows topology exactly.** `solid_bounds = union(lump_bounds) = union(region_bounds) = union(shell_bounds) = union(face_bounds)`. Skipping a level is wrong.
- [ ] **Empty topology produces `None`, not a default AABB.** An empty shell/region/lump yields `None` that propagates up correctly.
- [ ] **Face bounds traverse ALL loops.** Not just the outer loop — faces with holes must include inner-loop vertices in their AABB.
- [ ] **Distance/scale utilities are parameterized.** `scale_factor`, `default_extent` come from the kernel, not hardcoded. The `1e-15` empty-arena floor is a denormalization guard, NOT a tolerance.

---

## Integrity — Invariants

- [ ] **Area threshold is per-entity.** `max(vertex_tolerance)²` across the face's vertices. Not a global constant.
- [ ] **Edge length threshold uses the worse endpoint.** `max(tol(origin), tol(target))`.
- [ ] **Shell volume is deterministic.** BFS uses `BTreeSet` for visited tracking (D1). Fan triangulation order must be stable.
- [ ] **Negative volume = error, not auto-flip.** `NegativeShellVolume` is reported — the crate does NOT silently fix normals.
- [ ] **Integrity checks skip non-planar geometry gracefully.** `is_planar` callback gates area/volume checks. When NURBS faces arrive, these checks must be skipped (not crash) until surface-integral versions exist.

---

## Stability vs Growth

**STABLE:** [SpatialAccelerator](file:///Users/spenstar/Documents/programming/Forge/crates/forge-spatial/src/classify/schema.rs#24-27) trait, `PointClassification` enum, position-callback pattern, [ToleranceProvider](file:///Users/spenstar/Documents/programming/Forge/crates/forge-core/src/tolerance.rs#39-73) threading, AABB hierarchy.

**WILL EXPAND:**
- Classification → curved-edge distance, UV-space proximity, multi-direction ray retry (D8 fix)
- Bounds → curve control-hull bounding, multi-loop face support
- Integrity → surface-integral area/volume for non-planar faces
- New modules: `nearest/` (closest-point-on-solid), `intersection/` (ray-solid hits)

---

## Reject on Sight

| Pattern | Why | Instead |
|---------|-----|---------|
| Hardcoded tolerance (`1e-10`, `const EPS`) | D2/D4 violation | Accept via [ToleranceProvider](file:///Users/spenstar/Documents/programming/Forge/crates/forge-core/src/tolerance.rs#39-73) or `f64` param |
| `HashMap` for visited-face/shell tracking | Non-deterministic (D1) | `BTreeSet` or `EntityBitset` |
| Importing `GeometryState` or `ExactPosition` | Wrong layer — positions come via callbacks | `Fn(VertexId) -> Option<[f64; 3]>` |
| Raw `f64 < f64` for orientation | D3 firewall violation | [orient2d](file:///Users/spenstar/Documents/programming/Forge/crates/forge-spatial/src/classify/sos.rs#17-40)/[orient3d](file:///Users/spenstar/Documents/programming/Forge/crates/forge-spatial/src/classify/sos.rs#41-85) from `worth-math` |
| Face normal from `GeometryState::get_face_plane` | Wrong layer | Compute Newell normal from vertex positions |
| `unwrap()` in non-test code | Panic-free zone | `?` or `ok_or_else` |
| Integrity check silently fixes instead of reporting | Violates envelope contract | Return `TopologyError` variant |
| `unsafe` blocks | No justification in this crate | Remove |
