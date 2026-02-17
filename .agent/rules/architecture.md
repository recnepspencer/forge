---
trigger: always_on
---

This is the "Gold Standard" version of your architecture.md. I have integrated the Provider/Source patterns, the Policy Escalation logic, and the Coordinate Purity rules we discussed.

This document serves as the mandatory specification for the Forge Kernel.

Forge Kernel Architectural Rules
Version: 1.1 (Transactional & Provider Pattern Update)

Status: Mandatory

0. Purpose
These rules define mandatory architectural boundaries for the Forge kernel. Violations are structural defects, not stylistic issues. All agents and developers must comply to ensure long-term stability for Phase 2 (Booleans) and Phase 3 (Curved Surfaces).

1. Layering Model (Strict Direction)
1.1 Dependency Direction
Dependencies must be strictly unidirectional. A crate may only depend on crates below it in this stack:

forge-math: The Foundation (No internal dependencies).

forge-geom: The Stateless Solver (Depends on math).

forge-topo: The Data Skeleton (Depends on geom + math).

forge-kernel: The Policy Engine (Depends on topo + geom + math).

forge-io / forge-test: The Perimeter (Depends on everything below).

1.2 Forbidden Coupling
No Upward Dependencies: forge-geom must NOT depend on forge-topo. forge-topo must NOT depend on kernel.

Circular Prevention: If a feature requires upward knowledge, the abstraction is incorrect. Use the Adapter Rule (Section 6).

2. Layer Responsibilities
2.1 forge-math
Scope: Pure numerical routines, deterministic predicates (orient3d), linear algebra, exact arithmetic wrappers.

Hard Invariants: Must NOT contain geometry types (Plane), topology types (FaceId), or Policy logic.

Determinism: All functions must be bit-level deterministic (D1).

2.2 forge-geom
Scope: Geometric invariants, intersection logic, constraint resolution, and the GeometrySource trait definitions.

Invariants: Geometry is stateless. It accepts raw values or abstract providers. It never owns an Arena or a Registry.

Error Reporting: Returns geometric residuals. It signals PolicyRequired, but never decides the policy itself.

2.3 forge-topo
Scope: Arena management, generational handles, structural invariants (Euler formula), and transaction logic (MutableDraft).

Geometry Firewall (D3): Topology calls geometry solvers by passing raw data. It never performs its own floating-point calculations (e.g., no dist < EPS in forge-topo).

Coordinate Purity: Stores data only in Global Modeling Space.

2.4 forge-kernel
Scope: The ModelingContext, policy decisions, tolerance configuration, and the "Adapter" implementations that bridge topo and geom.

Responsibility: The only layer allowed to "log" a decision or "forgive" a geometric error based on user settings.

3. Communication Rules (The "Provider" Pattern)
3.1 Anonymous Data Access
Lower layers (like geom) should never see higher-layer containers.

The Source Trait: forge-geom defines trait GeometrySource.

The Adapter: The kernel implements this trait.

Resolution: Topology resolves a Handle (FaceId) to a Geometry Index (PlaneRef). The Adapter resolves that index to a Value (&Plane).

3.2 Value-Only Communication
Avoid passing entire states. Pass the minimal required values:

Forbidden: fn solve(topo: &TopologyState)

Allowed: fn solve(planes: &dyn GeometrySource, config: &ToleranceConfig)

4. Tolerance and Policy Rules (Doctrine D2)
4.1 No Hardcoded Globals
forge-geom and forge-topo must contain ZERO hardcoded const EPS: f64. All thresholds must flow from the ToleranceConfig owned by the kernel.

4.2 Ambiguity Escalation
Geometry Layer calculates an intersection and finds a residual of 1e-9.

Geometry Layer sees this is below the residual_tolerance but is "ambiguous."

Geometry Layer returns KernelError::PolicyRequired { residual, location }.

Kernel Layer catches this, checks ModelingContext, and applies the policy (e.g., "Merge Vertices").

5. Determinism and Safety
5.1 Panic-Free Zone
Non-test code must not contain unwrap(), expect(), or panic!(). All failures (including topological violations) must return a structured KernelError.

5.2 Structural Hashing
The topology_hash must reflect connectivity and lineage only. It is strictly forbidden from depending on floating-point positions, timestamps, or transient memory addresses.

6. The Adapter Rule (Rule 6)
If a lower layer requires information from a higher layer:

Extract the minimal value required.

Pass it explicitly via function parameter.

Abstract via a Trait defined in the Lower Layer if access is frequent.

Never introduce an upward use statement or Cargo.toml dependency.

7. Coordinate & Transformation Rules
7.1 Global Purity
forge-topo always operates in the global coordinate system. Local-to-Global transformations are the responsibility of the kernel or io layers.

7.2 Transformation Logic
Geometry solvers that require local space (e.g., 2D sketching) must be passed transformed values. forge-topo must not own or apply Matrix4 transformations to its arena.