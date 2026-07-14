# worth-pack-registry

Generated from the machine-owned Road 1 boundary model. Do not edit by hand.
Canonical machine constitution: `tools/boundary-check/config/road1.toml`

- Constitutional class: `worth/pack`
- Domain noun: `registry`
- Crate root: `cad/workspaces/worth-packs/crates/worth-pack-registry`
- Road 1 exemplar role: Road 1 pack-seam descriptor specimen
- Deferred next homes:
- `worth-entry-adoption` -> contribution adoption into runtime-owned work (Milestone 3)
- Public surface: facade-only
- Facade exports: `ContributionDescriptor, ContributionKind, InvalidPackName, PackName, PackRegistration`
- Owned internal modules: `contribution_descriptor, contribution_kind, pack_name, registration`
- Allowed in-tree dependency bands: `schema`

Machine fences:
- Must not depend on worthy-* crates.
- Must not depend on Query engine `worth-query` directly; consume only through configured audience facades.
- No Query audience facade is legal for this band; derived and other ordinary bands have no Query audience in this milestone.
- Must not depend on Query audience facade `worth-query-decl` (allowed bands: entry, cert).
- Must not depend on Query audience facade `worth-query-host` (allowed bands: entry, cert).
- Must not depend on Query audience facade `worth-query-replay` (allowed bands: cert).
- Must not depend on replay surface families such as certification replay [worth-cert-replay, worthy-cert-replay; cert domains: replay, reconstruction].

Skeleton fence:
- Seed skeleton is machine-fenced by boundary-check; undeclared files and mixed-class modules are denied.
