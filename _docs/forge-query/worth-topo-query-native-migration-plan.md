# Worth Topo Query-Native Migration Plan

`worth-topo` already uses `forge-query` heavily, but it still exposes local
entry, assembly, and orchestration surfaces that sit beside Query rather than
beginning in Query. The target state is one where serious topology reads and
workflow entry start from Query-owned typed domain entry, configured handles,
declaration orchestration, contribution composition, and recovery.

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
- route new public topology entry through Query domain entry instead of local
  assembly roots

Phase-completion rule:

- phase 1 is not complete when the typed domain marker exists but legacy public
  entry roots remain the real front door
- phase 1 is complete only when the Query domain entry surface is the canonical
  public beginning for topology workflow entry
- any remaining local assembly roots at the end of phase 1 must already be on
  the path to adapter-only status, not still treated as peer public entry

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

- keep the current read kernels
- rehang them under admitted-handle helper entry
- demote raw workspace-taking entry points to lower adapter seams

Phase-completion rule:

- phase 2 is not complete when handle-bound helpers exist but the old raw
  workspace-taking neighborhood methods still remain a public caller-facing
  alternative
- phase 2 is complete only when public topology reads begin from admitted
  configured handles and the old raw read seam is reduced to adapter-only
  status
- documentation, compile-fail proofs, and public API certification must all
  agree on that boundary before phase 2 is called done

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
- replace direct mutation-batch lowering as the public workflow seam
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
- no remaining public operator may continue to depend on the old direct
  mutation-batch workflow once phase 3 is declared done

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

- phase 4 is not complete when one migrated declaration family has
  contribution-composed workflow but the rest of the topology operator surface
  still depends on local sidecars for continuity, fallback, or explanation
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
- stop letting it remain a local planning layer that only happens to speak
  Query vocabulary

Completion rule:

- the construction migration is not complete when write and inspect planning
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

- keep projection-building machinery
- demote raw declaration and maintainer exports from daily-driver public API
- make the Query-native topology entry the primary front door

Completion rule:

- this subsystem is not complete when the Query-native topology front door
  exists but callers are still expected to assemble live views, computed
  surfaces, or maintainers directly as a normal public workflow
- it is complete only when those low-level projection surfaces are clearly
  lower-level infrastructure and the public story begins from the topology
  Query-native entry

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
- keep assembly only as adapter infrastructure if it cannot yet be deleted

Completion rule:

- this subsystem is not complete when historical correctness still depends on
  `worth-topo` reconstructing missing basis/materialization truth locally as a
  normal part of the public story
- it is complete only when Query-owned retained-artifact and historical-truth
  handling are the authority and any surviving assembly layer is strictly
  adapter infrastructure

### 8. Bridge Registration

Current `worth-topo` surfaces:

- `build_milestone_one_bridge`
- bridge mapping registrations
- bridge aspect registrations

Target Query surfaces to reference:

- Query-native topology entry as the caller-facing front door
- lower bridge seams only as adapter infrastructure

Required migration:

- classify bridge registration explicitly as retained adapter infrastructure or
  wrap it behind the Query-native topology public story

Completion rule:

- bridge registration is not complete while callers still have to understand
  bridge wiring as part of the normal topology public entry story
- it is complete only when bridge seams are either clearly internal adapter
  infrastructure or fully hidden behind the Query-native topology boundary

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

- committed-artifact alignment is not complete while downstream workflows still
  need to choose between a local topology artifact story and a Query receipt /
  envelope story
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
   mutation-batch public path rather than leaving mixed execution stories in
   place.
6. Demote old public assembly and low-level projection entry surfaces only
   after the public operator/read front doors are fully replaced.

## Query Bug Rule

If a query-related bug blocks this migration, fix the surface in `forge-query`
rather than painting a workaround in `worth-topo`.

## Query Notes

- A prior broken merge temporarily left `forge-query` in a partially landed
  state. That issue has been corrected. Any future Query regression discovered
  during this migration should be fixed at the Query surface itself.
