---
description: Workflow for implementing a proof system milestone from PROOF_SYSTEM.md
---

# Add Proof Milestone Workflow

Use this workflow to implement any milestone (P0.x–P4.x) from `PROOF_SYSTEM.md`.
Reference `DEPENDENCY_ROADMAP.md` for crate placement and dependency impact.

> **Before writing any code**, read `CRATE_MAP.md` to know which crate owns
> which abstractions, and `DEPENDENCY_ROADMAP.md` for the target crate graph.

## Step 1: Read the Milestone Spec

1. Open `PROOF_SYSTEM.md` and locate the milestone (e.g., P0.1, P1.2, P2.1).
2. Extract:
   - **Goal**: One sentence describing what the milestone proves.
   - **Crate(s)**: Which crates are touched (listed in the milestone header).
   - **PV suites**: Which `PV-xx` tests define acceptance.
   - **MB tests**: Which `MB-*` tests stress this milestone.
   - **Dependencies**: What prior milestones or infrastructure must exist.

## Step 2: Check Dependency Readiness

1. Open `DEPENDENCY_ROADMAP.md` and find the phase containing this milestone.
2. Verify:
   - [ ] "What's already fine" items exist in the codebase.
   - [ ] No new Cargo.toml dependencies are needed (or add them if flagged).
   - [ ] Prior milestones this one depends on are complete.

## Step 3: Identify Crate Placement

Use the "I Need To…" table in `CRATE_MAP.md`. Common placements:

| Milestone Type                   | Primary Crate                 | Pattern                                    |
| -------------------------------- | ----------------------------- | ------------------------------------------ |
| Invariant check (P0.x)           | `forge-topo`                  | Extend `integrity/validate.rs`             |
| Geometry solver (P1.2, P4.3)     | `worth-geom`                  | New module in `algorithms/` or `spatial/`  |
| Math primitive (P2.1)            | `worth-math`                  | New module in `arithmetic/` or `numeric/`  |
| Policy/config (P0.5, P2.2)       | `forge-kernel`                | Extend `core/` (context, tolerance)        |
| Proof orchestration (P1.1, P1.3) | `forge-kernel`                | New module in `operations/` or `analysis/` |
| Replay/causal (P3.x)             | `forge-kernel` + `forge-core` | Extend `tracing/` and `envelope/`          |
| Test infrastructure (P4.x)       | `forge-test`                  | New module in `harness/` or `generators/`  |

## Step 4: Create the Bento Box

Follow `/new-module` if creating a new directory. Standard proof milestone files:

- `schema.rs` — Data shapes: result structs, config enums, report types.
- `eval.rs` — Pure computation: the solver, checker, or classifier.
- `tests.rs` — PV suite implementations (one `#[test]` per PV-xx).

Not every milestone needs all three. A validator extension (P0.1) might only
add functions to an existing `validate.rs`. An `Interval` type (P2.1) needs
`schema.rs` + `eval.rs` + `tests.rs` in `worth-math`.

## Step 5: Implement the PV Suites

Each PV suite is a `#[test]` function named after its PV number:

```rust
/// PV-01: Zero-area face injected into a valid cube → validator detects it.
#[test]
fn pv_01_zero_area_face_detection() {
    // 1. Build valid topology
    // 2. Inject the defect
    // 3. Run the validator
    // 4. Assert the specific failure is detected
}
```

**Naming convention**: `pv_XX_snake_case_description`

**Rules**:

- Each PV test must be straight-line code (no loops/conditionals).
- Each PV test must assert a specific, named failure mode — not just "it failed."
- PV tests for the same milestone live in the same `tests.rs`.

## Step 6: Run Applicable MB Tests

If the milestone is the final milestone in a phase (P0.6, P1.5, P2.5, etc.),
implement the corresponding MB test series. Otherwise, verify existing MB tests
still pass.

// turbo

```bash
cargo test -p forge-kernel -- --test-threads=1 2>&1 | tail -20
```

**MB test naming convention**: `mb_t1_500_step_boolean_chain`, `mb_d3_near_tangent_gap`

## Step 7: Architecture Compliance Checklist

Verify every item before considering the milestone complete:

### Layering

- [ ] No upward dependencies added
- [ ] All shared types come from `forge-core`
- [ ] `GeometrySource` trait comes from `worth-math::data_access`
- [ ] Lower crates receive individual `f64` tolerance values, never `ToleranceConfig`

### Proof Doctrines

- [ ] **P0 (Independence)**: Proof layer shares no mutable state with other layers
- [ ] **P2 (Monotonic Corpus)**: New test cases are additive, none removed
- [ ] **P3 (Quantifiable)**: Proof output includes numeric confidence/margin, not just pass/fail
- [ ] **P4 (Observable)**: Results are machine-readable, consumable within token budget
- [ ] **P5 (Deterministic Fuel)**: No `Instant::now()` branching; iterative algorithms use fuel counters
- [ ] **P6 (Generational Handles)**: No raw index types introduced

### Safety

- [ ] Zero `unwrap()` / `expect()` outside `#[cfg(test)]`
- [ ] All mutations through `MutableDraft`
- [ ] All fallible functions return `Result<T, KernelError>` (or `MathError` in worth-math)

### Verification

- [ ] `cargo check --workspace` clean
- [ ] `cargo test --workspace` — no regressions
- [ ] `cargo clippy --workspace -- -D warnings` clean

### Trace Verification

- [ ] Run `/testing-and-tracing` to inspect kernel decisions
- [ ] `forge-trace-cli issues` reports zero unexpected decisions
