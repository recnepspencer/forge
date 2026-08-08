# worth-proof

`worth-proof` is WORTH's compile-time proof-bearing progression substrate.

Use it when a crate needs to represent:

- what phase an artifact or recipe is in
- what facts have already been proven
- what witness or authority is required to progress
- what kind of failure or degradation happened
- when a boundary crossing forces rebind or revalidation
- when fixed-arity composition must stay deterministic and static

Do not use it as:

- a runtime execution framework
- a dynamic workflow engine
- a diagnostics, provenance, or support-report crate
- a generic graph runtime

Those boundaries matter. `worth-proof` owns progression law. `worth-foundational` owns shared descriptive and boundary vocabulary.

## What You Reach For First

Most consumers should start with:

- the blessed import lane:
  - `use worth_proof::prelude::*;`
- helper constructors in that lane:
  - `recipe(payload)`
  - `pair(left, right)`
  - `non_empty(head, tail)`
  - `sym(symbol)`
  - `member(member_id)`
- common pleasant-lane progression verbs:
  - `.resolve_with(authority, basis)`
  - `.lower_with(capability)`
  - `.admit_with(authority)`
  - `.ready_with(authority, runtime)`
  - `.execute()`
- checked pleasant-lane progression verbs:
  - `.try_resolve(gate)`
  - `.try_resolve_ready(basis, authority)`
  - `.try_lower(readiness)`
  - `.try_lower_ready(capability)`
  - `.try_admit(readiness)`
  - `.try_admit_ready(authority)`
  - `.try_ready(readiness)`
  - `.try_ready_now(runtime, authority)`
  - `.try_execute()`
- narrow checked-input helpers for the common ready lane:
  - `gate_ready(...)`
  - `ready_now(...)`
- trust-boundary pleasant verbs:
  - `.bridge_trust_boundary()`
  - `.rebind_with(authority, basis)`
  - `.readmit_with(authority, basis)`
- family authoring helpers:
  - `create(...)`
  - `rewrite(...)`
  - `supersede(...)`
  - `retire(...)`
  - `family_pair(...).lower_by(...)`
- pleasant ready-join helpers:
  - `join_ready(left, right)`
  - `compose_ready(left_outcome, || right_outcome)`
- explicit scoped progression defaults:
  - `proof_flow()`
  - `.resolution_authority(...)`
  - `.lowering_capability(...)`
  - `.readiness_authority(...)`
  - `.recipe(...).resolve(...).lower().ready(...).execute()`
- grouped read helpers:
  - `.stage()`
  - `.basis_posture()`
  - `.has_strong_basis()`
  - `joined.summary()`
  - `family_action.kind()`
  - `lowered_family.action_kinds()`
- the raw escape hatch:
  - `use worth_proof::raw::*;`
- `Recipe<Unresolved, T>::new(payload)` when you intentionally want the raw
  substrate spelling for staged recipe progression
- transition helpers in the facade such as:
  - `ResolveRecipeTransition`
  - `LowerRecipeTransition`
  - `AdmitRecipeTransition`
  - `ExecuteReadyRecipeTransition`
  - `resolve_lower_and_admit_recipe(...)`
  - `resolve_checked_lower_and_admit_recipe(...)`
- fixed-shape helpers such as:
  - `Pair<T>`
  - `NonEmpty<T>`
  - `CanonicalVec<T>`
  - `UniqueVec<T>`
- composition helpers such as:
  - `fork_artifact_pair(...)`
  - `join_artifact_pair(...)`
  - `join_ready_recipe_pair(...)`
  - `compose_join_ready_recipe_pair(...)`
  - `resolve_family_symbol(...)`
  - `lower_deterministic_family_pair(...)`

If you only read one thing before using the crate, read this file and then jump to the feature doc that matches the surface you plan to use.

The pleasant lane is additive guidance, not a second semantic system. If you
drop to the raw lane, you are still using the same proof-bearing substrate.

Raw equivalent for the common fluent lane:

```rust
let executed = recipe("payload")
    .resolve_with(resolution_authority, 8_u8)
    .lower_with(lowering_capability)
    .ready_with(readiness_authority, "runtime admission")
    .execute();

let resolved = ResolveRecipeTransition.transition(
    Recipe::<Unresolved, _>::new("payload"),
    RecipeResolutionContext::new(8_u8, resolution_authority),
);
let lowered = LowerRecipeTransition::new(lowering_capability)
    .transition(resolved.into_value())
    .into_value();
let ready = AdmitExecutionReadyRecipeTransition.transition(
    lowered,
    ExecutionReadinessContext::new("runtime admission", readiness_authority),
);
let executed = ExecuteReadyRecipeTransition.transition(ready.into_value()).into_value();
```

Drop to the raw lane when the pleasant lane would stop being semantically
obvious, not because the pleasant lane owns different behavior.

Checked outcome view for the pleasant lane:

- checked DX methods return `ProofOutcome<...>`, not `Result`
- `ProofOutcome` is a narrow view over `TransitionOutcome`
- use `.kind()` to branch without flattening:
  - `Success`
  - `Denied`
  - `Deferred`
  - `Stale`
  - `RebindRequired`
  - `Failed`

Common checked-ready lane:

```rust
let outcome = recipe("payload")
    .try_resolve_ready(7_u8, resolution_authority)
    .try_lower_ready(lowering_capability)
    .try_ready_now("runtime admission", readiness_authority)
    .try_execute();
```

Common composition and family lane:

```rust
let joined = join_ready(left_ready_recipe, right_ready_recipe);

let lowered_family = family_pair(
    create::<u8, u16, _>(sym(2_u8), "create"),
    supersede::<u8, u16, _>(member(11_u16), sym(3_u8), "replace"),
)
.lower_by(family_action_key);
```

Common grouped-read lane:

```rust
let stage = ready_recipe.stage();
let posture = ready_recipe.basis_posture();
let has_strong_basis = ready_recipe.has_strong_basis();
let join_summary = joined_ready_recipe.summary();
let family_kinds = lowered_family.action_kinds();
```

Common scoped-default lane:

```rust
let executed = proof_flow()
    .resolution_authority(resolution_authority)
    .lowering_capability(lowering_capability)
    .readiness_authority(readiness_authority)
    .recipe("payload")
    .resolve(7_u8)
    .lower()
    .ready("runtime admission")
    .execute();
```

Scoped defaults are explicit carriage, not ambient configuration:

- inherited witnesses and capabilities are visible in the code
- each inherited witness is still consumed exactly once
- local overrides stay explicit through `.resolve_with(...)`, `.lower_with(...)`, and `.ready_with(...)`

## Raw Escape Hatch

The pleasant lane is the default recommendation, not the only honest surface.

When the domain needs the semantic substrate directly, reach for:

```rust
use worth_proof::raw::*;
```

That module keeps the raw proof-bearing surface first-class without asking
callers to import through internal module topology.

Use the raw escape hatch when:

- a convenience verb would hide a real adversarial boundary
- a review needs the transition nouns visible
- a domain helper is still being designed and the compressed lane would guess
  too much

Do not use it to rebuild stronger forms manually. The raw lane is still the
same semantic substrate with the same sealed minting and progression law.

## Core Mental Model

`worth-proof` gives you typed carriers whose shape says what is true right now.

The main kinds of truth it represents are:

- phase truth
  - an `Artifact<P, ...>` is in phase `P`
  - a `Recipe<S, ...>` is in stage `S`
- proof truth
  - `Proof<P, A>` means authority `A` has established fact `P`
  - proof sets let multiple facts travel together without collapsing into runtime lookup
- basis truth
  - `AssumptionBasis<B>` says the current form depends on some explicit basis
  - freshness wrappers say whether that basis is current, stale, rebind-required, or authority-revalidation-required
- witness truth
  - witnesses do not prove a semantic fact
  - they prove that a trusted capability or authority lane is present for a transition
- outcome truth
  - success, denial, deferment, stale, rebind-required, and failure remain distinct categories

The crate is intentionally static-first:

- phase and proof distinctions are encoded in types
- stronger forms are sealed against public minting
- trust-boundary weakening is explicit
- fixed-arity composition stays explicit and size-honest

## The Three Main Things You Hold

### 1. Artifacts

`Artifact<P, T, S, A>` is the general proof-bearing carrier:

- `P` = phase
- `T` = payload
- `S` = proof set
- `A` = assumption basis

Use artifacts when you need a phase-tagged payload that is not specifically a recipe progression surface.

### 2. Recipes

`Recipe<S, T, A>` is the main staged progression carrier:

- `Unresolved`
- `Resolved`
- `Lowered`
- `Admitted`

Later surfaces wrap recipes again when additional guarantees are needed:

- `ExecutionReadyRecipe<T, A>`
- `ExecutedRecipe<T, A>`

### 3. Outcomes

Progression APIs return `TransitionOutcome<...>` and related wrappers so that non-success states stay typed and visible.

This is deliberate. `worth-proof` does not collapse denial, deferment, stale inputs, rebind-required inputs, and hard failure into one undifferentiated error channel.

## Proofs, Witnesses, And Bases

These three things are easy to confuse at first.

### Proofs

`Proof<P, A>` means a fact has been established by an authority allowed to prove that fact.

Examples:

- `Proof<CanonicalOrder, StructuralProofAuthority>`
- `Proof<Uniqueness, StructuralProofAuthority>`
- `Proof<Disjointness, StructuralProofAuthority>`
- `Proof<Normalization, StructuralProofAuthority>`

Proof minting is sealed and proof-kind authorized. Public callers can carry proofs and observe them, but they cannot WORTH stronger proof-bearing forms directly or reuse an unrelated authority to mint a different proof kind.

### Witnesses

`AuthorityWitness<A>` and `CapabilityWitness<C>` mean a trusted progression lane is available.

They are used to authorize transitions such as:

- resolving with some authority
- lowering with some capability
- readmitting or revalidating across a trust boundary

Witnesses are not generic tokens for semantic laundering. They authorize transitions; they do not replace the proof-bearing forms those transitions produce.

### Bases

`AssumptionBasis<B>` carries the explicit basis a form depends on.

That basis can then be wrapped with freshness state:

- `FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>`
- `StaleReadableBasis<B>`
- `RebindRequiredBasis<B>`
- `AuthorityRevalidationRequiredBasis<B>`

When a trust boundary is crossed, the basis can be weakened again with `BoundaryBridged<...>` wrappers.

## Freshness And Trust Boundaries

Freshness is not a side note in this crate. Losing basis confidence is a first-class state transition.

The main downgrade shapes are:

- `StaleReadable`
  - still readable, but no longer current
- `RebindRequired`
  - semantic rebinding is required before trustworthy use
- `AuthorityRevalidationRequired`
  - authority must revalidate before trusted use resumes

Trust-boundary bridging makes that weakening explicit:

- current resolved recipes bridge to rebind-required forms
- current lowered recipes bridge to stale-readable forms
- current admitted forms bridge to authority-revalidation-required forms

This is how `worth-proof` prevents hidden "deserialize and hope" flows.

## Transition Topology

The crate keeps progression outcomes distinct.

The main categories are:

- success
- denied
- deferred
- stale
- rebind-required
- failed

That distinction shows up in:

- `TransitionOutcome<...>`
- `SuccessfulTransitionOutcome<...>`
- `DeferredTransitionOutcome<...>`
- `DenialTransitionOutcome<...>`
- `FreshnessTransitionOutcome<...>`

If your domain logic needs to preserve why progression did not continue, use checked transitions or explicit readiness gates instead of flattening everything into `Result<T, E>`.

## Execution Readiness

`worth-proof` distinguishes:

- lowered
- admitted
- execution-ready
- executed

That matters when a plan or recipe must be prepared statically but only applied at runtime.

Representative surfaces include:

- `AdmitExecutionReadyRecipeTransition`
- `CheckedAdmitExecutionReadyRecipeTransition`
- `ExecuteReadyRecipeTransition`
- `admit_ready_and_execute_recipe(...)`
- `checked_admit_ready_and_execute_recipe(...)`
- `readmit_ready_and_execute_recipe(...)`
- `checked_readmit_ready_and_execute_recipe(...)`

The point is not to create a runtime engine inside `worth-proof`. The point is to keep execution-admitted law honest at compile time.

## Fixed-Arity Composition

This crate supports static composition where the shape is known and fixed.

Examples:

- `ForkOutputs2<T>`
- `JoinInputs2<T, U>`
- `fork_artifact_pair(...)`
- `join_artifact_pair(...)`
- `join_ready_recipe_pair(...)`
- `compose_join_ready_recipe_pair(...)`

It does not provide:

- arbitrary dynamic graph scheduling
- N-ary runtime planning engines
- generic graph execution frameworks

The composition law here is intentionally narrow and static.

## Composition Families

Some same-commit composition flows need temporary sibling-local symbolic identity.

That is what these surfaces are for:

- `CompositionFamilySymbol<S>`
- `AuthoritativeFamilyMember<A>`
- `FamilyResolvedReference<S, A>`
- `FamilyLifecycleAction<S, A, P>`
- `LoweredFamilyProgram2<S, A, P>`
- `resolve_family_symbol(...)`
- `lower_deterministic_family_pair(...)`

The key law is:

- family symbols are not authoritative identities
- authoritative members are not symbolic placeholders
- deterministic family lowering must happen before one coherent authority boundary closes

## Relationship To worth-foundational

Use `worth-proof` for:

- phase and stage progression
- proof-bearing forms
- witness-authorized transitions
- freshness and rebind law
- fixed-arity static composition
- composition-local symbolic family lowering

Use `worth-foundational` for:

- diagnostics and explanatory surfaces
- provenance and lineage
- receipts and support artifacts
- profiles and descriptive elision policy
- shared equivalence, locator, and boundary vocabulary

In short:

- `worth-proof` says what is legal and what has been proven
- `worth-foundational` says how to describe and package that at boundaries

## Up-Front Contract Vocabulary

When the question is "what must be true before this operation is legal?", use:

```rust
use worth_proof::contracts::*;
```

This lane exports six reusable contracts without turning `worth-proof` into a
runtime framework:

| Contract | What it prevents | Runtime-owned half |
|---|---|---|
| `Branded<'id, T>` | values from two scoped instances becoming interchangeable | process identity and counters |
| `LinearResource<Id, Terminal, Authority>` | a second terminal transition | registries, enumeration, and `Drop` leak checks |
| `Binding<Axes>` + `binding_axes!` | silently omitting a comparison axis | none |
| `FreshnessSource` + `evaluate_freshness` | callers choosing the observation moment | the clock or generation source |
| `Inverts` / `DerivedFrom` | caller-authored causal claims | portable lineage and provenance descriptions |
| `Performed<Action, Authority, Outcome>` | treating permission to attempt an effect as proof it happened | the transport and its outcome |

The binding macro declares each field, drift variant, comparison, and axis name
in one entry. Pair it with `binding_axis_drift_certification!` so every declared
axis also has a positive and one-axis-drift twin. A declarative macro is used
instead of a derive crate so the certified zero-normal-dependency contract
remains true.

Freshness evidence retains the exact source and policy types. Code expecting an
owner clock and owner policy cannot accept a sample or classification produced
by a caller-defined substitute. Likewise, `Performed` is created only where an
owner actually observes the action outcome; an admitted action is not a
performed action.

Query Phase 8 is the reference adoption. Query keeps runtime identities,
recovery registries, clocks, `Drop` checks, receipts, dispatch, and lineage in
the runtime owner. Its binding comparison, freshness classification, causal
undo proof, and performed-redispatch gate use these generic contracts beneath
the Query facade.

## Owner-Specific Runtime Types

Runtime crates use Proof progression beneath stronger private-minted types.
For example, Query’s bound, executed, published, consumed, and settled
operation phases retain Proof phase and basis law, but ordinary callers use the
Query facade rather than constructing a generic `Artifact` or `Recipe`.

The same rule applies to Relational authoritative publication, Runtime Bridge
installed correspondence, and Signal conditional decisions. A generic Proof
carrier cannot replace an owner-specific authority, and a caller-selected
`AuthorityMarker` cannot open a governed runtime door.

Use raw Proof APIs when you are designing a genuinely reusable progression.
Use the owner crate’s facade when the progression controls that runtime’s
operation, truth, correspondence, or evaluation authority.

## Small Examples

### Minimal Recipe

```rust
use worth_proof::{Recipe, Unresolved};

let unresolved = Recipe::<Unresolved, _>::new("payload");
assert_eq!(unresolved.payload(), &"payload");
```

This is the smallest honest entrypoint for staged progression. It starts with no basis and no stronger guarantees.

### Minimal Fixed Shape

```rust
use worth_proof::{NonEmpty, Pair};

let pair = Pair::new("left", "right");
let items = NonEmpty::new("head", vec!["tail"]);

assert_eq!(pair.left(), &"left");
assert_eq!(items.first(), &"head");
```

Use these when the shape itself carries a real invariant and should not stay implicit in a raw tuple or `Vec`.

## Real Example

```rust
use worth_proof::{
    AdmitExecutionReadyRecipeTransition, AuthorityMarker, AuthorityWitness, CapabilityMarker,
    CapabilityWitness, ContextualTransition, ExecuteReadyRecipeTransition,
    ExecutionReadinessContext, LowerRecipeTransition, Recipe, RecipeResolutionContext,
    ResolveRecipeTransition, Transition, Unresolved,
};

struct ResolutionAuthority;
impl AuthorityMarker for ResolutionAuthority {}

struct LoweringCapability;
impl CapabilityMarker for LoweringCapability {}

struct ReadinessAuthority;
impl AuthorityMarker for ReadinessAuthority {}

fn progress(
    resolution_authority: AuthorityWitness<ResolutionAuthority>,
    lowering_capability: CapabilityWitness<LoweringCapability>,
    readiness_authority: AuthorityWitness<ReadinessAuthority>,
) {
    let unresolved = Recipe::<Unresolved, _>::new("payload");
    let resolved = ResolveRecipeTransition.transition(
        unresolved,
        RecipeResolutionContext::new(8_u8, resolution_authority),
    );
    let lowered = LowerRecipeTransition::new(lowering_capability)
        .transition(resolved.into_value())
        .into_value();
    let ready = AdmitExecutionReadyRecipeTransition.transition(
        lowered,
        ExecutionReadinessContext::new("runtime admission", readiness_authority),
    );
    let executed = ExecuteReadyRecipeTransition.transition(ready.into_value()).into_value();

    let _payload = executed.payload();
}
```

This is verbose on purpose:

- resolution, lowering, readiness, and execution stay distinct
- each trust-bearing progression step names its witness or context
- the final form is stronger than the initial one in a way the compiler can see

## Recommended Reading Order

If you are new to the crate, read these next:

1. [Artifact](./docs/features/artifact.md)
2. [Assumption Basis](./docs/features/assumption-basis.md)
3. [Freshness And Downgrade](./docs/features/freshness-and-downgrade.md)
4. [Boundary Readmission](./docs/features/boundary-readmission.md)
5. [Proof Markers And Sets](./docs/features/proof-markers-and-sets.md)
6. [Structural Facts](./docs/features/structural-facts.md)
7. [Witnesses](./docs/features/witnesses.md)
8. [Fixed-Shape Collections](./docs/features/fixed-shape-collections.md)
9. [Proven Vectors](./docs/features/proven-vectors.md)
10. [Recipes And Stages](./docs/features/recipes-and-stages.md)

## Glossary

### admitted

A recipe stage that has passed admission law but is not yet necessarily execution-ready.

### artifact

A general proof-bearing carrier `Artifact<P, T, S, A>` with explicit phase, payload, proof set, and basis.

### assumption basis

The explicit basis a stronger form currently depends on.

### authority witness

A zero-sized trusted witness that authorizes a transition requiring authority.

### boundary bridged

A form whose basis was explicitly weakened by crossing a trust boundary.

### capability witness

A zero-sized trusted witness that authorizes a transition requiring some capability lane.

### checked transition

A transition surface that preserves denial, deferment, stale, rebind, or failure categories instead of flattening them.

### composition family

A same-family composition scope where symbolic siblings and authoritative members must stay distinct until deterministic lowering completes.

### current validity

The freshness state that says the basis is still strong and current.

### executed

A stronger form representing a recipe that has passed through execution progression.

### execution-ready

A stronger form representing a lowered recipe that is admitted for execution.

### lowered

A recipe stage where higher-level intent has already been lowered into the next executable or operational form.

### phase

The static phase parameter carried by `Artifact<P, ...>`.

### proof

A sealed zero-sized carrier representing an established fact such as canonical order or uniqueness.

### proof set

A statically known grouping of proofs, such as `NoProofs`, `Proof<P, A>`, or `ProofSetCons<Head, Tail>`.

### rebind required

A freshness state meaning the form must be rebound before trusted use can continue.

### readmission

The explicit act of regaining a strong basis after a boundary bridge or similar weakening.

### recipe

The main staged progression carrier `Recipe<S, T, A>`.

### resolved

A recipe stage after authority-backed resolution but before lowering.

### stale readable

A freshness state meaning the form is still readable but no longer current.

### structural fact

A named proof fact such as canonical order, uniqueness, disjointness, or normalization.

### transition outcome

A typed outcome surface that preserves success and non-success categories separately.

### unresolved

The initial recipe stage before authority-backed resolution.

### witness

A sealed zero-sized authority or capability marker used to authorize transitions.
