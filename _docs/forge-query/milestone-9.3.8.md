# Milestone 9.3.8 Engineering Spec: Query-As-Beginning Platform Entry

> **Status:** Proposed
>
> **Roadmap parent:** [forge_query_roadmap.md](./forge_query_roadmap.md)
>
> **Vision parent:** [forge_query_vision.md](./forge_query_vision.md)
>
> **Prior milestone:** [milestone-9.3.7.md](./milestone-9.3.7.md)
>
> **Prior closeout:** [milestone-9.3.7-closeout.md](./milestone-9.3.7-closeout.md)

## Goal

Make `forge-query` the true first-class platform entry for serious downstream
domain work so a domain can enter Forge once through Query and then progress
through declaration, legality, proof-bearing admission, foundational
description, authority routing, preparation, continuation, inspection, and
certification without rebuilding a local pseudo-Query seam above
`forge-relational`, `forge-runtime-bridge`, `forge-signal`, `forge-proof`, or
`forge-foundational`.

## Adversarial Constraint

A geometry-kernel-grade domain must be able to declare serious domain meaning,
prove declaration legality, carry provenance/support/explanation posture, and
prepare for later lower-authority routing through one Query-owned lifecycle.
Alternate authoring paths, alternate diagnostics richness, and alternate
checked/proof lanes must converge to the same canonical declaration meaning,
while wrong domain families, wrong handles, wrong verbs, and illegal
progression must fail before later routing, continuation, or runtime contact
can occur.

## Governing Summaries

- `MENTALITY.md`: the hard problem is the missing platform-entry boundary, not
  a missing helper. The first eight phases must build the boundary law before
  any convenience lowering.
- `arch_laws.md`: each phase must own a real artifact boundary and make the
  wrong thing unrepresentable, uncompilable, or explicitly denied.
- `composition_laws.md`: entry DX, canonical artifact formation, legality,
  proof progression, and foundational description are separate responsibilities
  and must stay separate in both the spec and tree shape.
- `domain_structure_laws.md`: Query-owned entry modules must remain visibly
  distinct from later relational-, bridge-, and signal-backed routing modules.
- `perf_laws.md`: declaration entry may not hide repeated canonicalization,
  repeated digest derivation, or repeated eligibility reconstruction behind
  cheap-looking common-lane APIs.
- `forge_query_vision.md`: Query is the daily-driver import and must own the
  public grammar even when lower crates remain authoritative.
- `forge_query_roadmap.md`: `9.3.8` now stands in the critical path before the
  runtime stabilization gate, so its early phases must be stable enough to
  anchor later routing and continuation work.
- `test-requirements.md`: every early phase needs typed denial lanes, parity
  lanes, compile-fail boundaries, and canonical bundle assertions rather than
  only happy-path examples.
- `milestone-9.3.6.md`: Query should reuse the route-plan / receipt / envelope
  lifecycle discipline even before actual lower-authority routing begins.

## Phase Authoring Rules

Every phase in this milestone is a boundary spec, not an implementation note.
That means each phase must carry:

- required artifacts
- locked references
- requirements
- compile-time enforcement
- acceptance evidence
- a `DX target` section

Documentation is not automatically a final-phase concern. Instead:

- if a phase changes or introduces a discoverable public surface, that phase
  must carry a `Documentation obligation`
- if a phase is purely internal and does not yet change the discoverable public
  product, it does not need phase-local docs work
- the final docs phase exists to close coverage, goldens, and teaching
  completeness, not to defer all documentation thought until the end

The same rule applies to DX:

- every phase must state the user-facing DX target for the boundary it closes
- later ergonomics phases may improve the experience substantially, but earlier
  phases are still required to describe the intended feel of the surface they
  introduce

## Decisions Locked Now

The following are not phase-local implementation details. They are milestone
shape decisions and must not be rediscovered later:

- `9.3.8` is one end-to-end platform-entry milestone, not three separate
  declaration/preparation/runtime mini-products.
- the ordinary public import path remains `forge_query::facade`; lower crates
  stay authoritative but are not the daily-driver entry surface.
- the primary ordinary lane must be typed and domain-first. Raw string domain
  ids may exist as compatibility or lower lanes, but they are not allowed to be
  the main public entry model.
- hidden ambient context may not creep back in through convenience APIs.
  Domain configuration, route intent, truth context, continuation mode,
  grouped declaration semantics, support/readiness queries, and recovery paths
  must all have typed input families underneath any ordinary-lane sugar.
- one canonical declaration identity must exist, and it must derive from
  foundational canonicalization rather than host-local hashing or lane-local
  builder shape.
- configured domain handles own stable admitted operating context only.
  Dynamic operation eligibility, exact preview/historical/runtime basis
  binding, and declaration-specific meaning must remain later-phase concerns.
- configured domain handle identity must include all typed stable operating
  context that changes the admitted world; equivalent contexts must converge
  canonically and materially different admitted worlds must diverge.
- one admitted declaration must yield one Query-owned route plan. Multiple
  lower-authority consequences are only allowed when the retained declaration
  family taxonomy explicitly admits mixed-authority or a later richer
  route-multiplicity posture; accidental widening is forbidden.
- caller-owned runtime builder assembly and caller-owned basis/preflight/
  historical-binding choreography are in-scope defects for this milestone, not
  later polish work.
- every real covered crossing must yield one Query-owned boundary receipt and
  one Query-owned boundary envelope.
- the lane hierarchy is mandatory: ordinary -> checked -> proof -> raw. No
  phase may collapse directly from ordinary into raw internals.
- support matrix breadth, crossing inventory breadth, docs coverage breadth,
  and certification bundle breadth must converge on the same live surface.
- relational remains the authority for truth semantics, bridge remains the
  authority for continuation/coordination semantics, signal remains the
  authority for derived execution semantics, `forge-proof` remains the owner of
  progression law, and `forge-foundational` remains the owner of shared
  boundary vocabulary.
- neighborhood and batch declaration surfaces are semantic grouping surfaces,
  not convenience `Vec<T>` overloads.

## Questions That May Stay Open Temporarily

The following may be decided phase-by-phase as long as they do not violate the
locked decisions above:

- exact Rust type names, trait names, and module names
- exact ordinary-lane verbs for each helper family
- whether a given internal phase needs one file or several modules
- the exact formatting of inspection/readiness reports
- the exact set of representative family examples used in docs and goldens
- whether adjacent phases should merge or split after hostile implementation QA,
  so long as the underlying boundary truths stay intact

## Phase Plan

1. **Phase 1: Public Domain Entry Boundary**
   Query gets one first-class typed domain entry surface. This is the moment domains stop starting outside the platform.

2. **Phase 2: Configured Domain Handle As Admitted Operating Context Boundary**
   Typed configured domain handles, their stable operating-context contract, and their validation/admission rules become explicit and sealed. This prevents stringly or bag-shaped domain entry while freezing one admitted operating world that later declaration work can honestly depend on.

3. **Phase 3: Canonical Declaration Artifact Boundary**
   Every domain declaration entering Query canonicalizes into one authoritative declaration artifact family. This is the "one meaning, one identity" boundary.

4. **Phase 4: Declaration Family Taxonomy Boundary**
   Query freezes the typed declaration family taxonomy: downstream semantic
   family identity plus Query-owned primary authority posture and modifiers.
   This stops generic-bag drift and prevents later phases from rediscovering
   family meaning from labels or payload folklore.

5. **Phase 5: Compile-Time Capability Matrix Boundary**
   Method presence and family eligibility become compile-time facts. This is where wrong families, wrong verbs, and wrong handles become unrepresentable or uncompilable.

6. **Phase 6: Declaration Legality Boundary**
   Query proves the declaration is structurally legal before richer progression continues. This is the declaration-side equivalent of Query's existing legality gates.

7. **Phase 7: Proof-Bearing Declaration Progression Boundary**
   Request, review, eligibility, admission, stale, rebind, denial, and stronger forms become a real `forge-proof` progression. This is where declaration entry stops being "typed helpers" and becomes a real proof chain.

8. **Phase 8: Foundational Description Boundary**
   Provenance, support, explanation, receipts, reports, summaries, artifacts, and equivalence vocabulary become first-class through `forge-foundational`. This is where declaration entry gains a shared descriptive language instead of local dialects.

9. **Phase 9: Query Route-Plan Boundary**
   Query owns an explicit route plan for every admitted declaration. This is where the platform decides which lower authority families are actually in play.

10. **Phase 10: Query Boundary-Receipt Boundary**
    Every real crossing produces a Query-owned boundary receipt. This is where declaration lowering becomes inspectable and certifiable as a public event, not an internal implementation detail.

11. **Phase 11: Query Boundary-Envelope Boundary**
    Query wraps route plan, receipt, foundational evidence, and denial topology into one self-describing declaration boundary envelope. This is the public crossing artifact boundary.

12. **Phase 12: Relational Truth-Routing Boundary**
    Query can now lower declaration families that are really about authoritative truth into relational-owned surfaces. This is where identity, lineage, invariants, history, merge/strategy truth become first-class routed declaration consequences.

13. **Phase 13: Bridge Continuation-Routing Boundary**
    Query can now lower declaration families that are really about continuation/coordination into bridge-owned surfaces. This is where preview, truth-view, basis, subscription, writeback, and cross-runtime continuity become first-class routed consequences.

14. **Phase 14: Signal Compatibility Boundary**
    Query freezes the exact continuation contract for later derived execution without yet pretending to execute through Signal here. This preserves a clean path into invalidation/recompute/observation later without semantic reset.

15. **Phase 15: Seam Classification Boundary**
    Every declaration-entry crossing gets classified as canonical reuse, Query boundary adapter, compatibility debt, deferred neighbor, or forbidden duplicate. This is where we stop tolerating ambiguous seams.

16. **Phase 16: Concrete Crossing Inventory Boundary**
    The milestone gains an executable inventory of all covered declaration-entry crossings. This is where we stop speaking in abstract families and name the actual public/internal seams.

17. **Phase 17: Cross-Authority Inspection Boundary**
    Query gets one unified inspection surface over declaration, route plan, receipt, envelope, relational lowering, bridge lowering, and later signal compatibility posture. This is where the platform becomes understandable.

18. **Phase 18: Support And Readiness Boundary**
    Query gets a first-class support/readiness story for declaration families and their routed authority posture. This is where "is this admitted/supported/deferred/denied here?" becomes a public product surface.

19. **Phase 19: Happy-Path Orchestration Boundary**
    Query becomes the compiler for the ordinary lane: strong defaults, short semantic verbs, automatic proof/routing prep, no caller-owned choreography. This is the Laravel boundary.

20. **Phase 20: Denial And Recovery UX Boundary**
    Typed denials, stale/rebind outcomes, fallback guidance, and route-sensitive explanations become product-quality. This is where failure becomes as usable as success.

21. **Phase 21: Family-Specific Ergonomics Boundary**
    The public lane gets the domain-shaped helpers for the major declaration families instead of forcing generic entry patterns everywhere. This is where the surface starts feeling native to real domain work.

22. **Phase 22: Neighborhood And Batch Declaration Boundary**
    Query supports meaningful groups of declarations as first-class units. This matters a lot for geometry because real work often happens in local neighborhoods, not isolated single declarations.

23. **Phase 23: Public Documentation And Golden Teaching Boundary**
    The docs, examples, and goldens all teach the exact public path honestly. This is where we make sure the platform is discoverable, not just implemented.

24. **Phase 24: Certification And Closeout Boundary**
    Compile-fail boundaries, parity tests, hostile certification, route/receipt/envelope digests, and end-to-end convergence proofs close the milestone. This is where the whole seam becomes production-grade rather than plausible.

## Phase Specifications

### Phase 1: Public Domain Entry Boundary

This phase establishes one Query-owned front door for serious domain work.
After it closes, a downstream domain is no longer expected to begin in a local
crate-specific declaration world and later "hand something to Query."

**Required Query artifacts**

- one ordinary-lane entry surface rooted under `forge_query::facade`
- one checked-lane entry outcome for typed denial and deferred posture
- one proof-lane root that later phases can strengthen without changing
  ordinary entry meaning
- one entry-surface support snapshot so callers can inspect what this Query
  build admits before authoring domain declarations

**Locked local surfaces**

- `forge_query::facade::ForgeQueryApplicationFacade`
- `forge_query::facade::ForgeQueryConfig`
- `forge_query::facade::ForgeQuerySupportMatrix`
- `forge_query::facade::ForgeQuerySupportReport`
- `forge_query::facade::ForgeQueryCapabilityResolution`

**Requirements**

- the ordinary lane must expose one obvious domain-first entry grammar and may
  not start from raw `String`, JSON, or map-shaped bags as the primary public
  authoring model
- Query owns the generic entry capability, progression reserve, and
  support/readiness posture; downstream crates own concrete domain identity
  through downstream-implemented marker types
- this phase may publicly commit to a generic marker trait, but it may not
  define or export Query-owned concrete domains now or later
- the public import path must stay inside `forge_query::facade`; no early
  phase may require direct imports from `forge-proof`, `forge-foundational`,
  `forge-relational`, `forge-runtime-bridge`, or `forge-signal`
- the phase must reserve named checked and proof continuations even if later
  phases define their full strengthening rules
- the public entry surface must emit support/readiness posture through Query's
  existing facade/support vocabulary rather than inventing a second readiness
  dialect

**DX target**

- the ordinary lane should feel like "enter the spatial/topology/domain world
  through Query" rather than "construct a Query substrate object"
- the first public touchpoint should make it obvious where serious domain work
  starts without making the user think about lower crates

**Documentation obligation**

- yes; this phase introduces the first discoverable platform-entry surface and
  therefore needs an explicit public entry doc and golden transcript once it
  lands

**Compile-time enforcement**

- direct construction of raw domain-entry artifacts outside Query-owned front
  doors must be impossible or crate-private
- ordinary entry must be anchored on downstream-owned typed domain markers or
  equivalent downstream-owned typed entry fronts, not unvalidated free-form
  strings as the primary lane
- `forge_query::facade` may export the generic marker contract, but it may not
  export real downstream domain nouns as if they were Query-owned concepts

**Acceptance evidence**

- golden DX transcripts for ordinary, checked, and proof entry roots
- compile-fail fixtures proving that internal entry constructors are not public
- compile-fail fixtures proving fake Query-owned concrete domain markers are not
  available from the facade
- parity assertions showing entry support/readiness inspection stays identical
  across equivalent ordinary and checked entry roots

**Open questions before implementation**

- the ordinary entry root is a method on `ForgeQueryApplicationFacade`; checked
  and proof siblings also live on the facade but do not displace the ordinary
  lane as the primary front door
- the public participation contract in this phase is a downstream-implemented
  `ForgeQueryDomainEntryMarker` trait; Query does not define concrete domain
  marker types
- checked entry posture uses a typed checked-entry family immediately rather
  than collapsing into `Result<T, E>` or waiting for raw declaration-phase
  strengthening later
- the minimum entry support/readiness snapshot in this phase is capability
  families plus config section posture and one stable digest

### Phase 2: Configured Domain Handle As Admitted Operating Context Boundary

This phase freezes the first Query-owned configured domain context that later
declaration work is allowed to depend on. The configured handle is not merely a
typed config wrapper. It is one admitted operating world composed from a
downstream-owned domain marker plus a downstream-owned operating-context input,
with Query owning the draft, validated, admitted, and checked lifecycle around
that pair.

**Required Query artifacts**

- one downstream-implemented operating-context contract for stable admitted
  regime inputs
- one Query-owned configured-handle draft family
- one Query-owned validated configured-handle family
- one Query-owned admitted configured-handle family
- one checked configured-handle denial family
- one stable digest for configured-handle identity across marker, operating
  context, and validated Query config posture

**Locked local surfaces**

- `forge_query::facade::ForgeQueryConfig`
- `forge_query::facade::ValidatedForgeQueryConfig`
- `forge_query::facade::ForgeQueryConfigSectionFamily`
- `forge_query::facade::ForgeQueryConfigSectionResolution`
- `forge_query::facade::ForgeQueryCapabilityRegistry`
- `forge_query::facade::ForgeQuerySupportMatrix`
- `forge_query::facade::ForgeQuerySupportReport`

**Requirements**

- Query owns the configured-handle lifecycle; downstream crates own concrete
  domain markers and concrete operating-context values
- the operating-context contract in this phase must stay declarative and narrow:
  it may name required capability families, required config sections, and one
  stable context-identity digest, but it may not smuggle in host-local policy
  callbacks, invariant callbacks, or builder-side ambient state
- the configured handle must represent stable admitted operating regime only.
  Allowed content includes policy/access class identity, invariant regime
  identity, assumption/tolerance regime identity, collaborator/tenant-like
  operating class identity when it changes the admitted world, and similar
  stable regime posture
- the configured handle must not carry declaration-specific meaning,
  per-operation trigger dependency graphs, exact preview/historical/runtime
  basis binding, raw collaborator/tenant/branch ids as Query authority, or bool
  shortcut flags such as `can_edit`, `preview`, or `physics_enabled`
- every configured handle must name the Query capability families and config
  sections that materially affect its admission
- handle construction must separate draft configured handle, validated
  configured handle, admitted configured handle, and checked denial posture
- this phase may reserve later relational/bridge/signal routing posture on the
  handle, but may not yet perform lower-authority routing
- configured-handle identity must include marker identity, all typed stable
  operating-context identity, required capability families, required config
  sections, operating-context digest, and validated Query config digest
- validation must remain Query-owned and structural only. It may canonicalize
  capability families, canonicalize required config sections, verify
  capability-to-section coverage, and stabilize identity; it may not pretend to
  evaluate downstream declaration semantics
- admission must deny early when required capability families are deferred or
  unsupported, when required config sections are disabled, or when the
  operating context asks for a regime Query cannot honestly admit yet
- canonicalization, digest derivation, and admission classification must happen
  once per configured-handle lifecycle and later phases must consume the
  validated/admitted proof-bearing handle rather than rediscovering those facts
  from raw marker/context inputs

**DX target**

- handles should read like stable admitted domain contexts, not service
  locators or raw config bags
- normal users should understand "I have a configured domain handle inside one
  operating regime" without needing to learn subsystem ownership details first

**Documentation obligation**

- yes; typed handles and their operating-context posture are public and need
  explicit usage guidance

**Compile-time enforcement**

- unvalidated configured handles may not masquerade as admitted configured
  handles
- Query-owned configured-handle wrappers must be sealed against downstream
  implementation even though the marker and operating-context contracts remain
  downstream-owned and open
- raw ids, bool shortcuts, and callback-shaped operating-context shortcuts may
  not become ordinary public Query authority lanes

**Acceptance evidence**

- compile-fail coverage for illegal direct handle construction
- checked-lane tests showing unsupported capability families deny handle
  admission before declaration authoring begins
- digest parity tests showing equivalent stable operating contexts yield one
  canonical configured-handle identity while materially different admitted
  worlds diverge
- adversarial tests proving stable regime inputs do not leak through ambient
  builder-side state once the typed operating-context input family exists
- denial tests proving deferred, unsupported, and invalid-context posture stop
  before declaration authoring exists
- counter or proof tests proving capability-family canonicalization,
  config-section canonicalization, digest derivation, and admission
  classification are not repeatedly rediscovered across the configured-handle
  lifecycle

**Open questions before implementation**

- should the operating-context contract stay as one shared generic contract or
  allow family-specific extensions while preserving one canonical lowering path?
- which stable regime facts belong in configured-handle identity from day one:
  policy/access class, invariant regime, assumption/tolerance regime,
  collaborator scope class, or a narrower initial subset?
- should admitted configured handles expose partial-support posture directly, or
  only through support/readiness inspection surfaces?

### Phase 3: Canonical Declaration Artifact Boundary

This phase creates the one authoritative declaration artifact family for domain
entry. From this point onward, alternate authoring paths may exist, but they
must canonicalize into one declaration meaning.

**Required Query artifacts**

- one raw declaration input family
- one canonical declaration artifact family
- one canonical declaration identity/digest
- one declaration comparison/equivalence surface for certification lanes

**Locked external surfaces**

- `forge_foundational::facade::CanonicalBasisFrontDoor`
- `forge_foundational::facade::prepare_canonical_basis_bundle`
- `forge_foundational::facade::derive_canonical_digest`
- `forge_foundational::facade::compare_canonical_basis`
- `forge_foundational::facade::CanonicalEquivalenceBasis`

**Requirements**

- canonical declaration formation must consume an admitted configured domain
  handle plus declaration-local input; it may not restart from bare domain
  markers or bypass the admitted operating-world proof established in Phase 2
- every domain declaration admitted by Query must lower into one canonical
  basis bundle before later legality, proof progression, or foundational
  description
- declaration identity must be derived from canonical basis preparation rather
  than host-local hashing rules
- declaration identity must compose handle-rooted admitted-world identity with
  declaration-local meaning; later phases may extend the proof chain around
  that identity, but they may not redefine the configured-handle contribution
- the ordinary lane may expose domain-first authoring verbs, but all authoring
  paths must converge before they can claim authoritative declaration meaning
- no later phase may redefine declaration identity or introduce a second
  authoritative declaration AST

**DX target**

- declaration authoring should feel flexible, but declaration identity should
  feel inevitable once meaning is fixed
- users should not need to care which authoring path they took in order to
  understand the resulting declaration artifact

**Documentation obligation**

- yes; canonicalization and equivalence are public meaning rules and cannot be
  left implicit

**Compile-time enforcement**

- canonical declaration artifacts must not be publicly mintable without
  passing through Query-owned canonicalization front doors
- comparison surfaces must consume canonical declaration artifacts or declared
  basis inputs, not arbitrary downstream structs

**Acceptance evidence**

- canonical digest parity across at least two semantically equivalent builder
  paths
- inequality assertions for intentionally different declaration meaning
- parity tests proving identical declaration-local input rooted in distinct
  admitted operating worlds does not collapse when the admitted world changes
  canonical meaning
- compile-fail coverage for direct artifact minting outside canonical front
  doors

**Open questions before implementation**

- should raw declaration input and canonical declaration artifacts be distinct
  public types in ordinary lanes, or should ordinary lanes mostly hide the raw
  form behind front doors?
- does family membership live structurally inside the canonical declaration
  artifact, or alongside it as a tightly-bound paired artifact before Phase 4
  fully freezes taxonomy?
- what comparison/equivalence helpers should be public now versus reserved for
  certification and inspection surfaces later?

### Phase 4: Declaration Family Taxonomy Boundary

This phase freezes the family system so later phases can gate verbs, legality,
progression, and lower-authority routing structurally instead of by informal
branching.

**Required Query artifacts**

- one sealed declaration family taxonomy
- one mapping from family to target authority class
- one mapping from family to later neighborhood/batch eligibility posture
- one public family inspection surface

**Locked authority references**

- `forge_relational::facade` reserved as the later authority for
  truth-bearing families
- `forge_runtime_bridge::facade` reserved as the later authority for
  continuation-bearing families
- `forge_signal::facade` reserved as the later authority for derived-execution
  compatible families

**Requirements**

- at minimum, the taxonomy must distinguish descriptive-only, relational-truth,
  bridge-continuation, mixed-authority, signal-compatible, and
  neighborhood-capable families
- the family system must be public enough for inspection and certification, but
  sealed enough that downstream code cannot invent unsupported family tags
- later phases may refine a family's internal states, but may not silently
  widen a family's authority class without a spec update
- family membership must become part of canonical declaration meaning before
  legality or proof progression begins

**DX target**

- the family model should make the surface feel more intentional, not more
  bureaucratic; the user should feel that Query knows what kind of declaration
  they are making
- wrong-family paths should feel absent, not merely "discouraged"

**Documentation obligation**

- yes; family distinctions are public product semantics and need explicit docs

**Compile-time enforcement**

- verbs unavailable to a family must be absent from the surface, not merely
  denied late at runtime
- families that are intentionally unsupported for a given handle must fail
  handle-plus-family composition before later legality or proof progression

**Acceptance evidence**

- compile-fail fixtures proving unavailable verbs are not callable on the wrong
  family
- certification bundles proving family tags participate in canonical
  declaration identity
- checked-lane parity showing equivalent declarations in the same family
  converge while cross-family declarations diverge

**Open questions before implementation**

- should declaration family taxonomy be modeled as a sealed enum, sealed trait
  family, or another typed representation that still supports compile-time verb
  gating cleanly?
- which family distinctions need first-class public names immediately versus
  being internal subcategories under the required top-level taxonomy?
- how should neighborhood-capable posture relate to family identity without
  prematurely collapsing grouped semantics into family tags alone?

### Phase 5: Compile-Time Capability Matrix Boundary

This phase makes family-taxonomy-aware eligibility, verb presence, and target
availability compile-time facts wherever the wrong combination is structurally
knowable.

**Required Query artifacts**

- one sealed capability matrix for `admitted handle x retained family taxonomy x
  verb`
- one checked-lane outcome family for data-dependent denials that remain after
  compile-time gating
- one support-matrix projection that exposes the same posture publicly

**Locked local surfaces**

- `forge_query::facade::ForgeQuerySupportMatrix`
- `forge_query::facade::ForgeQuerySupportReport`
- `forge_query::facade::ForgeQueryCapabilityRegistry`
- `forge_query::facade::ForgeQueryCapabilityResolution`

**Requirements**

- if an admitted-handle/retained-family-taxonomy/verb combination is invalid by
  admitted operating world, retained primary authority posture, retained
  modifier posture, or compile-time-visible route/continuation posture alone,
  it must be unrepresentable or uncompilable
- only data-dependent, policy-dependent, or declaration-content-dependent
  failures may survive into checked-lane denial outcomes
- family visibility may depend on admitted operating-world proof from Phase 2;
  taxonomy is global, but public family availability need not be
- compile-time gating must consume retained canonical declaration family proof
  from Phase 4 rather than rediscovering family meaning from semantic family
  keys, payload shape, or host-local routing heuristics
- the matrix must be inspectable through Query's public support/readiness
  surfaces so ordinary users can understand why something is unavailable
- the matrix must reserve later authority-routing eligibility columns for
  relational, bridge, and signal without performing the routing yet

**DX target**

- unsupported combinations should mostly disappear from the surface before the
  user can make a mistake
- when something is unavailable, the checked lane should explain that clearly
  without making the user feel they fell into internals

**Documentation obligation**

- yes; the support/readiness story is public and needs to be taught where it
  affects public entry choices

**Compile-time enforcement**

- wrong-family-taxonomy verbs, wrong-handle verbs, and wrong-target verbs must
  have UI compile-fail coverage
- wrong operating-world verbs and wrong continuation-mode verbs must also have
  compile-fail coverage wherever the mismatch is structurally knowable
- support-matrix posture may not disagree with actual method presence on the
  public lane

**Acceptance evidence**

- compile-fail suites for illegal handle/family-taxonomy/verb combinations
- parity tests between support-matrix rows and real public method presence
- checked-lane denial tests proving only content-dependent and policy-dependent
  failures survive past compile-time gating

**Open questions before implementation**

- should the capability matrix be materialized as explicit Rust marker/state
  types, generated code, table-driven compile-time traits, or a hybrid?
- which capability decisions are truly compile-time knowable versus requiring a
  checked-lane runtime/config denial even after type selection?
- how should support-matrix rows map back to the capability matrix so docs and
  compile-fail fixtures stay synchronized?

### Phase 6: Declaration Legality Boundary

This phase proves the declaration is structurally legal before it can enter the
stronger proof-bearing progression. It is the declaration-entry equivalent of
Query's earlier legality gates, and it must consume retained family-taxonomy
proof rather than reclassifying declarations ad hoc.

**Required Query artifacts**

- one legality input artifact family over canonical declaration artifacts
- one legality verdict family with non-binary denial causes
- one legality evidence artifact that later proof phases can consume

**Locked external surfaces**

- `forge_foundational::facade::evaluate_boundary_role_claim_legality`
- `forge_foundational::facade::evaluate_boundary_surface_disposition_legality`
- `forge_foundational::facade::FoundationalBoundaryRoleClaim`
- `forge_foundational::facade::FoundationalBoundarySurfaceDisposition`
- `forge_foundational::facade::FoundationalBoundaryDecisionCause`

**Requirements**

- legality must consume canonical declaration artifacts rooted in admitted
  configured handles; it may not evaluate declarations in a worldless vacuum
- legality must run after canonical declaration formation and family taxonomy
  freeze, but before proof-bearing admission and foundational materialization
- legality must consume the retained semantic family key plus Query-owned
  taxonomy posture from Phase 4; it may not rediscover authority class,
  grouped posture, or signal posture from payload inspection or route-local
  heuristics
- legality must distinguish unsupported structure, illegal authority claims,
  illegal disposition claims, and later-content-dependent denials rather than
  collapsing all failure into one generic rejection
- later phases must consume legality evidence instead of recalculating the same
  role/disposition judgments from scratch
- legality must remain Query-owned even when it delegates specific boundary
  legality vocabulary to `forge-foundational`
- legality must stop at structural legality. Dynamic per-operation eligibility,
  live truth predicates, preview/historical basis sensitivity, and runtime
  trigger conditions remain later-phase concerns even when the admitted
  operating world constrains the legal space

**DX target**

- legality should feel like an honest gate, not arbitrary framework rejection
- the user should be able to tell whether they made an illegal declaration or
  simply asked for something unsupported

**Documentation obligation**

- yes; legality versus unsupported posture is a public distinction and needs
  explicit examples

**Compile-time enforcement**

- legality evidence artifacts must not be publicly constructible without a real
  legality pass
- checked-lane outcomes must preserve typed denial variants instead of
  degrading to `Result<T, String>`

**Acceptance evidence**

- hostile legality suites with equivalent declarations across alternate builder
  paths
- typed-failure assertions for role/disposition violations
- proofs that later phases consume legality evidence rather than re-running
  structural legality ad hoc

**Open questions before implementation**

- should legality evidence be a standalone artifact family or a strengthening
  wrapper around canonical declarations?
- what is the exact split between legality denials that belong here versus
  capability/readiness denials that should stay in Phase 5 or Phase 18?
- how much foundational legality vocabulary should be re-exposed directly in
  Query types versus wrapped in Query-owned denial/readout families?

### Phase 7: Proof-Bearing Declaration Progression Boundary

This phase turns declaration entry into a real `forge-proof` progression rather
than a chain of typed helper structs.

**Required Query artifacts**

- one declaration recipe/progression family rooted in `forge-proof`
- one checked outcome family that surfaces `ProofOutcomeKind` honestly
- one strengthening path from raw declaration input to admitted declaration

**Locked external surfaces**

- `forge_proof::facade::create`
- `forge_proof::facade::recipe`
- `forge_proof::facade::sym`
- `forge_proof::facade::gate_ready`
- `forge_proof::facade::ready_now`
- `forge_proof::facade::proof_flow`
- `forge_proof::facade::ProofFlow`
- `forge_proof::facade::ProofOutcome`
- `forge_proof::facade::ProofOutcomeKind`
- `forge_proof::facade::RecipeStageKind`

**Requirements**

- the phase sequence must at minimum represent declaration request,
  legality-cleared declaration, review/eligibility, admitted declaration, and
  typed stale/rebind/denied branches where applicable
- the proof chain must inherit admitted configured-handle identity and
  operating-world proof from Phase 2 rather than re-binding it as ambient
  metadata on later declaration stages
- Query may wrap proof surfaces for DX, but it may not replace
  `forge-proof` with a Query-local typestate imitation
- checked outcomes must preserve the next lower proof truth so callers can
  branch honestly without dropping immediately to raw proof APIs
- proof-bearing declaration artifacts must be stable inputs to later
  foundational description and route-plan phases

**DX target**

- ordinary users should benefit from proof-backed progression without having to
  think in proof jargon
- advanced users should be able to drop to checked/proof lanes and still feel
  like they are in one coherent progression model

**Documentation obligation**

- yes; the ordinary/checked/proof relationship is a core public teaching
  concern

**Compile-time enforcement**

- stronger declaration states must be unforgeable outside Query-owned and
  `forge-proof`-owned strengthening APIs
- out-of-order progression must be uncompilable or sealed behind typed denial
  outcomes

**Acceptance evidence**

- parity suites showing ordinary and proof-lane progression converge to the
  same admitted declaration meaning
- proof-chain parity tests proving declaration progression preserves admitted
  configured-handle identity rather than re-deriving it from raw inputs
- compile-fail tests for illegal out-of-order strengthening
- checked-lane tests proving stale, rebind, and denial variants remain typed
  and inspectable

**Open questions before implementation**

- what is the smallest proof-stage family that still captures the required
  declaration request, legality-cleared, reviewed, admitted, stale, rebind, and
  denied distinctions honestly?
- should the ordinary lane surface one main checked outcome family throughout
  progression, or should each major phase expose its own checked outcome type?
- where should phase-local convenience wrappers stop so Query does not
  accidentally imitate `forge-proof` instead of reusing it?

### Phase 8: Foundational Description Boundary

This phase gives domain entry one shared descriptive language for provenance,
support, explanation, attachments, receipts, and canonical evidence bundles.

**Required Query artifacts**

- one declaration-entry boundary evidence bundle
- one provenance surface for declaration origin and carry-forward posture
- one support surface for declaration-time support/readiness disclosure
- one receipt/report/summary distinction for the entry lifecycle
- one canonical digest derivation surface for foundational evidence bundles

**Locked external surfaces**

- `forge_foundational::facade::FoundationalBoundaryEvidenceFrontDoor`
- `forge_foundational::facade::FoundationalBoundaryEvidenceProvenanceFrontDoor`
- `forge_foundational::facade::FoundationalBoundaryEvidenceSupportFrontDoor`
- `forge_foundational::facade::FoundationalBoundaryEvidenceReceiptFrontDoor`
- `forge_foundational::facade::prepare_boundary_evidence_attachment_bundle_for_canonical_basis`
- `forge_foundational::facade::derive_boundary_evidence_attachment_bundle_digest`
- `forge_foundational::facade::FoundationalBoundaryEvidenceAttachmentBundle`
- `forge_foundational::facade::FoundationalBoundaryEvidenceProvenanceArtifact`
- `forge_foundational::facade::FoundationalBoundaryEvidenceSupportAttachment`
- `forge_foundational::facade::FoundationalBoundaryEvidenceReceiptKind`

**Requirements**

- declaration entry must speak in foundational receipt/report/support/provenance
  vocabulary rather than inventing a Query-local descriptive dialect
- the phase must materialize enough foundational evidence that later route
  planning, receipts, and envelopes can compose it rather than recreate it
- support/readiness disclosures emitted here must line up with Query's public
  support-matrix story from Phases 1, 2, and 5
- foundational evidence digests must become part of later certification bundles
  and parity suites

**DX target**

- provenance, support, and receipt language should make the system easier to
  trust rather than more abstract
- users should feel like they can explain what happened without learning a
  second private vocabulary

**Documentation obligation**

- yes; this is the first phase where the shared descriptive language becomes a
  real public contract

**Compile-time enforcement**

- foundational evidence bundles must be built through Query-owned and
  foundational front doors, not by public struct literals
- receipt/report/summary surfaces must remain distinct artifact families, not
  aliases over one bag-shaped payload

**Acceptance evidence**

- canonical digest parity for equivalent foundational evidence bundles
- compile-fail tests for direct public construction of evidence artifacts
- parity suites proving ordinary, checked, and proof declaration paths yield
  the same foundational provenance/support/receipt meaning when semantically
  equivalent

**Open questions before implementation**

- what is the minimum foundational evidence bundle that later route plans,
  receipts, and envelopes can consume without forcing re-materialization?
- should provenance, support, and receipt/report/summary surfaces be separate
  top-level artifacts from the start or grouped behind one declaration-entry
  evidence bundle with typed views?
- how should foundational evidence digests compose with canonical declaration
  digests so later certification can compare both cleanly?

### Phase 9: Query Route-Plan Boundary

This phase introduces the first explicit crossing artifact for domain entry:
the Query-owned route plan that decides which lower-authority families are in
play for an admitted declaration and why, using retained declaration family
taxonomy proof rather than family-label folklore.

**Required Query artifacts**

- one declaration route-plan artifact family
- one route-plan input family over admitted declarations plus foundational
  evidence bundles
- one route-plan classification vocabulary for later relational, bridge,
  signal-compatible, mixed, deferred, and forbidden routes
- one route-plan digest and inspection surface
- one typed route-intent input family for declaration families whose routing
  posture is caller-meaningful rather than purely inferred

**Locked local and adjacent references**

- Query must reuse the route-plan posture established in
  [milestone-9.3.6.md](./milestone-9.3.6.md)
- route planning must consume outputs from Phase 7 admitted declarations and
  Phase 8 foundational evidence bundles
- reserved authority targets remain `forge_relational::facade`,
  `forge_runtime_bridge::facade`, and `forge_signal::facade`

**Requirements**

- every admitted declaration family taxonomy posture must lower into exactly
  one explicit Query route-plan artifact before any lower-authority receipt or
  continuation can be claimed
- a route plan may reference more than one lower authority family only when the
  retained declaration family taxonomy is explicitly typed as mixed-authority
  or another later route-multiplicity posture that the Query taxonomy admits by
  spec; otherwise one declaration maps to one lower-authority family or one typed
  deferred/forbidden posture
- route plans must distinguish "no lower authority yet", "relational-backed",
  "bridge-backed", "mixed-authority", "signal-compatible later", "deferred
  neighbor", and "forbidden" as separate public postures
- route planning must preserve whether continuation participation is required,
  optional, deferred, or absent whenever that distinction is part of the
  retained declaration family's typed posture; Query must not collapse those
  cases into one coarse "mixed-authority" bucket
- route planning must consume retained semantic family identity and retained
  Query taxonomy posture directly from canonical declaration artifacts; it may
  not infer lower-authority class from family labels alone or from declaration
  payload structure
- route planning must consume admitted operating-world proof from Phase 2 plus
  admitted declaration proof from earlier phases; route intent may narrow or
  select crossings within that world, but it may not redefine the world itself
- route planning must remain Query-owned orchestration, not a restatement of
  lower-crate semantics
- route plans must be derived once from admitted declaration meaning and
  foundational evidence; later phases may consume them but not re-decide them
- declarations that can continue into Query runtime must carry one explicit
  runtime-continuation preparation posture in the route plan rather than
  leaving runtime builder assembly or basis-binding choreography to caller-owned
  code
- where callers are allowed to distinguish truth-only, continuation-bearing, or
  mixed-authority intent, that distinction must travel through a typed
  route-intent input family rather than ambient method choice or comment-level
  convention
- route intent and operating context are distinct typed inputs. The former asks
  what crossing is desired from within an admitted world; the latter defines the
  admitted world itself
- route plans must remain explicit about route multiplicity and continuation
  participation for geometry-style or other multi-seam domains whose later
  routing consequences cannot be described honestly by a single yes/no mixed
  flag alone

**DX target**

- users should be able to understand "where Query plans to send this" without
  having to know how to call lower crates directly
- route plans should make the system feel legible, not expose implementation
  clutter

**Documentation obligation**

- yes; route plans are public inspection artifacts and need concise teaching

**Compile-time enforcement**

- lower-authority receipt/envelope APIs must not accept raw declarations in
  place of route plans
- route-plan artifact construction must remain behind Query-owned front doors

**Acceptance evidence**

- parity suites showing equivalent admitted declarations produce the same route
  plan
- inequality assertions for declarations whose retained family taxonomy or
  authority posture differs intentionally
- compile-fail coverage proving later receipt/envelope phases cannot skip the
  route-plan artifact
- representative continuation tests proving runtime-capable route plans encode
  enough preparation posture to remove caller-owned runtime assembly sequencing
  from downstream domain code

**Open questions before implementation**

- should route intent be a standalone typed input family, a property of certain
  declaration families, or both with one canonical lowering path?
- what is the smallest route-plan artifact shape that can still carry runtime-
  continuation preparation posture without becoming a second envelope too early?
- should continuation participation and route multiplicity remain simple public
  postures on route plans, or should later phases promote one or both into
  richer route-contract artifacts once mixed-authority domains need more than a
  coarse flag?
- how should deferred and forbidden route rows be represented so they stay
  certifiable without imitating successful lower-authority routes?

### Phase 10: Query Boundary-Receipt Boundary

This phase makes every real declaration-entry crossing produce a Query-owned
boundary receipt rather than an invisible implementation transition.

**Required Query artifacts**

- one boundary-receipt artifact family keyed by declaration route plan
- one receipt-kind vocabulary for relational, bridge, mixed, deferred, and
  denied crossings
- one receipt integrity/digest surface
- one checked-lane receipt denial family

**Locked external surfaces**

- `forge_foundational::facade::boundary_receipt_definition`
- `forge_foundational::facade::FoundationalBoundaryReceiptSurface`
- `forge_foundational::facade::FoundationalBoundaryEvidenceReceiptFrontDoor`
- `forge_foundational::facade::FoundationalBoundaryEvidenceReceiptKind`

**Requirements**

- every admitted crossing covered by this milestone must emit one Query-owned
  receipt artifact even when the lower authority also emits its own local
  evidence
- receipts must say what crossed, why it was admitted, which route-plan row it
  followed, and which authority family it touched
- deferred-neighbor and forbidden crossings must still produce Query-owned
  typed receipt/denial posture rather than silent absence
- receipts must be inspectable without lower-crate spelunking
- runtime continuation receipts must record whether workspace/runtime
  preparation and current/historical basis binding were Query-owned outcomes
  rather than caller-owned choreography

**DX target**

- every crossing should leave behind something the user can point to and say
  "this is what happened"
- receipt language should feel operational and concrete, not ceremonial

**Documentation obligation**

- yes; receipts are public-facing artifacts and need examples

**Compile-time enforcement**

- route-plan consumers that claim successful crossing must return a receipt
  artifact family, not `()` or free-form text
- receipt kinds must be typed rather than host strings

**Acceptance evidence**

- parity suites showing equivalent route plans emit equivalent receipts
- typed denial assertions for denied and deferred crossing kinds
- compile-fail coverage proving public crossing APIs cannot claim success
  without a receipt artifact
- continuation tests proving runtime-facing receipts expose runtime preparation
  and basis-binding admission as first-class receipt facts

**Open questions before implementation**

- should boundary receipts be one unified typed family with variants or several
  tightly-related receipt families keyed by crossing class?
- how much lower-authority detail belongs in a receipt versus being deferred to
  envelopes and inspection surfaces?
- what is the minimum receipt identity needed so equivalent crossings converge
  without erasing meaningful route differences?

### Phase 11: Query Boundary-Envelope Boundary

This phase wraps declaration meaning, route plan, foundational evidence, and
boundary receipt into one self-describing public envelope.

**Required Query artifacts**

- one declaration boundary-envelope family
- one envelope digest surface
- one envelope inspection surface
- one envelope comparison basis for certification

**Locked external surfaces**

- envelope posture must follow the public crossing lifecycle taught by
  [milestone-9.3.6.md](./milestone-9.3.6.md)
- foundational attachments and receipts must reuse Phase 8 surfaces rather than
  introducing a second descriptive bundle format

**Requirements**

- envelopes must contain, at minimum: declaration identity, family posture,
  legality posture, admitted progression posture, route-plan posture, receipt
  posture, and foundational evidence posture
- envelopes must become the primary public artifact for later inspection,
  diagnostics, and certification
- lower crates may remain authoritative for their own evidence, but Query must
  own the envelope that composes those facts into one public crossing story
- envelopes must remain self-describing enough to support hostile certification
  without ambient reconstruction
- runtime-capable envelopes must make runtime/workspace preparation and
  current/historical basis-binding outcomes visible without requiring
  downstream glue code to reconstruct those facts

**DX target**

- envelopes should feel like the one thing a user can inspect or pass around to
  understand the crossing
- users should not need to mentally assemble route plan + receipt + evidence by
  hand

**Documentation obligation**

- yes; envelopes are a major public artifact and need explicit docs

**Compile-time enforcement**

- later inspection/certification surfaces must consume envelopes or stronger
  derived artifacts, not ad hoc tuples
- public envelope construction must stay behind Query-owned front doors

**Acceptance evidence**

- canonical digest parity for equivalent envelopes across alternate ordinary
  and proof lanes
- inequality assertions for envelopes whose route-plan or receipt posture
  differs semantically
- compile-fail coverage for direct envelope minting outside Query-owned
  boundaries
- continuation tests proving runtime-capable envelopes can explain runtime
  preparation and basis-binding outcomes without consulting local runtime glue

**Open questions before implementation**

- should envelopes store foundational evidence by value, by digest plus lookup
  reference, or through typed attachment handles?
- how much route/receipt detail should be normalized into the envelope itself
  versus accessed through envelope-backed inspection views?
- what comparison basis should the envelope use so certification can compare
  semantically equivalent crossings across ordinary and proof lanes cleanly?

### Phase 12: Relational Truth-Routing Boundary

This phase turns relational-backed declaration families into real public
Query-mediated crossings rather than future intentions.

**Required Query artifacts**

- one relational-routing declaration family set
- one relational route-plan lowering family
- one relational boundary-receipt and envelope population path
- one relational support/readiness posture row set

**Locked external surfaces**

- `forge_relational::facade::history`
- `forge_relational::facade::grouped_truth`
- `forge_relational::facade::commit_strategies`
- `forge_relational::facade::runtime`
- `forge_relational::facade::bridge::RuntimeBridgeRelationalSource`

**Requirements**

- truth-bearing declaration families must lower through Query into explicit
  relational-backed routes rather than making callers import relational
  directly
- this phase must explicitly cover relational families for identity, lineage,
  historical truth, invariant-bearing truth, grouped/materialized truth, and
  strategy-backed truth where those meanings belong to relational authority
- Query may compile onto relational APIs, but it may not redefine truth,
  lineage, invariant, or commit-strategy semantics locally
- relational-backed receipts and envelopes must preserve which relational
  authority family was reached

**DX target**

- relational-backed declarations should still feel like Query features, not
  like accidental wrappers over relational internals
- users should gain relational power without being forced to think in
  relational-first API choreography

**Documentation obligation**

- yes; once Query exposes relational-backed families, the docs need to teach
  what truth claims those families actually make

**Compile-time enforcement**

- declaration families that are not relational-truth-bearing must not expose
  relational-routing verbs or route-plan outcomes
- route-plan rows marked relational-backed must only lower through the named
  relational facade families, not internal crate-private shortcuts

**Acceptance evidence**

- parity suites showing equivalent relational-backed declarations converge to
  the same relational route-plan/receipt/envelope posture
- compile-fail coverage for wrong-family relational routing
- support/readiness tests proving relational-backed posture is public and
  accurate before runtime execution

**Open questions before implementation**

- should identity, lineage, history, invariant, grouped-truth, and strategy-
  backed relational routes be one relational family with typed submodes or
  several public relational route families?
- which relational-backed declaration shapes should surface as ordinary-lane
  verbs first versus remaining checked/proof-only until later?
- how should Query preserve relational power without leaking relational-first
  API choreography into the ordinary lane?

### Phase 13: Bridge Continuation-Routing Boundary

This phase turns bridge-continuation declaration families into real
Query-mediated bridge crossings, and it must consume retained taxonomy proof
instead of reinterpreting declarations by name or payload.

**Required Query artifacts**

- one bridge-routing declaration family set
- one bridge route-plan lowering family
- one bridge boundary-receipt and envelope population path
- one bridge support/readiness posture row set
- one Query-owned runtime continuation/preparation artifact family for bridge-
  routed declarations that continue into runtime-backed workspaces or
  basis-bound reads
- one typed continuation-request family covering runtime, workspace, preview,
  truth-view, current-snapshot, and historical continuation modes
- one typed truth-context family covering current, historical, and preview
  worlds for continuation-bearing declarations

**Locked external surfaces**

- `forge_runtime_bridge::facade::RuntimeBridge`
- `forge_runtime_bridge::facade::BridgeRouteRequest`
- `forge_runtime_bridge::facade::BridgeTruthViewEvaluationRequest`
- `forge_runtime_bridge::facade::BridgeSpeculativeSessionRequest`
- `forge_runtime_bridge::facade::BridgeSpeculativePromotionRequest`
- `forge_runtime_bridge::facade::HistoricalEvaluationDeclaration`
- bridge subscription and writeback facade exports re-exported from
  `forge_runtime_bridge::facade`

**Requirements**

- continuation-bearing declaration families must lower through Query into
  explicit bridge-backed routes for preview, truth-view, basis, subscription,
  writeback, and cross-runtime continuity semantics
- Query must mediate the bridge crossing, but the bridge remains authoritative
  for continuation/coordination semantics
- bridge routing must consume the canonical declaration's retained family
  taxonomy and the Phase 9 route plan rather than rediscovering continuation
  class from semantic family labels or declaration payload structure
- bridge continuation artifacts must consume admitted configured-handle proof as
  prior operating-world authority, not reconstruct policy/access/invariant
  posture from ad hoc declaration metadata
- bridge-backed receipts and envelopes must preserve which bridge family was
  reached and which continuation posture was admitted or denied
- this phase must not hide bridge-sensitive differences behind one fake
  universal continuation family
- bridge continuation artifacts must preserve whether continuation
  participation was required, optional, or deferred for the routed declaration
  family whenever Phase 9 route plans carry that distinction
- Query must provide bridge-routed continuation artifacts that replace
  caller-owned runtime builder assembly and caller-owned current/historical
  basis-binding choreography for supported runtime-capable declaration families
- current-snapshot, historical, preview, and subscription-basis continuation
  must all be expressible through Query-owned bridge continuation artifacts
  rather than downstream glue code
- continuation mode and truth-context differences must enter Query as typed
  inputs, not as ambient builder flags, optional tokens, or host-local control
  flow around a generic continuation method

**DX target**

- bridge-backed declarations should feel like powerful continuation features
  available through Query, not like users are being pushed into bridge
  specialist mode
- the surface should make preview, truth-view, subscription, and writeback
  differences understandable without overwhelming the user
- runtime-capable bridge paths should read like "continue into
  runtime/workspace/basis" rather than "assemble backend parts and bind a basis
  context yourself"

**Documentation obligation**

- yes; these families change discoverable public behavior and need explicit
  docs with examples

**Compile-time enforcement**

- non-continuation families must not expose bridge-routing verbs
- bridge-backed routes must consume route plans rather than ambient
  caller-owned bridge setup
- runtime continuation verbs may appear only for declaration families whose
  route plans and support matrices admit Query-owned runtime preparation and
  basis-binding continuation
- truth-context and continuation-request families must gate which continuation
  verbs and bridge families are visible at compile time

**Acceptance evidence**

- parity suites showing equivalent bridge-backed declarations converge to the
  same bridge route-plan/receipt/envelope posture
- typed denial assertions for unsupported continuation families
- compile-fail coverage proving direct bridge contact is not the ordinary lane
- replacement tests proving representative runtime-capable domain flows no
  longer require caller-owned `ForgeQueryRuntime::builder()` assembly or
  caller-owned basis/preflight/historical binding choreography once they pass
  through Query-owned bridge continuation artifacts

**Open questions before implementation**

- should runtime continuation, preview, truth-view, subscription, and writeback
  be one continuation-request family with typed modes or several sibling
  families sharing a common trait?
- should truth context be a separate typed input family from continuation
  request, or should continuation requests carry truth-context variants
  structurally?
- if later mixed-authority or geometry-heavy families need richer route-
  multiplicity or continuation-participation contracts, should Phase 13 consume
  those as route-plan postures or promote them into first-class bridge route
  contract artifacts?
- what is the minimal Query-owned runtime continuation artifact that fully
  replaces local runtime builder and basis-binding choreography without hiding
  bridge-specific differences?

### Phase 14: Signal Compatibility Boundary

This phase freezes the declaration-to-derived-execution continuation contract
without yet claiming that `9.3.8` executes those declarations through Signal.

**Required Query artifacts**

- one signal-compatibility contract family
- one signal-eligibility posture on route plans and envelopes
- one signal-compatibility support/readiness inspection surface

**Locked external surfaces**

- `forge_signal::facade::runtime`
- `forge_signal::facade::diagnostics`
- `forge_signal::facade::history`

**Requirements**

- this phase must state which declaration families are later allowed to
  continue into signal-backed derived execution and which are not
- signal compatibility must remain a typed public contract, not a note in
  prose or a host-local convention
- no public API in this phase may imply that `9.3.8` already owns signal
  execution semantics
- the route plan and envelope must preserve whether a declaration is
  signal-compatible, signal-incompatible, or signal-deferred
- signal compatibility must remain a modifier over the declaration family's
  primary lower-authority posture rather than being reinterpreted as a peer
  authority family in later phases
- later derived-execution continuation must consume typed continuation and
  truth-context inputs rather than ambient execution-mode switches bolted on
  after declaration entry

**DX target**

- users should be able to tell whether a declaration can later continue into
  derived execution without mistaking that for immediate signal execution
- compatibility should feel like a real contract, not a vague future promise

**Documentation obligation**

- yes; compatibility contracts are public and should be explained where they
  affect user expectations

**Compile-time enforcement**

- families that are not signal-compatible must not claim signal continuation
  flags or methods
- signal-compatibility posture must be derivable from typed family/capability
  state, not from free-form tags

**Acceptance evidence**

- compile-fail tests for illegal signal-compatibility claims
- parity tests showing equivalent declarations expose identical
  signal-compatibility posture
- denial tests for families intentionally excluded from signal continuation

**Open questions before implementation**

- should signal compatibility be modeled as a simple typed posture on route
  plans/envelopes or as a richer contract artifact with its own identity and
  inspection view?
- which continuation and truth-context inputs need to be preserved verbatim so
  later signal execution can resume without semantic reinterpretation?
- how should signal-deferred posture differ from signal-incompatible posture in
  public checked/readiness surfaces?

### Phase 15: Seam Classification Boundary

This phase forces every declaration-entry crossing to be classified explicitly
so the milestone cannot hide ambiguous or convenience-driven seams.

**Required Query artifacts**

- one seam classification enum/family
- one classification row per covered crossing
- one public inventory surface exposing those rows

**Locked classification vocabulary**

- `canonical reuse`
- `Query boundary adapter`
- `compatibility debt`
- `deferred neighbor`
- `forbidden duplicate`

**Requirements**

- every crossing covered by Phases 9 through 14 must have exactly one seam
  classification row
- classification rows must say which lower crate owns the semantics, what Query
  owns, and what exit criteria apply if the seam is debt or deferred
- no direct lower-crate path may remain uncategorized
- classification must become part of the certification and documentation story
  later in the milestone

**DX target**

- this phase is mostly internal, but its outcome should make future public docs
  and inspection read more honestly
- users should indirectly benefit from cleaner seam naming even if they never
  see the raw classification table

**Documentation obligation**

- no public feature doc required at this phase by default; classification is
  primarily internal until surfaced through later inspection/docs phases

**Compile-time enforcement**

- adding a covered crossing without classifying it must fail the inventory or
  compile-boundary checks
- forbidden-duplicate seams must not have ordinary-lane entry points

**Acceptance evidence**

- executable inventory checks proving every covered crossing has one
  classification
- compile-fail or harness-fail coverage for uncategorized seam additions
- certification parity showing classification posture aligns with actual route
  plan and receipt behavior

**Open questions before implementation**

- should seam classification live as code-level typed rows, generated metadata,
  or certification-harness fixtures that are checked against code?
- how much of the classification table should be inspectable publicly versus
  reserved as internal certification metadata?
- what is the exact exit-criteria shape for `compatibility debt` and `deferred
  neighbor` rows so they remain actionable rather than decorative?

### Phase 16: Concrete Crossing Inventory Boundary

This phase replaces abstract family talk with a named, executable inventory of
the actual public and internal crossings that `9.3.8` covers.

**Required Query artifacts**

- one crossing inventory row family
- one locked covered-crossing table
- one digest over the covered inventory for certification
- one support matrix projection over the same inventory

**Locked inventory content**

- every ordinary-lane entry surface added in Phases 1 through 15
- every checked/proof lane surface that can reach a lower-authority crossing
- every relational-backed and bridge-backed route-plan/receipt/envelope family
- every signal-compatibility contract row

**Requirements**

- each row must name the public entrypoint, declaration family, handle posture,
  route-plan class, receipt family, envelope family, seam classification, and
  owner crate
- the inventory must be executable and certification-readable rather than a
  prose appendix only
- support/readiness posture must be derivable against the same row set
- later docs and hostile certification must use this inventory as the source
  of truth for coverage

**DX target**

- this phase is mostly internal, but it should guarantee that the public story
  stays synchronized with the actual surface
- users should benefit later by not encountering undocumented or
  inconsistently-supported surfaces

**Documentation obligation**

- no standalone public doc required at this phase, but the inventory becomes a
  mandatory backing source for later docs and goldens

**Compile-time enforcement**

- inventory rows for public surfaces must stay synchronized with live APIs
- adding a new covered crossing without updating the inventory must fail the
  certification or compile-boundary harness

**Acceptance evidence**

- digest parity over the covered crossing inventory
- compile-fail or harness-fail checks for unsynchronized public surfaces
- certification bundles proving the live ordinary lane breadth matches the
  inventory exactly

**Open questions before implementation**

- should the crossing inventory be hand-authored, generated from typed route
  definitions, or assembled through a hybrid model?
- what row granularity is sufficient to keep coverage honest without exploding
  the inventory into low-value duplication?
- how should inventory rows map back to support matrix rows, docs, and golden
  transcripts so synchronization stays cheap enough to maintain?

### Phase 17: Cross-Authority Inspection Boundary

This phase makes the full platform-entry seam understandable through one
Query-owned inspection surface instead of requiring callers to inspect each
lower crate independently.

**Required Query artifacts**

- one unified declaration-entry inspection surface
- one inspection result family spanning declaration, route plan, receipt,
  envelope, relational posture, bridge posture, and signal-compatibility
  posture
- one inspection digest/evidence surface for certification

**Locked local and external surfaces**

- Query inspection should follow the public-inspection posture already taught by
  the runtime and `milestone-9.3.1.md`
- bridge-backed explanation surfaces must compose through
  `forge_runtime_bridge::facade` diagnostics exports
- signal-backed compatibility explanations must remain compatible with
  `forge_signal::facade::diagnostics`
- foundational evidence and receipt surfaces from Phase 8 and Phase 10 remain
  the descriptive substrate

**Requirements**

- one inspection call must be able to explain what was declared, what family it
  belongs to, what legality and progression posture it achieved, how it routed,
  what lower authority it touched, and what remains deferred or denied
- inspection must remain a public product surface, not a debug-only or
  certification-only artifact
- Query may compose lower-crate evidence, but the final inspection result must
  be Query-shaped and stable
- inspection must not silently erase authority distinctions in the name of one
  pretty summary

**DX target**

- inspection should feel like a superpower: one place to ask what happened and
  why
- users should be able to trust Query inspection instead of spelunking lower
  runtimes

**Documentation obligation**

- yes; inspection is a major public contract and needs strong docs and
  examples

**Compile-time enforcement**

- inspection surfaces must consume typed route/receipt/envelope artifacts, not
  free-form logs or host-assembled summaries
- ordinary-lane inspection must remain available without importing lower-crate
  inspection helpers directly

**Acceptance evidence**

- parity suites showing equivalent paths yield identical unified inspection
  results
- hostile inspection cases proving intentionally different route, receipt, or
  denial posture remains visible
- compile-fail coverage proving direct lower-crate inspection types are not the
  required public lane

**Open questions before implementation**

- should unified inspection return one deeply-structured result family or a
  smaller top-level result with typed expandable views?
- how much lower-authority evidence should be embedded directly in inspection
  results versus referenced through route/receipt/envelope-backed links?
- what is the minimum inspection digest/evidence surface needed for
  certification without turning inspection into a second envelope system?

### Phase 18: Support And Readiness Boundary

This phase makes support, readiness, deferral, and denial first-class public
surfaces across the entire declaration-entry seam.

**Required Query artifacts**

- one platform-entry support matrix
- one readiness/disposition report for retained family taxonomy rows and
  crossings
- one denial/deferred residual-debt projection over the same matrix
- one typed support/readiness request family for querying capability and
  admission posture explicitly

**Locked local surfaces**

- `forge_query::facade::ForgeQuerySupportMatrix`
- `forge_query::facade::ForgeQuerySupportReport`
- `forge_query::facade::ForgeQueryCapabilityDescriptor`
- `forge_query::facade::ForgeQueryCapabilitySupportStatus`

**Requirements**

- support posture must be visible for domain entry, family-taxonomy selection,
  route planning, lower-authority routing, signal compatibility, and
  neighborhood readiness where applicable
- support/readiness posture must also be visible for admitted operating-world
  questions: which configured regimes are supported, which continuation modes
  are supported for a given admitted world, and which truth contexts are valid
  inside that world
- the support matrix must be the same source of truth used by ordinary DX,
  checked outcomes, docs, and certification
- support/readiness rows must be keyed by retained domain-scoped family
  identity plus Query-owned taxonomy posture where family meaning matters; the
  matrix may not fall back to raw family-label folklore
- unsupported, deferred, and forbidden surfaces must remain explicitly named
  and may not degrade into "not implemented" folklore
- support/readiness posture must be inspectable before expensive work happens
- callers must be able to ask explicit typed support/readiness questions about
  operating context, route families, continuation modes, truth contexts,
  grouped declaration semantics, and handle-vs-declaration posture rather than
  inferring readiness from trial and error

**DX target**

- support posture should help users choose viable paths early rather than
  discover dead ends late
- readiness language should feel crisp and decision-useful, not bureaucratic

**Documentation obligation**

- yes; support/readiness is a public-facing product surface and must be taught

**Compile-time enforcement**

- public surfaces advertised as ordinary-lane features must have a matching
  support/readiness row
- checked-lane denial posture must not contradict support matrix posture

**Acceptance evidence**

- parity tests between live public surfaces and the support matrix
- typed denial tests for deferred and unsupported family-taxonomy rows
- certification bundles proving support/readiness digests align with the
  crossing inventory from Phase 16

**Open questions before implementation**

- should support/readiness requests be family-specific typed inputs, one shared
  request family with typed selectors, or a hybrid?
- how much readiness detail belongs in ordinary-lane support answers versus
  checked/inspection-only surfaces?
- what is the cheapest synchronization mechanism between the live support
  matrix, compile-time gating, and documented feature coverage?

### Phase 19: Happy-Path Orchestration Boundary

This phase is the Laravel boundary: Query becomes the compiler for the ordinary
lane instead of making callers sequence proof, foundational, and lower-authority
steps themselves.

**Required Query artifacts**

- one ordinary-lane orchestration surface over Phases 1 through 18
- one checked orchestration lane
- one proof-preserving orchestration lane that stays honest about what it
  automates
- one orchestration transcript inventory for certification and docs

**Locked local and adjacent references**

- orchestration must compile onto the prior phase artifacts rather than
  bypassing them
- public route-plan / receipt / envelope outputs remain required products of
  orchestration, not hidden internals

**Requirements**

- the ordinary lane must let a serious domain author establish one admitted
  operating world, express declaration intent within that world, and have Query
  perform canonicalization, legality, progression, foundational materialization,
  route planning, and crossing assembly automatically where admitted
- the ordinary lane must stay domain-first in naming and shape; it may not
  devolve into `ForgeQuery*` ceremony or substrate-driven builder steps
- orchestration may automate sequencing, but it may not hide authority changes,
  denial classes, or expensive work
- checked and proof lanes must remain available and truthful rather than
  turning the ordinary lane into the only real implementation path
- orchestration must compile from admitted configured-handle proof rather than
  re-binding operating-world facts through parallel helper parameters later in
  the flow
- for supported runtime-capable families, ordinary-lane callers must not need
  to manually assemble Query runtime builders, backend parts, workspace entry,
  or current/historical basis binding once the declaration has been admitted
  and routed through Query

**DX target**

- this phase should make the platform feel inevitable and framework-quality in
  the common case
- users should feel like they stated intent once and Query handled the hard
  choreography honestly
- runtime-capable happy paths should read like "continue into
  runtime/workspace/basis" rather than "build adapters, wire backend parts,
  then bind a basis context"

**Documentation obligation**

- yes; this is one of the most visible public phases and needs exemplary docs
  and goldens

**Compile-time enforcement**

- the ordinary lane must compile onto typed prior-phase artifacts, not
  host-owned ad hoc helper chains
- orchestration helpers may not return opaque success types that erase receipt,
  envelope, or denial posture

**Acceptance evidence**

- golden DX transcripts for the ordinary lane across representative family
  types
- parity suites showing orchestrated ordinary paths converge with explicit
  checked/proof paths
- hostile tests proving orchestration does not invent unsupported combinations
  by convenience
- end-to-end examples proving representative domain declarations can continue
  into runtime-backed workspace and basis-bound read paths without any
  caller-owned runtime builder or basis-binding glue

**Open questions before implementation**

- what is the exact orchestration front-door shape for ordinary lanes so it
  stays domain-first without fragmenting into too many equal entry paths?
- how much orchestration transcript detail should be retained publicly versus
  reserved for certification and debugging?
- where should ordinary-lane sugar stop so advanced callers can still see the
  real underlying artifacts without duplicating orchestration logic?

### Phase 20: Denial And Recovery UX Boundary

This phase makes failure as usable as success by turning denials, stale states,
rebind requirements, and recovery posture into public product artifacts.

**Required Query artifacts**

- one denial family for ordinary and checked lanes
- one recovery/rebind/stale posture family
- one explanation surface that ties denial posture back to route, family, and
  authority context
- one typed recovery-request family for rebind, retry, repair, or reroute paths
  that are actually supported

**Locked local and external surfaces**

- checked/proof progression outcomes from Phase 7
- foundational receipt/support/provenance surfaces from Phase 8
- bridge and relational diagnostics only as composed evidence, not as direct
  public substitutes

**Requirements**

- denials must distinguish unsupported family, illegal structure, denied
  legality, denied route, stale basis, rebind required, deferred neighbor, and
  forbidden duplicate posture where those are semantically different
- denial and recovery posture must distinguish at least three repair surfaces
  when they differ semantically: admitted operating-world repair,
  declaration-meaning repair, and truth/continuation-context repair
- the recovery story must explain what the user can do next without forcing
  them to reverse-engineer lower-crate semantics
- ordinary-lane denial UX must be concise but truthful; checked and proof lanes
  must expose the exact typed structure underneath
- denial explanations must remain route-sensitive and family-sensitive
- supported recovery paths must take typed recovery inputs rather than relying
  on developers to remember which bits of context need to be repaired or
  replayed after a denial, stale, or rebind outcome

**DX target**

- failure should feel actionable, not like the framework just said no
- users should leave denial paths knowing whether to rebind, narrow, pick a
  different family, or stop

**Documentation obligation**

- yes; denial and recovery posture are public behavior and need explicit docs

**Compile-time enforcement**

- denial surfaces must be typed and non-binary; they may not collapse into one
  generic error string
- stale/rebind posture must remain a distinct outcome family rather than an
  attached note on generic failure

**Acceptance evidence**

- hostile denial suites covering unsupported, illegal, stale, rebind, deferred,
  and forbidden cases
- parity tests proving equivalent failure causes converge to the same denial
  posture across ordinary and checked lanes
- compile-fail coverage for code paths that try to treat typed denial families
  as opaque success/failure booleans

**Open questions before implementation**

- should recovery requests be one typed family with submodes or several
  dedicated recovery input families for rebind, retry, repair, and reroute?
- which recovery paths are ergonomic enough for the ordinary lane versus better
  left to checked/proof guidance only?
- how much automatic repair guidance can Query offer without pretending to own
  lower-authority semantics it does not actually control?

### Phase 21: Family-Specific Ergonomics Boundary

This phase gives the major declaration families native-feeling public helper
surfaces instead of forcing everything through one generic orchestration shape.

**Required Query artifacts**

- one family-specific helper surface per admitted major family
- one mapping from helper surface back to canonical family identity
- one support/readiness and denial posture projection per helper family

**Requirements**

- helper surfaces must read like the user's domain intent, not like generic
  framework contribution plumbing
- helpers must compile onto the same canonical declaration, route, receipt, and
  envelope artifacts as the generic lane
- helper surfaces may not invent new authority classes, new progression rules,
  or new receipt semantics
- any helper that meaningfully changes semantics must declare a distinct family
  rather than posing as a synonym

**DX target**

- major declaration families should feel native and pleasant to author, not
  like users are always falling back to the generic substrate path
- helpers should increase confidence and speed without increasing ambiguity

**Documentation obligation**

- yes; helper families are public conveniences and need family-specific docs

**Compile-time enforcement**

- helper surfaces must remain family-gated and unavailable on the wrong handle
  or family posture
- helper-generated artifacts must remain equivalent to the generic canonical
  path when semantically identical

**Acceptance evidence**

- golden DX transcripts for the major helper families
- canonical digest parity between helper surfaces and generic surfaces for
  equivalent meaning
- compile-fail suites proving helper families are not callable where the
  capability matrix forbids them

**Open questions before implementation**

- which family-specific helpers are important enough to deserve first-class
  surfaces in the initial build versus later follow-on polish?
- should helpers be methods on typed family handles, generated helper modules,
  or another form that still maps cleanly back to canonical family identity?
- how do we keep helper naming pleasant without creating overlapping synonyms
  that weaken the “one obvious path” rule?

### Phase 22: Neighborhood And Batch Declaration Boundary

This phase makes meaningful grouped declarations first-class so geometry and
topology domains can work in local neighborhoods rather than only isolated
single declarations.

**Required Query artifacts**

- one grouped declaration artifact family
- one grouped route/receipt/envelope family
- one grouped support/readiness and denial posture family
- one grouped canonical digest/equivalence surface
- one typed grouped-declaration input family that captures grouping semantics
  explicitly rather than accepting a bare collection

**Requirements**

- neighborhood and batch declarations must remain explicit groups, not ad hoc
  arrays of unrelated single declarations
- grouped declaration semantics must preserve shared posture, shared rationale,
  and shared route/denial context where that grouping is semantically real
- batching may improve ergonomics and cost posture, but it may not silently
  merge declarations whose semantics should stay distinct
- grouped artifacts must still map back to single-declaration family and route
  inventory rows in a certification-readable way
- grouped declaration inputs must carry typed semantics for shared posture,
  continuity assumptions, ordering/atomicity, and exploratory vs authoritative
  grouping where those distinctions are admitted

**DX target**

- grouped authoring should feel natural for geometry neighborhoods and other
  real multi-declaration work
- users should be able to think in meaningful local structures rather than
  manually coordinating many isolated declarations

**Documentation obligation**

- yes; grouped/neighborhood authoring changes public usage substantially and
  needs dedicated docs

**Compile-time enforcement**

- grouped declaration APIs must distinguish intentional neighborhood groups from
  arbitrary collections
- unsupported family mixes inside one grouped declaration must deny or fail
  explicitly rather than widening by convenience

**Acceptance evidence**

- parity suites showing semantically equivalent grouped authoring paths
  canonicalize identically
- hostile tests for illegal grouped mixes and illegal shared-posture claims
- cost/readiness evidence showing batching does not hide widened work

**Open questions before implementation**

- should grouped declarations start with one general typed input family or a
  small set of specialized neighborhood/batch families with shared traits?
- which grouped semantics must be first-class from day one: shared rationale,
  shared support posture, ordering, atomicity, exploratory grouping, or all of
  them?
- how should grouped route/receipt/envelope artifacts expose member-level versus
  group-level facts without becoming unreadable?

### Phase 23: Public Documentation And Golden Teaching Boundary

This phase makes the platform-entry seam teachable and ensures the docs do not
lose critical behavior to oral tradition.

**Required Query artifacts**

- one documentation inventory over the admitted platform-entry surfaces
- one golden transcript catalog that matches the ordinary public lane
- one coverage map from docs and transcripts back to the crossing inventory

**Requirements**

- every admitted ordinary-lane family and helper surface must have one honest
  documented path
- docs must teach support/readiness posture, denial posture, route/receipt/
  envelope meaning, and lower-authority ownership honestly
- golden transcripts must be treated as product artifacts, not blog-style
  examples
- docs must not reintroduce obsolete three-milestone or split-seam framing

**DX target**

- the docs should make the right path feel obvious enough that most users never
  need to guess
- examples should teach confidence and mental model clarity, not just syntax

**Documentation obligation**

- yes by definition; this phase is the docs closure phase

**Compile-time enforcement**

- documented ordinary paths must stay synchronized with live public surfaces and
  golden transcript coverage
- undocumented public surfaces in the covered inventory must fail doc coverage
  checks or closeout certification

**Acceptance evidence**

- doc coverage inventory aligned with Phase 16 crossing rows
- golden transcript parity checks against live public APIs
- QA pass proving no critical platform-entry behavior is lost to history

**Open questions before implementation**

- what is the exact doc inventory format that keeps feature docs, goldens, and
  crossing inventory synchronized without too much manual bookkeeping?
- which examples should be the canonical teaching cases for the first public
  rollout, especially for runtime-capable and grouped declaration paths?
- how should docs surface lower-authority ownership honestly without forcing
  users to learn relational/bridge/signal internals too early?

### Phase 24: Certification And Closeout Boundary

This phase closes the milestone with hostile proof rather than plausibility.

**Required Query artifacts**

- one certification bundle for the full platform-entry seam
- one compile-fail suite spanning Phases 1 through 23
- one parity suite spanning ordinary, checked, and proof lanes
- one hostile certification harness over route plans, receipts, envelopes,
  support posture, denials, grouped declarations, and docs coverage

**Locked certification expectations**

- the bundle must extend the certification mentality already used in
  `forge-query`
- route-plan, receipt, envelope, support/readiness, denial, and grouped
  declaration digests must all be machine-checkable

**Requirements**

- equivalent public/proof/generic/helper/grouped paths must converge
  canonically when semantically identical
- intentionally different family, route, authority, denial, and support posture
  must diverge observably and predictably
- compile-fail coverage must match the live ordinary lane breadth
- inventory, docs, support matrix, and certification bundle breadth must all
  agree exactly

**DX target**

- users should feel that the platform is trustworthy because every admitted path
  is proven, not just plausible
- the final public experience should feel coherent across ordinary, checked,
  proof, helper, and grouped lanes

**Documentation obligation**

- no new feature docs by default, but closeout must verify that all required
  phase-local docs obligations were actually satisfied

**Compile-time enforcement**

- milestone closure is blocked if any covered public surface lacks compile-fail,
  parity, or inventory/doc coverage
- pseudo-Query bypasses that skip canonicalization, legality, progression,
  route planning, or envelope production must fail closed

**Acceptance evidence**

- end-to-end hostile certification bundles
- compile-fail suites for the full public breadth
- parity suites covering ordinary vs checked vs proof vs helper vs grouped
  paths
- support matrix, inventory, docs, and certification digest equality checks

**Open questions before implementation**

- what certification harness layout best keeps compile-fail, parity, hostile,
  and docs-coverage checks maintainable as the public surface grows?
- which digest families need to be compared independently versus rolled into a
  higher-level milestone closeout digest?
- how should milestone closeout report residual deferred/debt seams, if any,
  without weakening the closure bar for the admitted surface?

## Remaining Phase Detail

Phases 1 through 24 now all have boundary-level requirements. Future hostile
QA passes should continue tightening wording as later phases land, but the
spec's current boundary set is now explicit enough to drive implementation
without falling back to milestone folklore.
