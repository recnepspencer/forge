# Domain Architecture & Structural Standards

## Purpose

This document defines the structural rules for introducing new domains and subsystems into the codebase.
These standards exist to preserve:

- Architectural clarity
- Long-term maintainability
- Low coupling
- High cohesion
- Domain alignment

> **Structural discipline is not optional.** It is a prerequisite for building a system that can scale in size, contributors, and lifespan.

> **When principles conflict:** Domain alignment and single responsibility take precedence over layer templates or structural convenience. Never sacrifice clarity for template compliance. When uncertain whether something should be one file or two, default to two. Merging files later is trivial. Untangling responsibilities that grew together is surgery.
> **Make this assumption:** if you are working on a long term roadmap, assume the file you are working will continue to grow. So organize your directories with future proofing in mind, not with an, "I'll refactor it later if it grows" mentality.

---

## 1. Organize by Components and Subsystems

### Principle

The top-level structure of the system must be organized around components, not file types.
A component is a cohesive subsystem responsible for a clearly defined functional capability. It should be conceptually understandable and evolvable in isolation.

### Requirements

- The root directory must be divided by domain components.
- Each component folder must contain all code necessary for that subsystem.
- Cross-cutting layers (e.g., global `models/`, `controllers/`, `helpers/`) are prohibited unless they represent true application-wide infrastructure.

### Rationale

Organizing by file type creates horizontal coupling. Organizing by component preserves vertical slices aligned with business capabilities.

---

## 2. Single Responsibility — The Most Important Rule

### Principle

Every module, file, and folder must represent a single, well-defined responsibility.
The question to ask is **not** _"are these related enough to live together?"_ — it is:

> **"Do these change for different reasons, fail independently, or get tested separately?"**

If the answer is **yes** to _any_ of those, they are separate responsibilities and must be separated — into separate files, and likely separate folders.

### The Decomposition Bias

When uncertain, **default to more decomposition, not less.** The instinct to consolidate is almost always wrong at scale. Ask _"should this be separate?"_ rather than _"can I fit this here?"_

### Anti-Pattern: "Category-as-Responsibility"

> _It's all validation, so it's one file. Right?_

```
topology/
  validation.rs                 ← radial edge consistency, euler genus checks,
                                   loop wiring, reference integrity, shell closure...
                                   all in one file because "it's all validation"
```

This looks reasonable at first glance. But radial edge consistency and Euler genus checking have nothing to do with each other. They check different invariants, fail for different reasons, and are tested independently. The _category label_ "validation" is not a responsibility — it's a filing cabinet.

### Correct: Responsibilities as Subdomains

```
topology/
  validation/
    euler_genus/
      genus.rs
      per_component_euler.rs
    loop_wiring/
      loop_closure.rs
      edge_endpoints.rs
      face_membership.rs
    radial_edge/
      ring_closure.rs
      edge_consistency.rs
      neighbor_consistency.rs
    reference_integrity/
      dangling_refs.rs
      bidirectional_links.rs
      orphan_half_edges.rs
    shell_closure/
```

Each invariant system gets its own folder because each is independently authored, tested, and debugged. The structure _teaches you the domain_ instead of hiding it behind a label.

### Anti-Pattern: "The Junior Dev Special"

> _Everything fits if you squint hard enough:_

```
topology/
  logic/
    euler_ops.rs                ← 4,000 lines of make/kill pairs
    traversal.rs                ← adjacency, boundary walks, hierarchy... all in one file
    validation.rs               ← every invariant check the system has, in a single file
```

### Correct: Depth Follows Complexity

```
topology/
  operations/
    entity_lifecycle/
      make_edge_vertex.rs
      kill_edge_vertex.rs
      make_face_vertex.rs
      split_edge.rs
    boundary_editing/
      join_faces.rs
      make_edge_kill_loop.rs
      kill_edge_make_loop.rs
    algorithms/
      simplify/
        cleanup.rs
      region_extraction.rs
      triangulate.rs
  traversal/
    adjacency.rs
    boundary.rs
    hierarchy.rs
  validation/
    ...
```

Operations are organized by _what kind of mutation_ they perform. Entity lifecycle (make/kill pairs), boundary editing (reshape without changing element counts), and algorithms are deeply different categories — different preconditions, different test strategies, different failure modes.

### Anti-Pattern: "The Flat Pretender"

> _The right names appear, but nothing is broken down:_

```
durability/
  checkpoints.rs                ← creation, listing, diffing, pruning... one file to rule them all
  recovery.rs                   ← planning AND execution in one file
  storage.rs                    ← local store, segments, images... all flattened
```

### Correct: Each Subdomain Gets Its Own Space

```
durability/
  checkpoints/
    lifecycle.rs                ← create, list, prune
    images.rs                   ← serialized state snapshots
    diff.rs                     ← comparing checkpoint states
  recovery/
    planning.rs                 ← deciding what needs to be recovered
    execution.rs                ← performing the recovery
  storage/
    local_store.rs
    segments.rs
  facade.rs
```

Recovery planning runs at startup; execution runs at runtime. Different lifecycle, different dependencies, different tests. They are separate responsibilities even though they are both "recovery."

---

## 3. Maintain Complete Domain Alignment

### Principle

The structure of the codebase must mirror the real-world problem domain.
Folder and file names must represent business concepts, not technical mechanics.

| Correct Examples | Incorrect Examples |
| :--------------- | :----------------- |
| Bodies           | Helpers            |
| Regions          | Managers           |
| Transactions     | Stuff              |
| Inventory        | Common             |
| Certification    | StringManipulators |

> **Goal:** If a subject matter expert unfamiliar with the implementation examines the directory structure, they should recognize domain concepts.

### Prohibited Patterns

- "misc" or "other" folders
- Generic "utils" folders that mix unrelated functionality
- Files named with conjunctions (e.g., `AuthAndRouting`)
- Multi-purpose modules that manage unrelated behaviors

### Enforcement Standard

If a name cannot be expressed as a singular, precise noun, the construct is likely not cohesive.

---

## 4. Separation of Concerns

### Principle

Within each component, responsibilities must be separated into distinct conceptual layers. The common layers are:

- **Boundary** — External interfaces, API surfaces, adapters, serialization
- **Core** — Domain rules, orchestration, use cases
- **Storage** — Persistence models, storage abstractions, schema definitions

### How to Apply

These layers are a _thinking tool_, not a folder template. Use them to audit whether concerns are properly separated, then name your folders after the domain concepts they contain. Not every component has all three layers. A component with no external API surface does not need a boundary layer. A component with no persistence does not need a storage layer. **Do not create folders to satisfy a template — create them to reflect genuine, distinct responsibilities.**

### Constraints

- Boundary concerns must not contain business logic.
- Core domain rules must not contain persistence details.
- Storage must not contain orchestration behavior.
- Cross-layer imports should flow inward, not cyclically.

---

## 5. Use Intent-Revealing Names

### Principle

Names are a navigational system. Imprecise names are structural debt.

### Standards

- Classes and files must be named as singular, descriptive nouns.
- Names must reflect domain meaning, not implementation detail.
- Avoid abbreviations unless they are industry-standard and unambiguous.
- Avoid conjunctions in names; they signal multiple responsibilities.
- Prefer accessible, intention-revealing names over compressed or overly technical names.
- If a longer name is easier for a smart generalist to understand on first read, prefer the longer name.

### Avoid Mechanics-First Names

Names should describe domain responsibilities, not implementation motions.

- Verbs and pipeline labels are suspicious by default.
- `apply`, `process`, `handle`, `run`, `manager`, `util`, and `helper` are common warning signs.
- Broad labels like `world`, `actions`, `workflows`, and `invariants` are also suspicious unless they are truly singular responsibilities.

Bad:

    apply.rs
    process.rs
    handle.rs
    manager.rs

Better:

    trade_corrections.rs
    checkpoint_images.rs
    branch_history.rs
    risk_limits.rs

### API Naming Guidance

Public API names should optimize for first-read comprehension, not technical elegance.

Examples:

- Prefer `depends_on_aspects` over a shorter but less obvious alternative.
- Prefer `condition` over jargon that hides what the field means.
- Prefer names that teach the model directly instead of requiring prior framework knowledge.

---

## 6. Restrict Scope via Packages and Namespaces

### Principle

Scope must reflect utility. Constructs should only be visible at the level where they are required.

### Requirements

- Treat folders as architectural boundaries.
- Default to internal visibility.
- Only expose what external components must use.
- Do not elevate modules to root scope unless the entire application depends on them.

---

## 7. Tests Follow the Same Rules

### Principle

Test code is not exempt from structural discipline.
Tests should teach the domain just as clearly as production code.

### Requirements

- Test directories must follow the same component and responsibility rules as production code.
- Folders like `helpers`, `fixtures`, `actions`, `workflows`, and `invariants` are not automatically valid responsibilities.
- If setup, assertions, mutations, and scenarios change or fail independently, they must be split.
- Shared test support is only acceptable when it represents true shared testing infrastructure.

Bad:

    tests/domains/fintech/world.rs
    tests/domains/fintech/actions.rs
    tests/domains/fintech/workflows.rs

Better:

    tests/domains/fintech/trades/booking.rs
    tests/domains/fintech/trades/corrections.rs
    tests/domains/fintech/market/shocks.rs
    tests/domains/fintech/recovery/checkpoint_rebuild.rs

---

## 8. Minimize Coupling Through Façades

### Principle

Components must communicate through controlled entry points.
Direct access into deep internal modules of another component is prohibited.

### Required Pattern

Each component must expose a single public façade file at its root:

    component/
      facade.rs

- External components must depend **only** on this façade.
- Internal complexity must remain hidden.

### Rationale

Façades prevent cross-component entanglement and preserve replaceability of subsystems.

---

## 9. Functions Must Be Composable Responsibilities

### Principle

The same responsibility rules that govern folders and files also govern functions.
A function should represent one semantic step, not an entire workflow.

### Requirements

- A function should do one thing.
- If a function loads state, computes values, validates, mutates storage, records diagnostics, and formats output, it is almost certainly too broad.
- Inline math should usually be extracted behind intention-revealing names.
- Orchestration is acceptable, but orchestration must call smaller responsibility-specific functions.

### Example

Bad:

    fn correct_trade(...) {
        // load account and trade state
        // compute risk delta inline
        // check limit conditions inline
        // mutate balances
        // record audit entries
        // return formatted output
    }

Good:

    fn correct_trade(...) {
        let state = load_trade_state(...);
        let delta = calculate_risk_delta(&state);
        validate_limit_window(&state, delta)?;
        apply_trade_correction(&state, delta)?;
        record_audit_entry(&state, delta)?;
        build_correction_summary(&state, delta)
    }

The goal is not small functions for their own sake.
The goal is that each function has one reason to change, and that math and mechanics are hidden behind domain intent.
