# worth-entry

This workspace owns platform-tier Road 1 entry crates.

Allowed here:
- `worth-entry-*` crates

Not allowed here:
- `worth-schema-*`, `worth-derived-*`, `worth-pack-*`, or `worth-cert-*`
- hidden runtime adapters or sibling package buckets outside `crates/`

Cross-workspace proof and tooling:
- replay fences, boundary denial, and generated-context proof belong in tool or
  certification surfaces rather than this workspace root

Package placement:
- Road 1 Rust packages in this workspace must be born under `crates/`
