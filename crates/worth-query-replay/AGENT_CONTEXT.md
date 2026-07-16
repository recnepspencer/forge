# worth-query-replay

Generated from the machine-owned Road 1 boundary model. Do not edit by hand.
Canonical machine constitution: `tools/boundary-check/config/road1.toml`

- Constitutional class: `framework/query-audience`
- Domain noun: `replay`
- Crate root: `crates/worth-query-replay`
- Road 1 exemplar role: Query replay audience facade over `worth-query`
- Deferred next homes:

- Public surface: facade-only
- Facade exports: `ScopedReplayBasis`
- Owned internal modules: `none`
- Allowed in-tree dependency bands: `none`

Machine fences:
- Framework Query audience facade (`replay`); legal consuming bands: cert.
- May depend only on engine package `worth-query`; must not depend on other audience facades.
- Leaf re-export surface only; guidance: cert-only reconstruction and replay.

Skeleton fence:
- Framework audience facade: re-export-only; no seed-skeleton allowlist.
