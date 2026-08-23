# Runtime Subsystems

This map is for contributors placing runtime work. Application developers
should start with [Application lifecycle](./application-lifecycle.md).

## Runtime Owners

| Owner | State or contract | Owns |
| --- | --- | --- |
| application | `WorthUiApplicationSessionState` | active generation, framework transitions, source basis, application cutover |
| graph | `UiGraphSnapshot` and graph indexes | graph truth, produced-fact consumers, and mutation successors |
| planning | allocation and rebind plan compilers | admitted inputs, affected scope, identity lifecycle, and sealed plans |
| interaction | gesture, draft, targeting, and semantic-interaction registries | presented-frame continuity, capture/loss, bounded local input, and interaction receipts |
| intent admission | route, payload, operability, confirmation, and admission registries | typed candidates, affine challenges, concurrency, bounded admission slots, and settlement |
| intent execution | definition/provider bindings and attempt registries | destination dispatch, versioned providers, polling, cancellation, terminal posture, and recovery |
| mounting | `WorthUiMountedSessionState` | mounted identities, frame assembly, retention, reconciliation, and mounted publication |
| presentation producer | `UiMountedPresentationState` | receipt-keyed retained commands, initial/delta/unchanged work, total order, damage, and post-settlement candidate commit |
| host exchange | `WorthUiHostExchangeSessionState` | structural host reports, measurement exchange, quarantine, and transport evidence |
| visual inspection | `WorthUiVisualInspectionAuthority` plus capture/overlay registries | grants, retained snapshots, comparison, overlays, and visual resource bounds |
| rebind lifecycle | `UiRebindRuntimeState` | plan, receipt, completion, recovery, terminal-decision, and causal-evidence capacity |
| inspection bridge | typed inspection queries and projections | read-only indexed summaries, exact references, support, and expiry posture |
| session | `WorthUiActiveApplicationSession` | thin composition of complete transitions across established owners |

The session is not a bag of the other owners' fields. It coordinates named
transitions and returns product outcomes. New state belongs in the owner that
can establish, rebuild, and dispose its truth.

Query projection state is not another session owner. Query owns the live
resource; `worth-ui-query-binding` converts Query-issued authority into
shape-specific UI observations and affine facts; the existing observation,
planning, mounting, and publication owners consume the resulting UI meaning.

Product intent effects follow the same rule. UI admission owns no product or
Query mutation authority. The exact typed provider calls a separately admitted
product action; only that owner-issued result may become a declared consequence
for ordinary rebind and mounted publication.

## Allowed Dependency Direction

```text
graph
  -> planning
host observation + mounting identity
  -> interaction
interaction + declarations + application facts
  -> intent admission
intent admission + definition/provider bindings
  -> intent execution
application + graph + planning
  -> mounting
mounting
  -> host exchange
application + graph + planning + mounting + host exchange
  -> rebind lifecycle
application + graph + planning + mounting + host exchange + rebind + visual
  -> inspection bridge (read-only)
named owners
  -> session composition
```

More exactly:

- graph depends on none of the mounted, host-exchange, visual, rebind, or
  inspection owners;
- planning may read graph and sealed application meaning;
- interaction may consume admitted host observations and exact presented
  targets but cannot route, execute, or publish product effects;
- intent admission may consume semantic interactions, declarations, and
  application facts but cannot call providers or mutate product domains;
- intent execution consumes one move-only admission and the exact typed
  destination binding; provider completion still cannot publish directly;
- mounting may consume application, graph, and sealed planning output;
- the presentation producer derives host work from mounted authority; hosts do
  not rediscover deltas from a complete projection;
- host exchange may observe mounted transport but cannot publish;
- rebind may coordinate owner-issued observation, plan, application, mounting,
  and host outcomes but does not absorb their source truth;
- visual inspection borrows exact mounted evidence and rebind affinity without
  becoming a publication owner;
- inspection reads named projections and cannot mutate or reconstruct them; and
- session composition may call complete transitions but cannot invent a
  parallel state store.

Graph must never import mounting. Planning cannot mutate mounted or observation
state. Observation cannot publish a frame. Inspection cannot reconstruct
operational truth.

## Rebind Construction

Every active session constructs `UiRebindRuntimeState` from the prepared
application's `UiRebindProfile`. The state owns bounded registration for plans,
terminal receipts, in-flight completion, uncertain recovery, terminal decision
records, and retained causal evidence. A profile is required; there is no
ambient default hidden in the executor.

The ordinary source route is:

```text
native shell
-> begin_source_rebind
-> application-owned observation/classification/scope/plan
-> rebind lifecycle final admission
-> mounting and host effects
-> application + mounted atomic cutover
```

The legacy whole-application replacement facade is not an alternate route.

## Failure Preservation

Each subsystem owns its denial:

- source/DSL denial retains the exact attempted revision and compile report;
- application denial retains predecessor app, graph, allocation, and Query
  state;
- graph or planning denial leaves current snapshots and indexes unchanged;
- mounting denial preserves prior mounted identity and publication;
- host-exchange denial cannot mutate application or mounted truth;
- rebind before-effect denial retains the predecessor and exact valid next
  action;
- uncertain effects retain completion or recovery authority until
  reconciliation or shutdown; and
- inspection denial has no operational side effects.

The session preserves this ordering and never publishes a partial
cross-subsystem successor.

## Cost Ownership

Owners report their own work. The session carries receipts but does not invent
duplicate totals.

- source acquisition and DSL compile are reconstructive source cost;
- classification/scope/planning use `O + F + A + C + R + G + M + B`;
- mounting and adapter effects report physical presentation cost;
- delta carriage, draw-list/order mutation, damage-index/replay work, pixels,
  GPU writes, passes, copies, acquisitions, submissions, and presents remain
  separately named counters;
- reconciliation reports recovery cost separately; and
- rich inspection/report materialization is explicit diagnostic cost outside
  measured executor intervals.

Saturation returns typed denial or backpressure. It must not trigger a hidden
whole-graph scan, universal remount, or unbounded queue.

## Successor Homes

| Capability family | Existing owner to extend | Forbidden alternate |
| --- | --- | --- |
| projected product data | Query binding consequence plus owner-specific observation facts | session data cache |
| semantic interactions and product intents | interaction, intent admission, and intent execution owners feeding declared consequences | host callback or session executor |
| portal, focus, and command services | Milestone 3.15 providers behind typed runtime-service destinations | reopening generic intent admission or using renderer state |
| portals, focus, motion, appearance | typed produced facts and consumed aspects feeding rebind planning | renderer-local state |
| expression evaluation | planning over sealed DSL expression artifacts | runtime reparsing |
| authored modules and composition | `worth-ui-dsl` before sealed semantic handoff | session composition |
| human and agent diagnostics | inspection projections over retained exact references | replay in ordinary runtime |

Future work should insert at these homes and reuse the canonical rebind
executor. If a feature requires moving source settlement, graph truth,
publication, or host authority, its architecture is not yet honest.

For projected text, planning selects the declared scalar or collection fact,
mounting owns the semantic text node and `BodyDefault` appearance role, and the
host adapter translates only the resulting mounted mechanics. A renderer-side
field lookup or Query call is therefore both an ownership violation and a
second data path.

## Related Docs

- [Worth UI architecture](./architecture.md)
- [Interaction and intents](./interaction-and-intents.md)
- [Hot rebind](./hot-rebind.md)
- [Application inspection](./inspection.md)
- [Visual inspection](./visual-inspection.md)
