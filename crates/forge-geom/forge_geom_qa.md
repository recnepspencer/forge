# forge-geom QA Checklist

> forge-geom is a **pure math library**. It takes raw values, returns computed values, has no side effects.
> It does not participate in policy, tolerance config, transactions, or tracing.

---

## 1. Plane

Planes store **exact Rational coefficients** with cached f64 approximations. The rationals are the source of truth.

### Postconditions

| Function | Must Be True |
|----------|-------------|
| [from_rationals(a,b,c,d)](file:///Users/spenstar/Documents/programming/Forge/crates/forge-geom/src/primitives/plane/mod.rs#43-76) | Rejects [(0,0,0,d)](file:///Users/spenstar/Documents/programming/Forge/crates/forge-topo/src/topology/history/lineage.rs#39-46). Cached f64 normal is unit-length. |
| [try_from_f64(n, d)](file:///Users/spenstar/Documents/programming/Forge/crates/forge-geom/src/primitives/plane/mod.rs#98-119) | Every finite f64 has exact Rational repr → **zero precision loss**. Rejects NaN/Inf. |
| [from_point_normal(p, n)](file:///Users/spenstar/Documents/programming/Forge/crates/forge-geom/src/primitives/plane/mod.rs#120-133) | Offset `d = -(n·p)` computed in **rational** arithmetic. |
| [classify_point(plane, pt)](file:///Users/spenstar/Documents/programming/Forge/crates/forge-geom/src/primitives/plane/eval.rs#15-26) | Returns `CertifiedTriSign` via [orient3d](file:///Users/spenstar/Documents/programming/Forge/crates/forge-geom/src/primitives/implicit_vertex.rs#78-117). If it says `Pos`/`Neg`, the answer is mathematically provable. |
| [classify_point_exact(plane, pt)](file:///Users/spenstar/Documents/programming/Forge/crates/forge-geom/src/primitives/plane/eval.rs#27-36) | Pure rational [sign(a*px + b*py + c*pz + d)](file:///Users/spenstar/Documents/programming/Forge/crates/forge-topo/src/topology/history/replay.rs#57-61). No floats. Period. |
| [signed_distance(plane, pt)](file:///Users/spenstar/Documents/programming/Forge/crates/forge-geom/src/primitives/plane/eval.rs#46-54) | f64 approximation. **Must NOT drive topology decisions.** |
| [intersect_three_planes(p0,p1,p2, degeneracy)](file:///Users/spenstar/Documents/programming/Forge/crates/forge-geom/src/primitives/plane/eval.rs#55-94) | `|det| < degeneracy` → `Err`. |
| [intersect_three_planes_exact](file:///Users/spenstar/Documents/programming/Forge/crates/forge-geom/src/primitives/plane/eval.rs#95-128) | Rational Cramer's rule. Result satisfies each plane equation **exactly**. |
| [exact_eq(a, b)](file:///Users/spenstar/Documents/programming/Forge/crates/forge-geom/src/primitives/plane/eval.rs#160-203) | Same half-space. Checks all 6 cross-products of 4 coefficients. Handles zero-anchor correctly. |
| [coplanar_eq(a, b)](file:///Users/spenstar/Documents/programming/Forge/crates/forge-geom/src/primitives/plane/eval.rs#255-288) | Same geometric surface, **either** normal direction. For boolean merge decisions. |
| [are_parallel_exact](file:///Users/spenstar/Documents/programming/Forge/crates/forge-geom/src/primitives/plane/eval.rs#289-306) | Rational cross product `n₁ × n₂ == 0`. No tolerance. |
| [intersect_edge_plane](file:///Users/spenstar/Documents/programming/Forge/crates/forge-geom/src/primitives/plane/eval.rs#129-159) | Near-degenerate → midpoint, never NaN. |

### Edge cases that need test coverage

- Axis-aligned planes (zero coefficients breaking anchor selection in [exact_eq](file:///Users/spenstar/Documents/programming/Forge/crates/forge-geom/src/primitives/plane/eval.rs#160-203))
- Near-parallel planes in [intersect_three_planes](file:///Users/spenstar/Documents/programming/Forge/crates/forge-geom/src/primitives/plane/eval.rs#55-94) (condition number)
- Anti-parallel normals in [coplanar_eq](file:///Users/spenstar/Documents/programming/Forge/crates/forge-geom/src/primitives/plane/eval.rs#255-288)
- Point exactly on plane returning certified `Zero`

---

## 2. VertexGeom — Tolerance Propagation

### Invariants

| Rule | Why |
|------|-----|
| **Tolerance ≥ 0** | Every constructor enforces. |
| **Coalescence: RSS** | `√(a² + b²)`. Always ≥ max(a,b). Tolerance **never shrinks**. |
| **Split: inherits max** | [max(origin, target)](file:///Users/spenstar/Documents/programming/Forge/crates/forge-geom/src/primitives/vertex_geom.rs#237-244). No worse than worst parent. |
| **Provenance required** | `VertexProvenance` on every vertex. No silent `Unknown`. |
| **Regime tracks trust** | `Modeled` / `Healed` / `Imported`. Downstream knows how trustworthy. |

---

## 3. Implicit Vertex (Symbolic)

| Function | Must Be True |
|----------|-------------|
| [orient3d_symbolic](file:///Users/spenstar/Documents/programming/Forge/crates/forge-geom/src/primitives/implicit_vertex.rs#78-117) | Rational 4×4 determinant → `TriSign`. **Exact.** |
| [resolve_position](file:///Users/spenstar/Documents/programming/Forge/crates/forge-geom/src/primitives/implicit_vertex.rs#167-187) | f64 via [intersect_three_planes](file:///Users/spenstar/Documents/programming/Forge/crates/forge-geom/src/primitives/plane/eval.rs#55-94). **Export/debug only** — never for classification. |
| [select_best_triple](file:///Users/spenstar/Documents/programming/Forge/crates/forge-geom/src/primitives/implicit_vertex.rs#187-224) | From N planes, picks highest `|det3|`. Minimizes condition number. |

**Key invariant:** Symbolic vertices **never store a position**. Position is derived on demand. Topology decisions through symbolic vertices are always exact.

> **Note:** [implicit_vertex.rs](file:///Users/spenstar/Documents/programming/Forge/crates/forge-geom/src/primitives/implicit_vertex.rs) uses `&impl GeometrySource` (forge-math trait) as a plane lookup table. This works but is arguably over-abstracted — a `&[Plane]` would be simpler. Not broken, but worth noting.

---

## 4. Shapes (Plane Generators)

- All return `Result<Vec<Plane>, MathError>` — never silent garbage
- Each [Plane](file:///Users/spenstar/Documents/programming/Forge/crates/forge-geom/src/primitives/plane/mod.rs#30-41) has non-zero normal (enforced by `Plane::from_point_normal`)
- Counts are deterministic: cube=6, tet=4, dodec=12, block=6, prism=n+2, pyramid=n+1, wedge=6
- Normals point outward (interior point has negative [signed_distance](file:///Users/spenstar/Documents/programming/Forge/crates/forge-geom/src/primitives/plane/eval.rs#46-54) to all planes)
- NaN/Inf inputs → `Err` from [Plane](file:///Users/spenstar/Documents/programming/Forge/crates/forge-geom/src/primitives/plane/mod.rs#30-41) constructors
- Dimension validation (zero/negative) is the **caller's** job (`mesh_builder::eval`)

---

## 5. Spatial Structures (BSP, BVH)

**BSP:** Deterministic build. Classifies any point as inside/outside — no "unknown" state. `PlaneSet` wraps `Vec<Plane>` as a `GeometrySource` for tests.

**BVH:** SAH-driven AABB tree. No false negatives in overlap queries. False positives are fine — it's a filter, not a certifier.

**Edge Match:** `fuzzy_match_edges` tolerance is an explicit param. Matched pairs are symmetrical. Unmatched edges are reported, not dropped.

---

## 6. Surface, Curve, Coedge (Phase 4+)

| Type | Postcondition |
|------|---------------|
| `SurfaceKind` | Plane, Cylinder, Cone, Sphere, Torus. Closed-form `point_at`/`normal_at` for analytics. |
| `SurfaceRelation::Undetermined` | Means "I can't decide" — it's a math result, not a policy action. Caller handles escalation. |
| `CurveGeom.tolerance` | Analytic = 0. SSI = solver residual. Never negative. |
| `SpCurveApproximation.error_bound` | Certified max deviation from true curve. |
| `Coedge` | UV-space anchor. Prevents drift across chained booleans. |

### What expands
- `SurfaceKind::Nurbs` (Phase 7) + numerical evaluation
- More `CurveKind` variants for fillets/NURBS
- More `classify_surface_pair` cases (cylinder-cylinder, etc.)
- Curved edge matching, curved boundary certification
