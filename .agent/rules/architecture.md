---
trigger: always_on
---

# Forge Kernel Architectural Rules

**Version:** 1.5 (Crate Structure Update)
**Status:** Mandatory — violations are structural defects, not style issues.

> See also: `CRATE_MAP.md` for the full type/import lookup table.

---

## 0. Purpose

These rules define mandatory architectural boundaries. All agents and developers
must comply. When in doubt, check `CRATE_MAP.md` for which crate owns what.

---

## 1. Layering Model (Strict Direction)

### 1.1 Dependency Direction

Dependencies are strictly unidirectional (bottom to top):

```
forge-math          ← pure math, no internal deps
  └─ forge-core     ← shared error/policy types
       ├─ forge-geom    ← stateless geometry solvers
       ├─ forge-signal  ← reactive dependency graph
       └─ forge-topo    ← topology arena + operators (depends on geom + core)
            └─ forge-kernel  ← policy engine, features (depends on core+geom+topo)
                 └─ forge-io     ← import/export
                      └─ forge-test  ← test harness (depends on everything)

forge-schema  ← declarative JSON schema (serde only, no kernel deps)
forge-view    ← trace viewer + CLI + representation (depends on forge-core)
```

### 1.2 Forbidden Coupling

- **DO:** Define shared types (errors, traits) in `forge-core`.
- **DO:** Use the Adapter Rule (§6) when a lower layer needs higher-layer info.
- **DON'T:** Add upward `use` statements or Cargo.toml dependencies.
- **DON'T:** Have `forge-geom` depend on `forge-topo`, or `forge-topo` on `forge-kernel`.

---

## 2. Layer Responsibilities

### 2.1 forge-math

**Scope:** Exact predicates, rational arithmetic, linear algebra.
**Structure:** `numeric/` (sign, rng), `arithmetic/`, `predicates/`, `linalg/`, `coincidence/`

- **DO:** Return `MathError` from all fallible functions.
- **DO:** Define `TriSign` and `CertifiedTriSign` in `numeric/sign.rs`.
- **DON'T:** Use `KernelError` (that lives in `forge-core`).
- **DON'T:** Define geometry types (`Plane`) or topology types (`FaceId`).

### 2.2 forge-core

**Scope:** Shared language — `KernelError`, `GeometrySource`, `PolicyResult<T>`, `PolicyKind`.

- **DO:** Put error variants, policy types, and data-access traits here.
- **DON'T:** Put business logic, arena, or geometry math here.

### 2.3 forge-geom

**Scope:** Stateless geometry solvers, intersection logic, constraint resolution.
**Structure:** `primitives/` (plane, ray, aabb, polygon, implicit_vertex), `spatial/` (bsp, bvh), `algorithms/`, `curve/`, `surface/`

- **DO:** Accept raw values (`[f64; 3]`, `&dyn GeometrySource`) and tolerance parameters.
- **DO:** Return `PolicyResult<T>` when a result is ambiguous.
- **DO:** Re-export commonly used types (`Plane`, `Aabb`, `BvhNode`) at the crate root.
- **DON'T:** Own an Arena or TopologyState.
- **DON'T:** Import `FaceId`, `VertexId`, or any topology handle.
- **DON'T:** Make policy decisions — signal `PolicyRequired`, let the kernel decide.

### 2.4 forge-topo

**Scope:** Arena management, generational handles, structural invariants, transactions.
**Structure:** `topology/` (handles, arena, state, operations/, queries/, integrity/, history/, tests/)

- **DO:** Call `forge-geom` functions for any floating-point geometry.
- **DO:** Drive topology decisions from `CertifiedTriSign` or `TriSign` values.
- **DO:** Store data in Global Modeling Space only.
- **DO:** All numeric comparisons in `forge-topo` must go through named helpers (`forge-math` predicates, `forge-geom` classification functions, or `CertifiedTriSign`).
- **DON'T:** Write **any** raw f64 comparisons (no `dist < EPS`, no `denom.abs() < 1e-30`). Code review rule: if you see `<` on an `f64` in `forge-topo`, it must be inside a named helper.
- **DO:** Enforce the 2-manifold invariant for solid shells at commit time via `validate_manifold_edges()`. Open shells may have boundary edges (valence 1). All shells reject valence > 2.
- **DON'T:** Hardcode tolerance constants — pass them as parameters from the kernel.
- **DON'T:** Apply Matrix4 transformations to the arena.

### 2.5 forge-kernel

**Scope:** ModelingContext, policy decisions, tolerance config, feature implementations.
**Structure:** `core/`, `features/`, `geometry_store/`, `mesh_builder/`, `analysis/`, `operations/` (boolean/, fillet/, extrude/, ...), `brep/`

- **DO:** Own `ToleranceConfig` (defined here) and pass individual `f64` threshold values down to geom/topo.
- **DO:** Log tolerance decisions via `check_tolerance!` macro (D2).
- **DO:** Implement `GeometrySource` adapters that bridge topo handles → geom values.
- **DO:** Place all modeling operations under `operations/` (boolean, fillet, extrude, etc.).

### 2.6 forge-signal

**Scope:** Reactive dependency graph for feature invalidation.
**Structure:** `handles.rs`, `schema.rs`, `graph.rs`, `evaluation/` (push.rs, pull.rs, context.rs)

- **DO:** Keep push/pull evaluation logic in `evaluation/`.
- **DON'T:** Import topology or geometry types.

---

## 3. Communication Rules (The "Provider" Pattern)

### 3.1 Anonymous Data Access

- **DO:** Define `trait GeometrySource` in `forge-core`.
- **DO:** Implement the trait in `forge-kernel` (or `forge-geom::PlaneSet` for tests).
- **DO:** Have topology resolve Handle → Index, then the adapter resolve Index → Value.
- **DON'T:** Pass entire `TopologyState` or `TopologyArena` to geometry functions.

### 3.2 Value-Only Communication

- **DO:** `fn solve(planes: &dyn GeometrySource, residual_tol: f64, degeneracy_tol: f64)`
- **DON'T:** `fn solve(topo: &TopologyState)`
- **DON'T:** `fn solve(planes: &dyn GeometrySource, config: &ToleranceConfig)` — lower crates cannot import `ToleranceConfig`; pass individual `f64` values.

---

## 4. Tolerance and Policy Rules (Doctrine D2)

### 4.1 No Hardcoded Globals

- **DO:** Put all numeric thresholds in `ToleranceConfig` (defined in `forge-kernel::core`; only `forge-kernel` may select/override presets).
- **DO:** Pass tolerance values as explicit `f64` function parameters to lower crates.
- **DON'T:** Write `const EPS: f64 = 1e-8` in `forge-geom` or `forge-topo`.
- **DON'T:** Write `if dist < 1e-6` anywhere outside `forge-kernel`.
- **DON'T:** Import `ToleranceConfig` in any crate below `forge-kernel`.

### 4.2 Ambiguity Escalation Flow

```
forge-geom:  Computes intersection → residual = 1e-9 → returns PolicyResult::Ambiguous
forge-kernel: Catches Ambiguous → checks ModelingContext → applies policy → logs decision
```

- **DO:** Return `PolicyResult::Ambiguous` from geometry when near a tolerance boundary.
- **DON'T:** Silently round, snap, or "forgive" in the geometry layer.

---

## 5. Determinism and Safety

### 5.1 Panic-Free Zone

- **DO:** Return `Result<T, KernelError>` (or `MathError` in forge-math).
- **DON'T:** Use `unwrap()`, `expect()`, or `panic!()` outside `#[cfg(test)]`.

### 5.2 Structural Hashing

- **DO:** Hash connectivity and lineage only.
- **DON'T:** Hash floating-point positions, timestamps, or memory addresses.

### 5.3 Collection Ordering

- **DON'T:** Rely on `HashMap`/`HashSet` iteration order for anything persisted, hashed, diffed, or used to drive topology outcomes.
- **DO:** Use `BTreeMap`/`BTreeSet` (or sort keys explicitly) when order matters.
- **DO:** If you must use hash maps for perf, require a deterministic strategy for any externally-visible ordering (sort-on-output).

---

## 6. The Adapter Rule

When a lower layer needs information from a higher layer:

1. **DO:** Extract the minimal value required.
2. **DO:** Pass it explicitly via function parameter.
3. **DO:** Abstract via a Trait defined in the lower layer if access is frequent.
4. **DO:** Adapters may cache derived values only if cache use is deterministic and does not change results (pure memoization).
5. **DON'T:** Introduce an upward `use` statement or Cargo.toml dependency.
6. **DON'T:** Let caching change tolerance decisions, ordering, or "first-hit wins" behavior.

---

## 7. Coordinate & Transformation Rules

- **DO:** Keep `forge-topo` in global coordinate space always.
- **DO:** Transform values before passing them to geometry solvers that need local space.
- **DON'T:** Store or apply Matrix4 transformations inside the topology arena.

---

## 8. Observability & Tracing

### 8.1 Automatic Trace Persistence

- `OperationResult::into_value()` auto-persists the `DecisionLog` to disk when `FORGE_TRACE_DIR` is set.
- This is gated by a `OnceLock` env check — zero cost when the env var is unset.
- Traces are panic-safe (wrapped in `catch_unwind`) and never crash tests.
- **DO:** Set `FORGE_TRACE_DIR` to an absolute path when running tests that should produce traces.
- **DON'T:** Manually wire `persist_trace` calls in test helpers — `into_value()` handles it.

### 8.2 Trace Inspection

- **GUI**: `cargo run -p forge-view --bin forge-trace-viewer <DIR>` — native egui app with live file watching, search, and tier filtering.
- **CLI**: `cargo run -p forge-view --bin forge-trace-cli -- <COMMAND> <DIR>` — progressive drill-down for AI agents.
  - `issues` — Show only traces with non-deterministic decisions (start here).
  - `list` — One-line summary per trace.
  - `show <ID>` — Span breakdown with timing and tier.
  - `decisions <ID> [SPAN]` — Individual decision details.
- **Workflow**: Use `/trace-drill-down` for the standard AI test-and-inspect flow.

### 8.3 Stderr Logging

- `FORGE_LOG` env var controls stderr output: `off`, `compact` (default), `full`.
- `forge_core::result::log_result()` formats the `DecisionLog` to stderr.
- Stderr logging is independent of trace persistence — both can run simultaneously.
