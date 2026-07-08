# worth-derived

This workspace owns platform-tier Road 1 derived crates.

Allowed here:
- `worth-derived-*` crates

Not allowed here:
- `worth-schema-*`, `worth-entry-*`, `worth-pack-*`, or `worth-cert-*`
- source-authority crates or sibling package buckets outside `crates/`

Cross-workspace proof and tooling:
- cross-workspace denial and replay proof belong in tool or certification
  surfaces rather than this workspace root

Package placement:
- Road 1 Rust packages in this workspace must be born under `crates/`
