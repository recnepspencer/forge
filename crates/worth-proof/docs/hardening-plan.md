# worth-proof Hardening Plan

**Opened:** 2026-08-06, after milestone 9.16 Phase 8 was found to have
hand-rolled fifteen sealed authority types in `worth-query-execution` — the one
Query crate that did not depend on this one — while the declaration lane next
door used this crate throughout.

**Mandate:** make this the strongest possible vocabulary for type progressions
and up-front contracts. **Backwards compatibility is not a constraint.** Rename,
reshape, and remove freely; consumers are in-repo and can be migrated.

This crate is well built — 7,086 lines, 21 test files, per-milestone
compile-fail suites, `codegen_honesty_report` assertions covering hidden dynamic
lookup, virtual dispatch, and introduced allocation. The problem is not rigor.
It is **doors and words**: where the crate lets a consumer in, and whether it
has a word for what the consumer is trying to say.

## The diagnostic

The strongest evidence for what is missing is what Phase 8 wrote *by hand*
rather than reach for. Every row below is a real type someone built because this
crate had no word for it. Verified absent: `grep` for binding/axis/drift,
clock/time/sample, and causal/inverse/lineage across `src/` returns **nothing**.

| Hand-rolled in Phase 8 | Missing vocabulary | Plan item |
|---|---|---|
| `WorthQueryRecoveryEffectAuthority` — authority keyed by a `u64` every runtime starts at 1 | instance identity | **V1** |
| `WorthQueryRecoveryHandle` — `live: bool` + `Arc` registry + `Drop` leak detection + three terminal states | linear resource with terminal states | **V2** |
| `WorthQueryRecoveryHandleBinding` — 13 axes, 11 hand-written comparisons, each with its own denial kind | binding with per-axis drift | **V3** |
| `WorthQueryRecoveryExpiryDecision` — clock sample the caller must not supply | freshness evaluated against an injected source | **V4** |
| `WorthQueryProvedUndo` — "transition B inverts transition A" | causal links between transitions | **V5** |
| `WorthQueryAdmittedExternalRedispatch` — proof an effect actually escaped | performed-effect evidence | **V6** |

Six missing words, fifteen hand-rolled types, one audit's worth of defects in
them. That is the case for this plan.

---

# Part I — Honesty repairs

**Status: LANDED 2026-08-06.** All five items below are implemented and
verified: 21 worth-proof suites green, `RUSTFLAGS=-Dwarnings` clean across
`worth-proof` and `worth-foundational`, `cargo fmt` applied. Findings from doing
the work are recorded inline under each item.

Fix what claims more than it delivers. These come first: until the existing
guarantees are trustworthy, nothing built on top of them is.

## H1 — The seal is proved on a dead path

```rust
#[allow(dead_code)]
pub(crate) fn mint() -> Self                     // sealed — only #[cfg(test)] callers
pub fn from_authority_marker(_marker: A) -> Self // pub — the live path
```

`tests/ui/milestone2/witnesses_are_not_publicly_mintable.rs` asserts `mint()`
does not compile. Two lines away in this crate's own unit test,
`from_authority_marker(DeploymentAuthority)` compiles fine. **No case anywhere
covers the live path.** The README's "sealed against public minting" describes
the dead one.

The underlying design — a witness is exactly as sealed as its marker's
constructor — is coherent, but undocumented, untested, and contradicted by every
example (`struct Foo;` is not a seal). In worth-query the practice is
inconsistent: module-private markers, `pub(crate)` markers, and one author who
understood and wrote `WorthQueryOperationalCompatibilityAuthority(())`.

**Fix:** document the delegation on the method; add the compile-**fail**
(private-constructor marker cannot be minted outside) and compile-**pass**
(freely-constructible marker can) that make it a tested property; then **make
the correct pattern the default** via H4.

Note for contrast: `Proof` *is* genuinely sealed — no public constructor exists,
and `AuthorityProves<P>` is a real proof-kind gate. Separate the two claims in
the docs rather than lumping them under one sentence.

## H2 — Exported types nobody can construct

`DisjointPair` is exported through `raw.rs` with `left()`, `right()`, `proof()`,
`into_parts()` all `pub`, and exactly one constructor: `pub(crate)`,
`#[allow(dead_code)]`, taking a `Proof` only the dead `mint_proof` can produce.
No consumer can build one; neither does the crate.

Its siblings do this correctly — `CanonicalVec::try_from_sorted`,
`UniqueVec::try_from_unique` are public checked constructors that validate and
mint internally. That is the right pattern; `DisjointPair` never got one.

**Fix:** `try_from_disjoint(left, right) -> Result<Self, Pair<T>>`, matching its
siblings. Plus the mechanical guard in **H5** so this class cannot recur.

## H3 — Eight suppressions marking vestigial raw doors

`artifact/constructors.rs:62`, `collections/disjoint_pair.rs:12`,
`collections/proven_vec.rs:25,76`, `proof/minting.rs:3`, `proof/sets.rs:23`,
`proof/witnesses.rs:16,35`.

Every one marks a raw `pub(crate)` constructor that bypasses the checked path so
tests can build a form directly. In a crate whose thesis is sealed construction,
a raw door kept alive by a suppression is the exact smell it teaches others to
remove — and it is how H2 stayed invisible.

**Fix:** route tests through checked constructors, or gate the raw ones behind
`#[cfg(test)]`. `#[cfg(test)]` is a boundary; `#[allow(dead_code)]` is a note.
Phase 8's slice 2 got this right independently — `mint_for_test` there is
`#[cfg(test)]` — which is the standard to match.

## H4 — Make the correct marker the easy one

Authoring a properly sealed marker today requires knowledge the crate never
hands over, which is why practice is inconsistent.

**Add** `authority_marker!` / `capability_marker!` macros generating a marker
with a private constructor plus an owner-only mint. There is precedent for
macro-based compile-time enforcement here already (`band_guard!` in `band.rs`).

## H5 — Certify authority boundaries and supported workflows

Authority-bearing operations accept only the concrete proof, authority, and
capability values issued by their owning worth-proof workflows. Maintained
downstream compile-fail contracts prove that counterfeit values and direct
sealed construction cannot satisfy those operations. Each supported public
workflow named in the feature contract has downstream compile-pass or
executable documentation evidence.

Two bounded catalogs own this guarantee:

1. **Authority boundary contracts.** Each sensitive operation has an
   owner-issued compile-pass case, a structurally similar counterfeit local
   value that fails at the protected call, and direct-construction compile-fail
   evidence when privacy is load-bearing.
2. **Supported workflow contracts.** Caller-level evidence covers checked
   `DisjointPair` construction, scoped brand usage, proof/capability
   progression, recipe resolution through execution readiness, trust-boundary
   bridging, and the primary transition workflow.

The negative authority oracle is the protected operation's concrete type
signature, not merely a private field. Private construction evidence closes the
additional minting door where possession of the marker value is the authority.
BC7001 also requires named production imports and reexports for governed crates;
glob imports and reexports are denied so the authority surface remains explicit.

This scope deliberately does not maintain a universal registry of public Rust
types or reconstruct Rust name resolution. Public data shape is not itself an
authority boundary. The durable caller contract is documented in
`docs/features/authority-and-workflow-contracts.md`.

---

# Part II — Missing vocabulary

**Status: LANDED 2026-08-06.** V1–V6 are implemented as a static substrate.
The owner-runtime halves identified below remain with their owners. V3 uses
`binding_axes!` plus `binding_axis_drift_certification!` instead of a proc-macro
derive: axes are still declared once and receive generated comparison, drift,
and coverage scaffolding, while H5's zero-normal-dependency guarantee remains
mechanically true.

The six words consumers keep inventing.

## Where each word belongs — the layering test

`worth-foundational` **depends on** `worth-proof` (122 files use it). Proof is
the lower layer: anything added here is available to foundational, and nothing
added to foundational is available here.

Their stated visions give a three-way test. Applying it honestly moves several
items out of this crate.

| Question | Owner |
|---|---|
| Does it decide whether a transition is **legal**? | **worth-proof** — "says what is legal and what has been proven"; static-first, type-level, generic |
| Does it **describe or package** what happened, identically across a boundary? | **worth-foundational** — "portable meaning… must remain identical when crossing a boundary"; owns evidence, provenance, lineage, support |
| Does it need a **live table, a clock, a process, or `Drop`**? | **neither** — owner runtime machinery. worth-proof's own README: *"Do not use it as a runtime execution framework."* Foundational holds no `Drop` impls or live registries. |

The third row is the one that catches design drift, and it caught this plan:
several items below were originally scoped entirely to worth-proof and are
genuinely split.

| Item | worth-proof | worth-foundational | owner runtime |
|---|---|---|---|
| **V1** instance identity | `Branded<'id, T>` — type-level non-interchangeability | `FoundationalAuthorityIdentity<Value, Authority, Kind>` **already exists** if the identity must cross a boundary | the process-unique value + its counter |
| **V2** linear resources | the linear type and terminal-state law | — | registry, enumeration, `Drop` leak detection |
| **V3** bindings / per-axis drift | all of it — admissibility, static, derive-generated | — | — |
| **V4** freshness | the `FreshnessSource` trait and the evaluation | — | the clock implementation |
| **V5** causal links | `Inverts<A>` as an action-kind legality gate | portable description of an already owner-established causal fact | exact occurrence relation and canonical history remain with their domain owners; for Phase 8 history that owner is Relational |
| **V6** performed-effect evidence | `Performed<Action, Authority>` as a gate | its description at a boundary (`boundary_evidence`) | — |

Two corrections this produced:

- **V1's value half does not belong here, and probably belongs nowhere central.**
  A process-local instance identity is *by definition* not portable, so it fails
  foundational's test, and it needs a monotonic counter, so it fails
  worth-proof's. Query's `WorthQueryRuntimeAuthorityIdentity` is **correctly
  placed where it is**. What is missing from this crate is only the
  type-level brand.
- **V5 does not have one generic "lineage half."** Foundational may describe an
  already owner-established causal fact at a boundary, but it cannot become the
  canonical history owner. `Inverts<A>` relates action kinds and gates legality;
  it does not identify exact committed occurrences. For Phase 8, Relational
  remains the owner of commit identity, parents, head, ancestry, and canonical
  publication, while any Query operation-semantic relation must be separately
  admitted by the Query product.

Each crate keeps its pure vision: proof stays static-first and free of time,
counters, and tables; foundational stays portable and descriptive; runtime
machinery stays with the owner that runs it.

## V1 — Instance identity, in two forms

Everything this crate expresses is type-level: `Proof` and `AuthorityWitness`
are `PhantomData`; only `AssumptionBasis<B>` carries a value. **Nothing can
distinguish two instances of the same type.** That is why Q8.20 existed.

**Scope here: the brand only.** `Branded<'id, T>` — generative invariant
lifetime, so cross-instance substitution becomes a **compile error** (rung 1),
with zero runtime representation. Known limit: all uses must sit inside the
branding scope, so it does not fit a carrier that outlives its mint call.

**Not here: the process-unique identity value.** It fails both tests — not
portable, so not foundational; needs a monotonic counter, so not proof. Query's
`WorthQueryRuntimeAuthorityIdentity` is correctly owner-local, and
`FoundationalAuthorityIdentity<Value, Authority, Kind>` already exists to wrap
such a value if it ever has to cross a boundary. The gap in *this* crate is that
a consumer who wants compile-time instance separation has no word for it and
must fall back to a runtime comparison.

## V2 — Linear resources with terminal states and leak detection

Phase 8's recovery handle is a general pattern wearing a domain name: a resource
minted once, reaching **exactly one** terminal state, enumerable by the
framework, whose non-terminal drop is detectable. Store, UI, and Signal all have
this shape.

**Scope here: the law, not the machinery.** `LinearResource<Id, Terminal>`:

- consumption by value; a second transition is unrepresentable
- a closed `Terminal` set the owner defines
- a terminal receipt naming which terminal was reached

**Not here: the registry.** Enumeration, forced termination, `Drop` leak
detection and `assert_no_live()` need a live table — runtime machinery this
crate explicitly disclaims. Those stay with the owner. What the owner should not
have to re-derive is the *law*: minted once, exactly one terminal, second
transition unrepresentable.

`Recipe` gives staged progression but not *linear consumption*, which is the
harder and more valuable half.

## V3 — Bindings with per-axis drift

Phase 8 hand-wrote a 13-field binding and **eleven** comparisons, each mapping
to its own denial kind, so a mismatch says *which* axis drifted. That is the
single most valuable "up-front contract" primitive in the whole audit, and it
does not exist here.

**Add** `Binding<Axes>` with `#[derive(BindingAxes)]` generating:

- the comparison
- a per-axis denial enum, one variant per field
- a per-axis drift test scaffold with positive twins

A consumer declares the axes once and gets drift detection, distinct causes, and
adversarial coverage. Today they write 11 comparisons and 11 tests by hand, and
an auditor checks whether any axis was forgotten.

## V4 — Freshness evaluated against an injected source

The crate has freshness *states* — `CurrentValidity`, `StaleReadable`,
`RebindRequired`, `AuthorityRevalidationRequired` — and **nothing that decides
which one you are in.** Consumers must supply the transition themselves, which
means a caller can choose their own freshness.

Phase 8 hit this and solved it locally: R8.7 required that "callers and adapters
cannot supply a sample or choose the evaluation moment," producing a hand-rolled
clock, sample type, and `evaluate_expiry`.

**Add** a `FreshnessSource` trait (host-supplied time or generation counter) and
`evaluate_freshness(basis, source) -> FreshnessScopedBasis<..>`, where the
sample is minted by the source and **cannot be caller-supplied**. That
constraint belongs in the substrate, not rediscovered per consumer.

## V5 — Causal links between transitions

No vocabulary for "transition B is causally derived from transition A." Phase 8
needed it twice — `WorthQueryProvedUndo` ("B inverts A") and a seven-posture
causality ladder where each posture links to its predecessor — and both were
hand-rolled. `WorthQueryProvedUndo` shipped as a **public five-raw-field
constructor** that proved nothing (Q8.23).

**Scope here: the legality gate only.** `Inverts<A>` / `DerivedFrom<A>`, minted
only from a completed predecessor, so "redo requires a proved undo" is a type
constraint rather than a caller-supplied field.

**Not here: canonical history or exact occurrence lineage.** Foundational may
carry a portable description of causality only after an owning runtime has
established it. Phase 8's former `WorthQueryLinearLineageChain` must not move to
Foundational: it duplicated Relational's canonical commit-history authority and
was correctly deleted. `Inverts<A>` also does not prove that one exact committed
occurrence inverted another; that relation remains a domain-owner concern.

`composition/{fork,join,family}` handles *structural* composition; this is
*temporal* composition, and the legality half of it is absent.

## V6 — Performed-effect evidence

Gate 8.7's whole lesson: a transition must consume **proof that an action
occurred**, not permission to attempt one. `WorthQueryAdmittedExternalRedispatch`
is that proof, hand-rolled.

**Add** `Performed<Action, Authority>` — mintable only by the code that
performed the action, carrying its outcome. The distinction between *admitted*
and *performed* is exactly what Q8.13 was, and the substrate should name it.

---

# Part III — Adoption

**Status: LANDED 2026-08-06.** The `contracts` lane and README decision table
are public. Query Phase 8 now consumes the substrate for recovery binding
comparison, runtime-sampled freshness, causal proved-undo construction, and
performed external redispatch. Query correctly retains its owner-local runtime
identity, live recovery registry, `Drop` enforcement, clock implementation,
portable lineage, receipts, and dispatch machinery.

The unsafe parallel doors are retired: there is no raw public proved-undo
constructor, no admitted-as-performed redispatch type, no caller-supplied
expiry sample, and no independent hand-written recovery binding comparison.

Why a team that knew about this crate built a parallel one.

- **A `contracts` prelude** surfacing V1–V6 as the up-front-contract vocabulary,
  separate from the recipe-progression prelude. A consumer designing an
  authority model should find the authority words first.
- **Checked constructors everywhere** (H2), so every proven form has a visible
  door. A type you cannot construct reads as "this machinery is inert."
- **A decision table** — Proof / Witness / Basis / Brand / Binding: what each
  carries (type-level vs value), what it proves, when to reach for it. The
  README currently says these are easy to confuse and then explains each
  separately, which is the layout that does not help someone choosing.
- **Worked failures instead of abstract warnings.** "Witnesses are not generic
  tokens for semantic laundering" says nothing about what breaks. "A witness
  cannot distinguish two instances of the same runtime — it proves the lane, not
  which instance; instance identity is a basis" would have prevented a real
  defect this week.

---

# Ordering

| Phase | Items | Rationale |
|---|---|---|
| 1 | H1, H3, H5 | Honesty and mechanical self-certification. Nothing built on an untrustworthy seal is worth building. |
| 2 | H2, H4 | Small, unblock adoption, remove the "inert machinery" impression. |
| 3 | V1, V2 | Highest reuse across Query, Store, UI, Signal. V1 also retires Q8.20's hand-rolled identity. |
| 4 | V3 | Largest single win for up-front contracts; wants a derive macro, so it earns its own slice. |
| 5 | V4, V5, V6 | Depend on V1/V2 shapes settling first. |
| 6 | Part III | Docs and prelude last, describing what exists rather than what is planned. |

Each phase ships its own compile-fail coverage under the H5 rule: every seal,
every door.

**Migration:** consumers are in-repo. When a `V*` item lands, the hand-rolled
equivalent in `worth-query-execution` is retired in the same slice under
**R8.0** — no second independently reachable lane, including transiently.
