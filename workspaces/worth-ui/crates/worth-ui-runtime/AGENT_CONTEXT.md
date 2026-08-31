# worth-ui-runtime

Generated from the machine-owned Road 1 boundary model. Do not edit by hand.
Canonical machine constitution: `tools/boundary-check/config/road1.toml`

- Constitutional class: `worth/ui`
- Domain noun: `runtime`
- Crate root: `workspaces/worth-ui/crates/worth-ui-runtime`
- Road 1 exemplar role: WORTH UI workspace-owned implementation surface.
- Deferred next homes:

- Public surface: workspace-owned; crate lib.rs remains the explicit export owner
- Facade exports: `none`
- Owned internal modules: `admission, capability, certification_support, declaration, evidence, facade, fact_contract, graph, host, host_exchange, inspection, lifecycle, mounting, native_platform, obligations, runtime, source`
- Allowed in-tree dependency bands: `WORTH UI manifest-declared dependencies`

Machine fences:
- Must not depend on worthy-* crates.
- Replay and reconstruction remain certification-only.
- Pure schema meaning must remain Query-agnostic.

Skeleton fence:
- No Road 1 seed skeleton applies; WORTH UI topology is workspace-owned and mechanically discovered.
