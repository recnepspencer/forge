# Runtime Subsystems

This map is for contributors placing runtime work. Application developers
should start with [Application lifecycle](./application-lifecycle.md).

## The Seven Owners

| Owner | State or contract | Owns |
| --- | --- | --- |
| application | `WorthUiApplicationSessionState` | active generation, framework transition, replacement preparation and commit |
| graph | `UiGraphSnapshot` | graph truth, indexes, and mutation successors |
| planning | `WorthUiAllocationPlanning` | admitted planning, allocation inputs, and sealed execution plans |
| mounting | `WorthUiMountedSessionState` | mounted identities, frame assembly, presentation, retention, reconciliation, and publication |
| observation | `WorthUiHostExchangeSessionState` | structural host reports, measurement exchange, quarantine, and retained transport evidence |
| inspection | `UiMountedInspectionRequest` and facade bridge | read-only indexed projections and evidence receipts |
| session | `WorthUiActiveApplicationSession` | thin composition of named transitions across the established owners |

The session is not a bag of the other owners’ fields. It coordinates complete
transitions and returns product outcomes.

## Allowed Dependency Direction

```text
graph
  -> planning
  -> application
application + graph + planning
  -> mounting
mounting
  -> observation
application + graph + planning + mounting + observation
  -> inspection (read-only)
application + mounting + observation + inspection
  -> session composition
```

More exactly:

- application may depend on graph and planning;
- graph depends on none of the other six owners;
- planning may depend on graph;
- mounting may depend on application, graph, and planning;
- observation may depend on mounting;
- inspection may read application, graph, planning, mounting, and observation;
  and
- session may coordinate application, mounting, observation, and inspection.

Graph must never import mounting. Observation cannot publish a frame.
Inspection cannot mutate or reconstruct operational truth. Planning cannot
mutate mounted or observation state.

## Failure Preservation

Each subsystem owns its denial:

- application denial retains the predecessor app, plan, allocation, and Query
  state;
- graph denial leaves the current snapshot and indexes unchanged;
- planning denial leaves the active plan and committed allocation unchanged;
- mounting denial preserves the prior mounted identity and publication;
- observation denial cannot mutate application or mounted truth; and
- inspection denial has no operational side effects.

The session preserves this ordering and never publishes a partial
cross-subsystem successor.

## Cost Ownership

Owners report their own work. The session carries receipts but does not invent
duplicate totals. Reconstructive source and replacement work remains separate
from steady-frame execution. Rich inspection/report materialization is an
explicit cost outside measured executor intervals.

## Future Insertion Points

| Milestone | Roadmap responsibility | Owner | Exact insertion | Forbidden alternate |
| --- | --- | --- | --- | --- |
| 3.11 | Visual snapshot receipts and hit-test identity bridge | application | application replacement preparation and commit | session |
| 3.12 | Semantic host-observation admission before bounded hot-rebind planning | observation | after structural host-report validation | mounting |
| 3.17 | Runtime evaluation and invalidation of sealed DSL expression artifacts | planning | planning input handoff before active-plan publication | session |
| 3.18 | DSL composition, modules, and lowering equivalence | worth-ui-dsl | before the sealed semantic handoff; no runtime subsystem insertion | session |

These rows are mechanically cross-checked with the Phase 4 runtime-subsystem
ledger and the exact current roadmap headings. Milestone 3.17 keeps authored
expression meaning in `worth-ui-dsl`; only evaluation over sealed artifacts
enters runtime planning. Milestone 3.18 has no runtime insertion at all.
