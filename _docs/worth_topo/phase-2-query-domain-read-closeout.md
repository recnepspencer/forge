# Worth-Topo Phase 2 Query-Domain Read Closeout

## Verdict

Closed.

Topology read entry now belongs to `worth_topo::query_domain`. The root facade
and `projection` no longer serve as competing public read-entry buckets.

## Final Shape

- Public topology entry lives in `query_domain`.
- Public read helpers are handle-bound through admitted configured domain
  handles.
- Public read product vocabulary uses `TopologyRead*` names.
- Neighborhood views and evidence rows are exported through `query_domain`
  because they are part of the handle-bound read product API.
- `QuerySchemaBasis::TopologyDomainQuery` remains only as Forge Query
  schema-basis vocabulary. It is not a topology product API or read-entry lane.

## Removed Shape

- Root-facade imports for topology read sessions and read proof products.
- `projection` re-exports for topology read sessions, products, views, and
  proof rows.
- `TopologyDomainQuery*` product-facing names.
- `domain_query` test/support folder names that made the read model look like a
  query-object root instead of a handle-bound topology read lane.

## Proof

Machine-checkable proof now includes:

- compile-fail coverage that rejects root-facade topology read imports
- compile-fail coverage that rejects removed `TopologyDomainQuery*` product
  names
- public API contracts importing topology read surfaces from `query_domain`
- projection closeout tests renamed to `topology_reads`
- runtime query tests renamed to `topology_reads`
- structural closeout certification updated so `query_domain` is the designated
  survivor for the public read seam

Verification commands run during closeout:

- `cargo test -p worth-topo`
- `cargo test -p worth-schema`
- `cargo test -p forge-query`
- `git diff --check`

## Non-Gaps

The remaining `TopologyDomainQuery` code reference is intentional:

- `crates/worth-topo/src/projection/runtime_boundary/read_lowering/schema.rs`

That reference names `QuerySchemaBasis::TopologyDomainQuery`, which is the
lower-level Forge Query schema basis used for read lowering. It does not expose
or preserve a topology-owned `TopologyDomainQuery` read model.
