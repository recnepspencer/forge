# Cross-Cutting Concern Lifecycle

How cross-cutting concerns (decisions, lineage, metrics, future repair/cost signals) flow through the kernel.

---

## The Two Roles

Every field on `OperationScope` is either **immutable context** or a **write recorder**.

| Role                                              | Examples                           | Mutability                                      |
| ------------------------------------------------- | ---------------------------------- | ----------------------------------------------- |
| **Context** — describes what this operation _is_  | `config`, `op_space`, `feature_id` | Read-only during execution                      |
| **Recorder** — captures what this operation _did_ | `DecisionSink`, `LineageRecorder`  | Write during execution, drained at finalization |

Adding a new cross-cutting concern means adding exactly one field to `OperationScope` — either a context reference or a recorder factory method. Zero downstream signature changes.

---

## Recorder Lifecycle Protocol

Any write recorder in the kernel MUST follow this lifecycle:

```
┌──────────────────────────────────────────────────────────┐
│  1. CONSTRUCT — from OperationScope, per-operation       │
│     let mut recorder = scope.lineage_recorder(mode);     │
│                                                          │
│  2. WRITE — &mut self during execution                   │
│     recorder.stamp(store, entity);                       │
│     sink.record_near_boundary(vid, distance, tol);       │
│                                                          │
│  3. DRAIN — into OperationResult at finalization         │
│     finalizer.collect_success(&mut envelope, ...);       │
│     // decisions, metrics, lineage all move into envelope │
│                                                          │
│  4. SEAL — immutable forever after                       │
│     // envelope.get_decision_log() returns &DecisionLog  │
│     // TopologyState.lineage_events is Arc<Vec<...>>     │
└──────────────────────────────────────────────────────────┘
```

### Rules

1. **Construct from scope, not free-floating.** Prevents sharing across operations, leaking ordinals, wrong `feature_id`.
2. **One recorder instance per operation invocation.** The pipeline creates the scope; the scope creates the recorder.
3. **Write-only during execution.** Recorders append; they never read back their own output during execution. Reading happens post-finalization on the sealed envelope.
4. **Drain into the envelope.** After finalization, the recorder is empty. The envelope owns all recorded data.
5. **Sealed after finalization.** `OperationResult<SolidEnvelope>` is the immutable audit record. Downstream consumers get `&`-references only.

### Current Recorders

| Recorder                               | Created from                   | Writes                                           | Drained into                                                       |
| -------------------------------------- | ------------------------------ | ------------------------------------------------ | ------------------------------------------------------------------ |
| `DecisionSink` (via `ModelingContext`) | Pipeline stage 6               | `record_near_boundary`, `start_span`, `end_span` | `OperationResult.decision_log`                                     |
| `LineageRecorder`                      | `scope.lineage_recorder(mode)` | `stamp`, `stamp_derived`, `stamp_deletion`       | `TopologyState.lineage_events` (via `LineageStore.drain_events()`) |

### Future Recorders (not yet built)

| Concern                 | Would record                                  | Drained into                     |
| ----------------------- | --------------------------------------------- | -------------------------------- |
| Repair/Healing          | Tolerance widening events, snap decisions     | `OperationResult.repair_log`     |
| Cost estimation         | Material volume, machining complexity signals | `OperationResult.cost_signals`   |
| Constraint satisfaction | Over/under-constrained sketch DOF events      | `OperationResult.constraint_log` |

These follow the identical lifecycle: construct → write → drain → seal.

---

## Adding a New Cross-Cutting Concern

1. Define the event type in `forge-core` (like `TracedDecision`, `LineageEvent`).
2. Define the recorder in the appropriate domain crate (like `LineageRecorder` in `forge-topo`).
3. Add a factory method to `OperationScope` (like `scope.lineage_recorder(mode)`).
4. Wire the drain into `OperationFinalizer` / `MutableDraft::commit()`.
5. Add a sealed accessor to `OperationResult` (like `get_decision_log()`).

**Zero changes to:** Feature implementations, shared_operations, Euler operators, pipeline stages, existing tests.
