# `worth-runtime-world`

This crate is the memory-resident composition owner for the 9.17.2 Runtime
World. Phase 1 freezes the contracts that later implementation lanes consume:
owner-issued identities, complete product-head observations, exact composite
bases, installed budgets, owner service bundles, publication progression,
linear terminal artifacts, and retained owner-effect records.

The only supported public import surface is:

```rust
use worth_runtime_world::facade::*;
```

The crate depends only on the public facades of Foundational, Proof,
Relational, Signal, and Runtime Bridge. It has no Query, Store, persistence,
replay, codec, or physical-runtime dependency. A Runtime World never discovers
an ambient current head and never accepts a raw component runtime or a generic
authority marker.

Phase 1 does not claim that bootstrap, retention, or publication is executable
yet. Those behaviors are owned by the later lanes recorded in
`COORDINATED_PUBLICATION.md`; the frozen types make their required transitions
and failure boundaries compiler-visible before behavior is added.

The composition root must eventually install every `RuntimeWorldBudgets` limit,
pass concrete owner service bundles in `RuntimeWorldOwnerInputs`, and provide
an explicit `RuntimeWorldClock`. The clock is meaningful only for deadlines
and cleanup eligibility; it cannot influence identity, basis, parentage, or
authority.

The Runtime World is intentionally in-memory. Restart recovery and durable
Store integration are outside this milestone.
