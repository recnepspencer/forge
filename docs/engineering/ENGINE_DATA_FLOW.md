# Engine Data Flow

How data moves from a user command to a stored `SolidEnvelope`.

---

## 1. Command → Feature

```
Command::AddBlock { origin, dimensions }
       │
       ▼
CommandDispatcher::dispatch()          ← crates/forge-kernel/src/registry/dispatch.rs
       │
       │  routes to handler, generates unique name ("block_0")
       ▼
handlers::add_block::add_block()       ← registry/handlers/add_block.rs
       │
       │  constructs MakePrimitiveFeature (params + name)
       │  wraps in NativeFeature::Primitive(...)
       ▼
FeatureTree::register_feature()        ← engine/feature_tree.rs
       │
       │  allocates NodeId in SignalGraph
       │  registers dependencies (edges in the DAG)
       │  enforces name uniqueness
       │  marks node dirty
       ▼
    NodeId (returned to caller)
```

## 2. Evaluation → Pipeline

```
FeatureTree::evaluate_feature(node_id)
       │
       │  walks dependency graph (topological order)
       │  for each node: checks if dirty via SignalGraph
       ▼
evaluate_feature_with_config(node_id)  ← engine/feature_tree.rs
       │
       │  clones SolidEnvelope from each dependency's cached output
       │  (this is the ONE clone — Arc topology O(1), geometry O(V+F))
       │  builds: HashMap<NodeId, SolidEnvelope>
       ▼
feature.execute_via_pipeline(input_map, config)
       │                                       ← registry/native_feature.rs
       │  dispatches to concrete feature kind
       ▼
FeaturePipeline::execute(&feature, raw_inputs, config)
                                               ← engine/pipeline/executor.rs
```

## 3. The Pipeline (11 stages)

```
  raw_inputs: HashMap<NodeId, SolidEnvelope>  (owned, not borrowed)
       │
  ┌────┴────────────────────────────────────────────────────────┐
  │  1. Resolve config (cascade: global → feature overrides)    │
  │  2. Pre-validate policies (fail-fast, no mutations yet)     │
  │  3. Hash inputs (wrapping_add, order-independent)           │
  │                                                             │
  │  4. CONDITIONING (pipeline-managed)                         │
  │     ├─ ConditioningMode::None → identity (zero cost)        │
  │     └─ Unary/Binary → analyze_envelopes() → OperationSpace │
  │        └─ if active: transform_store() on each input        │
  │           (in-place, geometry moves to local coords)        │
  │                                                             │
  │  5. parse_inputs(raw_inputs) → Self::Inputs  (owned)        │
  │     validate() semantic checks                              │
  │                                                             │
  │  6. execute_typed(inputs, &mut scope)                       │
  │     └─ scope carries: config, DecisionSink, &OperationSpace │
  │     └─ feature does its work, returns OperationResult       │
  │                                                             │
  │  7. Restore world coordinates (inverse of step 4)           │
  │  8. Hash output topology                                    │
  │  9. Finalize — drain ModelingContext → envelope              │
  │     (decisions, metrics, lineage all move into envelope)     │
  │ 10. Validate post-invariants (ManifoldEdges, G1, etc.)      │
  │ 11. Audit filter (None/Summary/Full)                        │
  └─────────────────────────────────────────────────────────────┘
       │
       ▼
  OperationResult<SolidEnvelope>
```

## 4. Storage

```
OperationResult<SolidEnvelope>        ← the "envelope"
       │
       ├── .value: SolidEnvelope      ← topology + geometry
       │      ├── TopologyState       ← Arc (cheap clone)
       │      ├── GeometryStore       ← vertex positions, face planes
       │      └── OnceCell caches     ← lazy handle lists (bodies, faces...)
       │
       ├── .decision_log              ← full TracedDecision trace
       ├── .warnings                  ← non-fatal KernelWarnings
       ├── .metrics                   ← duration, entity counts
       ├── .lineage_delta             ← faces/edges/vertices created
       └── .state_hash_before/after   ← topology fingerprints
       │
       ▼
FeatureTree.envelopes[node_id] = envelope
       │
       │  cached until node is marked dirty
       │  downstream features clone .value when they evaluate
```

## Key Types

| Type                    | Lives In                 | Role                                                         |
| ----------------------- | ------------------------ | ------------------------------------------------------------ |
| `Command`               | `forge-schema`           | User-facing intent (AddBlock, BooleanSubtract)               |
| `NativeFeature`         | `forge-kernel/registry`  | Enum dispatching to concrete features                        |
| `Feature` trait         | `engine/contracts`       | parse_inputs + execute_typed                                 |
| `FeatureContract` trait | `engine/contracts`       | Policies, invariants, conditioning mode                      |
| `FeaturePipeline`       | `engine/pipeline`        | The 11-stage executor                                        |
| `OperationScope`        | `context/scope`          | Cross-cutting bundle (config + sink + op_space)              |
| `OperationSpace`        | `engine/operation_space` | Local↔world coordinate lens                                  |
| `SolidEnvelope`         | `engine/output`          | Topology + geometry, lazy handle caches                      |
| `OperationResult<T>`    | `forge-core/envelope`    | Universal audit wrapper (decisions, metrics, lineage)        |
| `ModelingContext`       | `context`                | Live decision sink during execution, drained at finalization |

## Ownership Flow

```
Signal graph cache ──clone──► HashMap<NodeId, SolidEnvelope>
                                        │ (owned by pipeline)
                               conditioning transforms in-place
                                        │
                               parse_inputs() moves into Self::Inputs
                                        │
                               execute_typed() consumes inputs
                                        │
                               returns OperationResult<SolidEnvelope>
                                        │
                               pipeline restores world coords in-place
                                        │
                               finalization drains ctx into envelope
                                        │
                               stored back in FeatureTree.envelopes
```

No unnecessary clones. The only copy is at the cache boundary.
