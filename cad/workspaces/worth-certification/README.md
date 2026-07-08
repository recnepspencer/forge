# worth-certification

This workspace owns platform-tier Road 1 certification crates.

Allowed here:
- `worth-cert-*` crates

Not allowed here:
- `worth-schema-*`, `worth-entry-*`, `worth-derived-*`, or `worth-pack-*`
- ordinary production packages or sibling package buckets outside `crates/`

Cross-workspace proof and tooling:
- ordinary package ownership stays with the owning workspace; this workspace is
  for certification-shaped proof crates only

Package placement:
- Road 1 Rust packages in this workspace must be born under `crates/`
