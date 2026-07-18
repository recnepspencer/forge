# Milestone 9.13.2: Query Authority Crate Decomposition

## Goal

Replace the `worth-query` monolith with a small, one-way package graph whose
physical boundaries match Query's declaration, installation, admission,
execution, publication, and certification authorities. A developer changing
one authority must be able to compile and run that authority's tests without
building unrelated Query authorities.

This milestone is a production decomposition, not a test-runner project. Cargo
package selection is the iteration mechanism.

## Why This Milestone Exists

Milestone 9.13.1 reduced warm compiler certification from roughly 399 seconds
to roughly 4 seconds, but the ordinary 2,981-test library lane still takes
roughly 118 seconds warm. That floor is expected: one package contains about
half a million lines across declaration, runtime, publication, and historical
certification responsibilities, so Cargo cannot select a smaller coherent
unit.

The existing module tree is not itself the desired package graph. In
particular, `runtime`, `application`, `domain_capabilities`, `consumer_kit`,
and `harness` contain multiple authorities and cannot be moved wholesale into
same-named crates. Migration follows semantic ownership, even when that means
splitting an existing folder.

## Governing Summaries

- `MENTALITY.md`: solve package selection as a production-boundary problem;
  do not replace one monolith with a manifest interpreter or proof platform.
- `arch_laws.md`: phase proofs flow one way, executors consume lowered plans,
  and public construction is sealed by the authority that proves it.
- `composition_laws.md`: every package and file owns a named responsibility;
  facades aggregate and do not implement.
- `domain_structure_laws.md`: package topology preserves truth source,
  lifecycle, failure mode, dependency direction, and test ownership.
- `perf_laws.md`: ordinary paths must not inherit reconstructive,
  certification, source-scanning, or diagnostic-richness cost.
- Road 1 `NAMING.md` and `BOUNDARIES.md`: Query is a reviewed framework-family
  exception; ordinary entry and cert consumers reach it only through the
  audience facade legal for their band.
- `WORTH_query_roadmap.md`: this split precedes 9.14 so installed operation and
  downstream authority are born in the final package graph.

## Adversarial Constraint

A change confined to one Query authority must compile and test through that
authority package without building later authorities, while the complete graph
must make all reverse knowledge, alternate authority roots, deep imports,
facade behavior, shared-support buckets, and compatibility re-export cycles
unrepresentable or mechanically rejected.

The hostile false success to refuse is a set of crates underneath an aggregate
`worth-query` package that every facade still compiles. That shape changes
folders while preserving the monolithic build and authority surface.

## Product Decision Lock

- The current `worth-query` package is a migration source, not the final
  aggregation root. It is deleted after the three audience facades and all
  consumers cut over.
- Query's internal packages are an exact, reviewed extension of the existing
  framework-family exception. They are not ordinary Road 1 band crates and are
  not legal consumer dependencies.
- `worth-query-decl`, `worth-query-host`, and `worth-query-replay` remain the
  only public Query package dependencies. Internal packages are reachable only
  by those facades and by later internal packages in the frozen DAG.
- Audience facades contain re-exports only. They may depend on the exact
  internal authority set declared below; they may not contain wrappers,
  constructors, orchestration, policy, or compatibility aliases.
- No `common`, `shared`, `helpers`, `support`, `types`, or test-platform crate
  is created. A type used by two authorities lives with the authority that
  defines its meaning, not in the lowest package that makes a cycle disappear.
- Cross-authority cycles are repaired by moving meaning to its real upstream
  owner or introducing a narrow upstream contract. Re-exporting a downstream
  type through an upstream package is not a cycle repair.
- Certification and replay are cold leaves. No ordinary package depends on
  them, and no ordinary test imports their fixtures or setup.
- Source scans, file catalogs, documentation-source audits, fixture manifests,
  and tests that prove another test exists are deleted. A load-bearing rule is
  enforced through visibility, the type system, the package DAG, boundary-check,
  or a product-behavior test.

## Frozen Package And Dependency Graph

The milestone adds these exact internal framework packages to the Query
exception in `NAMING.md` and `tools/boundary-check/config/road1.toml`:

- `worth-query-declaration`
- `worth-query-installation`
- `worth-query-admission`
- `worth-query-execution`
- `worth-query-publication`
- `worth-query-certification`

Their dependency graph is:

```text
worth-query-declaration
  -> worth-query-installation
       -> worth-query-admission
            -> worth-query-execution
                 -> worth-query-publication

worth-query-certification
  -> declaration + installation + admission + execution + publication

worth-query-decl
  -> declaration

worth-query-host
  -> declaration + installation + admission + execution + publication

worth-query-replay
  -> certification
```

The arrows above mean "is consumed by." In Cargo dependency notation the
direction is reversed: publication depends on execution, execution depends on
admission and installation, and so on. Additional transitive shortcuts are
forbidden unless a public signature genuinely carries an earlier authority
type and the direct edge is recorded in the machine constitution.

### Authority Ownership

`worth-query-declaration` owns canonical query intent, declaration identity,
authoring grammar, result-shape meaning, schema-visible validation contracts,
canonicalization, collection/view declarations, and Query-specific value
typing over Foundational authority. It does not import relational, bridge,
signal, workspace, execution, live, or replay machinery.

`worth-query-installation` owns portable domain package definitions,
installation admission inputs, runtime generation identity, installed domain
and operation identity, conflict semantics, and rebuildable installed indexes.
It does not execute work or mint basis, publication, or consumer authority.

`worth-query-admission` owns basis capabilities, tenant/policy/relationship
admission, intent decision lattices, graph access and obligation admission,
support decisions, and the proof-bearing handoff into execution. Denial
precedes planning and lower-runtime contact.

`worth-query-execution` owns planning, lowering, the installed operating-world
root, workspace and provider binding, effect/workflow progression, lower-
runtime routing, execution receipts, operational counters, and ordinary
re-execution. It cannot construct publication or replay authority.

`worth-query-publication` owns derived publication, authorized projections,
projection consumption, result settlement, live/subscription maintenance,
continuation, collection delivery, shared-consumer lifecycle, and invalidation
meaning. It consumes execution authority and never reopens source truth or
certification replay.

`worth-query-certification` owns cert-only replay/reconstruction, hostile
cross-authority certification, the retained compiler portfolio, and any
consumer kit that truly exists to certify adoption. It contains no production
source scanner and is depended on by no ordinary path.

### Lower-Runtime Direction

- declaration may depend on `worth-foundational` and sealed proof vocabulary
  only where declaration meaning genuinely carries it
- installation may additionally depend on proof authority but not physical
  runtime providers
- admission may consume lower-runtime contract types needed to decide
  eligibility, but performs no lower-runtime work
- execution may depend on relational, runtime bridge, and signal facades
- publication may depend on lower-runtime observation and delivery contracts,
  never on their internal modules
- certification may depend broadly; nothing depends back on it

## Intended Consumer DX

The split changes package ownership, not the ordinary capability grammar:

```rust
use worth_query_decl::facade::CanonicalQueryArtifact;
use worth_query_host::facade::{domain, runtime};

fn install_and_run(
    declaration: CanonicalQueryArtifact,
    workspace: &mut runtime::WorthQueryWorkspace,
) {
    // Domain installation, admission, and execution remain available through
    // the host audience facade. Internal authority crates are not imported.
    let _ = (
        declaration,
        workspace,
        domain::WorthQueryCapabilityFamily::QueryComposition,
    );
}
```

Cert-only code may separately import `worth_query_replay`; ordinary entry,
derived, app, and UI code cannot.

## Phase Plan

### Phase 1: Delete Meta-Proof Machinery And Amend The Query Constitution

Delete source-topology authorities before moving code. This includes the
Milestone 9.6 embedded-source inventory and match table, source-scan closure
fields, documentation-source agreement tests, production-owned test catalogs,
and any remaining proof whose subject is another proof or file location.
Preserve the underlying product invariant through types, visibility,
boundary-check, or a behavior test where it is still load-bearing.

Then amend `NAMING.md`, `road1.toml`, boundary-check, and agent-context with the
six exact internal packages and the per-facade dependency sets above. Replace
the current "every audience depends only on one engine" law; retaining it would
force every audience to compile the aggregate and defeat this milestone.

Record, but do not turn into tests, the current baseline:

- 2,981 ordinary library tests, about 118 seconds warm
- 230 selected compile-fail fixtures, about 4 seconds warm
- 149 tracked Rust files above the cap and not on the workspace allowlist,
  including 78 in `worth-query`

**Adversarial evidence**

- boundary-check rejects an ordinary entry, derived, app, or UI crate that
  imports any internal Query authority package directly
- boundary-check rejects a facade with behavior, an unlisted internal
  dependency, a cross-facade dependency, or a re-export from the wrong
  authority root
- a one-time manual deletion review records that no production module embeds
  Rust source merely to assert another source file contains or omits a token;
  this review is not implemented as another source-scanning test

**Exit condition**

The machine constitution accepts the frozen DAG and rejects every bypass
without requiring the final packages to contain migrated behavior yet. The three
temporary 9.13.1 line-cap exceptions are deleted with their source-audit
responsibility.

### Phase 2: Extract Declaration Authority

Create `worth-query-declaration` and move canonical intent, authoring,
canonicalization, binding grammar, schema-visible validation, identity,
result-shape, collection declaration, and view declaration meaning into it.
Split existing mixed modules by responsibility instead of importing runtime
types back into the declaration package.

`worth-query-decl` becomes a narrow re-export facade over this package. Move
declaration-local unit and integration tests with their production owner. AST
libraries (`syn`, `quote`, `proc-macro2`) may not remain normal declaration
dependencies unless a real declaration feature, rather than source auditing,
requires them.

**Adversarial tests**

- the declaration package builds with relational, runtime bridge, signal,
  workspace, live, and certification packages absent from its dependency graph
- equivalent public construction paths still produce identical canonical
  declarations, result shapes, native value identity, and exact counters
- compile denial proves a declaration cannot mint installation, admitted,
  executed, publication, or replay authority

**Exit condition**

`cargo test -p worth-query-declaration` proves declaration meaning without
building later Query authorities, and declaration consumers compile through
`worth-query-decl` only.

### Phase 3: Extract Installation Authority

Create `worth-query-installation` and move portable domain packages, installed
domain identity, generation affinity, canonical operation and contribution
definitions, conflict detection, and derived installation indexes into it.
Separate portable definition from volatile runtime providers. Rebuildable
indexes remain derived from installed artifacts and are not serialized as
authority.

Existing `domain_installation`, `domain_capabilities`, `application`, and
`runtime` folders are split where necessary. This phase must not move their
execution, projection, workflow, or certification portions into installation
merely to avoid a dependency edge.

**Adversarial tests**

- equivalent packages converge across declaration order while one-field
  semantic conflicts fail atomically with zero installed residue
- foreign-runtime, stale-generation, copied semantic-key, and locally
  reconstructed handles cannot resolve installed authority
- destroying and rebuilding every installation index preserves exact lookup,
  denial, identity, and counter outcomes

**Exit condition**

`cargo test -p worth-query-installation` runs without admission, execution,
publication, or certification packages, and no portable definition contains a
callback or provider object.

### Phase 4: Extract Admission Authority

Create `worth-query-admission` and move basis lifecycle, intent decision
lattice, tenant/policy/relationship admission, graph access and obligation
admission, support decisions, and typed execution handoffs into it. Admission
consumes declaration and installation authority and produces the only type
accepted by execution.

Planning, provider calls, workspace mutation, publication, and replay remain
outside this package. If admission currently calls runtime code to discover a
decision, repair the contract so the required authoritative input is presented
at admission rather than retaining a reverse edge.

**Adversarial tests**

- stale basis, wrong tenant, foreign installation, unsupported capability, and
  policy violation deny before planning, allocation, or lower-runtime counters
  increment
- success, advisory, and violation traces retain exact typed context across
  facade and direct-authority paths
- compile denial proves raw declarations and reporting digests cannot be passed
  to execution as admitted handoffs

**Exit condition**

`cargo test -p worth-query-admission` builds declaration and installation but
not execution, publication, or certification, and every denial path performs
zero later-phase work.

### Phase 5: Extract Installed Execution Authority

Create `worth-query-execution` and move planning, lowering, the installed
operating-world root, workspace/provider binding, lower-runtime routing,
effect/workflow progression, recovery posture, execution receipts, and
operational counters into it. The package consumes admitted handoffs; it does
not re-decide declaration legality, installation compatibility, basis, policy,
strategy, or artifact posture.

Split the current 168k-line `runtime` tree by authority. Runtime observation
and delivery behavior moves later with publication; test-only and cert-only
behavior moves later with certification. `application` orchestration moves only
when its named operation belongs to execution.

**Adversarial tests**

- executors reject raw declarations, unadmitted plans, foreign providers,
  alternate operating roots, and stale installation generations before
  lower-runtime contact
- serial/parallel, direct/facade, and rebuilt-index execution converge on
  outcomes, receipts, warnings, result state, and exact counters
- compile denials prove execution inputs are move-only and an executor cannot
  construct publication, consumption, settlement, or replay authority

**Exit condition**

`cargo test -p worth-query-execution` builds no publication or certification
package. The host facade can execute the ordinary read and workflow transcripts
using one installed root without deep imports.

### Phase 6: Extract Publication And Consumption Authority

Create `worth-query-publication` and move derived publication, authorized
projection, projection consumption, settlement, live/subscription maintenance,
continuation, invalidation, collection delivery, and shared-consumer lifecycle
into it. It consumes execution receipts and admitted publication semantics; it
does not query source truth to reconstruct missing meaning.

Move publication-owned portions out of `subscription`,
`projection_consumption`, `authorized_projection`, `view_shape_live`,
`ordinary`, `runtime`, and `domain_capabilities`. Delete parallel miniature
consumers and any compatibility tuple that can combine detached receipts,
facts, bases, or lifecycle artifacts.

**Adversarial tests**

- cross-run, cross-basis, cross-installation, cross-operation, stale, disposed,
  and digest-lookalike publications fail at the first boundary with zero
  consumption or maintenance work
- one-shot and live delivery converge with fresh execution for identity,
  ordering, result state, warnings, facts, patches, and exact counters
- compile denials prove consumption cannot precede publication, settlement
  cannot precede consumption, and move-only authority cannot be reused

**Exit condition**

`cargo test -p worth-query-publication` builds the ordinary upstream graph but
not certification. A consumer reaches publication and settlement only through
`worth-query-host`.

### Phase 7: Extract Certification, Replay, And Owned Compiler Proof

Create `worth-query-certification` as the cold leaf. Move cert-only replay,
reconstruction and hostile cross-authority behavior into it. A reusable
consumer certification helper is permitted only when it executes a real
public journey; it may not expose a registry, manifest, source scanner, or
proof-bundle API. Delete milestone-shaped harness modules,
fixture catalogs, source manifests, report digests that only certify test
topology, and test setup that fabricates already-solved receipts.

Move each ordinary behavioral test to its production authority. Retain at most
one direct compile-fail target per authority package, and only where a
compiler-visible public invariant cannot be proven behaviorally. Cross-package
and replay-fence denials live in certification. Do not recreate 38 trybuild
harnesses in six smaller crates.

**Adversarial tests**

- removing the certification package from the graph leaves declaration,
  installation, admission, execution, and publication builds unchanged
- cert-only replay can reconstruct declared scenarios, while host, entry,
  derived, app, and UI packages cannot name replay types or import the replay
  facade
- retained compile fixtures fail first at their named authority boundary;
  orphan `.stderr`, generic privacy, historical tombstone, and proof-of-proof
  fixtures remain absent

**Exit condition**

Authority-local test commands no longer build `worth-query-certification`.
The full cert command runs the union of owned behavioral, selected compiler,
boundary, and replay proof without production fixture registries.

### Phase 8: Cut Facades And Consumers, Then Delete The Monolith

Expand the three audience facades only with explicit re-exports from their
allowed internal packages. Cut root workspace consumers, `worth-server`, Worth
UI's Query binding crate, Road 1 entry crates, and cert crates to the legal
facade for their audience. No consumer receives a temporary direct internal
dependency.

Once consumers are green, delete `crates/worth-query`, its compatibility
re-exports, obsolete root workspace dependency, and any alias that preserves
the old crate spelling. Update boundary-check snapshots and generated agent
contexts in the same phase.

**Adversarial tests**

- repository dependency inspection reports zero packages depending on
  `worth-query` or any internal authority package outside the exact facade and
  internal DAG allowlists
- declaration-only, ordinary host, and cert/replay consumer transcripts each
  compile with exactly their intended package closure; app/UI and derived
  replay imports fail mechanically
- deleting the former monolith directory changes no canonical declaration,
  installation, admission, execution, publication, settlement, replay, or
  counter outcome

**Exit condition**

The monolith and all compatibility paths are absent. Boundary-check and
agent-context enforce the new graph, and every supported consumer journey uses
one of the three audience facades.

## Test Ownership Rules

- Unit tests live in the authority package whose private invariant they
  falsify.
- Integration tests live in the package whose public contract they exercise.
- A test may build upstream dependencies, but not later or sibling authorities
  irrelevant to the behavior under test.
- Cross-authority end-to-end journeys and cert-only reconstruction live in
  `worth-query-certification`.
- Compiler fixtures are selective counterexamples, not exhaustive privacy
  mirrors. One fixture may represent a structurally equivalent family.
- Test helpers follow the same dependency direction as production. No global
  test world, shared fixture package, or test-only authority constructor is
  permitted.
- No test scans another test, counts tests, asserts a fixture path exists, or
  compares a product digest to a test-manifest digest.

## Complexity And Iteration Obligations

- `cargo check -p worth-query-declaration` and its ordinary tests build no
  other Query authority package.
- Each later authority-local command builds only its transitive upstream Query
  authorities; certification is absent unless explicitly selected.
- Per-authority warm elapsed observations are recorded once at phase close.
  They are decision evidence, not timing tests or permanent thresholds.
- The declaration and installation packages contain no lower-runtime calls.
- Admission denials perform exact zero planning, allocation, provider, and
  publication work.
- Execution does no certification, replay, source scanning, or rich diagnostic
  materialization unless an admitted artifact policy requests operational
  diagnostics.
- Publication work scales with declared/affected publication and consumer
  breadth, not unrelated workspace, registry, test, or diagnostic inventory.
- Every touched Rust code or test file is at most 400 lines unless the phase
  spec names a short-lived exception with owner and deletion phase. The
  migration may not expand the existing workspace allowlist to normalize debt.

## Must Preserve

- Foundational remains value, contract, canonical wrapper, and validation
  authority.
- Relational remains truth, commit, snapshot, branch, and merge authority.
- Runtime bridge remains cross-runtime routing, preview, writeback, and bridge
  protocol authority.
- Signal remains scheduling, invalidation execution, observation, temporal,
  and async lifecycle authority.
- Query remains canonical declaration, installation, admission, execution
  orchestration, result shaping, publication/consumption, and public
  certification authority at the package named for each responsibility.
- Store-facing declarations and provider-independent semantics remain
  unchanged; physical persistence stays in the Store roadmap.
- Every retained compiler-owned authority, substitution, phase-order,
  move-only, and facade denial survives with the production owner.

## Allowed Debt

- Store-backed execution, durable restore, restart-stable continuation, and
  physical provider integration remain Store milestones.
- Later 9.14 operation, workflow, sharing, lease, invalidation, and patch
  semantics remain unimplemented until 9.14, but their package homes and
  dependency direction must be ready.
- No compatibility re-export, alternate engine root, direct internal-package
  consumer edge, source-audit authority, shared test platform, reverse
  dependency, or ordinary-to-certification dependency may remain debt.

## Verification

At each phase, run the new authority-local package tests before any all-up
command. At milestone close run:

```text
cargo fmt --all -- --check
cargo check --workspace --tests
cargo test -p worth-query-declaration
cargo test -p worth-query-installation
cargo test -p worth-query-admission
cargo test -p worth-query-execution
cargo test -p worth-query-publication
cargo test -p worth-query-certification
cargo test -p worth-query-decl
cargo test -p worth-query-host
cargo test -p worth-query-replay
cargo run --manifest-path tools/boundary-check/Cargo.toml -- --root .
cargo run --manifest-path tools/agent-context/Cargo.toml -- check
scripts/ci/check_workspace_rust_line_caps.sh
git diff --check
```

Also run the Worth UI-owned Query binding suite from its workspace and the
root consumers that use `worth-query-host`. Full workspace verification is a
closeout gate, not the inner development loop.

## Acceptance Evidence

- the exact package DAG and audience matrix are machine-enforced
- every authority-local package command omits unrelated later authorities
- ordinary declaration, installation, admission, execution, publication, and
  settlement journeys preserve canonical outcomes and exact counters
- cert-only replay remains inaccessible to ordinary consumers
- Worth UI and server consumers use legal audience facades with no direct
  internal or former-engine dependency
- retained compiler denials remain load-bearing and the compiler portfolio
  does not regress into per-type privacy mirrors
- source-audit catalogs, proof-of-proof tests, shared test platforms,
  compatibility aliases, and the `worth-query` monolith are absent
- the workspace line-cap guard is green for the migrated packages
- one closeout report records per-authority and full-graph elapsed observations
  without enforcing them through a timing framework

## Sequencing Notes

This milestone follows 9.13.1 because the obvious compiler waste and consumer
coupling must be removed before a multi-week package migration. It precedes
9.14 because installed operation semantics, bound execution progression,
publication, consumer support, sharing, and settlement must land directly in
their permanent authority packages.

Phases are ordered by the dependency DAG. Work inside a phase may be parallel,
but a downstream package must not be created as a dumping ground before its
upstream contracts are frozen.
