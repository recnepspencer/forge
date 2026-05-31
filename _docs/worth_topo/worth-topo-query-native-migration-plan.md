# Worth Topo Query-Native Migration Plan

`worth-topo` already uses `forge-query` heavily, but it still exposes local
entry, assembly, and orchestration surfaces that sit beside Query rather than
beginning in Query. The target state is a clean break: serious topology reads
and workflow entry begin only from Query-owned typed domain entry, configured
handles, declaration orchestration, contribution composition, and recovery. Old
topology-owned entry systems are not to remain as secondary, internal, adapter,
or fallback peers in live code. If a surface has been replaced, it must be
deleted from live code rather than merely deprioritized.

## Scope

The migration plan covers these public and semi-public subsystem surfaces:

- topology reads
- topology edits and operator workflows
- construction planning, certification, and fact reporting
- public projection and truth-surface declarations
- query assembly and snapshot materialization
- bridge and invalidation registration classification
- committed artifact alignment with Query receipts and envelopes

## Subsystem Plan

### 1. Domain Entry And Configured Handles

Current `worth-topo` surfaces:

- `TopologyQueryAssembly`
- `TopologyDomainQuery::load()`
- broad public `projection` and `facade` exports

Target Query surfaces to reference:

- `ForgeQueryApplicationFacade::domain(...)`
- `ForgeQueryApplicationFacade::domain_checked(...)`
- `ForgeQueryApplicationFacade::domain_proof_root(...)`
- `ForgeQueryDomainEntryRoot`
- `ForgeQueryDomainEntryChecked`
- `ForgeQueryDomainEntryProofRoot`
- `ForgeQueryConfiguredDomainHandleDraft`
- `ForgeQueryConfiguredDomainHandleChecked`
- `ForgeQueryAdmittedConfiguredDomainHandle`
- `ForgeQueryDomainOperatingContext`

Required migration:

- add one typed topology domain marker
- add typed operating contexts for current-head authoritative work
- add typed operating contexts for snapshot read-only work
- route public topology entry only through Query domain entry
- remove local assembly-first entry as a live topology entry model

Phase-completion rule:

- phase 1 is not complete while `TopologyQueryAssembly`, projection-first
  entry, or any other topology-owned root remains in live code as a topology
  entry option
- phase 1 is complete only when Query domain entry is the only topology entry
  model in live code
- the only allowed remaining references to replaced phase-1 entry surfaces are
  historical engineering docs or intentionally archived material, not
  production code, certification code, or active support code

### 2. Read Helpers

Current `worth-topo` surfaces:

- `TopologyDomainQuery`
- neighborhood helpers under `projection/read_views/domain/views`
- workspace-taking helper entry points

Target Query surfaces to reference:

- admitted configured domain handles
- declaration entry seam readiness and inspection helpers where useful
- handle-owned helper APIs rather than raw workspace seams

Required migration:

- rehang the current read kernels under admitted-handle helper entry
- remove raw workspace-taking entry points from live topology read code
- remove query-object-root read entry as a live topology read model

Phase-completion rule:

- phase 2 is not complete while `TopologyDomainQuery`, raw workspace-taking
  neighborhood entry, or any equivalent read-root object remains in live code
  as a topology read option
- phase 2 is complete only when admitted configured handles and handle-bound
  read sessions are the only topology read model in live code
- documentation, compile-fail proofs, public API certification, and active
  certification support code must all agree on that boundary before phase 2 is
  called done

### 3. Edit And Declaration Workflow

Current `worth-topo` surfaces:

- `TopologyEditBatch`
- `TopologyEditContract`
- `TopologyOperatorExecution`
- `topology_operators/application`

Target Query surfaces to reference:

- `ForgeQueryDeclarationInput`
- `ForgeQueryDeclarationFamilyMarker`
- `ForgeQueryDeclarationEntryOrchestrationChecked`
- `ForgeQueryDeclarationEnvelope`
- ordinary, checked, and proof orchestration lanes

Required migration:

- recast topology edits as Query declarations
- remove direct mutation-batch lowering as a live operator workflow seam
- expose ordinary, checked, proof, and recovery lanes instead of one local
  execution product

Phase-completion rule:

- phase 3 is not complete when one topology edit family has been migrated
- phase 3 is complete only when every public topology operator family currently
  exposed through `TopologyEditBatch`, `TopologyEditContract`, and the
  topology-operator facade has moved onto Query declaration entry orchestration
- migrating one narrow family first is allowed only as the opening
  implementation slice inside phase 3, not as the definition of phase-3
  completion
- no remaining public or internal live operator may continue to depend on the
  old direct mutation-batch workflow once phase 3 is declared done
- `TopologyEditBatch`, `TopologyEditContract`, batch-promotion helpers, and
  batch-admission helpers must be removed from live operator code for
  transitioned families rather than left behind as compatibility authoring
  layers

### 4. Contribution-Composed Workflow

Current `worth-topo` surfaces:

- naming continuity support
- derived fallback policy
- local execution aftermath and explanation sidecars

Target Query surfaces to reference:

- declaration entry contribution composition
- contribution-composed orchestration
- ordinary outcome surfaces
- recovery boundary

Required migration:

- move topology-specific continuity and explanation artifacts onto Query
  contributions
- use declaration-plus-contribution orchestration where topology semantics
  extend bare declaration entry

Phase-completion rule:

- phase 4 is not complete while any migrated declaration family still depends
  on local sidecars for continuity, fallback, or explanation
- phase 4 is complete only when every operator family that requires
  topology-specific continuity, aftermath, or explanation semantics carries
  those semantics on Query contribution surfaces rather than local execution
  sidecars

### 5. Construction

Current `worth-topo` surfaces:

- `construction/authority.rs`
- `construction/execution.rs`
- `construction/certification.rs`
- `construction/facts.rs`

Target Query surfaces to reference:

- typed domain entry
- configured handles
- declaration workflows for write and inspect families
- Query-owned envelopes and receipts where construction crosses authority

Required migration:

- treat construction as a first-class Query-native migration target
- remove the local construction-first planning story from live code

Completion rule:

- the construction migration is not complete while write and inspect planning
  still begin from local construction authority surfaces with Query used only as
  a downstream tool
- it is complete only when construction entry, execution planning, and
  authority-crossing outputs are aligned with the same Query-native domain
  entry, handle, and envelope story as the rest of `worth-topo`

### 6. Projection And Truth Surfaces

Current `worth-topo` surfaces:

- `declare_topology_entity_live_view`
- `declare_topology_materialized_surface`
- `topology_*_computed_declaration`
- `Topology*Maintainer`

Target Query surfaces to reference:

- Query live-view and computed declarations
- typed entry and handle-owned helpers at the public boundary

Required migration:

- keep projection-building machinery only where still required as internal
  implementation detail
- remove raw declaration and maintainer exports from the live topology-facing
  API
- make the Query-native topology entry the only public front door

Completion rule:

- this subsystem is not complete while callers in live code can still assemble
  live views, computed surfaces, or maintainers directly as a topology entry
  workflow
- it is complete only when those low-level projection surfaces are absent from
  the live topology-facing API and the topology Query-native entry is the only
  public front door

### 7. Query Assembly And Historical Materialization

Current `worth-topo` surfaces:

- `TopologyQueryAssembly`
- `historical_rows`
- runtime-basis posture inference in query runtime contracts

Target Query surfaces to reference:

- domain entry and configured handles
- retained artifact and next-step workflow surfaces
- Query-owned historical and progressed-truth handling

Required migration:

- collapse local historical-basis rebuilding onto Query-owned retained artifact
  semantics
- remove local assembly-owned historical entry once Query-owned historical
  handling exists

Completion rule:

- this subsystem is not complete while historical correctness still depends on
  `worth-topo` reconstructing missing basis or materialization truth locally in
  live code
- it is complete only when Query-owned retained-artifact and historical-truth
  handling are the only authority and local assembly-owned historical entry has
  been removed from live code

### 8. Bridge Registration

Current `worth-topo` surfaces:

- `build_milestone_one_bridge`
- bridge mapping registrations
- bridge aspect registrations

Target Query surfaces to reference:

- Query-native topology entry as the caller-facing front door
- lower bridge seams only as adapter infrastructure

Required migration:

- remove bridge registration from the live topology-facing entry story
- keep only the minimum internal bridge machinery still required by the Query
  native boundary

Completion rule:

- bridge registration is not complete while callers in live code still have to
  understand bridge wiring as part of topology entry
- it is complete only when bridge wiring is absent from the topology-facing API
  and fully hidden behind the Query-native topology boundary

### 9. Committed Artifact Alignment

Current `worth-topo` surfaces:

- `TopologyCommittedArtifact`

Target Query surfaces to reference:

- declaration receipts
- declaration envelopes
- retained artifact to next-step workflow

Required migration:

- reconcile local committed artifacts with Query-owned receipt and envelope
  truth so downstream workflows are not split across two artifact stories

Completion rule:

- committed-artifact alignment is not complete while downstream workflows in
  live code still need to choose between a local topology artifact story and a
  Query receipt or envelope story
- it is complete only when there is one authoritative artifact progression for
  downstream topology workflows

## Sequencing

1. Add topology domain entry and operating contexts.
2. Rehang read families under admitted-handle helper entry.
3. Open phase 3 by routing one narrow topology edit family through declaration
   entry orchestration, then continue until every public topology operator
   family has moved.
4. Attach topology-specific contribution-composed workflow, starting with the
   first migrated family and continuing until the full operator surface is on
   the contribution-capable path where required.
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
- "internal only", "secondary", "adapter-only", "reduced", "still available",
  or similar coexistence language is not sufficient completion for any phase
- the only acceptable surviving references to replaced surfaces are old
  engineering docs or intentionally archived historical material

## Query Bug Rule

If a query-related bug blocks this migration, fix the surface in `forge-query`
rather than painting a workaround in `worth-topo`.

## Query Notes

- A prior broken merge temporarily left `forge-query` in a partially landed
  state. That issue has been corrected. Any future Query regression discovered
  during this migration should be fixed at the Query surface itself.
