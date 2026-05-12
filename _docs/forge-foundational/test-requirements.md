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

The certification goal is not "the crate has unit tests." The certification
goal is:

> Every shared boundary noun has canonical construction, canonical ordering,
> typed admissibility, hostile rejection cases, and digest-preparation parity
> before any adopting crate depends on it.

## Test Strategy

The test suite must be layered.

### Compile-Fail Tests

Compile-fail tests are mandatory for rules that should be enforced by
visibility, sealed constructors, phantom types, or proof-bearing wrappers.

Required compile-fail families:

- raw `AspectValue` cannot be inserted into authoritative state without
  contract validation
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
- derived/report/receipt/support artifact categories cannot be passed where
  authoritative state is required

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
- digest-preparation sequences are stable across independent construction paths
- digest-preparation explicitly distinguishes semantic categories that share
  storage shape
- final digest algorithms remain Milestone 2 work, but Milestone 2 does not
  need to revisit Milestone 1 ordering/equality law

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
- provenance plus receipt plus digest basis over one canonical boundary
  artifact
- reduced-richness profile over lineage/provenance/diagnostic materialization
  proving authoritative truth remains unchanged

These tests may use small synthetic fixtures. They must not depend on
`forge-relational`, `forge-query`, `forge-signal`, or `forge-store` behavior.
The point is to certify the shared language before migration pressure begins.

## Migration Readiness Gates

Before any adopting crate begins a large refactor onto `forge-foundational`, the
foundational crate must provide migration readiness evidence.

Required readiness artifacts:

- an API inventory of public foundational surfaces intended for adopting crates
- a debt inventory naming any transitional compatibility surfaces
- golden canonicalization fixtures for values, contracts, masks, patches,
  locators, profiles, diagnostics, provenance, receipts, and digest bases
- compile-fail suite results for category and phase-boundary violations
- property-test seed capture for hostile ordering and patch cases
- a migration adapter checklist describing how crate-local dialects should
  lower into foundational meaning

The adopting crate refactors should then add migration parity tests, but those
tests are not allowed to be the first proof that foundational semantics work.

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
- derived/report/support/receipt artifacts are accepted as authoritative state
- digest-preparation input can be built from an insertion-order-dependent path
- reduced-richness profiles change authoritative outcomes
- materialization cost is hidden behind cheap-looking accessors

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
- the migration readiness artifacts exist

If these requirements feel heavy, that is the correct signal. The planned
implementation strategy moves risk forward into the foundational crate. The
test suite must move proof forward with it.
