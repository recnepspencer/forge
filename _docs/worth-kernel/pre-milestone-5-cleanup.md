# Pre-Milestone 5 Cleanup

This document captures robustness work worth doing before Milestone 5 planning
begins in earnest.

The central admitted-scaffold closeout seam is in the right shape:

- family-local admission and realization stay small
- shared birth lowering owns placement embedding, realization-report closeout,
  scaffold digesting, and spatial birth handoff
- direct planar families do not pretend to share the same realization mechanics
  as realized solids

That part looks healthy.

The cleanup list below focuses on the remaining shortcuts and weakly enforced
truth surfaces that could become more dangerous once Milestone 5 broadens the
authority and replay surface.

## Priority Bands

- `P1`: should be addressed before or as part of early Milestone 5 work
- `P2`: important structural cleanup; acceptable only as explicit named debt
- `P3`: lower urgency, but should be resolved deliberately rather than left
  vague

## P1 - Must Harden Soon

### 1. Geometry truth is not committed strongly enough by scaffold and realization digests

Current scaffold digesting in
`crates/worth-kernel/src/construction/phase_chain/admitted_scaffold/family_birth_input/birth_scaffold.rs`
binds:

- intent digest
- family
- topology counts
- realization report digest

The problem is that the lower-layer realization report digest in
`crates/worth-geom/src/primitives/shape_realization/schema.rs` is derived from:

- family
- strategy and attempted strategies
- stability class
- conditioning witness summary fields

It does **not** directly commit the actual support planes or the actual vertex
positions.

That means two materially different scaffold geometries could preserve the same
high-level conditioning/report summary and still look too similar at the digest
layer. Before replay, parity, or certification truth expands further, we should
freeze one geometry-committing digest protocol for:

- support plane identity
- embedded vertex identity
- realized support identity

### 2. Lower-layer digest protocols are still split between canonical SHA-256 and local `DefaultHasher`

Kernel proof digests already use a versioned, scope-separated SHA-256 protocol
in `crates/worth-kernel/src/construction/proof/digest_protocol.rs`.

By contrast:

- `crates/worth-spatial/src/bindings/primitive_birth.rs`
- `crates/worth-geom/src/primitives/shape_realization/schema.rs`
- `crates/worth-geom/src/primitives/shape_realization/exhaustion.rs`
- `crates/worth-geom/src/primitives/shape_realization/witnesses.rs`

still use `DefaultHasher`.

Even if those digests are stable enough for current local use, this is the kind
of split protocol that becomes confusing once more surfaces start treating the
digest as canonical identity instead of incidental bookkeeping.

We should unify on one explicit digest protocol and version story across
kernel/spatial/geom truth artifacts.

### 3. `shell_with_hole` planar witness geometry is under-admitted

Current shell-with-hole witness geometry in
`crates/worth-kernel/src/construction/phase_chain/admitted_scaffold/family_birth_input/geometry.rs`
uses fixed layout constants:

- outer radius `3.0`
- hole center ring radius `1.2`
- hole radius `0.4`

Admission in
`crates/worth-kernel/src/construction/phase_chain/admitted_scaffold/family_birth_input/families/shell_with_hole.rs`
only enforces:

- outer loop has enough edges
- each hole loop has enough edges
- at least one hole exists

It does **not** enforce:

- hole-to-hole clearance
- hole-to-outer-boundary containment
- layout validity as hole count grows
- non-overlap / non-self-intersection guarantees for the canonical witness

This is the clearest current example of a scaffold geometry helper whose
shortcuts can silently become invalid as request shape changes.

### 4. Planar family geometry needs an explicit authority story

Today:

- `wire_body` uses a fixed witness radius `1.5`
- `shell_with_hole` uses fixed radii and fixed center placement

These may be acceptable if they are explicitly declared to be
**canonical scaffold geometry** whose job is only to witness topology and
placement structure.

They are not acceptable if downstream code starts treating them as stronger
geometric truth than that.

Before Milestone 5 expands further, we should choose one of two honest models:

1. These are canonical scaffold witnesses only, and that status is documented
   and mechanically reflected in naming/tests.
2. These should evolve toward support-derived or request-derived witnesses with
   stronger geometric authority.

### 5. Canonical local witness geometry is duplicated across crates

The simplex/tetrahedron witness coordinates currently appear in both:

- `crates/worth-kernel/src/construction/phase_chain/admitted_scaffold/family_birth_input/geometry.rs`
- `crates/worth-geom/src/primitives/shape_realization/support/simplex.rs`

including the same `0.7071` approximation.

That duplication creates drift risk:

- one side can change without the other
- realization and birth embedding can stop describing the same canonical local
  witness
- digest and parity surfaces may then diverge for reasons that are hard to see

We should consolidate each family's canonical witness geometry into one shared
source of truth and have both support realization and birth embedding consume
it.

## P2 - Important Structural Cleanup

### 6. Family topology contracts are manually restated across layers

The same family count rules are currently re-expressed in multiple places:

- kernel family helpers through
  `family_birth_input/topology_counts.rs`
- spatial birth validation through
  `crates/worth-spatial/src/bindings/primitive_birth_contract.rs`
- topology admitted-handoff admission through
  `crates/worth-topo/src/construction/query_native_boundary/admission.rs`

This is better than no checking because the downstream layers do revalidate the
shape. Still, it is brittle:

- contract changes must be updated in multiple places
- omission risk grows as families expand
- duplicated formulas make the contract harder to audit as one authority

We should consider one canonical family contract registry or one type-directed
source that downstream layers derive from.

### 7. Certification is stronger on architecture boundaries than on geometry honesty

The Phase 5 boundary tests in
`crates/worth-kernel/src/construction/tests/boundary_phase_five.rs`
are doing good work:

- they prevent duplicate bridge choreography
- they prevent family helpers from reclaiming shared authority
- they keep the admitted-scaffold seam honest

What is lighter today is adversarial geometry certification around the witness
helpers themselves.

We should add hostile proof coverage for:

- digest sensitivity to actual geometry changes
- witness parity between kernel embedding geometry and geom realization geometry
- shell-with-hole layout legality under growing hole counts
- planar witness non-overlap and containment

### 8. Family-local counts are checked, but still entered manually

`PrimitiveConstructionTopologyCounts::new(...)` is still populated manually by
each family helper. Spatial and topology both validate the resulting shape, so
this is not an invisible risk. But it is still a hand-entered truth surface.

Longer term, the counts should ideally be derived from one family contract or
from one family-local witness descriptor rather than typed in as loose numeric
tuples.

## P3 - Lower Priority but Worth Naming

### 9. The simplex approximation should stop being an unnamed magic decimal

The `0.7071` tetrahedron coordinate shortcut may be acceptable for now, but it
should not remain ambiguous.

We should either:

- promote it into an explicitly named canonical scaffold ratio, or
- replace it with a cleaner exact/construction-derived formulation

The important thing is honesty. A magic decimal that quietly carries witness
authority is harder to defend than a named canonical ratio or an exact
construction story.

## Suggested Pre-Phase-5 Exit Criteria

Before we feel good about Milestone 5 starting, the minimum useful hardening
bar is probably:

1. one explicit decision on canonical scaffold geometry versus stronger witness
   geometry for planar families
2. one unified digest protocol story across kernel/spatial/geom
3. geometry-committing identity for scaffold/realization truth
4. shell-with-hole layout legality guarded by admission or certification
5. shared-source canonical witness geometry for simplex at minimum

## Summary

The admitted-scaffold architecture looks good.

The remaining risk is not that the birth bridge is structurally wrong. The risk
is that some family-local witness geometry and digest truth are still more
scaffold-ish than canonical, and that ambiguity will get more expensive once
Milestone 5 expands the surfaces that depend on them.
