# Forge Runtime Bridge DX Plan

## Purpose

This is the execution plan for hardening `forge-runtime-bridge` before the main
Milestone 13 implementation push.

This is not a publication vanity pass.

It is a pre-certification boundary-hardening program with one specific job:

- make the bridge boundary deliberate enough that Milestone 13 can be proven
  through real end-to-end and integration tests rather than through fixture-only
  seam stitching and facade-adjacent folklore

The bridge is in a different state from `forge-signal` and `forge-relational`.

Those crates already have:

- explicit DX plans
- canonical surface specs
- boundary specs
- layered daily-use vs specialist thinking
- compatibility planning

The bridge currently does not.

If we push straight into the reference workload and certification matrix without
this pass, we risk freezing today’s subsystem-shaped facade and ad hoc harness
usage into tomorrow’s test contract.

---

## Inputs

This plan builds on:

- [`_docs/forge-runtime-bridge/forge_runtime_bridge_roadmap.md`](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-runtime-bridge/forge_runtime_bridge_roadmap.md)
- [`_docs/forge-runtime-bridge/forge_runtime_bridge_vision.md`](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-runtime-bridge/forge_runtime_bridge_vision.md)
- [`_docs/forge-runtime-bridge/test-requirements.md`](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-runtime-bridge/test-requirements.md)
- [`_docs/forge-runtime-bridge/milestone-13.md`](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-runtime-bridge/milestone-13.md)
- [`_docs/forge_signal/dx_plan.md`](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge_signal/dx_plan.md)
- [`_docs/forge_signal/dx_boundary_spec.md`](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge_signal/dx_boundary_spec.md)
- [`_docs/forge-relational/dx_plan.md`](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-relational/dx_plan.md)
- [`_docs/forge-relational/dx_boundary_spec.md`](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-relational/dx_boundary_spec.md)
- [`crates/forge-runtime-bridge/src/lib.rs`](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-runtime-bridge/src/lib.rs)
- [`crates/forge-runtime-bridge/src/facade.rs`](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-runtime-bridge/src/facade.rs)

---

## Adversarial Constraint

The bridge DX hardening pass must survive this hostile condition:

> A new engineer or AI agent must be able to build a bridge, route truth
> changes, read from a chosen truth view, open a speculative session, discard or
> promote it, and inspect the resulting bridge artifacts through one obvious
> public boundary without needing to understand the internal subsystem tree,
> without stitching together raw milestone-local types by guesswork, and
> without reaching for test-only helpers. If the same user can accidentally take
> three equally plausible public paths to the same job, the bridge boundary is
> not hardened enough for Milestone 13 certification.

This constraint is load-bearing because Milestone 13’s reference workload must
be expressed through the real bridge boundary.

If the workload can only be expressed by bypassing that boundary, the tests are
not product-honest.

---

## North Star

After this DX pass, the bridge should feel like this:

- one obvious construction path
- one obvious routing path
- one obvious truth-view evaluation path
- one obvious speculative-session path
- one obvious discard or promote path
- one obvious diagnostics entrypoint
- specialist surfaces remain available, but they do not dominate everyday
  bridge work
- the harness proves the public bridge model instead of compensating for its
  ambiguity

The dominant mental model should be:

- causal protocol boundary between authoritative truth and derived computation

Not:

- a giant export inventory
- a bag of milestone nouns
- a harness substrate with a public shell

---

## Non-Negotiable Standards

1. Do not let test fixtures define the bridge product boundary.
2. Do not let host adapters define the bridge product boundary.
3. Do not keep giant subsystem export walls as a substitute for design.
4. Do not make users memorize milestone-local taxonomies to perform ordinary
   bridge jobs.
5. Do not let certification-only bundle or record nouns dominate day-one usage.
6. Do not harden the API cosmetically while leaving the actual job flows
   fragmented.

---

## Completion Criteria

This DX work is only done when all of the following are true:

- the bridge has one explicit canonical surface spec
- the bridge has one explicit boundary spec
- the public boundary has one obvious path for each primary bridge job
- specialist and certification-heavy surfaces are contained rather than
  dominating the first impression
- Milestone 13’s pricing-shock reference workload can be written against the
  intended public bridge boundary rather than raw seam stitching
- the diagnostics entrypoint story is coherent enough that bundle capture and
  inspection do not require separate subsystem-specific entry guesses
- bridge integration tests can target the intended facade, not internal modules

In addition, these design artifacts must exist and stay current:

- bridge DX canonical surface spec
- bridge DX boundary spec
- this bridge DX plan

---

## Current DX Reality

The bridge has real architectural strength and weak boundary shape.

Working in its favor:

- the crate is deeply decomposed by subsystem
- the public surface does flow through one facade file
- harness code is test-only in `lib.rs`
- most major protocol domains now exist as named subdomains

Not yet productized:

- the facade is still primarily a large re-export wall
- there is no written canonical memory shape
- there is no daily-use vs specialist boundary policy
- there is no explicit product story for diagnostics entry
- there is no explicit decision about what Milestone 13 should call through the
  public boundary versus what remains certification substrate

This is exactly the point where a boundary-hardening pass is highest leverage.

---

## Canonical Daily-Use Jobs

The bridge boundary must optimize for these jobs first:

1. build the bridge
2. deliver or route truth changes into invalidation
3. evaluate against a chosen truth view
4. open a speculative session
5. discard or promote speculative outcomes
6. inspect what happened through one diagnostics door

If those jobs are not smooth, Milestone 13 implementation should not be treated
as the next step.

---

## Canonical Public Layers

### Layer 1: Everyday Bridge Operations

This layer should define the bridge product.

Required characteristics:

- short import story
- obvious construction path
- obvious route/evaluate/speculate verbs
- one diagnostics door nearby

### Layer 2: Advanced Runtime Control

This layer is for users who need:

- history-aware evaluation
- policy refinement
- bulk planning
- structural comparison
- merge-aware flows
- writeback family selection

Required characteristics:

- explicit
- still coherent
- discoverable after Layer 1, not before it

### Layer 3: Certification And Specialist Infrastructure

This layer is for:

- certification bundle assembly
- replay and forensic inspection
- family-level writeback detail
- structural and merge proof surfaces
- host-adapter authoring

Required characteristics:

- real
- public where needed
- clearly specialist
- not the first thing ordinary bridge callers learn

### Layer 4: Internal Support

This should not define the public product boundary.

Includes:

- test-only harness substrate
- support-only helpers
- internal stitching artifacts

---

## Phase 0: Define The Canonical Surface

### Goal

Stop treating the current facade export list as the product boundary.

### Required Deliverable

- [`_docs/forge-runtime-bridge/dx_canonical_surface_spec.md`](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-runtime-bridge/dx_canonical_surface_spec.md)

### It Must Define

- the one obvious import path
- the one obvious bridge setup flow
- the one obvious truth-routing flow
- the one obvious truth-view evaluation flow
- the one obvious speculative-session flow
- the one obvious discard or promote flow
- the one obvious diagnostics entry flow

### Exit Criteria

- we have a positive target public shape rather than only a complaint about the
  current facade

---

## Phase 1: Define The Bridge Boundary

### Goal

Write down what is primary, what is advanced, what is specialist, and what is
not allowed to define the product.

### Required Deliverable

- [`_docs/forge-runtime-bridge/dx_boundary_spec.md`](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-runtime-bridge/dx_boundary_spec.md)

### Tasks

- define the intended role of `RuntimeBridge`
- define the intended role of `RuntimeBridgeBuilder`
- define the intended place of diagnostics
- define which writeback, merge, structural, and stream surfaces remain
  specialist
- define what Milestone 13 harness code is allowed to call directly

### Exit Criteria

- the bridge has an explicit boundary target strong enough to guide real API
  cleanup and test design

---

## Phase 2: Harden Only The Surfaces Milestone 13 Needs

### Goal

Do the minimum real boundary hardening necessary for honest end-to-end testing.

### Required Output

The bridge must present clean, guided, named entry flows for:

- setup
- routing
- evaluation
- speculation
- promotion or discard
- diagnostics

### Tasks

- condense high-ceremony multi-step flows into request, session, or builder
  objects where the current API forces memory-based sequencing
- keep raw specialist controls where architecturally legitimate
- avoid broad facade churn that does not improve the Milestone 13 path
- ensure the diagnostics entrypoint can answer the reference workload’s main
  jobs without subsystem-specific entry guessing

### Explicit Rule

This is not a full publication pass.

If a cleanup does not materially improve the honesty of the Milestone 13
reference workload or certification surfaces, it should not block this phase.

### Exit Criteria

- the pricing-shock reference workload can be implemented against the intended
  public bridge path
- the core Milestone 13 tests can be end-to-end and integration-heavy rather
  than seam-heavy

---

## Sequencing Rule

The bridge DX work must land before the main Milestone 13 implementation work,
but it must not balloon into a publication-era side quest.

The order should be:

1. define the canonical bridge surface
2. define the bridge boundary
3. harden only the bridge jobs Milestone 13 needs
4. build the reference workload and certification harness on top of that

That gives us real integration tests over the real bridge instead of tests that
quietly certify internal scaffolding.

---

## Bottom Line

The bridge does not need a giant publication program before Milestone 13.

It does need enough DX hardening that:

- the boundary is deliberate
- the main jobs are obvious
- the diagnostics story is coherent
- the tests can target the real bridge

That is the bar this plan exists to reach.
