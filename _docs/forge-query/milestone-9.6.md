# Milestone 9.6 Engineering Spec: Product Boundary Debt Closure For Evidence Identity, Typed Stop Classes, And Session Label Identity

> **Status:** Draft
>
> **Roadmap parent:** [forge_query_roadmap.md](./forge_query_roadmap.md)
>
> **Primary predecessors:** [milestone-9.5.md](./milestone-9.5.md), [milestone-9.4.md](./milestone-9.4.md)
>
> **Purpose:** close the remaining product-boundary debt where Query-owned
> identity and diagnostic contracts are still carried by string folklore:
> digest identity assembled from `Debug`/`Display` formatting, stop classes
> matched by message text, and preview/branch session labels passed as
> free-form strings.

## Goal

Make evidence identity, stop-class matching, and session label identity
runtime-owned, structurally encoded, and machine-checkable so that downstream
consumers never have a reason to format runtime values into digests, string-
match error messages in decision paths, or mint colliding free-form session
labels against a runtime that is otherwise fanatical about canonical identity.

## Why This Milestone Exists

The first serious downstream consumer (`worth-kernel`) exposes three boundary
defects that are invisible from inside `forge-query` but obvious from outside:

- consumers build admission, report, and parity digests by hashing
  `format!("{:?}", value)` strings joined with `|` separators, which means
  evidence identity changes when a `Debug` derive changes and collides when a
  field contains the separator
- consumer tests match runtime denials with `message.contains("temporal")`
  because the typed stop class either does not carry the denied family as a
  matchable value or does not make typed matching the path of least resistance
- preview and branch sessions are opened with free-form
  `impl Into<String>` labels that have no namespace authority, no collision
  semantics, and no canonical identity participation, in a system whose core
  rule is that repeatable work carries canonical declaration identity

Milestone `9.5` closes lane debt — unfinished productization surfaces.
This milestone closes identity and diagnostic boundary debt — places where a
finished lane still leaks folklore into consumers. It must land before
Milestone `9.7` builds concurrency receipts on top of evidence identity and
before Milestone `9.8` ships consumer-facing report scaffolding that would
otherwise freeze the string-folklore digest scheme into a public kit.

## Governing Summaries

- `MENTALITY.md`: enforcement over convention. A digest scheme or stop-class
  taxonomy documented as "please don't string-match" is category-5 hope;
  this milestone moves both to compiler- and test-enforced contracts.
- `arch_laws.md`: Law 12 (typed, queryable error topologies), Law 26
  (explicit equivalence contracts for every reuse surface — digests are
  equivalence contracts), Law 40 (names and identities mean exactly one
  thing), Law 41 (proof-carrying types with sealed constructors).
- `composition_laws.md`: digest construction, stop classification, and label
  identity each get one named boundary home rather than being smeared through
  call sites as inline mechanics.
- `domain_structure_laws.md`: identity primitives are stable concepts that
  must own their volatile encodings; consumers must depend on the contract,
  not on the formatting accident.
- `perf_laws.md`: no reuse without an explicit equivalence contract; digest
  identity is the equivalence basis for receipts, suppression, and
  certification comparison, so it must be stable, canonical, and cheap.
- `forge_query_roadmap.md`: declare once, lower once, execute through
  canonical artifacts — identity is part of declaration, so identity
  construction is runtime work, not consumer folklore.

## Adversarial Constraint

For the same runtime fact — an admission denial, a basis admission, a
receipt, a support row, a session identity — Query must produce the same
canonical evidence identity and the same typed stop-class meaning under
hostile drift pressure: `Debug` derive reordering, field renaming, message
rewording, separator injection inside field values, and session label
collision attempts.

This milestone fails if any covered path:

- produces a digest whose value depends on `Debug`/`Display` formatting or on
  unescaped string joining
- forces a consumer to call `.to_string()` or string-match a message to make
  a control-flow decision a typed stop class should carry
- admits two distinct session identities that collide into one label, or one
  session identity that silently changes meaning across preview, branch, and
  replay
- closes the debt by adding a parallel digest API or a parallel error family
  beside the existing surfaces

## Product Decision Lock

- This is a debt-closure milestone for identity and diagnostic boundaries,
  not a new capability family.
- The canonical digest basis is a runtime-owned structural encoding with
  field tagging and scheme versioning — never format strings.
- Error message text is presentation. Stop-class matching is a type-level
  operation with typed context payloads.
- Session labels become canonical identity artifacts with explicit namespace
  and collision posture; the raw-string lane does not survive as the ordinary
  path.
- Existing surfaces are extended in place. No parallel digest, error, or
  label API may ship beside the surfaces this milestone hardens.

## Phase Plan

### Phase 1: Canonical Evidence Identity Primitive Boundary

Freeze one runtime-owned structural digest contract that all Query evidence
identity lowers through: field-tagged canonical encoding, explicit scheme
version identity, and sealed construction so a digest value cannot exist
without passing through the canonical encoder.

**Relevant subsystems**
- evidence identity primitive (new boundary home inside `forge-query`)
- digest scheme versioning
- canonical field encoding

**Relevant Query source surfaces**
- [runtime/support_matrix.rs](../../crates/forge-query/src/runtime/support_matrix.rs)
- [runtime/state_snapshot.rs](../../crates/forge-query/src/runtime/state_snapshot.rs)
- [runtime/workspace_contracts.rs](../../crates/forge-query/src/runtime/workspace_contracts.rs)

**Relevant APIs and product surfaces**
- the new canonical evidence-identity constructor surface (sealed; the only
  legal digest producer for covered evidence)
- digest scheme version identity carried inside every produced digest value

**Target shape (illustrative, not frozen API)**

The consumer folklore this primitive replaces, as it exists today in
`worth-kernel` (`crates/worth-kernel/src/construction/runtime_proof/runtime_basis.rs`):

```rust
// BEFORE: identity is whatever Debug/Display prints, joined with pipes
let admission_digest = digest_owned_parts(&[
    label.to_string(),
    effect_policy.to_string(),
    authority_lane.to_string(),
    evidence.join("|"),
]);
```

The target shape after this phase:

```rust
// AFTER: typed, tagged contributions into the sealed runtime-owned encoder;
// scheme version rides inside the value
let admission_digest = ForgeQueryEvidenceIdentity::compose(EvidenceScope::BasisAdmission)
    .field(evidence_tag::LABEL, admission.label())
    .field(evidence_tag::EFFECT_POLICY, admission.effect_policy())
    .field(evidence_tag::AUTHORITY_LANE, admission.authority_lane())
    .field_seq(evidence_tag::EVIDENCE, admission.evidence())
    .seal();
// admission_digest.scheme_version() — comparable only against same-scheme
// digests; cross-version comparison is a typed error, not a byte mismatch
```

**Warnings**
- Do not encode fields by formatting them. The encoder consumes typed field
  contributions with stable tags, not pre-rendered strings.
- Do not make the primitive generic enough to hash arbitrary `Debug` output;
  that re-opens the folklore door with extra steps.
- Do not skip scheme versioning. The first digest migration is proof the
  scheme will evolve; unversioned identity cannot evolve honestly.
- Do not mint a scheme that competes with `forge-foundational` canonical
  digest surfaces. Downstream domains already digest artifacts through
  `forge_foundational::facade` directly (for example `hadwiger-research`'s
  `forge.hadwiger.*` schema digests); two parallel canonical-digest
  authorities at adjacent layers is the drift this milestone exists to kill.

**Test requirements**
- Add a `Canonical Evidence Identity Stability Test` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Adversarial equivalence: prove digest values are invariant under `Debug`
  derive reordering, field renaming, and formatting changes of the source
  types for the same semantic field contributions.
- Adversarial injection: prove field values containing separator bytes,
  delimiter strings, or tag-shaped content cannot produce colliding digests
  with distinct field sets.
- Adversarial drift: prove a scheme-version bump is detectable from the
  digest value alone and that cross-version comparison fails typed rather
  than comparing raw bytes.

**Engineering decisions**
- The primitive is part of the public Query product surface from birth,
  because Milestone `9.8` ships consumer scaffolding on top of it.
- Sealed construction per arch law 41: external code cannot synthesize a
  canonical digest without the proving encoder.
- The primitive composes with `forge-foundational`'s canonical basis/digest
  surfaces rather than replacing or shadowing them: one encoding authority,
  with Query owning the evidence-scope vocabulary and scheme versioning on
  top. A domain that today calls foundational digest surfaces directly must
  be expressible through this primitive without changing its digest
  authority story.

**Open questions**
- None.

### Phase 2: Query-Owned Digest Surface Migration Boundary

Migrate Query's own covered digest emission — public API family contract
digests, support matrix row digests, state snapshot digests, and runtime
certification digests — onto the Phase 1 primitive, so the runtime stops
teaching the folklore scheme by example.

**Relevant subsystems**
- runtime support/contract digest emission
- state snapshot digest emission
- certification digest emission

**Relevant Query source surfaces**
- [runtime/support_matrix.rs](../../crates/forge-query/src/runtime/support_matrix.rs)
- [runtime/support/profile.rs](../../crates/forge-query/src/runtime/support/profile.rs)
- [runtime/state_snapshot.rs](../../crates/forge-query/src/runtime/state_snapshot.rs)
- [runtime/public_api_transcript.rs](../../crates/forge-query/src/runtime/public_api_transcript.rs)
- [application/support/report.rs](../../crates/forge-query/src/application/support/report.rs)

**Warnings**
- Do not preserve old digest values by re-implementing the old string scheme
  inside the new encoder. Covered digests refreeze on the canonical scheme,
  and anything that recorded old values re-records against the new scheme in
  the same change program.
- Do not leave a covered surface half-migrated; a runtime that emits both
  schemes teaches consumers that identity is negotiable.

**Test requirements**
- Add a `Query Digest Surface Migration Closure Test` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Adversarial equivalence: prove every covered digest surface produces
  scheme-versioned canonical digests and that re-deriving the digest from the
  same typed evidence reproduces it exactly.
- Adversarial residue: prove zero remaining format-string digest
  construction in the covered Query surfaces via an exact structural
  assertion, not a code-review convention.

**Engineering decisions**
- Migration is per covered surface and completes inside this milestone; the
  string scheme does not survive as deprecated-but-present in covered lanes.
- Recorded certification artifacts refreeze once, in this phase, against the
  canonical scheme.

**Open questions**
- None.

### Phase 3: Typed Stop-Class Taxonomy Boundary

Freeze the typed stop-class taxonomy and its accessor on the existing error
topology: every covered public denial and stop path classifies to exactly one
stop class with typed context, without flattening or replacing the existing
rich denial payloads.

**Relevant subsystems**
- runtime error topology
- stop-class taxonomy and accessor

**Relevant Query source surfaces**
- [runtime/error.rs](../../crates/forge-query/src/runtime/error.rs)
- [runtime/support/profile.rs](../../crates/forge-query/src/runtime/support/profile.rs)

**Relevant APIs and product surfaces**
- `ForgeQueryRuntimeError` and its covered denial payloads
- the typed stop-class accessor consumers match on instead of message text

**Warnings**
- Do not flatten the existing rich denial payloads into a new generic enum;
  the work is making the existing topology matchable, not replacing it.
- Do not add a second error family beside `ForgeQueryRuntimeError`.
- Do not ship a `Other`/`Unknown` catch-all class that lets future variants
  silently escape classification.

**Test requirements**
- Add a `Typed Stop Class Taxonomy Test` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Adversarial equivalence: exhaustively walk every covered error and denial
  variant and prove each classifies to exactly one stop class whose typed
  context preserves the variant's payload meaning.
- Adversarial drift: prove that adding a new covered denial variant without a
  stop-class mapping fails a completeness assertion rather than falling into
  a catch-all.

**Engineering decisions**
- The taxonomy is an accessor over the existing topology, not a replacement
  vocabulary.
- Message-bearing variants keep their messages for human diagnostics; the
  typed class is the machine lane.

**Open questions**
- None.

### Phase 4: Admission Denial Payloads And Consumer Matching Closure Boundary

Close the consumer-facing half: admission denials carry the denied facade
family and posture as typed values, and a consumer-shaped suite proves every
covered stop class is handleable end to end with zero string operations.

**Relevant subsystems**
- public API family admission denials
- consumer-shaped matching certification

**Relevant Query source surfaces**
- [runtime/error.rs](../../crates/forge-query/src/runtime/error.rs)
- [runtime/workspace.rs](../../crates/forge-query/src/runtime/workspace.rs)

**Relevant APIs and product surfaces**
- `workspace.admit_public_api_family(...)` denial surface
- the Phase 3 stop-class accessor, consumed consumer-side

**Target shape (illustrative, not frozen API)**

The consumer-side string matching this phase eliminates, as it exists today
in `worth-kernel` (`crates/worth-kernel/src/construction/authoring.rs`):

```rust
// BEFORE: the consumer's own test has to grep the message
let message = match error {
    WorthKernelAuthorityError::QueryRuntime(inner) => inner.to_string(),
};
assert!(message.contains("temporal"));
```

The target shape after this phase:

```rust
// AFTER: the denial carries the family as a value; message text is
// presentation and freely rewordable
assert_eq!(
    error.stop_class(),
    ForgeQueryStopClass::FamilyAdmissionDenied {
        family: ForgeQueryRuntimeFacadeFamily::Temporal,
        posture: ForgeQueryRuntimeFamilySupportStatus::Unsupported,
    }
);
```

**Warnings**
- Do not treat `Display` output as part of the contract. Message text must be
  freely rewordable without breaking any consumer test this phase certifies.
- Do not certify with runtime-internal tests only; the matching suite must be
  consumer-shaped, reaching the stop classes through public surfaces.

**Test requirements**
- Add a `Typed Stop Class Matching Closure Test` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Adversarial equivalence: a consumer-shaped test that handles every covered
  stop class — including unsupported-family admission denial with the family
  value extracted — using only type-level matching, with zero string
  operations.
- Adversarial drift: reword every covered denial message and prove the typed
  matching suite still passes while a message-matching probe fails, proving
  text is presentation rather than contract.

**Engineering decisions**
- Admission denials carry the denied `ForgeQueryRuntimeFacadeFamily` as a
  typed value, not as a substring.
- Consumer-shaped certification is the closure bar for this boundary, not
  accessor existence.

**Open questions**
- None.

### Phase 5: Canonical Session Label Artifact Boundary

Freeze the canonical session label artifact: namespace, name, and segment
identity with canonical-scheme digest participation, so a session label is a
typed identity value rather than a display string.

**Relevant subsystems**
- session label identity artifact (new boundary home)
- canonical evidence identity (from Phase 1)

**Relevant Query source surfaces**
- [runtime/workspace.rs](../../crates/forge-query/src/runtime/workspace.rs)
  (the label intake the artifact will replace in Phase 6)

**Relevant APIs and product surfaces**
- the canonical session label constructor surface (scoped namespace + name
  segments)
- label identity digest participation through the Phase 1 primitive

**Warnings**
- Do not model the artifact as a validated string wrapper; namespace and name
  segments are typed parts with their own equality, not parse results.
- Do not let display rendering participate in identity; rendering is
  presentation over the artifact, never its equality basis.

**Test requirements**
- Add a `Canonical Session Label Artifact Test` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Adversarial equivalence: prove the same label identity produces the same
  canonical digest participation regardless of construction path or segment
  formatting accidents.
- Adversarial collision: prove two distinct label identities that render to
  the same display string remain distinct artifacts with distinct digests.

**Engineering decisions**
- Label identity digests lower through the Phase 1 primitive.
- The artifact is sealed against post-construction mutation per arch law 41.

**Open questions**
- None.

### Phase 6: Session Entry Label Intake And Collision Admission Boundary

Migrate preview/branch session entry onto the Phase 5 artifact: typed label
intake on the workspace, explicit collision posture per session family, and
canonical label identity recorded in basis admission evidence.

**Relevant subsystems**
- preview session entry
- branch session entry
- basis admission evidence

**Relevant Query source surfaces**
- [runtime/workspace.rs](../../crates/forge-query/src/runtime/workspace.rs)

**Relevant APIs and product surfaces**
- `workspace.preview_with_options(...)` and `workspace.branch_with_options(...)`
  label intake
- preview/branch basis admission evidence that records the canonical label
  identity

**Target shape (illustrative, not frozen API)**

The free-form label lane this phase retires, as it exists today in
`worth-kernel` (`crates/worth-kernel/src/construction/runtime_proof/runtime_basis.rs`):

```rust
// BEFORE: free-form strings with no namespace authority or collision semantics
let preview = workspace.preview_with_options(
    format!("worth-kernel.{}.preview", family.as_str()),
    ForgeQueryPreviewOptions::sandboxed_write_intent(),
)?;
```

The target shape after this phase:

```rust
// AFTER: canonical label identity; collisions stop typed instead of merging
let label = ForgeQuerySessionLabel::scoped("worth-kernel", family.as_str(), "preview");
let preview = workspace.preview_with_options(
    label,
    ForgeQueryPreviewOptions::sandboxed_write_intent(),
)?;
// a second admission of an equivalent identity yields
// ForgeQueryStopClass::SessionLabelCollision, never a silent merge
```

**Warnings**
- Do not keep the raw-string overload as the ordinary lane with the typed
  label as opt-in ceremony; the typed artifact is the ordinary path.
- Do not invent a global label registry that turns session entry into a
  coordination bottleneck; collision posture is scoped to the workspace that
  admits the session.

**Test requirements**
- Add a `Canonical Session Label Intake Test` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Adversarial equivalence: prove the same label identity yields the same
  admission evidence and digest participation across preview, branch, and
  replayed admission of the covered session families.
- Adversarial collision: prove admitting a second session with an equivalent
  label identity stops with a typed collision class — never a silent merge —
  and that the raw-string intake lane carries zero remaining ordinary-path
  call sites.

**Engineering decisions**
- Collision posture is explicit per session family rather than a global
  uniqueness folklore rule.
- Collision stops classify through the Phase 3 stop-class taxonomy.

**Open questions**
- None.

### Phase 7: Support, Docs, And Hostile Certification Closure Boundary

Close the milestone with support/profile honesty, documentation follow-
through, and one hostile certification program proving the three boundaries
hold together under combined drift pressure.

**Relevant subsystems**
- `application` support/profile reporting
- public documentation coverage
- milestone certification

**Relevant Query source surfaces**
- [application/support/report.rs](../../crates/forge-query/src/application/support/report.rs)
- [application/tests.rs](../../crates/forge-query/src/application/tests.rs)
- [public_doc_coverage/tests/support.rs](../../crates/forge-query/src/public_doc_coverage/tests/support.rs)

**Documentation follow-through**
- Evidence-identity, stop-class, and session-label contracts enter the public
  docs as ordinary product surfaces in the same phase, and any doc text that
  still teaches string digests, message matching, or raw labels is removed.

**Warnings**
- Do not close on broad equality of support reports; certification compares
  narrow canonical artifacts per boundary.
- Do not let docs and support output disagree about whether the canonical
  identity surfaces are the ordinary path.

**Test requirements**
- Add a `Milestone 9.6 Identity And Stop-Class Hostile Certification Matrix`
  to [test-requirements.md](./test-requirements.md) and close it in this
  phase.
- Combined adversarial matrix: drive digest drift pressure, message rewording,
  and label collision attempts in one program and require narrow canonical
  artifacts for the evidence-identity scheme, the typed stop-class taxonomy,
  and session label identity.
- Exact-zero assertions: zero format-string digest construction, zero
  string-matched control flow, and zero raw-string session admissions in the
  covered ordinary paths.

**Engineering decisions**
- Support/profile output is authoritative for whether these boundaries are
  closed.
- This milestone closes on hostile proof, not on API presence.

**Open questions**
- None.

## Must Ship

- one sealed, scheme-versioned canonical evidence-identity primitive owned by
  `forge-query`
- migration of covered Query-owned digest surfaces onto that primitive with
  zero format-string residue
- typed stop-class matching across covered denial paths, including typed
  family payloads on admission denials
- canonical session label identity for preview/branch entry with explicit
  collision posture
- support/profile, docs, and hostile certification closure for all three
  boundaries

## Must Preserve

- the existing rich error topology — extended into matchability, never
  flattened
- existing public facade shape; no parallel digest, error, or label APIs
- canonical declaration identity semantics everywhere the new label and
  digest artifacts participate
- human-readable diagnostics as presentation atop, never instead of, typed
  contracts

## Acceptance Evidence

This milestone is complete only when `forge-query` can prove:

- the Milestone `9.6` certification suites added to
  [test-requirements.md](./test-requirements.md) pass with narrow
  machine-checkable artifacts
- covered digest surfaces emit scheme-versioned canonical digests and contain
  zero format-string digest construction
- a consumer-shaped matching suite handles every covered stop class without
  string operations, and message rewording cannot break it
- preview/branch session entry flows through canonical label identity with
  typed collision posture
- docs, support profiles, and certification agree the three boundaries are
  closed ordinary product surface

## Sequencing Notes

- This milestone belongs immediately after [milestone-9.5.md](./milestone-9.5.md)
  so productization lanes are closed before their identity and diagnostic
  boundaries refreeze on canonical contracts.
- It belongs before Milestone `9.7` because concurrency receipts, journal
  identity, and published-artifact digests must be born on the canonical
  evidence-identity scheme rather than migrated afterward.
- It belongs before Milestone `9.8` because the consumer product kit ships
  report scaffolding directly on the Phase 1 primitive.
