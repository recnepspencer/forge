# Conditions And Comparators

This guide exists because these concepts are easy to blur together:

- aspects
- evaluation conditions
- comparator policy
- tolerance
- custom conditions

They are related, but they are not the same thing.

## 1. Aspects

An `Aspect` is a user-defined version slot.

You create aspects with:

```rust
use forge_signal::facade::Aspect;

const PRICE: Aspect = Aspect::new(0);
const TAX: Aspect = Aspect::new(1);
const GEOMETRY: Aspect = Aspect::new(2);
```

What an aspect is:

- a stable slot index inside a node's `AspectVersion`
- a way to say "this dependency or invalidation concerns this dimension of change"

What an aspect is not:

- a field name registry
- a semantic type system
- something `forge-signal` interprets for you

Recommended practice:

- define aspects as constants
- keep them close to the host-domain subsystem that owns them
- avoid ad hoc `Aspect::new(...)` scattered through the codebase

## 2. Evaluation conditions

Evaluation conditions answer:

"When is this node allowed to run?"

Main enum:

- `EvaluationCondition`

Builder helpers:

- `.always()`
- `.on_demand()`
- `.debounce(milliseconds)`
- `.aspect_filter(mask)`
- `.delta_threshold(value)`
- `.custom_condition(key)`

### `Always`

Run whenever the node is dirty.

```rust
let node = graph.node().always().build();
```

Use when:

- the node should recompute as soon as its dependencies require it

### `OnDemand`

Run only when explicitly requested.

```rust
let node = graph.node().on_demand().build();
```

Use when:

- the node is expensive
- the node feeds a pull-based consumer
- you want lazy evaluation

### `Debounce(milliseconds)`

Run only after a quiet period.

```rust
let node = graph.node().debounce(50).build();
```

Use when:

- updates arrive in bursts
- you want to avoid thrashing

### `AspectFilter(mask)`

Run only when matching aspects were dirtied.

```rust
let mask = PRICE.bit() | TAX.bit();
let node = graph.node().aspect_filter(mask).build();
```

Use when:

- a node depends on many things but should only wake up for some aspect classes

### `DeltaThreshold(value)`

Run only when the upstream delta crosses a threshold.

```rust
let node = graph.node().delta_threshold(0.05).build();
```

Use when:

- tiny deltas should not trigger this node
- you want condition-level gating by magnitude

### `Custom(key)`

Defer the decision to the embedding runtime.

```rust
let node = graph.node().custom_condition("market-open").build();
```

Use when:

- the condition depends on host policy or context outside `forge-signal`
- you want named condition semantics that mean something to your application

## 3. Custom conditions: what they actually mean

This is the part most docs usually hand-wave.

`forge-signal` stores the condition policy on the node:

```rust
let node = graph.node().custom_condition("market-open").build();
```

That key is just a stable identifier inside `forge-signal`.

`forge-signal` does **not** decide what `"market-open"` means on its own.

The embedding runtime is responsible for resolving it during condition-aware execution.

Architecturally:

- `forge-signal` owns the node policy declaration
- the host runtime owns the semantic meaning of the custom key

So a custom condition is appropriate when the host knows something like:

- market session state
- request mode
- editor/tool mode
- simulation phase
- authorization or environment gating

## 4. Comparator policy

Comparator policy answers a different question:

"If dependency versions changed, is that change meaningful for this node?"

Main surface:

- `VersionComparatorPolicy`

Builder helpers:

- `.comparator(...)`
- `.tolerance(epsilon)`
- `.output_identity()`

Examples:

```rust
let tolerant = graph.node().tolerance(2).build();

let identity_aware = graph.node().output_identity().build();

let explicit = graph
    .node()
    .comparator(VersionComparatorPolicy::Exact)
    .build();
```

## 5. Tolerance vs delta threshold vs output identity

These are easy to confuse.

### `tolerance(...)`

This is comparator policy.

It means:

- dependency version changes smaller than `epsilon` are treated as not meaningful for this node

It does **not** mean:

- the node uses floating-point tolerance for its own domain math
- the node is conditionally skipped by magnitude gating

### `delta_threshold(...)`

This is an evaluation condition.

It means:

- the node may be deferred unless the change magnitude crosses a threshold

It is about whether the node is allowed to run, not about whether an upstream version difference is considered meaningful.

### `output_identity()`

This is comparator policy plus downstream suppression behavior.

It means:

- if the output artifact identity is unchanged, downstream propagation may be suppressed

This is useful when:

- recomputation happens, but the produced artifact is semantically still "the same object"

## 6. End-to-end example

```rust
use forge_signal::facade::*;

const PRICE: Aspect = Aspect::new(0);

let mut graph = SignalGraph::new();
let source = graph.node().build();

let view_node = graph
    .node()
    .on_demand()
    .tolerance(2)
    .build();

let market_gated_node = graph
    .node()
    .custom_condition("market-open")
    .output_identity()
    .build();

graph.add_dependency(view_node, source, PRICE)?;
graph.add_dependency(market_gated_node, source, PRICE)?;
# Ok::<(), SignalError>(())
```

Reading that configuration correctly:

- `view_node`
  - only runs when explicitly requested
  - ignores small upstream version drift below `2`

- `market_gated_node`
  - asks the host runtime to decide what `"market-open"` means
  - uses output identity semantics for downstream suppression

## 7. Practical rule of thumb

If your question is:

- "Should this node run at all?"
  - think **condition**

- "Did the upstream change matter semantically?"
  - think **comparator**

- "What does this aspect slot mean?"
  - that is **your host-domain contract**
