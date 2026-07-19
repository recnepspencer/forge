# worth-query-decl

Generated from the machine-owned Road 1 boundary model. Do not edit by hand.
Canonical machine constitution: `tools/boundary-check/config/road1.toml`

- Constitutional class: `framework/query-audience`
- Domain noun: `declaration`
- Crate root: `workspaces/worth-query/crates/worth-query-decl`
- Road 1 exemplar role: Query declaration audience facade over `worth-query-declaration`
- Deferred next homes:

- Public surface: facade-only
- Facade exports: `CanonicalQueryArtifact, authoring, binding, canonicalization, collection, diagnostics, identity, identity_authority, schema_view, typed, validation, view_declaration`
- Owned internal modules: `none`
- Allowed in-tree dependency bands: `none`

Machine fences:
- Framework Query audience facade (`declaration`); legal consuming bands: entry, cert.
- May depend only on its configured authority packages: `worth-query-declaration`; must not depend on other audience facades.
- Leaf re-export surface only; guidance: declaration artifacts and handles.

Skeleton fence:
- Framework audience facade: re-export-only; no seed-skeleton allowlist.
