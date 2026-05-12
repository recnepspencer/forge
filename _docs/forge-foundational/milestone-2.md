# Milestone 2: Canonical Digest And Canonicalization Substrate

## Goal

Define the shared canonicalization and digest-basis substrate that makes
foundational boundary meaning reproducible, comparable, replayable, and
certifiable across crates.

Milestone 2 turns Milestone 1's stable nouns into stable evidence. Values,
aspect contracts, masks, authoritative state, patches, identities, locators, and
compatibility-lowered inputs must be able to produce versioned, ordered,
equivalence-aware, mismatch-explainable canonical bases without depending on
producer-private state, transport encoding, or one runtime representation.

This milestone does not own final cryptographic digest algorithms, receipt
semantics, diagnostics ontology, lineage/provenance ontology, profile
semantics, branch/merge/commit semantics, or adopting-crate migrations. It owns
the canonical evidence grammar those later surfaces must consume.

## Governing Document Summaries

### `MENTALITY.md`

Protects adversarial-constraint-first engineering, mechanical enforcement, and
architecture tests over MVP plausibility. The shaping constraint for Milestone 2
is that digest and canonicalization law must be proven before later reports,
profiles, diagnostics, receipts, or migrations rely on it.

### `arch_laws.md`

Protects proof-bearing phase boundaries, explicit equivalence contracts,
canonical artifacts, authority/derivation separation, and self-describing
boundary envelopes. The shaping constraint is that digest-readiness must be a
typed progression over canonical basis, not a boolean or ad hoc hash helper.

### `composition_laws.md`

Protects responsibility-shaped files, named semantic steps, and test topology
that identifies the responsibility that failed. The shaping constraint is that
canonical basis construction, equivalence basis, mismatch basis, algorithm
registry, and export fixtures must not collapse into a generic
`canonicalization` dump.

### `domain_structure_laws.md`

Protects structure as responsibility topology with source truth, projections,
boundary crossings, and test support separated by meaning. The shaping
constraint is that digest basis, canonical export, mismatch evidence, and
readiness artifacts must occupy distinct homes whose names reject unrelated
future profile, diagnostic, receipt, or migration code.

### `perf_laws.md`

Protects cost-honest boundaries, no repeated rediscovery of proven facts,
explicit equivalence basis, and materialization costs that are visible in API
shape. The shaping constraint is that canonicalization must be an explicit
boundary operation with counters/basis evidence where useful, not a hidden
getter or automatic hot-path reconstruction.

### `forge_foundational_vision.md`

Protects the thesis that `forge-foundational` standardizes shared semantic
language without forcing one runtime representation. The shaping constraint is
that digest/canonicalization helpers must make boundary artifacts stable and
auditable while preserving crate-local layout freedom.

### `forge_foundational_roadmap.md`

Protects the sequence: Milestone 1 establishes aspect-native substrate,
Milestone 2 establishes canonical digest/canonicalization substrate, and later
milestones layer profiles, artifact categories, branch/merge/commit evidence,
diagnostics, provenance, performance vocabulary, and migrations on top. The
shaping constraint is that Milestone 2 must unblock all later digest-basis
participation without stealing their domain-specific ontology.

### `test-requirements.md`

Protects standalone proof before adopting runtime migration. The shaping
constraint is that Milestone 2 must certify canonical bases through hostile
local producers, blind consumers, golden artifacts, compile-fail category
boundaries, property seeds, production-test readiness evidence, and
forge-harness expansion points rather than waiting for real runtimes to reveal
canonicalization mistakes.

### `milestone-1-closeout.md`

Protects what is already complete: canonical values, aspect contracts, masks,
authoritative state, patches, identities, locators, compatibility lowering, and
initial digest-preparation readiness. The shaping constraint is that Milestone 2
should build on those certified surfaces and must not revisit their basic value
or aspect-state law unless it exposes an explicitly named Milestone 1 defect.

## Why This Milestone Exists

Milestone 1 made canonical meaning constructible. Milestone 2 makes canonical
meaning reproducible.

Later milestones cannot honestly define profiles, reports, receipts,
diagnostics, provenance, branch/merge/commit evidence, performance reports, or
cross-crate migration parity if each surface invents its own digest basis,
ordering law, mismatch vocabulary, export fixture shape, or equivalence claim.

Milestone 2 exists to give every later milestone a paved road:

- define the semantic surface
- define its canonical basis entries
- define its equivalence basis
- define how mismatch is localized
- define its readiness artifact
- lock golden fixtures and hostile construction-path parity

The milestone is successful when adding a later foundational surface requires
declaring new basis entries and tests, not rediscovering canonicalization law.

## Adversarial Constraint

Two Forge crates with different internal layouts, insertion orders,
compatibility pressures, display labels, builder strategies, and transport
encodings must be able to materialize the same foundational meaning into the
same canonical basis, while semantically different meanings never collapse just
because their storage shape, debug string, JSON encoding, enum order, or local
construction path happens to look similar.

This milestone fails if:

- a digest can be assembled from insertion-order-dependent data
- a hash value becomes the semantic authority instead of a compression of the
  canonical basis
- equivalence, reuse, suppression, parity, or certification sameness can be
  claimed without a declared basis
- mismatches require producer-private state to explain
- canonical export fixtures compare debug output, transport JSON, or display
  strings instead of semantic basis entries
- canonicalization erases meaningful distinctions between adjacent categories
- the same semantics produce different bases because two crates store data
  differently
- final digest algorithms, receipts, diagnostics, profiles, or migrations get
  smuggled into this milestone because the basis substrate is too vague

## Boundary Thesis

Milestone 2 owns canonical digest-preparation and canonicalization law.

It owns:

- canonical basis entry vocabulary
- canonical basis sequences and bundles
- canonicalization rule versioning
- basis-domain typing
- equivalence basis participation
- mismatch-basis primitives
- proof-bearing readiness artifacts
- canonical export/golden fixture shapes
- hostile independent-construction parity
- digest algorithm slots and algorithm metadata sufficient to keep hash
  derivation downstream of basis semantics
- production-test readiness evidence for canonicalization surfaces

It does not own:

- final cryptographic digest policy for all Forge artifacts
- report, summary, artifact, or receipt taxonomy
- profile vocabulary
- diagnostics and explanation ontology
- lineage and provenance ontology
- branch, merge, and commit vocabulary
- performance/layout vocabulary beyond cost-honest canonicalization boundaries
- real adopting-crate migrations
- a universal serializer, storage engine, executor, or runtime container

The semantic authority is the canonical basis. A digest is a derived
compression of that basis.

## Practical Type Targets

The implementation may choose better names, but it must contain these
responsibilities somewhere. If a responsibility has no concrete type, artifact,
or sealed construction path, the milestone has not closed honestly.

```rust
pub struct CanonicalizationRuleVersion(/* version identity */);

pub enum CanonicalBasisDomain {
    Value,
    AspectContract,
    AspectMask,
    AuthoritativeState,
    AuthoritativePatch,
    Identity,
    Locator,
    CompatibilityLowering,
    Future(&'static str),
}

pub struct CanonicalBasisEntry {
    domain: CanonicalBasisDomain,
    locus: CanonicalBasisLocus,
    kind: CanonicalBasisEntryKind,
    value: CanonicalBasisValue,
}

pub struct CanonicalBasisSequence {
    version: CanonicalizationRuleVersion,
    domain: CanonicalBasisDomain,
    entries: CanonicalBasisEntries,
}

pub struct CanonicalBasisBundle {
    version: CanonicalizationRuleVersion,
    sequences: CanonicalBasisSequences,
}

pub enum CanonicalEquivalenceBasis {
    ExactCanonicalBasis,
    DeclaredAspectEquivalence,
    CompatibilityLoweredNativeEquivalence,
    ProjectionEquivalent,
    DigestEquivalent,
}

pub struct CanonicalMismatchBasis {
    left: CanonicalBasisLocus,
    right: CanonicalBasisLocus,
    kind: CanonicalMismatchKind,
}

pub enum CanonicalMismatchKind {
    MissingEntry,
    AdditionalEntry,
    EntryKindMismatch,
    ValueMismatch,
    OrderingMismatch,
    EquivalenceBasisMismatch,
    VersionMismatch,
    UnsupportedComparison,
}

pub struct CanonicalDigestAlgorithmId(/* identity only */);

pub struct CanonicalDigestAlgorithmSlot {
    id: CanonicalDigestAlgorithmId,
    input_domain: CanonicalBasisDomain,
    rule_version: CanonicalizationRuleVersion,
}

pub type CanonicalBasisReadyArtifact =
    forge_proof::Artifact<CanonicalBasisReady, CanonicalBasisSequence>;

pub type CanonicalExportReadyArtifact =
    forge_proof::Artifact<CanonicalExportReady, CanonicalBasisBundle>;
```

These sketches are intentionally basis-centered. No type should imply that the
hash algorithm is the source of truth. No type should allow arbitrary byte blobs
to claim digest equivalence without canonical basis evidence.

## Compile-Time Enforcement Targets

| Rule | Preferred enforcement |
| --- | --- |
| Raw unordered maps cannot satisfy digest-basis APIs. | Basis builders consume canonical iterators, Milestone 1 readiness artifacts, or sealed ordered wrappers. |
| A digest algorithm cannot accept category-erased blobs as semantic input. | Algorithm slots consume `CanonicalBasisReady` or `CanonicalExportReady` artifacts, not `Vec<u8>` or strings. |
| Equivalence claims cannot be made without an equivalence basis. | Comparison APIs require `CanonicalEquivalenceBasis` or a proof-bearing equivalence artifact. |
| Mismatch explanation cannot depend on producer-private paths. | Mismatch APIs return canonical loci and entry kinds, not raw debug strings. |
| Canonical export fixtures cannot be constructed from debug output. | Export constructors consume basis sequences/bundles, not `Debug`, display strings, or transport JSON. |
| Canonicalization rule version cannot be omitted. | Basis sequence and bundle constructors require a `CanonicalizationRuleVersion`. |
| Value, contract, state, patch, identity, locator, and compatibility bases cannot be substituted silently. | Domain-typed basis wrappers or phantom-typed domain markers where APIs need one domain. |
| Final receipt/report/profile/diagnostic types cannot be smuggled into Milestone 2. | Milestone 2 exposes extension slots and future domains, not final later-milestone category constructors. |

## Phases

Phases are mandatory and linear. Each phase must leave behind proof-bearing
types, tests, or facade boundaries that the next phase consumes. Later phases
must not defensively rediscover facts that earlier phases are required to prove.

Phase progression gates:

| Phase | Gate before next phase |
| --- | --- |
| Phase 1 | Canonical basis domains, version identity, and basis entry grammar exist before any surface-specific basis builder lands. |
| Phase 2 | Milestone 1 surfaces produce canonical basis sequences before equivalence or mismatch can compare them. |
| Phase 3 | Equivalence basis and mismatch-basis primitives exist before export fixtures or digest slots can claim parity. |
| Phase 4 | Canonical export bundles and golden fixture shapes exist before algorithm slots compress basis evidence. |
| Phase 5 | Algorithm slots are explicitly downstream of basis evidence before production-test readiness can close. |
| Phase 6 | Hostile producer parity, compile-fail boundaries, and readiness artifacts exist before Milestone 3 profile work begins. |

### Phase 1: Define Canonical Basis Grammar

Purpose:

Create the generic canonical evidence grammar that every Milestone 2 and later
surface will use.

Must ship:

- `CanonicalizationRuleVersion`
- `CanonicalBasisDomain`
- `CanonicalBasisLocus`
- `CanonicalBasisEntry`
- `CanonicalBasisEntryKind`
- `CanonicalBasisValue`
- `CanonicalBasisSequence`
- `CanonicalBasisBundle`
- deterministic entry ordering law
- sealed constructors or builders that reject duplicate, unordered, or
  domain-incoherent basis input where the API can know it

Must preserve:

- canonical basis is semantic evidence, not a transport encoding
- every sequence and bundle carries a rule version
- domains are visible and typed enough to prevent category-erased comparison
- future domains can be reserved without implementing future milestone
  ontology early

Acceptance evidence:

- basis-entry ordering tests across hostile insertion orders
- duplicate and domain-incoherence rejection tests
- compile-fail tests proving raw unordered maps or raw byte blobs cannot satisfy
  basis-ready APIs
- golden tests for the basis grammar itself

### Phase 2: Build Basis Sequences For Milestone 1 Surfaces

Purpose:

Promote Milestone 1 digest-preparation readiness into the shared canonical
basis grammar without changing Milestone 1 semantics.

Must ship basis builders for:

- canonical values
- aspect contracts
- struct fields and declared field order
- masks and mask modes
- authoritative aspect state
- authoritative aspect patches
- identities and basis ids
- locators
- compatibility-lowered values and state

Must preserve:

- no revisiting Milestone 1 value or aspect-state law unless a defect is named
- native and compatibility-originated construction paths produce the same basis
  when they mean the same thing
- semantically distinct variants with equal storage shape remain distinct
- basis builders consume readiness artifacts or canonical Milestone 1 types,
  not raw unvalidated values where validation was previously required

Acceptance evidence:

- cross-construction parity tests for each Milestone 1 surface
- compatibility-originated versus native basis parity tests
- hostile tests for storage-equal but meaning-distinct variants
- compile-fail tests proving raw unvalidated Milestone 1 inputs cannot satisfy
  basis builders that require proof-bearing readiness

### Phase 3: Define Equivalence Basis And Mismatch Basis

Purpose:

Make sameness and difference explainable before any digest, export, support, or
later diagnostic surface claims parity.

Must ship:

- `CanonicalEquivalenceBasis`
- equivalence-scope vocabulary for exact, compatibility-lowered,
  projection-equivalent, digest-equivalent, and declared-aspect equivalence
- `CanonicalMismatchBasis`
- `CanonicalMismatchKind`
- canonical mismatch loci that reference basis entries, fields, masks,
  contracts, values, identities, locators, and compatibility origins without
  producer-private paths
- comparison APIs that return structured match/mismatch outcomes rather than
  booleans where explanation matters

Must preserve:

- a digest match is not automatically semantic equivalence unless the basis says
  so
- mismatch explanation remains primitive and canonical, not a full diagnostics
  ontology
- equivalence claims do not erase meaningful variant distinctions
- unsupported comparisons fail closed with a structured unsupported mismatch
  kind

Acceptance evidence:

- equivalence-basis tests for exact, declared-aspect, and
  compatibility-lowered equivalence
- mismatch tests for missing entries, additional entries, value mismatch,
  entry-kind mismatch, ordering mismatch, version mismatch, and unsupported
  comparison
- compile-fail or facade tests proving comparison APIs cannot be called without
  an explicit equivalence basis where the API can enforce it
- blind-consumer tests proving mismatch output can be interpreted without
  producer-private state

### Phase 4: Define Canonical Export And Golden Fixture Bundles

Purpose:

Create stable exportable evidence for tests, support bundles, and future
forge-harness replay without making transport encoding the authority.

Must ship:

- `CanonicalExportReady` proof phase
- export-ready basis bundles
- golden fixture bundle shape for canonical basis sequences
- stable semantic fixture comparison helpers
- fixture metadata for rule version, basis domain, producer shape, and
  equivalence basis
- rejection of debug-output, display-string, and transport-JSON fixture sources
  as canonical authority
- fixture debt markers for surfaces intentionally deferred to later milestones

Must preserve:

- fixtures compare semantic basis entries, not incidental formatting
- export shape is a test/support boundary, not a universal runtime serializer
- transport encoding remains downstream from canonical basis
- golden fixtures can be used by `forge-harness` later without changing their
  proof grammar

Acceptance evidence:

- golden fixture tests for every Milestone 1 basis domain
- fixture round-trip tests that preserve canonical basis meaning
- hostile formatting tests proving debug/display/JSON ordering cannot alter
  fixture comparison
- forge-harness expansion seed inventory for parity and replay suites

### Phase 5: Define Digest Algorithm Slots And Derived Digest Values

Purpose:

Allow canonical basis evidence to be compressed into digest values without
letting hash algorithms become the semantic authority.

Must ship:

- `CanonicalDigestAlgorithmId`
- `CanonicalDigestAlgorithmSlot`
- derived `CanonicalDigestValue` or equivalent digest carrier
- algorithm metadata that names input domain, canonicalization rule version,
  and digest output shape
- digest derivation APIs that consume `CanonicalBasisReady` or
  `CanonicalExportReady` artifacts
- explicit test-only deterministic digest support if needed for fixtures
- debt markers for production cryptographic policy if the final algorithm
  policy remains later work

Must preserve:

- digest values are derived from canonical basis only
- algorithm identity is not semantic category identity
- algorithm slots do not accept raw bytes, debug strings, or transport JSON as
  semantic input
- final receipt, provenance, branch/merge/commit, and diagnostic digest
  semantics remain later milestone work

Acceptance evidence:

- digest derivation tests from canonical basis artifacts
- compile-fail tests proving raw bytes/category-erased blobs cannot satisfy
  digest derivation APIs
- algorithm-version tests proving rule-version changes are visible in digest
  metadata
- collision-shaped hostile tests proving equal display strings or storage bytes
  in different domains remain distinct digest inputs

### Phase 6: Certify Production-Test Readiness

Purpose:

Close the milestone with adversarial local proof that the canonicalization
substrate is ready for production-shaped testing, without pretending real
runtime adoption has already been proven.

Must ship:

- Milestone 2 production-test readiness artifact
- certified-surface inventory for basis grammar, Milestone 1 basis builders,
  equivalence basis, mismatch basis, export bundles, and digest algorithm slots
- synthetic runtime pressure inventory covering ordered authority producers,
  reordered compatibility producers, support/export consumers, and
  category-adjacent hostile producers
- compile-fail boundary inventory
- canonical golden artifact inventory
- property seed inventory for ordering, category adjacency, compatibility
  lowering, equivalence, mismatch, and digest slot hostility
- forge-harness expansion points for future run matrices and replay/export
  suites
- runtime adoption assumptions and non-assumptions
- residual debt inventory

Must preserve:

- production-shaped tests may assume only what the readiness artifact names
- adopting crates still own proof that their real runtime lowering is correct
- local doubles do not grow into a generic runtime, scheduler, serializer, or
  storage engine
- Milestone 3 may consume profile-ready basis slots without reworking
  Milestone 2 canonicalization law

Acceptance evidence:

- readiness artifact tests or golden snapshot
- no milestone-owned test file or fixture directory becomes a responsibility
  dump
- topology check proving basis, equivalence, mismatch, export, digest slots,
  and readiness tests live in responsibility-owned homes
- full crate tests and compile-fail tests pass with the Milestone 2 surfaces
  enabled

## Must Ship

- canonical basis entry grammar with versioned rule identity
- typed basis domains and canonical basis loci
- canonical basis sequences and bundles
- proof-bearing canonical basis readiness and export readiness artifacts
- basis builders for every completed Milestone 1 surface
- equivalence-basis vocabulary and comparison APIs
- mismatch-basis primitives with canonical loci and mismatch kinds
- canonical export/golden fixture bundle shape
- digest algorithm slots and derived digest value carriers that consume basis
  artifacts rather than raw blobs
- hostile producer/consumer test fixtures that simulate runtime diversity
- production-test readiness artifact for Milestone 2

## Must Preserve

- canonical basis, not hash output, is the semantic authority
- no canonicalization meaning depends on insertion order, builder order,
  transport encoding, display labels, debug output, or crate-local layout
- equivalence, reuse, suppression, parity, and certification claims require an
  explicit basis
- mismatch explanation is self-describing enough for blind consumers
- `forge-proof` owns proof progression law; `forge-foundational` owns the
  shared canonical evidence language
- domain crates keep ownership of storage layout, runtime execution, durability,
  scheduling, and migration behavior
- later milestone surfaces get extension points, not premature final ontology
- local adversarial doubles stay small semantic fixtures, not fake adopting
  runtimes

## Acceptance Evidence

- cross-construction canonical-basis parity tests for every Milestone 1 surface
- property tests or hostile seed suites proving ordering independence across
  insertion order, builder order, serializer order, local map layout, and
  compatibility-origin ordering
- golden canonical basis fixtures for values, contracts, masks, state, patches,
  identities, locators, compatibility lowering, equivalence basis, mismatch
  basis, export bundles, and digest slots
- compile-fail tests proving raw unordered maps, raw bytes, debug strings,
  category-erased blobs, and unvalidated values cannot satisfy proof-bearing
  canonicalization APIs
- mismatch tests proving blind consumers can interpret differences without
  producer-private state
- digest derivation tests proving digest values are derived from basis artifacts
  and carry algorithm/rule-version metadata
- production-test readiness artifact naming certified surfaces, synthetic
  runtime pressures, compile-fail boundaries, golden artifacts, property seeds,
  forge-harness expansion points, runtime assumptions, non-assumptions, and
  residual debt
- topology review showing canonical basis, equivalence, mismatch, export,
  digest-slot, and readiness tests are responsibility-shaped rather than a flat
  dump

## Architectural Notes

The implementation should preserve distinct responsibility homes. A likely
shape is:

```text
crates/forge-foundational/src/
  canonicalization/
    basis/
    domains/
    readiness/
    equivalence/
    mismatch/
    export/
    digest_slots/
    production_readiness/
```

Names may adapt during implementation, but the structure must keep these
responsibilities separable:

- basis grammar owns entry, sequence, bundle, ordering, and version law
- domain builders own conversion from completed foundational surfaces into
  canonical basis
- equivalence owns sameness claims and comparison scope
- mismatch owns difference localization and primitive mismatch kinds
- export owns golden/support/replay fixture shape
- digest slots own derived compression metadata and algorithm identity
- production readiness owns handoff evidence, not semantic construction

Public exports should remain facade-controlled. Internal modules may reorganize,
but external callers should depend on durable canonicalization capabilities, not
deep topology.

## Test Topology

Expected test homes:

```text
crates/forge-foundational/tests/certification/canonicalization/
  basis/
  domains/
  equivalence/
  mismatch/
  export/
  digest_slots/
  production_readiness/

crates/forge-foundational/tests/ui/canonicalization/
  basis/
  equivalence/
  digest_slots/
  export/
```

Test support must stay narrow. A fixture that only constructs a value basis
belongs with value-basis tests. A fixture that simulates a hostile producer for
multiple domains should either be a small local semantic fixture or become a
`forge-harness` expansion point. It must not become an unowned generic harness
inside `forge-foundational`.

## Production-Test Readiness Requirements

Milestone 2 must close with a concrete production-test readiness artifact.

It must name:

- certified canonicalization surfaces
- hostile producer and blind consumer shapes tested locally
- compile-fail boundaries for raw/unordered/category-erased inputs
- golden canonical basis fixtures
- property seeds for ordering, equivalence, mismatch, compatibility, and digest
  slot hostility
- forge-harness expansion points for replay/export and parity run matrices
- assumptions adopting runtimes may make about canonical basis law
- non-assumptions adopting runtimes must prove for themselves
- residual debt for final cryptographic policy, later milestone domains, or
  migration-specific parity

Production-shaped tests may assume:

- Milestone 1 surfaces can be converted into canonical basis through the
  certified Milestone 2 APIs
- canonical basis entries are ordered, versioned, and domain-typed
- equality, mismatch, and digest-slot operations are basis-driven

Production-shaped tests may not assume:

- real adopting-runtime lowering is correct
- final receipt, profile, diagnostic, provenance, branch/merge/commit, or
  performance ontology exists
- digest algorithm policy is final unless the readiness artifact explicitly
  says so
- local synthetic producers prove real runtime invariants

## Sequencing Notes

Milestone 2 belongs immediately after Milestone 1 because every later
foundational surface needs canonical basis participation before it can claim
digest parity, replay stability, support comparability, or migration readiness.

- Milestone 3 profiles need canonical basis slots before profile identity and
  profile-driven elision can be digestible.
- Milestone 4 artifact/materialization categories need basis and mismatch law
  before reports, summaries, artifacts, and receipts can compare honestly.
- Milestone 5 branch/merge/commit vocabulary needs basis domains and mismatch
  loci before commit evidence can be replayed and compared.
- Milestone 6 diagnostics need mismatch primitives and locators before
  explanations can point at canonical difference.
- Milestone 7 lineage/provenance/receipts need digest-basis and export bundles
  before provenance can be self-describing and certifiable.
- Milestone 8 performance/layout vocabulary needs cost-visible
  materialization boundaries before performance reports can attach to canonical
  artifacts honestly.
- Milestone 9 migrations need canonical basis parity before crate-local
  dialects can converge with confidence.

## Explicit Non-Goals

- final cryptographic digest policy for every Forge artifact
- final report, summary, artifact, or receipt taxonomy
- profile vocabulary or profile-elision policy
- diagnostics and explanation ontology
- lineage and provenance ontology
- branch, merge, and commit vocabulary
- performance and layout vocabulary beyond canonicalization boundary honesty
- adopting-crate migrations or real runtime lowering proof
- generic serializer/deserializer runtime
- generic executor, scheduler, storage engine, or workflow harness
- replacing `forge-proof` artifact, witness, phase, or transition machinery

## Self-Check

- Does the milestone solve a real structural problem or just package work
  cosmetically? Yes. It creates the canonical evidence substrate every later
  shared surface needs for replay, comparison, certification, and migration.
- Is the adversarial constraint precise and load-bearing? Yes. It attacks
  construction order, local layout, transport encoding, debug/display strings,
  category collapse, and hash-as-authority shortcuts.
- Does the milestone preserve crate authority boundaries? Yes.
  `forge-foundational` owns canonical basis language only; domain crates keep
  runtime, storage, execution, durability, and migration authority.
- Does the milestone define proof obligations, not just implementation tasks?
  Yes. Closure requires compile-fail boundaries, property/hostile tests,
  golden fixtures, mismatch evidence, digest-slot derivation, and
  production-test readiness.
- Could a competent engineer map this spec into honest types, modules, and
  tests? Yes. The phases map directly to basis grammar, domain builders,
  equivalence, mismatch, export fixtures, digest slots, and readiness evidence.
- Does the milestone belong in this roadmap sequence, or is it out of order?
  Yes. It must come after Milestone 1's semantic substrate and before profiles,
  artifacts, branch/merge/commit evidence, diagnostics, provenance, performance
  vocabulary, and migrations consume canonical evidence.
