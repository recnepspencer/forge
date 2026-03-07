# ⚡ forge-signal

**Reactive state graph with transactions, rollback, and zero domain coupling.**

Think of it like a `useEffect` dependency graph — except it runs in Rust, compiles to Wasm, tracks _which part_ of a node changed, and can roll back your entire state if anything fails mid-computation.

> **Replaces:** hand-rolled dirty flags, ECS query systems, Redux-like state trees, and `is_dirty` booleans scattered across your codebase.

---

## 10-Second Overview

```rust
use forge_signal::facade::*;

const VALUE: Aspect = Aspect::new(0);

let mut graph = SignalGraph::new();

// Wire: A → B → C
let a = graph.create_node();
let b = graph.create_node();
let c = graph.create_node();
graph.add_dependency(b, a, VALUE).unwrap();
graph.add_dependency(c, b, VALUE).unwrap();

// Push: "A changed"
mark_dirty(&mut graph, a, VALUE).unwrap();
// B = Dirty, C = MaybeStale

// Pull: "Give me C"
let mut compute = |_id, _g: &SignalGraph| {
    Ok(AspectVersion::from_updates([(VALUE, 1)]))
};
evaluate(&mut graph, c, &mut compute).unwrap();
// Only recomputes what actually changed.
```

That's it. `forge-signal` tracks which nodes are dirty, skips nodes that didn't actually change, detects cycles at runtime, and gives you deterministic evaluation order — every single time.

---

## Why This Exists

Most reactive systems are fire-and-forget. You mutate, it propagates, and if something fails halfway through you get a corrupted graph. Good luck debugging that.

`forge-signal` is **transactional**. You open a transaction, do your mutations and evaluations, and either commit or rollback. If your compute function returns an error, the graph reverts to its exact pre-transaction state. No leaked side effects. No partial updates.

```rust
let mut txn = runtime.begin();

txn.mark_dirty(price_node, PRICE).unwrap();
txn.emit_event("TICK:AAPL".into());

match evaluate_in_txn(&mut txn, strategy_node, &mut compute, DefaultComparatorResolver) {
    Ok(_)  => txn.commit(&mut ctx).unwrap(),   // graph + events go live
    Err(_) => txn.rollback(&mut ctx).unwrap(),  // everything reverts. events dropped.
};
```

---

## What Makes It Different

|                             | Most Reactive Systems | forge-signal                                                                                   |
| --------------------------- | --------------------- | ---------------------------------------------------------------------------------------------- |
| **Rollback**                | ❌                    | Full state rollback via sparse copy-on-write patches                                           |
| **Multi-aspect versioning** | ❌                    | Independent version counters per concern. Auth changes don't re-evaluate the cart.             |
| **Evaluation gating**       | ❌                    | `OnDemand`, `Debounce`, `DeltaThreshold`, `AspectFilter` — nodes decide _whether_ to recompute |
| **Deterministic order**     | Sometimes             | Always. Explicit stack traversal, no hash map iteration.                                       |
| **Domain coupling**         | Usually baked in      | Zero. Same engine works for dashboards, trading, games, and data pipelines.                    |
| **Cycle detection**         | At build time, maybe  | Runtime — during `mark_dirty` and `evaluate` traversals                                        |
| **GC**                      | Manual / none         | Generational arena with tombstoning + `run_gc_epoch()`                                         |
| **Telemetry**               | DIY                   | Built-in counters for evaluations, skips, rollbacks, GC epochs                                 |

---

## How Evaluation Works

### Push Phase — something changed

```
user action  →  mark_dirty(node, aspect)
```

- Direct dependents on the same aspect → **Dirty**
- Direct dependents on a different aspect → **MaybeStale**
- Everything further downstream → **MaybeStale**

### Pull Phase — I need a result

```
evaluate(target_node, compute)
```

- `Clean` → skip entirely
- `MaybeStale` → check if upstream versions actually changed. If not, revert to `Clean` without calling compute (**version-gated skip**)
- `Dirty` → call `compute`, record new version

### Your Compute Closure

```rust
fn(NodeId, &SignalGraph) -> Result<AspectVersion, SignalError>
```

**`forge-signal` doesn't know what a node represents.** Your app maintains the mapping:

```rust
let mut state: HashMap<NodeId, MyThing> = HashMap::new();

let mut compute = |id: NodeId, _graph: &SignalGraph| {
    let thing = state.get(&id).unwrap();
    let result = thing.recalculate();
    Ok(AspectVersion::from_updates([(VALUE, result.version)])
};
```

This is intentional. The runtime stays generic. Your closures bring the domain knowledge.

---

## Architecture

```
┌─────────────────────────────────┐
│       Your Application          │
│  Maps NodeId → your stuff       │
└────────────┬────────────────────┘
             │
             ▼
┌─────────────────────────────────┐
│     SignalRuntimeState          │
│       ├── SignalGraph (DAG)     │
│       ├── CheckpointRuntime     │
│       └── EventBus              │
│                                 │
│     SignalTransaction           │
│       mark_dirty → evaluate     │
│       commit / rollback         │
└────────────┬────────────────────┘
             │
             ▼
┌─────────────────────────────────┐
│     Your Compute Closure        │
│  fn(NodeId, &SignalGraph)       │
│      → Result<AspectVersion>    │
└─────────────────────────────────┘
```

---

## Example: Wasm + React

Ship your dependency graph as a Wasm module. React just calls two methods.

### `store.rs` — Rust side

```rust
use wasm_bindgen::prelude::*;
use forge_signal::facade::{
    SignalGraph, SignalRuntimeState, NodeId, Aspect,
    evaluate_in_txn, AspectVersion, CheckpointPolicy,
    DefaultComparatorResolver,
};

const UI: Aspect = Aspect::new(0);
type Dom = u32; type Imp = u32; type Evt = String;

#[wasm_bindgen]
pub struct Store {
    runtime: SignalRuntimeState<Dom, Imp, Evt, ()>,
    input: NodeId,
    output: NodeId,
    version: u64,
}

#[wasm_bindgen]
impl Store {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut graph = SignalGraph::new();
        let input = graph.create_node();
        let filter = graph.create_node();
        let output = graph.create_node();
        graph.add_dependency(filter, input, UI).unwrap();
        graph.add_dependency(output, filter, UI).unwrap();

        let runtime = SignalRuntimeState::with_policy(graph, CheckpointPolicy::new());
        Self { runtime, input, output, version: 0 }
    }

    /// Push: something changed
    pub fn set_input(&mut self, _value: String) {
        let mut txn = self.runtime.begin();
        txn.mark_dirty(self.input, UI).unwrap();
        txn.commit(&mut ()).unwrap();
    }

    /// Pull: recompute what's needed
    pub fn tick(&mut self) -> Result<(), JsValue> {
        self.version += 1;
        let v = self.version;
        let target = self.output;
        let mut txn = self.runtime.begin();
        let mut compute = |_id: NodeId, _g: &SignalGraph| {
            Ok(AspectVersion::from_updates([(UI, v)]))
        };
        evaluate_in_txn(&mut txn, target, &mut compute, DefaultComparatorResolver)
            .map_err(|e| JsValue::from_str(&format!("{e:?}")))?;
        txn.commit(&mut ()).unwrap();
        Ok(())
    }
}
```

### `App.tsx` — React side

```tsx
import { useEffect, useRef, useState } from "react";
import init, { Store } from "forge-wasm";

export const App = () => {
  const store = useRef<Store | null>(null);
  const [ready, setReady] = useState(false);

  useEffect(() => {
    init().then(() => {
      store.current = new Store();
      setReady(true);
    });
    return () => store.current?.free();
  }, []);

  const onInput = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!store.current) return;
    store.current.set_input(e.target.value); // push
    store.current.tick(); // pull
  };

  if (!ready) return <div>Loading…</div>;
  return (
    <div>
      <input type="text" placeholder="Search…" onChange={onInput} />
      <canvas id="render-target" />
    </div>
  );
};
```

---

## Example: Trading Engine With Rollback

### `market.rs`

```rust
use forge_signal::facade::{SignalRuntimeState, NodeId, Aspect, CheckpointPolicy};

pub const PRICE: Aspect = Aspect::new(0);
type Dom = u32; type Imp = u32; type Evt = String;

pub struct MarketFeed {
    pub runtime: SignalRuntimeState<Dom, Imp, Evt, ()>,
    pub tickers: std::collections::HashMap<String, NodeId>,
}

impl MarketFeed {
    pub fn on_tick(&mut self, ticker: &str, _price: f64) {
        if let Some(&node) = self.tickers.get(ticker) {
            let mut txn = self.runtime.begin();
            txn.mark_dirty(node, PRICE).unwrap();
            txn.emit_event(format!("TICK:{ticker}"));
            txn.commit(&mut ()).unwrap();
        }
    }
}
```

### `strategy.rs`

```rust
use forge_signal::facade::{
    evaluate_in_txn, DefaultComparatorResolver,
    AspectVersion, NodeId, SignalGraph, SignalError,
};
use crate::market::{MarketFeed, PRICE};

pub struct Strategy {
    pub feed: MarketFeed,
    pub signal_node: NodeId,
    pub tick_id: u64,
}

impl Strategy {
    pub fn run(&mut self) {
        self.tick_id += 1;
        let tick = self.tick_id;
        let target = self.signal_node;
        let mut txn = self.feed.runtime.begin();

        let mut compute = |_id: NodeId, _g: &SignalGraph| {
            if tick % 100 == 0 {
                return Err(SignalError::internal("circuit breaker"));
            }
            Ok(AspectVersion::from_updates([(PRICE, tick)]))
        };

        match evaluate_in_txn(&mut txn, target, &mut compute, DefaultComparatorResolver) {
            Ok(_) => {
                txn.emit_event("TRADE_EXECUTED".into());
                txn.commit(&mut ()).unwrap();
            }
            Err(_) => {
                // Full revert. Events dropped. Broker never sees them.
                txn.rollback(&mut ()).unwrap();
            }
        }
    }
}
```

---

## Evaluation Conditions

Nodes can be configured to control _whether_ they recompute at all:

| Condition             | When it fires                                            |
| --------------------- | -------------------------------------------------------- |
| `Always`              | Default. Recompute whenever dirty.                       |
| `AspectFilter(mask)`  | Only if specific aspects changed.                        |
| `OnDemand`            | Only when explicitly requested via `evaluate_on_demand`. |
| `DeltaThreshold(f64)` | Only if upstream version delta exceeds threshold.        |
| `Debounce(ms)`        | Only after a quiet period.                               |
| `Custom(key)`         | Your resolver decides.                                   |

---

## Telemetry

Built-in `RuntimeTelemetry` — no extra instrumentation needed:

`nodes_evaluated` · `nodes_recomputed` · `skipped_by_comparator` · `evaluation_nanos` · `invalidation_nodes_visited` · `transaction_commit_count` · `transaction_rollback_count` · `gc_epoch_count` · `condition_skip_count` · `debounce_deferred_count`

Access via `graph.telemetry()`. Reset with `reset_telemetry()`.

---

## Node Lifecycle

Generational arena. `NodeId` = index + generation. Unregister nodes via `unregister_node()`. Call `run_gc_epoch()` when `should_gc()` returns true (default threshold: 1024 tombstones). Dead handles are safely rejected — no use-after-free, no panics.
