# Worth Topology Domain Structure Closeout

> **Status:** Closed
>
> **Parent gate:** [worth-topo-domain-structure-gate.md](/Users/Esther/Documents/Programming/forge_workspace/worktree_2/_docs/worth/worth-topo-domain-structure-gate.md)
>
> **Migration map:** [worth-topo-domain-structure-migration-map.md](/Users/Esther/Documents/Programming/forge_workspace/worktree_2/_docs/worth/worth-topo-domain-structure-migration-map.md)
>
> **Roadmap:** [worth_roadmap.md](/Users/Esther/Documents/Programming/forge_workspace/worktree_2/_docs/worth/worth_roadmap.md)

## Closeout Decision

The Worth topology domain-structure gate is closed.

`worth-topo` now presents a topology-domain skeleton instead of a runtime,
query, provenance, or fixture-shaped skeleton. The permanent roots are:

- `brep`
- `derived_topology`
- `validation`
- `topology_operators`
- `projection`
- `certification`
- `test_support`

The gate is closed by structure, tests, and CI enforcement, not by a tree
snapshot alone.

## Acceptance Evidence

- The proof-carrying migration map exists and uses controlled responsibility
  and move-type vocabularies.
- The old permanent `query`, `edit`, `fixtures`, `helpers`, `utils`, `common`,
  `query_native`, `query_integration`, `milestone_three`, `milestone_two`,
  `runtime_invariants`, and `validators` folders are absent from
  `crates/worth-topo/src`.
- `crates/worth-topo/src/certification/structure_guard.rs` enforces root
  skeleton shape, forbidden folder names, dependency direction, projection
  read-view thinness, validation/certification separation, geometry purity,
  dense direct-file review, and closeout-document discipline.
- `scripts/ci/check_worth_topo_domain_structure.sh` runs the gate in CI through
  formatting, `worth-topo` check, focused structure guards, facade/privacy
  trybuild contracts, the full `worth-topo` suite, and a worth-topo source
  line-cap scan.
- `scripts/ci/check_worth_topo_domain_structure.ps1` runs the same local gate on
  Windows for this workspace.
- Public facade contract tests are owned by
  `certification/public_facade_contracts`; no root `tests` folder remains in
  `worth-topo`.
- No legacy facade/export shims were added for old internal folder names.
- Public Milestone 3 report names remain only as public audit contracts, not as
  internal provenance folders.

## Intentional Deviations

- The target skeleton does not create empty leaf folders. Empty ceremony is
  forbidden; leaves exist only when real code, tests, or proof artifacts have a
  responsibility to own.
- `topology_operators/application/mod.rs` is the single reviewed dependency
  exception that imports projection query assembly while operator application
  still bridges admitted effects into runtime-facing execution. Local rewrite
  families are blocked from importing projection.
- The public facade continues to expose stable public contract names even where
  those names carry audit/milestone language. Internal folders do not use
  milestone-provenance names.

## Dense Direct-File Clusters

The structure guard requires any folder with more than eight direct Rust files
to split or enter an explicit reviewed list. The current reviewed clusters are:

- `certification`: root proof orchestration and public closeout entry points.
- `certification/public_facade_contracts/compile_fail`: trybuild fixture cases
  where each file is a deliberately isolated public/private access scenario.
- `certification/topology_operator_closeout`: proof-surface cluster for current
  topology-operator closeout rows.
- `derived_topology/materialized_graph`: materialized graph responsibility
  cluster split by relation and graph product.
- `projection/diagnostic_surfaces/read_proof`: read-proof row/report cluster,
  not runtime execution.
- `projection/runtime_boundary/query_assembly`: runtime row assembly cluster at
  the Query boundary.
- `projection/runtime_boundary/query_runtime/tests`: runtime-boundary test
  cases kept beside the boundary they falsify.
- `validation/reference_integrity`: invariant-family checks split by topology
  relation pressure.

These are not blanket exemptions. New dense folders fail until reviewed.

## Verification Commands

Final QA passed with:

- `powershell -ExecutionPolicy Bypass -File scripts\ci\check_worth_topo_domain_structure.ps1`
- `cargo fmt --package worth-topo --check`
- `cargo test -p worth-topo certification::structure_guard --quiet`
- `cargo test -p worth-topo --test ui --quiet`
- `cargo test -p worth-topo --quiet`
- `git diff --check`

The bash CI gate could not be executed directly in the Windows local shell
because `bash` is not available on this machine, but the script is wired into
GitHub's Ubuntu CI path and mirrors the verified PowerShell gate.

## Residual Non-Blockers

- Future topology-operator expansion must continue splitting by neighborhood
  family rather than growing `topology_operators/application` or any runtime
  boundary into a new bucket.
- Future geometry binding must remain outside `worth-topo`; this gate only
  permits topology-safe identifiers and topology semantics.
- Future hostile topology programs may split into additional certification
  subfolders when real pressure families earn them. Empty hostile folders are
  not required for closeout.

Milestone 3 can resume on top of this closed topology-domain structure.
