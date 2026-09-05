# `worth-runtime-world`

This crate is the memory-resident composition owner for the 9.17.2 Runtime
World. The implementation now includes:
owner-issued identities, complete product-head observations, exact composite
bases, installed budgets, owner service bundles, publication progression,
linear terminal artifacts, retained owner-effect records, and the retention
lane's dependency vocabulary.

The only supported public import surface is:

```rust
use worth_runtime_world::facade::*;
```

The crate depends only on the public facades of Foundational, Proof,
Relational, Signal, and Runtime Bridge. It has no Query, Store, persistence,
replay, codec, or physical-runtime dependency. A Runtime World never discovers
an ambient current head and never accepts a raw component runtime or a generic
authority marker.

Bootstrap, retention, branch creation, publication, recovery, and close execute
through the managed owner's internal service seams. Phase 5 assembles and
freezes the public builder and service facade; this crate is not yet the 9.17.3
Query cutover surface. Ordinary publication now preserves owner-addressable
custody across caller Drop and unwind and guards materialization with the final
branch comparison. Branch creation carries the same custody through both forks
and destination installation; its registry records actual installation even
when the caller unwinds or the installed branch is later retired.

Construction installs `RuntimeWorldBudgets`, concrete component service bundles
in `RuntimeWorldOwnerInputs`, and an explicit `RuntimeWorldClock`. The clock
controls deadlines and cleanup eligibility, never identity, basis, parentage,
or authority.

The Runtime World is intentionally in-memory. Restart recovery and durable
Store integration are outside this milestone.
