# worth-entry

This workspace owns platform-tier Road 1 entry crates.

Allowed here:
- `worth-entry-*` crates
- the one installed operating-world root contract and typed borrowed entry-
  family facade grammar
- named graph-participation admission/lowering for genuinely separate graph
  authorities

Not allowed here:
- `worth-schema-*`, `worth-derived-*`, `worth-pack-*`, or `worth-cert-*`
- hidden runtime adapters or sibling package buckets outside `crates/`
- alternate operating-world roots, generic adapter bags, direct graph-to-graph
  bridges, or trace-driven replay surfaces

Cross-workspace proof and tooling:
- replay fences, boundary denial, and generated-context proof belong in tool or
  certification surfaces rather than this workspace root

Package placement:
- Road 1 Rust packages in this workspace must be born under `crates/`
