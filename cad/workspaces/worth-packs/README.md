# worth-packs

This workspace owns platform-tier Road 1 pack crates.

Allowed here:
- `worth-pack-*` crates

Not allowed here:
- `worth-schema-*`, `worth-entry-*`, `worth-derived-*`, or `worth-cert-*`
- hidden runtime adapters or sibling package buckets outside `crates/`

Cross-workspace proof and tooling:
- boundary denial, generated-context proof, and certification proof belong in
  dedicated tool or certification surfaces rather than this workspace root

Package placement:
- Road 1 Rust packages in this workspace must be born under `crates/`
