# Forge Kernel Architecture

This document will grow as the kernel is built. For now, see
[DEVELOPMENT_BLUEPRINT.MD](../DEVELOPMENT_BLUEPRINT.MD) for the full engineering specification.

## Crate Structure

```
forge-math          (no internal deps)
  └─→ forge-topo    (depends on forge-math)
        └─→ forge-geom   (depends on forge-math, forge-topo)
              └─→ forge-kernel (depends on all above)
                    └─→ forge-io     (depends on all above)

forge-test depends on everything (test-only)
```

## Global Doctrines

- **D0** — Topology-First: certified predicates drive all decisions
- **D1** — Determinism & Replay: every operation is reproducible
- **D2** — Explicit Policy at Ambiguity: no silent heuristics
- **D3** — Topology–Geometry Firewall: geometry proposes, topology decides
- **D4** — Canonical Orientation: outward normals, consistent winding
- **D5** — Sliver Generation Budget: no surprise thin faces
- **D6** — Atomic Transactionality: valid state or error, never partial
- **D7** — Visualization Subordination: B-Rep is truth
- **D8** — Environmental Isolation: results are hardware-independent
