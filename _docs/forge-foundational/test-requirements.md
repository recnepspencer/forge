# Forge Foundational Test Requirements

## Purpose

`forge-foundational` will be implemented as a complete shared semantic crate
before the major adopting crates are refactored onto it.

That changes the proof bar.

The crate cannot rely on incremental migrations through `forge-relational`,
`forge-query`, `forge-signal`, or `forge-store` to reveal whether its vocabulary
is correct. It must certify its own boundary language before adoption begins.

These test requirements define the proof suite required for
`forge-foundational` to be trusted as a platform-grade substrate.

They do not assume each milestone will be implemented directly into
`forge-relational`, `forge-query`, `forge-signal`, `forge-store`, or any other
real runtime as soon as the milestone lands. Runtime adoption is downstream
evidence. It is not the first enforcement layer.

## Testing Thesis

`forge-foundational` must prove that shared meaning is canonical without
depending on any one runtime representation.

The tests must answer:

- can two independent construction paths produce the same canonical meaning?
- can the compiler prevent semantically invalid boundary states?
- can compatibility inputs lower into native meaning without becoming authority?
- can masks, patches, locators, profiles, diagnostics, receipts, and digests
  compose without category collapse?
- can future crate migrations consume this vocabulary without discovering that
  a key assumption was left implicit?
- can synthetic hostile producers and consumers use the vocabulary without
  relying on hidden runtime state?
- do proof-bearing Milestone 1 APIs use `forge-proof` progression primitives
  instead of reinventing local proof machinery?

The certification goal is not "the crate has unit tests." The certification
goal is:

> Every shared boundary noun has canonical construction, canonical ordering,
> typed admissibility, hostile rejection cases, and digest-preparation parity
> before any adopting crate depends on it.

No milestone may close by saying "the real runtime will prove this later."
Every milestone must include enough adversarial local proof that a later runtime
migration is an adoption exercise, not a discovery phase.

## Test Strategy

The test suite must be layered.

### Runtime-Independence Rule

`forge-foundational` tests must certify shared boundary semantics without
depending on direct integration into an adopting runtime.

For every milestone after Milestone 1, the tests must include at least two
independent local construction paths that behave like different crates with
different internal layouts. Those construction paths should be deliberately
small, synthetic, and hostile:

- one path should look like an authority-first producer with ordered internal
  state
- one path should look like a compatibility or projection producer with
  different ordering, naming, or representation pressure
- where the surface is mutation-like, one path should look like a staged
  candidate and one like committed authority
- where the surface is descriptive, one path should look like rich support
  materialization and one like reduced-richness operational materialization

The point is not to build fake versions of `forge-relational`,
`forge-query`, `forge-signal`, or `forge-store`. The point is to prove the
foundational surface survives hostile producer diversity before real runtimes
adopt it.

Required runtime-independent proof styles:

- `producer diversity`: two or more synthetic producers with different local
  layouts must produce the same canonical meaning
- `consumer blindness`: a consumer must be able to interpret the foundational
  artifact without producer-private state
- `authority separation`: candidate, projected, descriptive, support-only, and
  committed authority categories must not satisfy each other's APIs
- `profile pressure`: reduced-richness construction must remove only optional
  descriptive materialization and must not change authority-bearing outcomes
- `ordering hostility`: insertion order, builder order, serializer order, and
  local map layout must not affect canonical meaning
- `category hostility`: semantically adjacent categories must have negative
  tests proving they are not substitutable
- `misuse pressure`: likely engineer mistakes such as unnamed defaults, partial
  builders, stale cached identity reuse, wrong target wrappers, silent
  category widening, or materialization-output reuse as semantic identity must
  be attacked explicitly rather than assumed away

### Synthetic Adversarial Runtime Doubles

Milestones that describe runtime-shaped concepts must use local doubles to
attack the vocabulary without turning `forge-foundational` into a runtime.

These doubles are allowed to model:

- unordered producers
- staged branch candidates
- committed authority records
- projected/read-model records
- support-only materializers
- reduced-richness materializers
- compatibility-originated boundary inputs
- replay/export consumers that know only the foundational artifact

These doubles must not become generic execution engines, harness dialects,
storage abstractions, schedulers, or mock versions of adopting crates. If a
double begins owning workflow execution, persistence, scheduling, or domain
truth mutation, it has crossed the boundary and should move to `forge-harness`
or to an adopting-crate migration test.

### Forge Harness Relationship

`forge-harness` is the shared Forge substrate for run matrices, parity suites,
replay records, diagnostics capture, workload budgets, and workflow
certification.

`forge-foundational` should not grow a second generic harness dialect for those
concerns. Narrow constructor, facade, and compile-fail certification may remain
local to `forge-foundational` while the proof surface is still a single type or
single boundary rule. Cross-construction parity, compatibility lowering,
digest-preparation sequences, scenario-style composition, golden artifact
bundles, and later migration-readiness runs should either use `forge-harness`
directly or keep their local adapters shaped so they can be lifted into
`forge-harness` without changing the proof grammar.

The boundary is:

- local foundational tests own foundational meaning, fixtures, and exact
  assertion semantics
- `forge-harness` owns reusable execution grammar, run matrices, parity lanes,
  replay/export records, diagnostics capture, and workflow certification
  mechanics
- no foundational test helper may become a generic run-matrix, parity, replay,
  or workflow harness in disguise
- no `forge-harness` adapter may smuggle foundational domain semantics into the
  harness core

`forge-harness` becomes mandatory once a proof requires reusable execution
grammar rather than a local semantic fixture. The line is:

- local doubles may construct, compare, lower, canonicalize, and inspect
  foundational artifacts
- `forge-harness` owns repeated scenario execution, run matrices, replay/export
  records, workload budget accounting, and workflow certification loops
- adopting crates own proof that their real runtime execution lowers into the
  foundational vocabulary correctly

No test should confuse those three layers. A foundational test that starts
executing a domain workflow is too large. An adopting-crate test that discovers
foundational category law for the first time is too late.

### Compile-Fail Tests

Compile-fail tests are mandatory for rules that should be enforced by
visibility, sealed constructors, phantom types, or proof-bearing wrappers.

Required compile-fail families:

- raw `AspectValue` cannot be inserted into authoritative state without
  contract validation
- raw `AspectValue` cannot satisfy APIs requiring a `forge-proof` artifact
  carrying contract-validation proof
- projection masks, mutation masks, diagnostic masks, and support/report masks
  cannot be substituted for each other
- scalar, opaque, reference-only, and content-only aspects cannot receive
  field-level patches
- identities, handles, basis ids, content refs, entity refs, locator ids, and
  digest ids cannot be interchanged just because their storage is identical
- authoritative state constructors cannot be bypassed through public fields
- compatibility JSON bridge types cannot be used where native authoritative
  value construction is required
- evolution-sensitive operations cannot compare, patch, or digest old/new
  contracts without an evolution verdict
- evolution-sensitive operations reject old/new contracts that have not
  progressed through the required `forge-proof` artifact phase
- derived/report/receipt/support artifact categories cannot be passed where
  authoritative state is required
- branch-local candidates, merge candidates, merge verdicts, and commit
  receipts cannot be substituted for committed authoritative state or for each
  other
- reduced-richness descriptive artifacts cannot be passed where full forensic
  evidence or committed authority evidence is required

### Property And Hostile Case Tests

Property tests are mandatory where canonical ordering, equivalence, and
construction-path independence matter.

Required property families:

- insertion order does not affect canonical aspect-state materialization
- insertion order does not affect canonical aspect-patch materialization
- struct field declaration order canonicalizes by declared field order, not
  builder order
- mask path order canonicalizes by locator order, not input order
- equivalent native and compatibility-originated values produce identical
  canonical forms
- no-op patches remain no-op under arbitrary state
- overlapping patch operations either canonicalize deterministically or reject
  deterministically
- digest-preparation basis is stable across independent construction paths
- branch, merge, and commit evidence remains stable across staged, reordered,
  and compatibility-originated construction paths
- reduced-richness profile elision preserves authority-bearing outcomes across
  arbitrary optional descriptive materialization choices

Property tests must attack semantic invariants, not merely sample a few
friendly permutations. Where exhaustive generation is not practical, the test
must name the hostile dimensions it covers and capture representative seeds so
future `forge-harness` runs can expand them.

### Misuse-Pressure Tests

Misuse-pressure tests are mandatory for boundary surfaces that are likely to be
used incorrectly by competent engineers under time pressure.

These tests must attack mistakes that are semantically plausible, not only
obviously invalid garbage input.

Required misuse-pressure families:

- unnamed default construction does not create silent semantic meaning unless a
  milestone explicitly standardizes one named default and certifies it
- partial builders cannot silently smuggle missing required semantic slots
- duplicate assignment cannot degrade into "last write wins" meaning
- stale canonical identity, digest basis, or profile identity cannot be reused
  after meaning-changing mutation or narrowing
- materialization outputs cannot be reused where canonical semantic identity or
  admitted meaning is required
- target-typed wrappers, profiled artifacts, and target-surface inventories
  cannot be substituted across statically distinct target kinds
- support-only, descriptive-only, or reduced-richness outputs cannot satisfy
  APIs requiring stronger evidence, authority, or forensic breadth

These tests may be compile-fail, runtime hostility, or both, but the suite
must make the misuse class explicit rather than relying on incidental coverage
from broader scenario tests.

### Golden Canonicalization Tests

Golden tests are mandatory for any boundary surface that must remain stable
across crate releases.

Golden artifacts must exist for:

- every primitive `AspectValue` family
- representative schema-declared struct aspect values
- whole-aspect patches
- field-level struct patches
- masks in projection, mutation, diagnostics, and support/report modes
- aspect contracts with scalar, struct, opaque, reference, and content-bearing
  shapes
- locators for aspects, struct fields, masks, mismatch loci, provenance/source
  loci, and artifact fields
- identity/basis-id wrapper families
- compatibility lowering examples
- digest-preparation sequences, even before Milestone 2 adds final digest
  algorithms
- branch, merge, and commit evidence examples once Milestone 5 exists
- reduced-richness and full-richness materialization examples for every
  surface whose semantics are profile-sensitive

Golden tests must compare semantic canonical forms, not incidental debug output.

## Milestone 1 Proof Requirements

Milestone 1 is closed only when these test groups exist.

### Value Vocabulary

Must prove:

- each exact relational `AspectValue` family is represented
- integer width and signedness remain distinct
- canonical float wrappers preserve bit identity and reject noncanonical helper
  shortcuts where applicable
- decimal, big-int, and rational forms preserve exactness
- date/time/timestamp forms preserve precision and offset semantics
- `EntityRef`, `ContentRef`, `Bytes`, `String`, and `Uuid` remain distinct
  semantic variants
- generic recursive JSON/document authority is unconstructable through native
  value APIs

### Aspect Contracts

Must prove:

- scalar, struct, opaque, reference, and content-bearing aspect contracts expose
  distinct admissibility rules
- struct fields carry stable identity, type, order, required/optional status,
  absence law, null law, default law, and evolution posture
- unsupported collection-like shapes are rejected or explicitly marked as debt
  rather than accidentally admitted
- every contract has an equivalence basis before equality, reuse, suppression,
  parity, or digest-preparation claims are allowed
- contract validation emits proof-bearing outputs using `forge-proof`
  artifacts, witnesses, or transition outcomes where progression state matters

### Masks

Must prove:

- masks are selectors/contracts, not values
- projection masks, mutation masks, diagnostic masks, and support/report masks
  are not interchangeable
- scalar and opaque aspects reject field masks
- struct aspects reject nonexistent field paths
- nested masks are admitted only if nested struct contracts are admitted
- mask canonicalization is independent of construction order

### Authoritative State

Must prove:

- authoritative state cannot be constructed from raw values without contract
  validation
- authoritative state admission consumes proof-bearing contract-validated
  entries, not raw values or local ad hoc wrappers
- canonical aspect-state ordering is stable
- state serialization does not depend on map insertion order or serializer
  object ordering
- authoritative state, derived state, projected state, and diagnostic/support
  views remain distinct categories

### Patches

Must prove:

- whole-aspect set and clear are distinct
- set dominates overlapping clear
- field-level set and field-level clear are distinct
- whole-aspect replacement and partial struct patching are distinct
- ambiguous overlap between whole-aspect and field-level operations is either
  rejected or canonicalized by explicit law
- no-op and empty patches remain canonical
- patch application produces contract-admissible authoritative state

### Absence, Null, Default, And Clear

Must prove:

- absence, null, default, and clear are distinct states/intents
- any contract that collapses two of those meanings does so explicitly
- compatibility lowering cannot silently turn missing JSON fields into null,
  default, or clear
- patches cannot smuggle default application through missing field entries

### Evolution

Must prove:

- additive field evolution classifies deterministically
- field removal classifies deterministically
- field rename, if admitted, carries explicit lineage or compatibility basis
- type widening and type narrowing classify separately
- default changes classify separately from value changes
- incompatible revisions fail closed before patching, equality, or digest-prep
  can proceed

### Locators And Identities

Must prove:

- every locator has a typed target category
- aspect locators, field locators, mask locators, mismatch locators,
  provenance/source locators, and artifact-field locators canonicalize
  independently of string formatting
- identity wrappers cannot be interchanged across public APIs
- display names cannot become canonical identity by accident

### Compatibility Bridges

Must prove:

- compatibility lowering is visibly named and opt-in
- compatibility lowering uses category-preserving `forge-proof`
  `TransitionOutcome`-style results where success, denial, deferred, stale,
  rebind-required, or failure can occur
- compatibility-originated inputs lower through aspect contracts
- ambiguous numeric width fails closed
- unordered JSON object shape cannot affect canonical meaning
- missing required fields fail closed
- ambiguous null/default/absence cases fail closed
- compatibility bridge output is indistinguishable from native construction
  after canonical lowering

### Digest-Preparation Readiness

Must prove:

- every Milestone 1 surface has a stable canonical iteration order
- digest-preparation readiness is represented as a proof-bearing progression
  state, not a boolean flag
- digest-preparation sequences are stable across independent construction paths
- digest-preparation explicitly distinguishes semantic categories that share
  storage shape
- final digest algorithms remain Milestone 2 work, but Milestone 2 does not
  need to revisit Milestone 1 ordering/equality law

### Forge-Proof Integration

Must prove:

- `forge-foundational` depends on `forge-proof` for proof-bearing progression
  surfaces instead of duplicating proof substrate concepts
- plain values, keys, masks, locators, and identity wrappers remain usable as
  lightweight foundational vocabulary without mandatory proof wrapping
- contract validation, evolution classification, authoritative-state
  admission, compatibility lowering, and digest-preparation readiness expose
  proof-bearing APIs where progression state matters
- negative tests prove raw/unproven values cannot satisfy proof-bearing APIs
- negative tests prove proof-bearing artifacts cannot be substituted across
  incompatible phases

## Whole-Crate Proof Requirements

Because `forge-foundational` will be implemented before broad adopting-crate
migration, each later milestone must add tests that certify its surfaces in
isolation and in composition with all previously completed foundational
surfaces.

### Digest And Canonicalization

Must prove:

- semantically identical boundary artifacts produce identical digest bases
- digest bases do not depend on transport encoding
- digest bases do not depend on insertion order, builder order, or local layout
- equivalence-basis declarations explain every sameness claim
- mismatch surfaces can explain why two artifacts did not compare equal
- digest helpers cannot accept category-erased blobs when typed basis inputs
  are required

### Profiles And Policy Vocabulary

Must prove:

- profile identity is typed and canonical
- richness, support, compatibility, admission/readiness, delivery, retention,
  and certification posture profiles remain distinct where they carry distinct
  meaning
- profile composition accepts compatible combinations and rejects incompatible
  ones
- reduced-richness profiles remove only optional descriptive materialization
- profile changes cannot alter authoritative value, patch, proof, or receipt
  meaning
- profile digest/preparation participates in support and certification
  artifacts without string-label drift
- requested, admitted, and materialized profile meaning remain mechanically
  distinct and cannot collapse into one mutable "effective profile" record
- target-aware profile legality is enforced mechanically where the target is
  statically known, not only documented or logged at runtime
- unnamed default profile construction, partial profile composition, duplicate
  family assignment, and illegal target-surface inventory construction fail
  closed

### Boundary Artifact Taxonomy

Must prove:

- `Summary`, `Report`, `Artifact`, and `Receipt` categories are not
  interchangeable
- authoritative, derived, projected, planned, descriptive, support-only, and
  receipt-bearing surfaces remain distinct where they matter
- materialization boundaries are explicit and cannot masquerade as cheap
  getters
- reduced-richness profiles suppress optional materialization at named seams
- plan-shaped artifacts cannot be confused with execution receipts
- support-only descriptions cannot be passed as authoritative truth
- branch/merge/commit authority-transition evidence is not smuggled through a
  generic `Artifact` or `Receipt` category

### Branching, Merging, And Commits

Must prove:

- branch-local candidate state, staged state, committed authority, merge
  candidates, merge verdicts, and commit receipts are mechanically distinct
  categories
- branch identity, branch lineage, commit parentage, merge basis, and
  committed-delta loci canonicalize deterministically
- two synthetic producers with different internal branch graph layouts can
  materialize the same canonical branch/commit evidence
- merge conflict, denial, advisory, accepted, superseded, and stale-basis
  outcomes remain distinguishable
- a commit receipt cannot be constructed from a merge candidate or planned
  branch-local intent without the required authority-transition proof
- reduced-richness branch/merge reporting removes only optional forensic detail
  and cannot change committed authority outcome
- branch/merge/commit digest bases are stable across reordered parents,
  reordered deltas, compatibility-originated metadata, and independent
  construction paths
- replay/export consumers can interpret commit parentage, merge basis, conflict
  loci, and committed deltas without producer-private state

### Diagnostics And Explanation

Must prove:

- diagnostic codes, scopes, severities, and artifact kinds canonicalize
  deterministically
- success, advisory, denial, violation, unsupported, partial, deferred, and
  mismatch outcomes remain distinguishable
- explanations point at canonical locators rather than producer-private strings
- diagnostics remain descriptive and cannot alter authoritative outcome meaning
- richness profiles change diagnostic breadth without changing truth
- proof-bearing artifacts can attach diagnostics without importing diagnostic
  storage into `forge-proof`

### Lineage, Provenance, And Receipts

Must prove:

- lineage, provenance, and receipt categories remain distinct
- provenance explains basis, authority path, profile basis, and source context
  without replacing proof law
- receipts attest completed effectful or authority-bearing boundaries, not
  planned intent
- planned-versus-executed surfaces remain mechanically distinct
- support/certification artifacts are derived proof-of-truth surfaces, not
  authoritative state
- a consumer can interpret a lineage/provenance/receipt artifact without
  producer-private state

### Performance And Layout Vocabulary

Must prove:

- layout vocabulary distinguishes AoS, SoA, AoSoA, sparse, packed, and custom
  without claiming cost equivalence
- performance-facing reports name the boundary where the claim is valid
- structural counters attach to boundary artifacts without forcing one internal
  measurement runtime
- shared vocabulary does not imply shared storage
- APIs that require expensive materialization make the boundary visible in the
  type or method shape

## Cross-Surface Composition Tests

The crate must include scenario-style tests that compose surfaces across
milestone boundaries before adopting crates migrate.

Required scenario families:

- value plus contract plus mask plus patch plus digest-preparation
- compatibility lowering plus contract validation plus authoritative state plus
  patch application
- struct aspect evolution plus locator plus mismatch explanation
- profile-controlled diagnostic materialization over the same authoritative
  state
- report/artifact/receipt category separation over one simulated authority
  boundary
- branch candidate plus merge verdict plus commit receipt plus digest basis
  over one simulated authority transition
- reduced-richness branch/merge materialization over the same committed
  authority transition proving optional detail elision does not alter the
  commit outcome
- provenance plus receipt plus digest basis over one canonical boundary
  artifact
- reduced-richness profile over lineage/provenance/diagnostic materialization
  proving authoritative truth remains unchanged

These tests must use small synthetic fixtures or `forge-harness` adapters. They
must not depend on `forge-relational`, `forge-query`, `forge-signal`, or
`forge-store` behavior. The point is to certify the shared language before
migration pressure begins.

The synthetic fixtures must be intentionally adversarial. They should vary
local ordering, internal ids, materialization richness, stale/candidate status,
compatibility-origin metadata, and category adjacency so the test proves the
foundational vocabulary survives realistic runtime pressure without importing a
real runtime.

## Production-Test Readiness Gates

Every milestone must produce a production-test readiness artifact before it can
be treated as ready for production-shaped runtime testing.

This artifact is not a production-readiness claim. It is a bounded handoff
contract that says the foundational surface is locally certified enough for
adopting crates, `forge-harness`, or production-like replay fixtures to begin
testing against it without discovering first-order category law.

The readiness artifact must be concrete. It may be a public readiness report,
golden fixture bundle, certification manifest, or closeout document, but it
must be inspectable and versioned with the milestone.

Required readiness fields:

- `certified_surfaces`: the public foundational surfaces whose semantics are
  locally certified
- `synthetic_runtime_pressures`: the adversarial local producer/consumer shapes
  used to simulate runtime pressure
- `compile_fail_boundaries`: the category, phase, proof, visibility, and
  substitution errors proven at compile time
- `canonical_golden_artifacts`: the golden fixtures or canonical digest bases
  that lock stable boundary meaning
- `property_seed_inventory`: hostile ordering, category-adjacency,
  profile-elision, compatibility, and mutation seeds captured for later
  expansion
- `forge_harness_expansion_points`: which local semantic tests should become
  reusable `forge-harness` run matrices, replay suites, workload budgets, or
  certification workflows later
- `runtime_adoption_assumptions`: the exact assumptions a real adopting runtime
  is allowed to make when testing against the milestone
- `runtime_adoption_non_assumptions`: the claims the milestone does not make
  until a real adopting crate proves them
- `runtime_adoption_failure_pressures`: the exact bug classes that downstream
  adoption must still try to discover even after local foundational proof
- `residual_debt`: any compatibility, profile, materialization, proof, or
  migration debt that remains intentionally open
- `adoption_shaped_followthrough`: the parity harness, replay suite, or
  adopting-crate migration pressure that must later attack this milestone in a
  less synthetic environment

A milestone is ready for production-shaped testing only if:

- every certified surface has at least one hostile local producer and one blind
  consumer test
- every adjacent category has a negative substitution test
- every runtime-shaped surface has explicit staged/candidate versus committed
  authority separation where that distinction applies
- every profile-sensitive surface proves reduced richness cannot change
  authority-bearing outcomes
- every golden artifact compares semantic canonical form rather than debug text
- every expensive materialization path is named as a boundary and tested as a
  boundary
- every claim deferred to adopting-crate migration is named as non-assumption
  rather than left implicit
- every likely misuse class that could silently weaken meaning is either proven
  impossible, proven compile-rejected, or named in
  `runtime_adoption_failure_pressures`

Production-shaped tests may assume:

- foundational categories named in `certified_surfaces` have local semantic
  proof
- compile-time boundaries listed in `compile_fail_boundaries` are intentional
  contracts
- golden artifacts listed in `canonical_golden_artifacts` are stable enough to
  build parity tests around
- `runtime_adoption_assumptions` are the only allowed assumptions for adopting
  runtimes

Production-shaped tests may not assume:

- the adopting runtime's lowering is correct until migration parity proves it
- the foundational crate owns execution, scheduling, storage, or durability
  mechanics
- reduced-richness profile behavior is safe for a surface not listed in the
  readiness artifact
- a local synthetic double is evidence that a real runtime preserved its own
  internal invariants
- absence from `residual_debt` is implied proof; unlisted uncertainty is a test
  requirements bug
- absence from `runtime_adoption_failure_pressures` means "no meaningful known
  downstream attack class remains," not "we forgot to think about it"

## Migration Readiness Gates

Before any adopting crate begins a large refactor onto `forge-foundational`, the
foundational crate must provide migration readiness evidence.

Required readiness artifacts:

- an API inventory of public foundational surfaces intended for adopting crates
- a debt inventory naming any transitional compatibility surfaces
- the production-test readiness artifact for every completed milestone
- golden canonicalization fixtures for values, contracts, masks, patches,
  locators, profiles, branch/merge/commit evidence, diagnostics, provenance,
  receipts, and digest bases
- compile-fail suite results for category and phase-boundary violations
- property-test seed capture for hostile ordering, patch, branch/merge/commit,
  profile-elision, and materialization cases
- a migration adapter checklist describing how crate-local dialects should
  lower into foundational meaning
- an adoption-shaped parity plan describing which local foundational proofs
  must later be attacked through `forge-harness`, replay/export suites, or
  real adopting-crate migration tests

The adopting crate refactors should then add migration parity tests. Those tests
are required to prove real runtime lowering, but they are not allowed to be the
first proof that foundational semantics work.

## Non-Negotiable Failure Cases

The test suite must fail if any of these become possible:

- a JSON-shaped object becomes ordinary authoritative value truth
- a raw value enters authoritative state without contract validation
- two independent construction paths produce different canonical ordering for
  the same meaning
- a mask mode is accepted by the wrong API
- a scalar/opaque/content aspect receives a field patch
- missing, null, default, and clear collapse silently
- old/new contracts are compared without an evolution verdict
- display names participate as canonical identity
- unnamed defaults or partial builders silently create canonical profile,
  artifact, or receipt meaning
- derived/report/support/receipt artifacts are accepted as authoritative state
- branch-local candidate state is accepted as committed authoritative state
- a merge candidate is accepted as a commit receipt
- a commit receipt can be constructed without authority-transition proof
- merge conflict, denial, advisory, stale-basis, and accepted outcomes collapse
  into a boolean
- digest-preparation input can be built from an insertion-order-dependent path
- reduced-richness profiles change authoritative outcomes
- requested, admitted, and materialized profile meaning collapse into one
  mutable "effective profile" surface
- target-specific optional surfaces can be named or attached through ad hoc
  strings, option fields, or wrong target wrappers
- materialization cost is hidden behind cheap-looking accessors
- a foundational test requires a real adopting runtime to discover whether a
  boundary category is valid
- a local test double grows into an unowned generic runtime, scheduler, storage
  engine, or workflow harness

## Test Topology Requirements

Tests should mirror responsibility boundaries.

Expected topology:

```text
crates/forge-foundational/tests/
  ui/
  values/
  aspects/
  masks/
  patches/
  locators/
  compatibility/
  canonicalization/
  profiles/
  artifacts/
  branches/
  commits/
  merges/
  diagnostics/
  lineage/
  performance/
  composition/
```

Names may adapt to the final module tree, but test files must be named by the
responsibility they certify, not by milestone number or temporary project
history.

## Closure Rule

`forge-foundational` is not ready for broad crate refactors until:

- every completed milestone has the required compile-fail, property, hostile,
  golden, and composition tests for its surfaces
- every shared boundary category has at least one negative test proving it
  cannot be substituted for a neighboring category
- every canonical surface has an independent-construction parity test
- every compatibility bridge has both parity and fail-closed tests
- every expensive materialization surface is visible in API shape and tested as
  a boundary
- every runtime-shaped surface has local adversarial doubles proving it does not
  rely on direct runtime integration for semantic correctness
- every profile-sensitive surface has full-richness and reduced-richness tests
  proving authority-bearing outcomes are unchanged
- every completed milestone has a production-test readiness artifact that names
  certified surfaces, runtime-shaped pressures, allowed runtime assumptions,
  non-assumptions, harness expansion points, and residual debt
- the migration readiness artifacts exist

If these requirements feel heavy, that is the correct signal. The planned
implementation strategy moves risk forward into the foundational crate. The
test suite must move proof forward with it, while keeping real runtime adoption
as later confirmation rather than the first line of defense.
