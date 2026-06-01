# Worth Topo Query-Native Migration Plan

`worth-topo` already uses `forge-query` heavily, but it still teaches too much
generic substrate instead of the full Query DX and product story. The target
state is a clean break: serious topology reads and workflow entry begin only
from Query-owned typed domain entry, configured handles, helper lanes, grouped
products, contribution composition, ordinary outcomes, recovery, retained
artifacts, and continuation workflows.

This document is not just a migration checklist. It is the precedent-setting
plan for how future kernel work should use Query. That means:

- if Query has a richer public helper, chooser, workflow, contribution,
  grouped, outcome, recovery, retained-artifact, or continuation surface for a
  job, `worth-topo` must target that surface
- direct use of generic substrate is allowed only as internal implementation
  detail underneath the preferred public lane
- no backward-compatibility lane is allowed in production code, support code,
  certification code, docs, or tests
- if a surface has been replaced, it must be deleted from live code rather than
  merely deprioritized

## Scope

The migration plan covers these public and semi-public subsystem surfaces:

- topology reads
- topology edits and operator workflows
- construction planning, certification, and fact reporting
- public projection and truth-surface declarations
- query assembly and snapshot materialization
- bridge and invalidation registration classification
- committed artifact alignment with Query receipts, envelopes, recovery, and
  retained-artifact workflows

## Global Query References

These Query docs are the shared reference set for the plan:

- `crates/forge-query/docs/domain-capabilities/platform-entry.md`
- `crates/forge-query/docs/domain-capabilities/configured-domain-handles.md`
- `crates/forge-query/docs/domain-capabilities/typed-binding-pipeline.md`
- `crates/forge-query/docs/domain-capabilities/ordinary-outcomes.md`
- `crates/forge-query/docs/domain-capabilities/recovery-boundary.md`
- `crates/forge-query/docs/domain-capabilities/continuation-pipeline.md`
- `crates/forge-query/docs/domain-capabilities/contribution-composed-orchestration.md`
- `crates/forge-query/docs/domain-capabilities/family-helpers.md`
- `crates/forge-query/docs/domain-capabilities/grouped-authoring.md`
- `crates/forge-query/docs/domain-capabilities/grouped-products.md`
- `crates/forge-query/docs/domain-capabilities/grouped-contributions.md`
- `crates/forge-query/docs/domain-capabilities/grouped-support-readiness.md`
- `crates/forge-query/docs/domain-capabilities/orchestration-inventory.md`
- `crates/forge-query/docs/domain-capabilities/declaration-entry-orchestration.md`
- `crates/forge-query/docs/domain-capabilities/declaration-boundary-receipts.md`
- `crates/forge-query/docs/domain-capabilities/declaration-boundary-envelopes.md`
- `crates/forge-query/docs/domain-capabilities/declaration-bridge-continuation-routing.md`
- `crates/forge-query/docs/domain-capabilities/workflow/single-declaration-to-envelope.md`
- `crates/forge-query/docs/domain-capabilities/workflow/grouped-neighborhood-workflow.md`
- `crates/forge-query/docs/domain-capabilities/workflow/retained-artifact-to-next-step.md`
- `crates/forge-query/docs/domain-capabilities/workflow/envelope-to-signal-or-continuation.md`
- `crates/forge-query/docs/domain-capabilities/workflow/stop-to-recovery.md`
- `crates/forge-query/docs/domain-capabilities/recipes/author-a-grouped-neighborhood-with-contributions.md`
- `crates/forge-query/docs/domain-capabilities/choosing/binding-vs-orchestration-vs-helpers.md`
- `crates/forge-query/docs/domain-capabilities/choosing/grouped-authoring-vs-grouped-products-vs-grouped-contributions.md`
- `crates/forge-query/docs/domain-capabilities/choosing/inspection-vs-readiness-vs-recovery.md`

## Subsystem Plan

### 1. Domain Entry And Configured Handles

Current `worth-topo` surfaces:

- `query_domain::TopologyQueryDomain`
- `query_domain::TopologyCurrentHeadAuthoritativeContext`
- `query_domain::TopologySnapshotReadOnlyContext`
- `query_domain::topology_query_domain_entry(...)`
- admitted configured handle aliases under `query_domain`

Target Query surfaces to reference:

- `ForgeQueryApplicationFacade::domain(...)`
- `ForgeQueryApplicationFacade::domain_checked(...)`
- `ForgeQueryApplicationFacade::domain_proof_root(...)`
- `ForgeQueryDomainEntryRoot`
- `ForgeQueryDomainEntryChecked`
- `ForgeQueryDomainEntryProofRoot`
- `ForgeQueryConfiguredDomainHandleChecked`
- `ForgeQueryAdmittedConfiguredDomainHandle`
- `ForgeQueryDomainOperatingContext`
- `platform-entry.md`
- `configured-domain-handles.md`

Required migration:

- keep one typed topology domain marker
- keep typed operating contexts for current-head authoritative work
- keep typed operating contexts for snapshot read-only work
- route public topology entry only through Query domain entry
- keep projection and the root facade from competing as entry surfaces

Phase-completion rule:

- phase 1 is not complete while projection-first entry or any other
  topology-owned root remains in live code as a topology entry option
- phase 1 is complete only when Query domain entry and admitted configured
  handles are the only topology entry model in live code
- the only allowed remaining references to replaced phase-1 entry surfaces are
  historical engineering docs or intentionally archived material, not
  production code, certification code, tests, or active support code

Phase 1 closeout:

- complete as of the dedicated `query_domain` boundary
- compile-fail proof rejects topology query-domain entry imports from the root
  facade and from `projection`
- root convenience re-exports are not an equal public entry lane

### 2. Read Helpers

Current `worth-topo` surfaces:

- `query_domain::TopologyCurrentHeadReadHandleExt`
- `query_domain::TopologySnapshotReadOnlyReadHandleExt`
- `query_domain::TopologyCurrentHeadReadSession`
- `query_domain::TopologySnapshotReadOnlyReadSession`
- `query_domain::TopologyRead*` request, report, parity, closeout, and
  no-N-plus-one proof products
- topology neighborhood view and evidence types exported through `query_domain`

Target Query surfaces to reference:

- admitted configured domain handles
- handle-bound read sessions and handle-owned helper entry
- `configured-domain-handles.md`
- `platform-entry.md`
- read-facing workflow and chooser guidance rather than declaration-entry seam
  vocabulary

Required migration:

- rehang the current read kernels under admitted-handle helper entry
- remove raw workspace-taking entry points from public topology read code
- remove query-object-root read entry as a live topology read model
- keep topology read DX centered on handle-bound sessions rather than local
  workspace seams
- use `TopologyRead*` for product-facing topology read reports; reserve
  `QuerySchemaBasis::TopologyDomainQuery` for the lower-level Forge Query
  schema-basis vocabulary only

Phase-completion rule:

- phase 2 is not complete while `TopologyDomainQuery`, raw workspace-taking
  neighborhood entry, or any equivalent read-root object remains in live code
  as a topology read option
- phase 2 is complete only when admitted configured handles and handle-bound
  read sessions are the only topology read model in live code
- documentation, compile-fail proofs, public API certification, support code,
  and tests must all agree on that boundary before phase 2 is called done

Phase 2 closeout:

- complete once the live read imports flow through `query_domain`
- `projection` no longer re-exports topology read sessions, products, views, or
  proof rows as a bucket seam
- the root facade no longer exports topology read sessions or read proof
  products
- compile-fail proof rejects root-facade read imports and rejects the removed
  `TopologyDomainQuery*` product names
- the only intentional surviving `TopologyDomainQuery` code reference is
  `QuerySchemaBasis::TopologyDomainQuery`, which names the Forge Query schema
  basis, not the topology product API

### 3. Edit And Declaration Workflow

Current `worth-topo` surfaces:

- topology declaration family types under `topology_operators::declaration_entry`
- topology mutation sequence and digest proof types
- internal lower-runtime mutation application adapters
- `topology_operators/application`

Deleted merge fossils:

- `TopologyEditBatch`
- `TopologyEditContract`
- `TopologyOperatorExecution`
- `TopologyQueryAssembly`

Target Query surfaces to reference:

- canonical substrate:
  - `ForgeQueryDeclarationInput`
  - `ForgeQueryDeclarationFamilyMarker`
  - `ForgeQueryDeclarationEntryOrchestrationChecked`
  - `ForgeQueryDeclarationEnvelope`
- preferred precedent-setting public lanes:
  - `family-helpers.md`
  - `grouped-authoring.md`
  - `grouped-products.md`
  - `grouped-contributions.md`
  - `grouped-support-readiness.md`
  - `workflow/grouped-neighborhood-workflow.md`
  - `recipes/author-a-grouped-neighborhood-with-contributions.md`
  - `ordinary-outcomes.md`
  - `recovery-boundary.md`
  - `choosing/binding-vs-orchestration-vs-helpers.md`
  - `choosing/grouped-authoring-vs-grouped-products-vs-grouped-contributions.md`

Required migration:

- recast topology edits as Query declarations
- remove direct mutation-batch lowering as a live operator workflow seam
- remove the precedent that generic `orchestrate_declaration_entry(...)` is the
  main caller-facing topology edit story when a richer helper or grouped lane
  exists
- for scalar families, generic declaration entry may remain the underlying
  substrate where no richer family-native lane exists yet
- for grouped topology families, target grouped authoring, grouped products,
  grouped contributions, grouped readiness, ordinary outcomes, and recovery as
  the intended public lane
- tests and certification must use the same intended DX/product lane instead of
  bypassing it through generic orchestration-only entry

Phase-completion rule:

- phase 3 is not complete when one topology edit family has been migrated
- phase 3 is complete only when every public topology operator family exposed
  through the topology-operator facade has moved onto Query declaration entry
  infrastructure and the best available Query DX/product lane for that family
- migrating one narrow family first is allowed only as the opening
  implementation slice inside phase 3, not as the definition of phase-3
  completion
- no remaining public or internal live operator may continue to depend on the
  old direct mutation-batch workflow once phase 3 is declared done
- old `TopologyEditBatch`, `TopologyEditContract`, batch-promotion helpers, and
  batch-admission helpers must stay deleted from live operator code rather than
  returning as compatibility authoring layers

### 4. Contribution-Composed Workflow

Current `worth-topo` surfaces:

- naming continuity support
- derived fallback policy
- local execution aftermath and explanation sidecars

Target Query surfaces to reference:

- `contribution-composed-orchestration.md`
- `ordinary-outcomes.md`
- `recovery-boundary.md`
- `workflow/stop-to-recovery.md`
- `choosing/inspection-vs-readiness-vs-recovery.md`

Required migration:

- move topology-specific continuity and explanation artifacts onto Query
  contributions
- use declaration-plus-contribution orchestration where topology semantics
  extend bare declaration entry
- keep stop handling on Query ordinary outcomes and Query recovery instead of
  topo-local stop translation sidecars
- require tests and support flows to use the same contribution and recovery
  lane

Phase-completion rule:

- phase 4 is not complete while any migrated declaration family still depends
  on local sidecars for continuity, fallback, or explanation
- phase 4 is complete only when every operator family that requires
  topology-specific continuity, aftermath, or explanation semantics carries
  those semantics on Query contribution and recovery surfaces rather than local
  execution sidecars

### 5. Construction

Current `worth-topo` surfaces:

- `construction/authority.rs`
- `construction/execution.rs`
- `construction/certification.rs`
- `construction/facts.rs`

Target Query surfaces to reference:

- `platform-entry.md`
- `configured-domain-handles.md`
- `declaration-entry-orchestration.md`
- `declaration-boundary-receipts.md`
- `declaration-boundary-envelopes.md`
- `workflow/single-declaration-to-envelope.md`
- `grouped-authoring.md`
- `workflow/grouped-neighborhood-workflow.md` where construction jobs are
  neighborhood-shaped

Required migration:

- treat construction as a first-class Query-native migration target
- remove the local construction-first planning story from live code
- keep construction entry and authority-crossing outputs aligned with the same
  handle, receipt, envelope, outcome, and recovery story as the rest of
  `worth-topo`

Completion rule:

- the construction migration is not complete while write and inspect planning
  still begin from local construction authority surfaces with Query used only as
  a downstream tool
- it is complete only when construction entry, execution planning, and
  authority-crossing outputs are aligned with the same Query-native domain
  entry, handle, receipt, envelope, and grouped/helper story as the rest of
  `worth-topo`

### 6. Projection And Truth Surfaces

Current `worth-topo` surfaces:

- `declare_topology_entity_live_view`
- `declare_topology_materialized_surface`
- `topology_*_computed_declaration`
- `Topology*Maintainer`

Target Query surfaces to reference:

- `typed-binding-pipeline.md`
- `choosing/binding-vs-orchestration-vs-helpers.md`
- `configured-domain-handles.md`
- `ordinary-outcomes.md`
- handle-owned helper or orchestration lanes at the public boundary rather than
  raw maintainer/declaration exports

Required migration:

- keep projection-building machinery only where still required as internal
  implementation detail
- remove raw declaration and maintainer exports from the live topology-facing
  API
- make the Query-native topology entry, binding, and handle-owned helper story
  the only public front door

Completion rule:

- this subsystem is not complete while callers in live code can still assemble
  live views, computed surfaces, or maintainers directly as a topology entry
  workflow
- it is complete only when those low-level projection surfaces are absent from
  the live topology-facing API and the topology Query-native entry and
  handle-owned workflow lane are the only public front door

### 7. Query Assembly And Historical Materialization

Current `worth-topo` surfaces:

- `declared_query_surfaces`
- runtime-basis posture inference in query runtime contracts
- topology materialization and historical read adapters

Target Query surfaces to reference:

- `workflow/retained-artifact-to-next-step.md`
- `declaration-boundary-envelopes.md`
- `continuation-pipeline.md`
- `workflow/envelope-to-signal-or-continuation.md`
- Query-owned retained artifact, historical truth, and continuation handling

Required migration:

- collapse local historical-basis rebuilding onto Query-owned retained artifact
  semantics
- remove local assembly-owned historical entry once Query-owned historical
  handling exists
- forbid topo-local reconstruction of missing progression or historical basis
  when retained artifacts or continuation workflows can carry the truth

Completion rule:

- this subsystem is not complete while historical correctness still depends on
  `worth-topo` reconstructing missing basis or materialization truth locally in
  live code
- it is complete only when Query-owned retained-artifact, historical-truth,
  and continuation handling are the only authority and local assembly-owned
  historical entry has been removed from live code

### 8. Bridge Registration

Current `worth-topo` surfaces:

- `build_milestone_one_bridge`
- bridge mapping registrations
- bridge aspect registrations

Target Query surfaces to reference:

- `declaration-bridge-continuation-routing.md`
- `platform-entry.md`
- `configured-domain-handles.md`
- Query-native topology entry as the caller-facing front door
- lower bridge seams only as adapter infrastructure

Required migration:

- remove bridge registration from the live topology-facing entry story
- keep only the minimum internal bridge machinery still required by the
  Query-native boundary

Completion rule:

- bridge registration is not complete while callers in live code still have to
  understand bridge wiring as part of topology entry
- it is complete only when bridge wiring is absent from the topology-facing API
  and fully hidden behind the Query-native topology boundary

### 9. Committed Artifact Alignment

Current `worth-topo` surfaces:

- `TopologyCommittedArtifact`

Target Query surfaces to reference:

- `declaration-boundary-receipts.md`
- `declaration-boundary-envelopes.md`
- `workflow/retained-artifact-to-next-step.md`
- `ordinary-outcomes.md`
- `recovery-boundary.md`

Required migration:

- reconcile local committed artifacts with Query-owned receipt and envelope
  truth so downstream workflows are not split across two artifact stories
- treat `TopologyCommittedArtifact` as acceptable only if it becomes a thin
  topo-owned projection over Query receipt/envelope truth or is deleted
- forbid tests and support code from teaching a parallel local artifact story
  once Query-owned retained artifacts are available

Completion rule:

- committed-artifact alignment is not complete while downstream workflows in
  live code still need to choose between a local topology artifact story and a
  Query receipt, envelope, outcome, recovery, or retained-artifact story
- it is complete only when there is one authoritative artifact progression for
  downstream topology workflows

## Sequencing

1. Add topology domain entry and operating contexts.
2. Rehang read families under admitted-handle helper entry.
3. Open phase 3 by routing one narrow topology edit family through Query
   declaration entry infrastructure, then revise that family onto the richest
   available helper/grouped/product lane before treating it as precedent.
4. Attach topology-specific contribution-composed workflow, ordinary outcomes,
   and recovery, starting with the first migrated family and continuing until
   the full operator surface is on the contribution-capable path where
   required.
5. Widen to grouped topology workflows and remove the old direct
   mutation-batch path from live code rather than leaving mixed execution
   stories in place.
6. Remove old public assembly and low-level projection entry surfaces once the
   Query-native front doors fully replace them.

## Clean-Break Rule

This plan follows a clean-break rule, not a compatibility-migration rule.

- replaced topology entry surfaces must be removed from live code
- replaced topology read surfaces must be removed from live code
- replaced topology operator authoring and execution surfaces must be removed
  from live code
- replaced surfaces must not survive in tests, certification harnesses,
  support code, or active docs as "temporary" precedent lanes
- "internal only", "secondary", "adapter-only", "reduced", "still available",
  or similar coexistence language is not sufficient completion for any phase
- the only acceptable surviving references to replaced surfaces are old
  engineering docs or intentionally archived historical material

## Precedent Rule

This migration is supposed to teach future kernel work how to use Query.

- if Query has a chooser, helper, grouped, workflow, contribution, ordinary
  outcome, recovery, retained-artifact, or continuation surface for the job,
  that is the surface the migration should target
- generic substrate is not the desired public precedent when a richer public
  Query lane exists
- tests are part of the precedent surface and must use the same intended DX and
  product lane as production code

## Query Bug Rule

If a Query-related bug blocks this migration, fix the surface in `forge-query`
rather than painting a workaround in `worth-topo`.

## Query Notes

- a prior broken merge temporarily left `forge-query` in a partially landed
  state. That issue has been corrected.
- any future Query regression discovered during this migration should be fixed
  at the Query surface itself.
