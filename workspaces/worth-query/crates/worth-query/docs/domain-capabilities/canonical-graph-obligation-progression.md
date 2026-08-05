# Canonical Graph Obligation Progression

## What This Feature Is

An installed application query or operation carries the graph work it requires.
Query selects that installed meaning, admits one bounded plan, executes it in
one managed session, and publishes only what the real terminal proves. Use this
model when application work reads graph relationships, checks access, mutates
graph state, or runs installed invariants.

The caller declares application meaning. The caller does not register runtime
obligations, choose lower owners, construct plans, or run validators directly.

## Why You Use It

- Give every application query and operation one inspectable graph-work
  contract.
- Keep graph-read planning, authorization, invariant execution, and commit in
  one affinity-bound progression.
- Preserve exact ownership: Relational owns graph truth, Runtime Bridge owns
  installed correspondence, Signal owns policy evidence, and Query composes
  their evidence.
- Prevent a read receipt or publication object from becoming execution
  authority.

## Stable Entry Points

Application authors use the declaration and host facades:

- `worth_query_declaration` application-schema, query, and operation macros;
- `worth_query_host::facade::domain` for installed query or operation
  inspection;
- `worth_query_host::facade::inspect_installed_graph_obligations(...)`
  for read-only downstream adoption evidence;
- the ordinary application runtime for query and operation execution; and
- `worth_query_host::facade::publication` for non-authoritative terminal
  receipts and inspection.

Raw selection, plan review, capacity reservation, session progression, and
terminal construction are integration boundaries. They are not caller
authoring APIs.

## Core Mental Model

An obligation is installed meaning, not a callback. For one exact installed
query or operation, Query derives a sealed set containing the required work,
the lower owners that must contribute evidence, the selection basis, resource
posture, effect posture, and required terminal.

```text
typed application declaration
  -> sealed installed obligation set
  -> selected obligation rows
  -> requirement and cost review
  -> capacity-reserved graph-work plan
  -> branch- and basis-bound provider session
  -> actual lower-owner work
  -> execution terminal
  -> non-authoritative publication and inspection
```

The typed branch is part of the authority. A version number from another
branch is not interchangeable, even when its numeric value is equal. Plan,
session, decision facts, proposal, invariant evidence, commit, retry, receipt,
and publication all retain the same branch-qualified basis.

## How It Executes

Read-only application work follows one path:

```text
installed query
  -> graph-read obligation selection
  -> specialized graph-read requirement and budget review
  -> managed session
  -> session-owned graph read
  -> read terminal and resource-release evidence
  -> disclosed result and publication receipt
```

Mutation work continues from the same session spine:

```text
installed operation
  -> complete authorization decision read-set
  -> proposed state
  -> every selected installed invariant runs through its real owner
  -> sealed Relational validated candidate
  -> compare-and-commit
  -> fresh or recovered commit terminal
  -> publication receipt
```

Selection never counts as execution. A selected invariant becomes successful
only after the installed provider executes it against the exact proposed state.
Publication consumes a terminal; it cannot execute, retry, or commit work.

## Small Example

Given an installed application query, downstream code may inspect its sealed
obligation set without acquiring planning or execution authority:

```rust
use worth_query_host::facade::inspect_installed_graph_obligations;

let proof = inspect_installed_graph_obligations(
    "reporting-service",
    installed_query.graph_obligations(),
)?;

assert_eq!(proof.subject_name(), "account_activity");
assert_eq!(proof.rows().len(), 1);
```

`WorthQueryGraphObligationAdoptionProof` exposes installed identities, kinds,
required owners, terminal requirements, and selector-index size. It exposes no
registration, selection, planning, execution, invariant, or commit transition.

## Real Example

The external host-consumer test installs a normal declarative query, obtains
its installed query handle, and compares the graph-read obligation's bound
planning identity with the installed query's canonical read graph. It also
proves the read-only adoption view reports one `GraphRead` row owned by
Relational and requiring a `GraphReadProduct` terminal.

The executable example is
`worth-query-host/tests/canonical_graph_progression.rs`. It deliberately lives
in the host crate, whose normal dependency graph contains destination packages
and no `worth-query` monolith dependency.

For application execution, use the ordinary typed host API. Do not reproduce
the admission sequence shown above in application code; Query performs it as
part of preparing and executing the installed query or operation.

## How It Relates To Other Features

- [Graph Read Access Planning](../authoring/graph-read-access-planning.md)
  explains the requirement, budget, and support review inside application-query
  admission.
- [Provider Sessions And Decision Read-Sets](./provider-sessions-and-decision-read-sets.md)
  explains session affinity and complete authorization facts.
- [Provisional State And Invariant Execution](./provisional-state-and-invariant-execution.md)
  explains real installed invariant execution before commit.
- [Graph Composition Authoring](../authoring/graph-composition-authoring.md)
  is the preserved generic workspace mutation engine. It is not an alternate
  application-obligation executor.
- The generic workspace-read engine remains available for its existing
  non-application responsibilities. It is not a second application-query plan.

## Inspection And Debugging

Inspect the progression at its stable boundaries:

- installed obligation identity and rows;
- selected-row counters and exact owner requirements;
- graph-read requirement, cost, budget, and inventory review;
- plan, session, managed-run, branch, and basis identities;
- complete authorization decision facts;
- invariant execution counts and dispositions;
- terminal cleanup and resource-release evidence; and
- publication work counters and fresh-versus-recovered posture.

When identities disagree, treat the result as affinity drift. Do not compare
rendered strings or reconstruct an identity from a version number.

## Anti-Patterns

- Registering graph obligations in a consumer crate.
- Treating selection, support, or a no-op callback as execution.
- Passing a manual invariant pack to generic graph or read composition.
- Importing admission internals to construct or review a raw plan.
- Calling Relational directly from an application-query lane.
- Defaulting a missing branch to `"main"`.
- Publishing a receipt before terminal cleanup is proved.
- Keeping a compatibility alias that can mint a retired authority product.

## Current Limits

- This progression does not add durability, restart recovery, or store-backed
  checkpointing.
- It does not implement multiple branch heads, branch creation, merge, rebase,
  or concurrent branch writers.
- Read-only adoption is inspection, not a consumer extension system.
- Generic monolith read and graph-composition features remain preserved until a
  later named migration proves their parity and authorizes their retirement.

## Compatibility And Migration Map

| Inherited surface | Preserved guarantee | Current owner and integration | Retirement posture |
|---|---|---|---|
| 9.9 graph-obligation meaning | Kinds, owner routes, selection basis, resource/effect posture, terminal requirements, denials, and bounded lookup | Installation owns the sealed set; admission selects and plans; execution obtains actual owner terminals | The monolith obligation tree, fake executor, dispatch envelopes, and authority-capable Consumer Kit registration/execution are retired; generic graph composition remains |
| 9.9 manual invariant callbacks | Lawful success and violation behavior for installed invariant families | The application-operation session invokes the installed provider; Relational seals the validated candidate after all real invariant phases | Callback and no-op composition authority is retired; contributed capability denial inspection remains non-executable |
| 9.10 graph-read access planning | Requirement derivation, inventory matching, cost, budget, typed denial, and inspection | Admission owns the specialized application-query review; execution consumes it through the session-owned read port | Public raw planning-input/review construction and the application-query monolith admitted-plan export are retired; generic workspace reads remain |
| 9.11 downstream adoption | Honest public-facade adoption and exact consumer residue | Host exposes read-only installed-obligation inspection; certification and consumer suites prove source residue | Local graph-obligation registration, execution proof workspaces, and support-pin authority are retired; the unrelated generic Consumer Kit remains |
| 9.15 provider sessions | Provider lifecycle, complete decision read-sets, provisional state, installed invariant execution, and cleanup | Execution binds the existing managed run as the exact worker inside the application graph-work session | Integrated unchanged; it is a lower worker, not a competing application session or receipt authority |
| 9.16 Phase 6 application queries | Identity, parameters, basis, ordering, projection, continuation, history, preview, live, result shaping, and publication behavior | Every lane consumes the destination graph-read plan and session; semantic result shaping remains with application query | Parallel direct Relational execution and parallel application-query plan authority are retired |
| 9.16 Phases 7.1-7.2 access | Installed capability meaning, purpose, request binding, trusted time, exact grant/path, policy evidence, and revalidation | The complete evidence set is observed and revalidated inside the same branch-bound session | Pre-session observation and any reconstructed authorization scope are retired |
| Query and operation publication | Existing result/commit identity, omission, work, cleanup, response-loss, and retry behavior | Publication derives read-only receipts from execution terminals and distinguishes fresh execution from recovered prior commit | Public constructors and any selected-versus-executed synthetic outcome path are retired |
| Facades and downstream consumers | Ordinary source behavior and full Query, Bank, and Worth UI features | Declaration and host facades lower one way into destination packages; Bank and Worth UI consume those facades | Only aliases that reopen retired authority are removed; unrelated adapters and monolith behavior remain |

## Successor Contract

Later disclosure and noninterference work begins from the same installed query,
typed branch, parameter binding, graph-read plan, managed session, access
decision, result shaping, continuation, history, preview, live, and publication
contracts. It must not add a disclosure-specific selector, planner, raw graph
executor, authorization lane, receipt, or branch default.

## Related Docs

- [Graph Touch Obligation Authority](../authoring/graph-touch-obligation-authority.md)
- [Graph Read Access Planning](../authoring/graph-read-access-planning.md)
- [Provider Sessions And Decision Read-Sets](./provider-sessions-and-decision-read-sets.md)
- [Provisional State And Invariant Execution](./provisional-state-and-invariant-execution.md)
- [Runtime-Installed Domains And Operations](./runtime-installed-domains.md)
