# Graph Touch Obligation Authority

Graph touch obligation authority is Query's public model for deciding which
graph obligations apply to a graph-shaped operation.

It exists so consumers do not build local validator tables, local legality
graphs, or command-adjacent callback piles to remember which checks must run.
Most runtimes can traverse a DAG or run callbacks near graph operations. This
feature does something stronger: Query turns declared graph meaning into
runtime obligations with evidence.

Use it when the operation is graph-shaped and the real question is not just
"what nodes are connected?" but "what does this graph touch require before the
operation is honest in this operating world?"

The ordinary path is:

```text
touch descriptor + operating world descriptor + obligation index
  -> selected obligations
  -> dispatch plan
  -> runtime executor verdict
  -> receipt, trace, support row, and diagnostic evidence
```

## What This Feature Is

A graph touch names the graph shape a Query operation is trying to access or
mutate. It can include entity kinds, relation kinds, authority lanes, branch or
preview posture, read shape, live retention, aspect reads or writes, ownership
movement, traversal breadth, and other touch facts that are relevant to
obligation selection.

Graph Touch Obligation Authority lets you move graph semantics out of callers
and into Query's runtime contract. Use it when you need graph checks selected
from declared touch meaning, obligation behavior that changes between
authoritative, preview, branch, live-read, policy-aware mutation, construction,
and downstream adoption worlds, or budget-aware execution that can deny with
`BudgetExceeded` before broad state load.

This applies to graph-bearing writes and graph-bearing reads. Read-family,
live-read, preview, branch, construction, operator-catalog, and policy-aware
lanes are part of the authority model when they carry covered graph meaning.

An operating world descriptor names where the touch is being evaluated:
authoritative workspace, branch, preview, runtime-backed test workspace, or a
future admitted world. It is part of the selector input. A branch-local graph
touch is not automatically equivalent to an authoritative graph touch.

An obligation index is the Query-owned registry of graph obligations. It is the
place consumers register domain obligations and the place Query derives the
dispatch plan from. This index is not a host-local lookup cache and not an
optional performance helper.

## Stable Entry Points

Use the public facade and graph-obligation modules rather than local ceremony:

- graph touch descriptor builders and inspected descriptor evidence
- operating world descriptor builders and world support posture
- graph obligation registration declarations
- graph obligation selector coverage reports
- graph obligation support rows and support pinning
- graph obligation dispatch envelopes
- graph obligation executor verdicts and in-memory proof workspaces
- graph obligation adoption manifests and residue manifests

The exact type names can evolve with the public facade, but those jobs must
remain Query-owned. If a consumer cannot perform one of these jobs through the
facade, that is a product gap, not permission to invent a private authority.

The feature sits across adjacent surfaces:

- graph composition authors graph-shaped mutations
- read composition and live views declare graph-shaped access
- intent admission carries selected obligation evidence through execution lanes
- support matrix rows say whether each lane is supported, unsupported,
  diagnostic-only, not applicable, or deferred
- Consumer Kit proves downstream adoption and residue honestly

## Obligation Kinds

Graph obligation support and certification rows must use the same obligation
kind vocabulary everywhere:

- `BlockingInvariant`
- `SchemaContractValidator`
- `AdvisoryObligation`
- `PreflightSequencingObligation`
- `CapabilityGapScreen`
- `OperatingContextGate`

These names are semantic. A `BlockingInvariant` is not merely a hard error. A
`CapabilityGapScreen` is not merely a validator that happens to deny. A
`OperatingContextGate` is not a branch flag check. The kind tells Query how the
obligation participates in admission, execution, diagnostics, and support
posture.

Canonical kind labels are `blocking-invariant`,
`schema-contract-validator`, `advisory-obligation`,
`preflight-sequencing-obligation`, `capability-gap-screen`, and
`operating-context-gate`.

## Support Statuses

Graph obligation support rows use the same support status vocabulary as the
certification and support matrix surfaces:

- `Supported`
- `Unsupported`
- `NotApplicable`
- `DiagnosticOnly`
- `DeferredToBackstop`

`DiagnosticOnly` means Query can explain the touch and selected obligation but
does not claim authoritative enforcement. `DeferredToBackstop` means the
ordinary graph obligation lane did not own the final decision and the docs must
name the backstop honestly.

Canonical support status labels are `supported`, `unsupported`,
`not-applicable`, `diagnostic-only`, and `deferred-to-backstop`.

## Covered Lanes

The graph touch obligation hostile certification matrix and public docs use the
same covered lane vocabulary:

- graph composition
- authoritative command batch
- scalar mutation
- effect-triggered write intent
- declaration entry
- contribution orchestration
- read family
- live read
- preview mutation
- preview intent
- branch intent
- policy-aware graph mutation
- primitive construction birth
- worth-topo operator catalog
- worth-kernel phase chain

Canonical covered lane labels are `graph-composition`,
`authoritative-command-batch`, `scalar-mutation`,
`effect-triggered-write-intent`, `declaration-entry`,
`contribution-orchestration`, `read-family`, `live-read`,
`preview-mutation`, `preview-intent`, `branch-intent`,
`policy-aware-graph-mutation`, `primitive-construction-birth`,
`worth-topo-operator-catalog`, and `worth-kernel-phase-chain`.

These lanes are authority surfaces, not implementation conveniences. A support
row may be `NotApplicable` or `DeferredToBackstop` for a lane, but it must not
hide the lane behind a generic "batch", "operator", or "validator" label.

## Execution And Budgets

Selection is deterministic and bounded by descriptor data plus the obligation
index. Execution is budgeted runtime work. Large graph and boolean-like
operations must never be documented as unbounded automatic execution.

Budget accounting must expose:

- `BudgetExceeded` for denied budget overflow
- `budget-exceeded` as the canonical execution-status label
- state-load counters for executor state reads
- cost classes such as `sparse-topology` for each planned obligation and
  executor step
- artifact-policy-gated diagnostics for expensive evidence

The escape hatch for a denied budget is not "try harder locally." Consumers can
ask for a smaller touch, a different admitted world, a more explicit selector,
a stronger support pin, or an artifact policy that permits the required
diagnostic evidence. They cannot bypass Query by running private graph walks and
calling the result equivalent.

## Consumer Contract

The Consumer Kit is the ordinary downstream adoption path for graph obligation
work. It must cover registration, selector coverage, support pinning,
in-memory execution proof, bypass audit, adoption manifests, and residue
manifests.

When a downstream crate adopts graph obligations, it should be able to prove:

- every supported obligation is registered through Query
- every relevant touch descriptor has selector coverage
- every depended-on support posture is pinned by obligation kind, support lane,
  expected status, and budget digest where the consumer depends on a specific
  execution budget
- every proof test can run in a real in-memory Query workspace
- execution-backed adoption proof connects selected obligations to real
  executor rows and a manifest execution proof digest
- every local bypass or local ceremony residue is visible in an audit
- every adoption manifest names what was moved into Query
- every residue manifest names what remains, why it remains, and whether it is
  compatibility-only or a product gap

## Anti-Patterns

- local validator maps keyed by node kind or relation kind
- recursive graph walks that rediscover the same touch facts per operation
- treating manual invariant packs as the primary covered graph obligation path
- treating this as a DAG traversal helper instead of an obligation authority
- unbounded graph expansion hidden behind "automatic index provisioning"
- support rows that name a family but not the covered obligation kinds
- test receipts or diagnostics that are fabricated without executor verdicts
- treating branch, preview, and authoritative worlds as the same selector input

Manual invariant packs are compatibility/custom extension surfaces, not the
primary covered graph obligation path.

Graph read access planning owns derived access requirements, typed admission or
denial, runtime-owned support rows, and receipt-backed no-N+1 proof for declared
graph-shaped reads. It does not replace graph touch obligation authority. Read
access planning changes how access structures are admitted and proved, not
whether graph obligations belong to Query.

## Related Docs

- [Graph Obligation Consumer Kit](graph-obligation-consumer-kit.md)
- [Graph Composition Authoring](graph-composition-authoring.md)
- [Intent Admission](../execution/intent-admission.md)
- [Writes And Intent Boundaries](../execution/writes-and-intents.md)
- [Branches And Previews](../foundations/branches-and-previews.md)
- [Support Matrix And Admission](../foundations/support-matrix-and-admission.md)
