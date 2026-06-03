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
- topology-named Query operator workflow helpers:
  - `TopologyOperatorWorkflowHandleExt`
  - `topology_grouped_operator_neighborhood(...)`
  - `TopologyOperatorDeclarationOutcome`
  - `TopologyOperatorGroupedInput`
  - `TopologyOperatorGroupedDeclaration`
  - `TopologyOperatorGroupedOutcome`
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

Phase 3 opening slice:

- `TopologyOperatorWorkflowHandleExt` is now the public topology-named lane over
  Query canonical declaration, legality review, ordinary declaration outcomes,
  grouped operator declaration, grouped outcomes, grouped support, and recovery
- public API certification for topology operator families now references the
  topology-named operator workflow lane instead of raw generic
  `orchestrate_declaration_entry*` methods as the precedent story
- grouped operator families now have an explicit
  `topology_grouped_operator_neighborhood(...)` entry in public certification so
  grouped authoring/product posture is visible before runtime lowering
- this does not complete phase 3 because the internal runtime application path
  still carries topo-owned mutation lowering/finalization and local artifact
  projection; those must be reconciled with Query receipt/envelope/outcome truth
  before the phase closes

Phase 3 application-anchor slice:

- successful topology operator application now retains a Query declaration
  envelope anchor on `TopologyDeclaredMutationArtifact`
- the internal declaration-entry boundary no longer throws away Query
  orchestration products after preflight; it carries declaration family,
  declaration digest, and envelope digest into the local post-write artifact
- scalar runtime proof and composed successor runtime proof assert that the
  local execution artifact is family-aligned with the Query-owned envelope
  anchor
- this still leaves local mutation lowering and post-write materialization as
  topo-owned implementation detail; the next slice should either narrow
  `TopologyDeclaredMutationArtifact` into a thin Query projection or replace
  more of the execution aftermath with Query receipt/envelope/retained-artifact
  products

Phase 3 artifact-narrowing slice:

- `TopologyDeclaredMutationArtifact` is now a narrower accessor-owned projection
  instead of a crate-wide public-field bag for post-write topology aftermath
- local artifact projection now fails closed if the retained Query envelope
  family key does not match the local semantic family key; topo can no longer
  mint a mismatched local aftermath product over Query truth
- runtime and closeout support code now consume the artifact through narrow
  accessors or owned materialization handoff rather than teaching direct field
  access as the product story
- the topology-facing product layer now also exposes a topology-named
  contribution-composed Query lane so declaration-plus-contribution entry does
  not have to fall back to raw generic Query naming at the public boundary
- grouped operator families now expose the topology-named grouped-contribution
  lane as well, so neighborhood authoring can attach shared Query contribution
  posture without dropping back to the weaker grouped-outcome-only precedent

Phase 3 retained-progression-and-receipt slice:

- the topology-facing Query workflow boundary now exposes a topology-named
  retained progression lane and a topology-named declaration-receipt checked
  lane, so callers and certification no longer need to treat ordinary envelope
  outcomes as the only retained pre-runtime operator truth
- the internal declaration-entry boundary now keeps admitted progression and
  Query declaration receipt truth before local runtime lowering; successful
  local application anchors carry declaration family, declaration digest,
  progression digest, and Query receipt digest instead of only an envelope
  souvenir
- non-success receipt orchestration now crosses the boundary through Query's
  declaration-receipt recovery lane, and topo-local declaration-entry errors
  retain the Query-owned authority surface and recommended action as explicit
  metadata
- runtime proof now asserts local execution alignment against retained Query
  progression and receipt truth instead of envelope-only truth

Phase 3 retained-route handoff slice:

- the topology-facing Query workflow boundary now also exposes a topology-named
  retained route-plan lane, so progression no longer jumps straight to receipt
  as if Query route planning were invisible substrate
- the internal declaration-entry boundary now crosses the route step explicitly
  and carries route-plan digest truth into the local application anchor before
  any receipt-owned or post-write-owned topology aftermath is projected
- route non-success now crosses the same topo-local declaration-entry error
  seam through Query's declaration-route recovery lane instead of collapsing
  route posture into a later generic receipt failure
- runtime proof now asserts that the retained local execution anchor stays
  aligned with the Query route-plan digest preserved on the issued receipt

Phase 3 post-write-query-aftermath slice:

- post-write Query aftermath is now assembled once inside the query-runtime
  boundary rather than being reconstructed separately inside declaration-entry
  finalization and local rewrite execution paths
- `TopologyDeclaredMutationArtifact` now retains one shared Query-owned
  post-write projection artifact that carries the batch-write receipt, receipt
  inspection, and post-write materialized topology view as one retained unit
- application and local rewrite files no longer inspect post-write receipts or
  materialize post-write topology views directly; those responsibilities are
  structurally quarantined behind query-runtime support code and machine-checked
  boundary tests
- this still does not finish phase 3 because topo still owns the actual
  mutation-lowering and post-write semantic closeout story after the shared
  Query aftermath is built; the next honest slice should replace more of that
  remaining local execution aftermath with Query-native retained artifact,
  continuation, or recovery progression rather than topo-local sequencing

Phase 3 topology-named-envelope precedent slice:

- `worth-topo` now exposes a topology-named single-declaration-to-envelope lane
  instead of forcing certification and support code to teach raw generic
  `orchestrate_declaration_entry*` calls as the public operator precedent
- the topology-named envelope lane is carried on the same workflow boundary as
  declaration outcomes, progression, receipts, grouped authoring, and
  contributions; `worth-topo` no longer splits envelope precedent across a
  second sibling extension seam
- declaration-entry projection-closeout tests now enter the envelope ceiling
  through topology-owned Query naming, while still preserving Query's checked,
  proof, and recovery semantics underneath
- a machine-checkable certification guard now fails if the declaration-entry
  proof slice drifts back to raw generic declaration-entry method names
- phase 3 still remains open because the runtime execution aftermath below the
  envelope and receipt ceiling is not yet fully Query-native; this slice fixes
  the public/operator precedent boundary, not the remaining local execution
  closeout seam

Phase 3 signal-or-continuation denial boundary slice:

- the topology-facing Query workflow boundary now exposes topology-named
  envelope-to-signal and envelope-to-continuation helpers instead of forcing
  callers to drop to raw generic Query request constructors at the first
  post-envelope branch point
- `worth-topo` now certifies topology-named checked, proof, ordinary-outcome,
  and recovery lanes for signal compatibility, prepared continuation, and
  continuation execution, even though topology families do not yet expose
  bridge-backed continuation contracts
- the current topology posture is deliberately honest and machine-checkable:
  signal orchestration stays unsupported for unsupported signal families, and
  continuation preparation stays unsupported when the declaration family does
  not expose a bridge continuation contract
- denial and unsupported proofs preserve retained declaration, route, receipt,
  and envelope linkage through Query-owned linked artifacts and Query recovery
  instead of collapsing the stop into topo-local fallback language
- this still does not close phase 3 because the actual post-write semantic
  closeout below retained Query aftermath remains topo-owned; this slice
  finishes the next Query-owned workflow branch above that seam without
  pretending continuation capability already exists

Phase 3 retained-application-handoff slice:

- the topology-facing Query workflow boundary now exposes topology-named
  retained receipt ordinary and proof lanes, plus a topology-named
  envelope-from-progressed ordinary, checked, and proof ladder, so callers do
  not have to treat the checked receipt lane as the only retained post-route
  public step before execution
- the internal declaration-entry seam now carries one proof-bearing retained
  application handoff containing progressed declaration, planned route, issued
  receipt, and issued envelope truth until final post-write closeout instead of
  downgrading those retained Query products into a digest-only local souvenir
  before execution begins
- successful local execution anchors now preserve Query envelope digest truth in
  addition to declaration, progression, route, and receipt digests, so runtime
  closeout asserts alignment against the full retained declaration-entry ladder
- this still does not close phase 3 because topo continues to own the actual
  semantic post-write closeout after the retained Query handoff is assembled;
  the remaining honest target is to replace more of that aftermath sequencing
  with Query-native retained-artifact or continuation/recovery progression

Phase 3 execution-outcome closeout slice:

- current-head runtime support no longer teaches every declaration execution
  aftermath as one bare `Result<artifact, error>` lane; it now exposes a
  topology-named execution outcome seam that distinguishes accepted execution,
  Query-owned declaration-entry stops, and local mutation-lowering failures
- declaration-entry application errors now retain the actual `ForgeQueryRecoveryBrief`
  when Query recovery exists instead of flattening authority and recommended
  action into topo-local string labels
- this is still not the end of phase 3 because the accepted branch remains a
  topo-owned post-write artifact projection; the slice closes the stop
  classification seam first so support and certification stop teaching Query
  stops and local execution failures as the same aftermath product

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

Phase 4 opening slice:

- `forge-query` contribution-composed orchestration now accepts
  declaration-bound continuity contribution intents in the same public lane as
  admission, support, explanation, and workflow contribution intents
- `worth-topo` no longer teaches `topology_operator_contribution_workflow(...)`
  as a thin constructor over `ForgeQueryContributionComposedOrchestrationInput`;
  it now auto-seeds declaration-bound topology continuity and fallback
  explanation posture from the declared mutation sequence itself
- preserved naming continuity now crosses the topology contribution lane as a
  Query continuity contribution, while ambiguous or rejected naming continuity
  crosses as Query explanation posture rather than a topo-local sidecar
- derived fallback posture now crosses the same lane as declaration-bound
  Query explanation posture instead of being left only on local mutation
  sequence vocabulary
- the phase still remains open because the accepted post-write aftermath on
  `TopologyDeclaredMutationArtifact` is still topo-owned; this slice moves the
  declaration-scoped continuity and explanation boundary, not the full
  aftermath progression

Phase 4 retained semantic aftermath slice:

- the declaration-entry runtime execution seam now enters through the
  topology-named contribution-composed Query lane instead of throwing away
  declaration-bound topology contribution truth before execution begins
- the retained application handoff now keeps Query contribution composition
  evidence alongside the retained declaration envelope, so post-write closeout
  no longer has to mint naming aftermath from a topo-only sequence sidecar
- `TopologyDeclaredMutationArtifact` now derives its naming continuity matrix
  and naming report from retained Query contribution evidence and validates
  that retained Query aftermath against the declared topology mutation sequence
  before projecting the accepted runtime artifact
- retained fallback posture is now also encoded and recovered from Query
  contribution evidence through machine-checkable semantic codes rather than
  by decoding prose details, and accepted runtime artifacts expose
  Query-derived fallback policy and explanation detail only after that
  retained evidence matches the declared mutation sequence
- this slice also keeps the structure guard honest by folding the retained
  semantic aftermath helper back into the accepted artifact seam instead of
  widening the top-level `topology_operators/application` flat cluster
- phase 4 still remains open because accepted runtime aftermath still projects
  only the naming and fallback-explanation side of retained contribution
  semantics; the broader local post-write aftermath and downstream explanation
  closeout are not yet fully carried on Query-owned contribution or
  retained-artifact surfaces

Phase 4 accepted closeout rehome slice:

- accepted hostile current-head certification no longer rebuilds replay-step or
  scenario-level digest and naming-continuity truth from declarations after
  runtime success; accepted replay rows and accepted hostile reports now source
  those semantics from `TopologyDeclaredMutationArtifact` and aggregate from
  retained Query-backed accepted step artifacts where multiple steps are
  involved
- accepted replay rows and accepted hostile scenario reports now retain
  Query-derived fallback policy and fallback explanation detail directly on the
  machine-checkable closeout surface instead of forcing downstream acceptance
  logic to infer fallback posture only from declaration-era digest counters
- direct-acceptance fallout rows now prefer the retained accepted fallback
  posture carried on the hostile scenario report, while rejected paths still
  fall back to declaration-era digest posture because they do not yet have an
  accepted retained artifact
- phase 4 still remains open because the branch-local committed-artifact lane
  and the broader accepted post-write aftermath projection are still separate
  topo-owned stories; this slice rehomes the accepted current-head hostile
  closeout boundary, not the full downstream artifact progression

Phase 4 branch-local committed projection slice:

- accepted branch-local parity no longer teaches the accepted row as an
  authority-shaped projection artifact reconstructed straight from
  declaration-era shape; it now enters one explicit committed-projection seam
  that couples the committed artifact to mutation digest shape, naming
  continuity, mutation families, and derived fallback posture
- accepted branch-local parity rows and branch-local acceptance gates now carry
  explicit derived fallback policy on the machine-checkable row surface instead
  of treating branch-local fallback posture as an implicit byproduct of the row
  digest marker
- accepted branch-local execution aggregates branch-truth digest and fallback
  posture from the committed projection seam, while declaration-driven
  scenarios fail closed if the committed projection drifts from the declared
  family sequence
- phase 4 still remains open because branch-local authoring still crosses the
  schema-owned branch commit lane and the committed projection seam still
  starts from `TopologyCommittedArtifact`, not retained Query receipt or
  envelope truth; this slice makes branch-local parity downstream-consistent
  without pretending the branch-write authority boundary has been rehomed yet

Phase 4 accepted scenario projection slice:

- accepted hostile scenario programs no longer hand-assemble accepted
  scenario-level digest, naming-continuity, and fallback-explanation posture
  field by field after execution; they now project those accepted aftermath
  semantics from one retained closeout summary seam built from Query-backed
  accepted step evidence
- `TopologyDeclaredMutationArtifact` now exposes one accepted closeout
  projection that packages mutation families, mutation digest, naming
  continuity, and fallback explanation posture together, and hostile accepted
  step rows now use that projection instead of copying the same aftermath
  accessors separately
- accepted-only scenarios and accepted branches of mixed scenarios both now
  derive their accepted report posture from the same step-row aggregation lane,
  and hostile proof explicitly checks that accepted scenario reports agree with
  the retained accepted step projection rather than merely coexisting with it
- phase 4 still remains open because this closes the accepted hostile scenario
  reporting seam, not the remaining schema-owned branch authoring lane or the
  broader committed-artifact split below it

Phase 4 branch-runtime honesty slice:

- the public topology runtime posture now distinguishes three separate branch
  claims instead of collapsing them into one vague branch-preview admission:
  basis/session selection, branch-local intent staging, and branch-local
  topology declaration execution
- current-head runtime explicitly admits preview and branch session basis
  selection, but it denies branch-local intent staging because the Query
  `Intent` family is not admitted on the current topology runtime
- current-head runtime explicitly denies branch-local topology declaration
  execution, and snapshot posture denies all three branch-facing capabilities
- this slice is intentionally a truth-telling boundary correction, not a fake
  branch authoring migration: branch-local topology authoring still crosses the
  schema-owned branch commit lane, and phase 4 remains open until that actual
  execution/authority seam is rehomed

Phase 4 committed-artifact public-boundary cleanup slice:

- `TopologyCommittedArtifact` is no longer exported from the public
  `worth-topo` facade, and the public certification boundary no longer teaches
  `certify_verified_topology_commit_traced(...)` or
  `certify_milestone_two_verified_topology_commit_traced(...)` as equal
  read-basis entry lanes
- public certification precedent is now the Query-aligned read-basis lane;
  committed-artifact certification remains internal residue for harnesses that
  still depend on the schema-owned branch commit path
- compile-fail proof now rejects facade imports of `TopologyCommittedArtifact`
  and the verified-commit certification helpers, so the parallel local artifact
  story cannot silently re-enter the public boundary
- this still does not complete phase 4 or phase 9 because branch-local
  authoring and the underlying committed-artifact execution path are still
  schema-owned internally; the slice removes public teaching debt first rather
  than pretending the remaining internal seam is solved

Phase 4 internal committed-artifact thinning slice:

- the remaining internal `TopologyCommittedArtifact` no longer carries stored
  `PersistedTopologyTruth` or a duplicate branch identity payload through
  `worth-topo`
- the artifact now retains only the authored raw intent, replay commit history,
  and `DerivedTopologyReadBasis`, and callers derive snapshot and branch truth
  from that retained read basis instead of reaching back into a broader
  schema-owned persisted-truth product
- certification and diagnostic support that still needs the internal committed
  lane now uses the thinner projection, which reduces the parallel local
  artifact story even before the branch-local authoring seam itself is
  rehomed
- this still does not close phase 4 or phase 9 because branch-local authoring
  and replay authority are still schema-owned internally; the slice narrows the
  residue but does not yet replace it with Query-owned retained artifact truth

Phase 4 internal commit-certification input slice:

- the internal milestone-one and milestone-two certification lane no longer
  teaches `TopologyCommittedArtifact` as the canonical input for replay-backed
  certification; it now uses one narrower `TopologyCommitCertificationInput`
  that retains only read-basis truth, authored mutation lineage, and replay
  commit history
- primitive-corpus, bridge-proof, parity, and certification tests that only
  need certification lineage now enter through committed primitive input
  helpers rather than a broader committed-artifact product
- `TopologyCommittedArtifact` still exists for the remaining branch-local
  committed projection seam, but the internal certification story is no longer
  coupled to that wider artifact shape
- this still does not close phase 4 or phase 9 because the branch-local
  schema-owned commit lane and the downstream committed projection seam are
  still real; the slice rehomes internal certification precedent first instead
  of pretending branch authoring has already moved onto Query-owned retained
  artifact truth

Phase 4 internal committed-artifact deletion slice:

- `TopologyCommittedArtifact` is now gone from live `worth-topo` code; the
  remaining schema-backed commit helpers and accepted branch-local parity lane
  now carry one narrower `TopologyCommitCertificationInput` plus the declared
  closeout mutation plan instead of preserving a second committed-artifact
  product
- accepted branch-local committed projection still exists as a named seam, but
  it now projects from retained read-basis truth, replay commit lineage, and
  declared mutation-plan semantics rather than an extra topo-owned committed
  artifact struct
- scale-pressure branch history and internal commit helpers now source their
  branch-local truth digest from the narrower committed input rather than from
  a wider artifact wrapper
- this still does not close phase 4 or phase 9 because branch-local authoring
  still crosses the schema-owned commit lane itself; the slice deletes the
  extra local artifact story, but it does not yet replace the underlying
  branch-write authority boundary with Query-owned retained artifact truth

Phase 4 schema-branch authority honesty slice:

- the remaining branch-local parity seam now names the real authority owner all
  the way through machine-checkable row proof: accepted branch-local parity rows
  no longer advertise a dead `committed_branch_projection` marker after
  `TopologyCommittedArtifact` has been deleted
- the accepted branch-local projection seam, schema-backed test-support commit
  boundary, and hostile acceptance gates now agree on one explicit
  `schema_branch_authority_projection` posture, so certification no longer
  hides the surviving schema-owned branch write lane behind generic topo or
  committed-artifact language
- this still does not close phase 4 or phase 9 because the underlying
  branch-local authoring execution itself is still schema-owned; this slice
  makes the proof boundary honest while that real authority seam remains

Phase 4 schema-branch provenance surface slice:

- the remaining schema-backed branch lane is now explicit on the public
  certification products that still depend on it rather than being hidden only
  in row-digest folklore or generic `branch_local` flags
- branch-local mutation parity rows, branch-local scale-pressure rows, and
  milestone-one and milestone-two branch-local topology reports now carry a
  machine-checkable `TopologyBranchAuthoringBoundary::SchemaTopologyAuthoring`
  posture whenever the surviving schema branch authoring lane is the writer
- hostile proof, public facade contracts, and branch-local certification tests
  now all require that explicit provenance surface, so future cleanup cannot
  silently keep the schema-owned branch lane while teaching a generic
  branch-local product story
- this still does not close phase 4 or phase 9 because the underlying
  branch-local authoring execution has not moved onto Query-owned retained
  artifact truth; this slice makes that surviving authority provenance explicit
  across the public certification surface

Phase 4 schema-branch seeding quarantine slice:

- branch-local read-proof, side-quest, and primitive-corpus support no longer
  import schema branch seeding helpers directly; they now cross one explicit
  topo-local `schema_topology_authoring_boundary` seam for both mainline and
  branch-local primitive seeding
- the surviving schema-backed branch seed lane is now structurally centralized
  alongside the existing schema-backed branch commit helpers instead of being
  scattered across certification and support files as ad hoc direct imports
- this is intentionally a quarantine slice, not a fake Query migration:
  branch-local authoring still uses schema-owned seeding and commit execution,
  but the remaining debt is now easier to audit because `worth-topo` no longer
  improvises that branch-write path in multiple independent places
- phase 4 still remains open because the underlying branch-local authoring
  authority and seeding execution have not moved onto Query-owned retained
  artifact truth; this slice centralizes the residue without overstating it

Phase 4 schema-branch session quarantine slice:

- branch-local certification, read-proof, side-quest, primitive-corpus, and
  scale-pressure code no longer creates schema-backed branches ad hoc; they now
  enter one explicit `open_schema_topology_authoring_branch(...)` seam under
  `test_support/schema_topology_authoring_boundary/`
- accepted branch execution and rejected branch-local parity now both consume
  the same explicit schema-branch session metadata instead of hand-assembling
  branch labels, branch ids, and main-head baselines independently
- `certification/structure_guard.rs` now fails if any new direct
  `.create_branch(...)` call appears outside the explicit schema-topology
  authoring boundary file, so the remaining schema-backed branch session lane
  cannot quietly re-scatter through `worth-topo`
- this still does not close phase 4 or phase 9 because the actual branch-local
  authoring and branch-local topology mutation execution still cross the
  schema-owned authority lane; this slice centralizes branch-session setup and
  keeps the remaining debt machine-checkable

Phase 4 schema-branch execution ledger slice:

- accepted branch-local parity and branch-local scale-pressure no longer
  aggregate schema-backed branch truth by hand after each commit; they now use
  one explicit `SchemaTopologyAuthoringBranchExecutionLedger` seam under
  `test_support/schema_topology_authoring_boundary/`
- the surviving schema-backed branch mutation execution lane now owns branch
  truth-digest accumulation and branch-vs-main divergence posture itself
  instead of forcing each certification slice to rebuild those facts from raw
  committed mutation rows
- `certification/structure_guard.rs` now also rejects any new direct
  `commit_topology_intent_on_branch_through_schema_authority(...)` call outside
  the schema authoring boundary file, so raw schema branch execution cannot
  quietly re-scatter through `worth-topo`
- this still does not close phase 4 or phase 9 because the branch-local
  mutation execution still crosses the schema-owned authority lane at all; this
  slice centralizes the last branch-execution residue without pretending it has
  moved onto Query-owned retained artifact truth

Phase 4 mainline schema-authoring quarantine slice:

- the remaining raw mainline schema commit helper no longer appears in live
  certification code outside `test_support/schema_topology_authoring_boundary`;
  rejection certification now enters through an explicit
  `commit_topology_intent_through_schema_execution(...)` seam instead of
  calling the raw schema-authority helper directly
- hostile closeout scenario programs, accepted branch-local scenario seeding,
  declared query-surface proof tests, and current-head relation-update runtime
  tests now all seed milestone-one primitives through the explicit schema
  authoring boundary rather than importing `seed_milestone_one_primitive(...)`
  from schema directly
- `certification/structure_guard_schema_authoring.rs` now machine-checks that
  direct mainline schema commit execution does not leak back out of the schema
  boundary file and that the migrated scenario/runtime proof lanes do not drift
  back to direct schema primitive seeding
- this still does not close phase 4 or phase 9 because other certification and
  runtime-support slices still seed or author through schema directly; this
  slice quarantines one coherent mainline proof-bearing cluster without
  pretending the entire remaining schema-owned authoring lane is gone

Phase 4 runtime-proof seeding quarantine slice:

- runtime mutation-application proof, runtime topology-read proof, and the core
  projection-closeout topology-read support lanes now seed both
  milestone-one primitives and minimal topology only through the explicit
  schema authoring boundary
- `test_support/schema_topology_authoring_boundary/` now owns the explicit
  `seed_minimal_topology_through_schema_execution(...)` seam alongside the
  existing milestone-one primitive seeding seam, so runtime proof entry no
  longer imports schema seeding helpers directly in the migrated clusters
- `certification/structure_guard_schema_authoring.rs` now machine-checks that
  `projection/runtime_boundary/query_runtime/tests/mutation_application`,
  `projection/runtime_boundary/query_runtime/tests/topology_reads`, and the
  core projection-closeout topology-read proof files do not drift back to
  direct schema seeding
- this still does not close phase 4 or phase 9 because many other proof lanes
  and support files still seed or author through schema directly; this slice
  quarantines the next coherent runtime-proof cluster rather than pretending
  the broader schema-owned authoring residue is gone

Phase 4 declaration-entry and runtime-foundation seeding quarantine slice:

- the remaining topology-read declaration-entry runtime proof family now seeds
  milestone-one primitives and minimal topology only through the explicit
  schema authoring boundary instead of importing schema seeding helpers
  directly across grouped, scalar, split, rehome, successor, and admission
  denial runtime proofs
- the remaining query-runtime foundation proof files
  (`tests/core.rs`, `tests/bridge_verification.rs`, and
  `tests/runtime_posture.rs`) now also seed minimal topology only through the
  explicit schema authoring boundary, so the current-head and snapshot runtime
  posture precedent no longer bypasses the quarantine seam
- `certification/structure_guard_schema_authoring.rs` now machine-checks both
  the full `certification/projection_closeout/tests/topology_reads/declaration_entry`
  tree and the remaining runtime-foundation proof files for direct schema
  seeding drift
- this still does not close phase 4 or phase 9 because other proof and support
  families still seed or author through schema directly; this slice eliminates
  the largest remaining declaration-entry/runtime-foundation bypass cluster
  without pretending the broader schema-owned authoring residue is gone

Phase 4 closeout-proof seeding quarantine slice:

- the remaining topology-operator closeout proof lanes that still shaped the
  hostile precedent surface now seed milestone-one primitives only through the
  explicit schema authoring boundary instead of importing schema seeding
  helpers directly
- that quarantine now covers the mutation-query traversal proof,
  primitive-family closure proof, wire split-collapse primitive closure proof,
  and the full scale-pressure proof cluster, including radial and detach
  sweeps
- `certification/structure_guard_schema_authoring.rs` now machine-checks that
  those closeout proof files do not drift back to direct schema primitive
  seeding, so the hostile closeout precedent stays aligned with the explicit
  quarantine seam
- this still does not close phase 4 or phase 9 because some smaller projection
  and bridge-support proof/support files still seed or bootstrap through schema
  directly; this slice removes the biggest remaining hostile-closeout bypass
  cluster without pretending the full schema-owned authoring residue is gone

Phase 4 projection-support and bridge-support seeding quarantine slice:

- the remaining projection-closeout support proofs (`materialization.rs` and
  `row_lookup.rs`), bridge runtime proof tests, and the adjacent local
  bootstrap/support seams now seed milestone-one primitives and minimal
  topology only through the explicit schema authoring boundary
- `test_support/schema_topology_authoring_boundary/` now owns
  `seed_minimal_topology_through_schema_execution(...)` as a real crate-private
  quarantine seam instead of a test-only helper, because primitive bootstrap
  support still needs that explicit boundary outside unit-test modules
- `certification/structure_guard_schema_authoring.rs` now machine-checks those
  projection-support, bridge-support, and bootstrap-support files for direct
  schema seeding drift
- this still does not close phase 4 or phase 9 because a few residual
  schema-bootstrap call sites remain outside the migrated clusters; this slice
  removes the next coherent support/proof bypass boundary rather than
  pretending the full schema-owned authoring residue is gone

Phase 4 validation-and-derived seeding quarantine slice:

- the remaining validation, runtime-invariant bootstrap, and derived-topology
  test files no longer seed minimal topology directly from schema; they now use
  the explicit schema authoring boundary like the other migrated proof/support
  clusters
- `certification/structure_guard_schema_authoring.rs` now machine-checks those
  validation and derived-topology files for direct primitive or minimal schema
  seeding drift
- this still does not close phase 4 or phase 9 because the broader
  schema-backed authoring lane remains, but it removes another coherent residue
  cluster instead of leaving the lower-level validation and derived-topology
  precedent behind

Phase 4 raw mutation-set quarantine slice:

- validation-side raw mutation-set commits no longer call
  `commit_topology_mutation_set(...)` directly from local proof/support code;
  they now cross the explicit schema authoring boundary through
  `commit_topology_mutation_set_through_schema_execution(...)`
- `certification/structure_guard_schema_authoring.rs` now machine-checks that
  direct schema mutation-set commit entry stays quarantined behind either the
  explicit schema authoring boundary or the intentional production runtime
  write adapter
- this still does not close phase 4 or phase 9 because the surviving
  production runtime write adapter remains a real schema-backed execution seam,
  but it removes the quieter validation-side bypass and makes the remaining
  mutation-set authority boundary explicit

Phase 4 production write-authority mutation-set boundary slice:

- the production query-runtime write adapter no longer calls
  `commit_topology_mutation_set(...)` inline; it now crosses one explicit
  runtime-boundary schema-write seam under
  `projection/runtime_boundary/query_runtime/adapters/schema_write_boundary.rs`
- `certification/structure_guard_schema_authoring.rs` now treats that new
  runtime-boundary seam as the only allowed production-site direct
  mutation-set entry, so raw schema mutation-set execution cannot quietly leak
  back into `write_authority.rs` or other runtime files
- this still does not close phase 4 or phase 9 because the remaining
  branch/session/intent schema-authoring seams still exist, but it finishes the
  mutation-set quarantine honestly by removing the last inlined production call

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

Phase 5 opening slice:

- the root facade and public API certification no longer teach the local
  stepwise construction pipeline (`authority -> lowering -> execution ->
  certification -> fact report`) as public topology API
- local construction planning, execution preparation, certification
  preparation, and fact reporting remain internal support machinery while the
  Query-native construction boundary is still unfinished, instead of being
  preserved as a false public front door
- compile-fail proof now rejects public imports of the old local construction
  authority, stepwise lowering/execution/certification helpers, and fact-report
  products
- this does not complete phase 5 because the replacement Query-native
  construction entry and authority-crossing workflow has not been built yet,
  but it removes the competing local public story before the real Query-native
  construction lane lands

Phase 5 internal construction boundary slice:

- the live internal construction cluster no longer preserves the local
  `authority -> lowering -> execution -> certification -> fact report` pipeline
  as five separate bucket seams
- `worth-topo` now keeps one topology-named
  `prepare_primitive_construction_query_boundary(...)` seam over
  `SpatialConstructionBirthPlan`, producing one consolidated
  `TopologyPrimitiveConstructionQueryBoundary` that carries compose-graph
  mutation posture, required Query families, inspection-backed read posture,
  and condensed construction fact rows together
- the old `construction/authority.rs`, `construction/lowering.rs`,
  `construction/execution.rs`, `construction/certification.rs`, and
  `construction/facts.rs` files are deleted from live code, and
  machine-checkable boundary tests now fail if those stepwise bucket files or
  module declarations come back
- this still does not complete phase 5 because construction entry still begins
  from `SpatialConstructionBirthPlan` rather than a topology-named
  Query-domain handle / receipt / envelope workflow, but it removes the last
  live stepwise construction-first internal precedent before that replacement
  lane lands

Phase 5 live construction consumer rehome slice:

- the topology-named construction boundary is no longer just an unused
  replacement seam inside `worth-topo`; it is now exported through the
  topology facade and consumed by the live `worth-kernel` construction result,
  artifact, phase-report, and Query-proof lanes
- `worth-kernel` no longer depends on topology's deleted public
  lowering/execution/certification/fact-report construction products for the
  rehomed consumer seam; it now crosses from spatial birth truth to one
  `TopologyPrimitiveConstructionQueryBoundary` and reads mutation, read,
  inspection, required-family, and fact-row posture from that single topology
  boundary
- construction graph-composition parity, inspection parity, projection-
  consumption receipt parity, and public construction API proof now certify the
  topology-named Query boundary instead of the old stepwise topology plans
- this still does not complete phase 5 because `worth-kernel` still has a
  broader preexisting dependency surface on deleted topology construction
  authority/runtime helpers outside the rehomed seam, and construction entry
  still begins from `SpatialConstructionBirthPlan` instead of a topology-named
  Query-domain handle / receipt / envelope workflow

Phase 5 runtime-setup and authoring-chain rehome slice:

- `worth-topo` now exposes a sanctioned milestone-one runtime setup seam again
  through the public facade, so `worth-kernel` proof, runtime-report, and
  public-contract surfaces can build canonical topology runtimes without
  reaching into crate-private validation helpers
- that restored public seam is runtime setup only; the deleted stepwise
  topology construction authority, lowering, execution, certification, and
  fact-report products remain non-exported and compile-fail guarded
- `worth-kernel` no longer depends on the deleted
  `TopologyConstructionAuthority` token in its public authoring-chain report;
  the report now names the topology Query-native construction boundary
  directly and records its write surface instead of teaching the old topology
  construction-authority story
- worth-topo and worth-kernel public compile-fail proof was refreshed so the
  new query-native construction boundary and the still-private stepwise
  products both have machine-checkable public-boundary precedent
- this still does not complete phase 5 because kernel construction still
  starts from `SpatialConstructionBirthPlan` and a kernel-side scaffold /
  execution pipeline, rather than a topology-owned Query-native construction
  workflow or receipt lane

Phase 5 construction receipt lane slice:

- the public topology construction seam no longer exposes a raw
  `TopologyPrimitiveConstructionQueryBoundary` bag or a
  `prepare_primitive_construction_query_boundary(...)` entry; the live public
  authority-crossing product is now
  `TopologyPrimitiveConstructionQueryReceipt`, admitted through
  `prepare_primitive_construction_query_receipt(...)`
- the over-cap `construction/query_native_boundary.rs` bucket is split into
  explicit surface-vocabulary, receipt, and admission sub-boundaries, so the
  live topology construction seam no longer hides receipt meaning, condensed
  fact rows, and birth-plan admission inside one file-sized private universe
- `worth-kernel` phase-chain execution, phase reports, canonical artifacts,
  prepared-result evidence, runtime-proof parity surfaces, and public
  construction API proof now retain the topology Query receipt directly rather
  than carrying the old boundary product as a universal digest and surface bag
- public compile-fail proof on both crates now rejects the replaced
  `TopologyPrimitiveConstructionQueryBoundary` type and
  `prepare_primitive_construction_query_boundary(...)` entry, while preserving
  the existing gap-register diagnostic seam as intentional historical naming
  residue until that report family is rehomed
- this still does not complete phase 5 because construction entry still begins
  from `SpatialConstructionBirthPlan` and kernel still owns a scaffold /
  execution pipeline over that birth-plan start, but the live authority-
  crossing output is now a topology-named Query receipt lane instead of a raw
  construction boundary bag

Phase 5 construction envelope lane slice:

- the public topology construction seam now exposes a topology-owned
  `TopologyPrimitiveConstructionQueryEnvelope`, admitted through
  `prepare_primitive_construction_query_envelope(...)`, so the construction
  receipt is no longer the highest live authority-crossing product
- the envelope retains the birth digest and topology birth class on the
  topology boundary while delegating mutation, read, inspection, family, and
  fact posture to the retained receipt, which narrows the need for kernel
  phase-chain and artifact surfaces to carry raw birth-plan truth just to keep
  topology construction identity alive
- `worth-kernel` execution preparation, phase reports, canonical artifacts,
  prepared-result evidence, graph-composition parity, inspection parity, and
  public construction API proof now consume the topology envelope seam instead
  of carrying the raw receipt as the universal authority-crossing product
- public compile-fail proof on both crates was refreshed so the replaced
  boundary symbols now point at the new envelope lane as the nearest live
  construction precedent
- this still does not complete phase 5 because kernel construction still
  begins from `SpatialConstructionBirthPlan` and still owns the scaffold /
  birth-completeness pipeline over that start, but the live construction
  output is now receipt plus envelope shaped rather than a kernel-side birth
  plan plus receipt pair

Phase 5 construction handoff lane slice:

- the public topology construction seam now exposes a topology-owned
  `TopologyPrimitiveConstructionQueryHandoff`, admitted through
  `prepare_primitive_construction_query_handoff(...)`, so the live
  authority-crossing construction entry is no longer a kernel-local
  `(SpatialConstructionBirthPlan, TopologyPrimitiveConstructionQueryEnvelope)`
  tuple
- the handoff retains the admitted `SpatialConstructionBirthPlan` and the
  topology Query envelope together under one topology-named construction
  boundary, with a handoff digest that kernel phase-chain execution and phase
  reports can use directly instead of re-pairing birth truth and envelope
  truth locally
- `worth-kernel` scaffold preparation, execution preparation, phase-chain
  reporting, prepared-result evidence, graph-composition parity, and public
  construction API proof now consume the topology handoff seam instead of
  passing the birth-plan-plus-envelope pair by convention
- public compile-fail proof and facade contracts on both crates were refreshed
  so the replaced construction boundary symbols now point at the handoff lane
  as the nearest live authority-crossing construction precedent
- this still does not complete phase 5 because kernel construction still
  begins from `SpatialConstructionBirthPlan` inside its own scaffold flow, and
  birth completeness / impossible attachment / canonical artifact assembly
  still remain kernel-owned workflow over that start rather than a fuller
  topology-owned Query-native construction workflow

Phase 5 admitted construction handoff slice:

- `worth-topo` now exposes
  `prepare_primitive_construction_query_admitted_handoff(...)` and
  `TopologyPrimitiveConstructionQueryAdmittedHandoff`, so impossible birth
  attachment, birth completeness, and birth mapping no longer have to be
  recomputed as a kernel-local post-handoff corridor
- the admitted handoff retains the earlier topology Query handoff plus the
  birth-completeness report, birth-mapping report, and one admitted-handoff
  digest, giving kernel a single topology-owned construction admission product
  instead of a raw handoff plus parallel spatial sidecars
- `worth-kernel` scaffold preparation, execution preparation, phase-chain
  reporting, prepared-result evidence, canonical artifact assembly, graph
  composition parity proof, public construction contract proof, and rejection
  mapping now consume the admitted handoff seam instead of performing
  impossible-attachment and completeness admission directly in the common path
- the prepared-result evidence model now retains one admitted handoff seam and
  projects completeness, mapping, and the underlying Query handoff from there
  rather than storing those products as parallel kernel-owned bags
- this still does not complete phase 5 because kernel construction still
  begins from scaffold-owned `PrimitiveConstructionBirthScaffoldInput` /
  `SpatialConstructionBirthPlan` preparation and still owns the broader
  scaffold-to-execution authoring flow, even though the post-handoff spatial
  admission corridor is now topology-owned

Phase 5 admitted handoff public-precedent slice:

- `worth-kernel` no longer exports or certifies the raw
  `prepare_scaffold_topology_query_handoff(...)` helper as a public phase
  surface; the admitted-handoff helper is now the only sanctioned
  scaffold-to-topology construction admission lane on the kernel public facade
- the now-dead kernel-local raw handoff helper and its `plan_birth()` wrapper
  are deleted from live code, and the dead `PrimitiveConstructionPhaseError`
  handoff variant is removed so the public type surface no longer teaches a
  raw handoff failure lane that the live common path cannot actually emit
- kernel compile-fail proof now rejects importing the raw handoff helper from
  the public facade and proves that even the admitted handoff cannot skip
  execution by calling made-up stepwise methods
- `PrimitiveConstructionAuthorityChainReport` now names the topology admitted
  handoff boundary directly while keeping the lower-layer spatial birth
  authority explicit, so the public authoring-chain report matches the real
  live construction boundary instead of preserving the older envelope-era
  precedent
- this still does not complete phase 5 because kernel still owns scaffold
  realization and the broader construction authoring/orchestration flow over
  that start, even though the public/kernel boundary no longer teaches the
  obsolete raw handoff lane

Phase 5 public scaffold-phase quarantine slice:

- `AdmittedPrimitiveConstructionIntent::build_scaffold()` is no longer public,
  so external callers cannot treat scaffold realization as a sanctioned public
  construction phase
- `worth-kernel` now exposes
  `prepare_admitted_primitive_construction_query_admitted_handoff(...)` as the
  sanctioned admitted-intent-to-topology lane instead of teaching
  `build_scaffold()` plus
  `prepare_scaffold_topology_query_admitted_handoff(...)` as a caller-facing
  workflow
- the public facade no longer exports `PrimitiveConstructionScaffold`,
  `PreparedPrimitiveConstructionExecution`,
  `prepare_scaffold_topology_query_admitted_handoff(...)`, or
  `build_canonical_primitive_construction_artifact(...)`, so the scaffold,
  execution, and artifact buckets are no longer taught as the public happy-path
  construction sequence
- public construction contract proof now goes through admitted intent,
  topology admitted handoff, and prepared-result surfaces rather than manually
  walking scaffold, execution, and artifact stepping
- compile-fail proof now rejects public scaffold building, public raw scaffold
  handoff imports, and public execution-phase export access
- this still does not complete phase 5 because kernel continues to own
  internal scaffold realization and the broader construction
  authoring/orchestration flow, but it removes scaffold realization as a
  caller-facing public precedent

Phase 5 internal scaffold-spread quarantine slice:

- kernel no longer fans local scaffold-phase planning back out across multiple
  live production paths when a prepared result or admitted-handoff common path
  already exists
- a new internal `PreparedPrimitiveConstructionPhaseChainCommonPath` now owns
  the one sanctioned local `admit -> build scaffold -> topology admitted
  handoff -> execution` assembly lane for the kernel common path, so
  `prepare_primitive_construction_result(...)` no longer rebuilds the scaffold
  twice through separate helper corridors
- query graph-composition parity now consumes the prepared-result common path
  instead of rebuilding scaffold, execution, and canonical artifact assembly
  locally just to re-derive the same topology mutation-surface truth
- corpus row-support breadth now derives from prepared-result birth-
  completeness evidence instead of reaching back into direct scaffold counts
- machine-checkable kernel boundary proof now fails if non-test production
  files reintroduce direct scaffold building, direct execution assembly, or
  direct canonical-artifact assembly in the quarantined common-path,
  graph-composition, or corpus-support lanes
- the old canonical artifact helper is now explicitly test-only, so the slice
  does not leave a fake live internal happy-path helper behind after the
  common-path collapse
- this still does not complete phase 5 because kernel still owns the remaining
  internal scaffold realization and the broader construction orchestration
  flow, but the live local construction-first planning spread is reduced to one
  explicit internal seam instead of multiple production fan-out sites

Phase 5 admitted-phase public-precedent quarantine slice:

- `worth-kernel` no longer exports `AdmittedPrimitiveConstructionIntent` or
  `prepare_admitted_primitive_construction_query_admitted_handoff(...)` from
  the root facade or public authoring-construction bucket
- public API certification for construction and spatial-intent placement now
  proves admitted-handoff truth through
  `prepare_primitive_construction_result(...).evidence()` instead of teaching
  `request.admit() -> admitted handoff helper` as a sanctioned caller-facing
  construction phase
- compile-fail proof now rejects public imports of the admitted intent type and
  the admitted-handoff helper, and the root happy-path demotion fixture treats
  both names as removed public exports alongside the earlier scaffold/execution
  helper removals
- the old admitted-handoff helper is now explicitly test-only inside kernel's
  internal scaffold boundary, so live production code stops carrying it as a
  quasi-public internal precedent
- this still does not complete phase 5 because kernel still owns internal
  admission, scaffold realization, and the broader construction orchestration
  flow, but it removes another public intermediate phase and leaves prepared-
  result evidence as the sanctioned caller-facing construction lane

Phase 5 public admission-entry quarantine slice:

- `PrimitiveConstructionRequest::admit(...)` and
  `PrimitiveConstructionIntent::admit(...)` are no longer public, so callers
  cannot enter the admitted construction phase directly through the root
  request/intent types
- compile-fail proof now rejects both raw request admission and construction-
  intent admission as public escape hatches, instead of only rejecting later
  scaffold or admitted-handoff helpers
- the admitted-handoff helper denial fixture was tightened so it proves the
  helper export is gone on its own terms rather than depending on a second
  admission leak to fail first
- this still does not complete phase 5 because kernel still owns the internal
  admission and scaffold-realization flow, but it removes the last caller-
  facing public entry into that phase and makes prepared-result outcome/evidence
  the only sanctioned public construction progression lane

Phase 5 internal admitted-scaffold seam consolidation slice:

- kernel's live common path now owns one prepared admitted-scaffold seam
  instead of passing `AdmittedPrimitiveConstructionIntent` across execution,
  phase-report, artifact, and result construction
- `prepare_primitive_construction_phase_chain_common_path(...)` now performs
  request admission, scaffold realization, and admitted-handoff preparation
  inside one internal seam before handing downstream code a prepared common path
- `PreparedPrimitiveConstructionExecution::from_phase_chain(...)`,
  `PrimitiveConstructionPhaseChainReport::from_phase_chain(...)`, and
  canonical artifact construction no longer depend on
  `AdmittedPrimitiveConstructionIntent`; they read retained request/scaffold/
  admitted-handoff truth instead
- machine-checkable boundary proof now rejects reintroduction of
  `AdmittedPrimitiveConstructionIntent` across the production execution/report/
  result/artifact seam
- dead residue from the older spread was removed too: unused admitted-intent
  accessors are gone, and `PrimitiveConstructionIntent::admit(...)` no longer
  exists as an internal duplicate helper
- this still does not complete phase 5 because kernel still owns the remaining
  internal admission and scaffold-realization choreography, but the admitted
  intent is no longer shared production currency outside that narrowed seam

Phase 5 admitted-intent deletion and file-honesty slice:

- the live internal admitted construction type is gone: kernel no longer keeps
  `AdmittedPrimitiveConstructionIntent` as a production staging object just to
  cross from request admission into scaffold realization
- `phase_chain/admitted_scaffold.rs` now honestly owns the admitted-scaffold
  seam, while the old realization bucket was moved to the predictive
  `phase_chain/scaffold_realization.rs` file instead of hiding scaffold
  realization behind a misleading filename/module alias
- request admission no longer exists as a request-owned helper; the remaining
  production path admits geometry directly inside
  `prepare_primitive_construction_admitted_scaffold(...)`, then realizes the
  scaffold and admitted handoff from that one seam
- construction tests and boundary proof now certify the stronger posture:
  selected production files must not reintroduce either
  `AdmittedPrimitiveConstructionIntent` or `request.clone().admit()` into the
  live construction path
- compile-fail proof for raw public request admission now certifies that the
  method is absent, not merely demoted
- this still does not complete phase 5 because kernel still owns the surviving
  internal geometry admission and scaffold-realization choreography, but the
  admitted-intent phase itself is deleted rather than merely quarantined

Phase 5 scaffold birth-bridge collapse slice:

- the broad `PrimitiveConstructionScaffold` surface no longer teaches the
  scaffold-to-topology birth bridge; `birth_input()` and
  `prepare_scaffold_topology_query_admitted_handoff(...)` were removed from the
  generic scaffold file
- the scaffold birth-input projection and topology admitted-handoff preparation
  now live inside `phase_chain/admitted_scaffold.rs`, so the remaining
  scaffold-to-topology choreography sits under the admitted-scaffold seam
  instead of being split across both `scaffold.rs` and the admitted-scaffold
  boundary
- machine-checkable boundary proof now fails if the broad scaffold file
  reintroduces `PrimitiveConstructionBirthScaffoldInput`,
  `prepare_primitive_construction_query_admitted_handoff(...)`, or a local
  `birth_input(...)` helper
- this still does not complete phase 5 because kernel still owns the remaining
  geometry admission and scaffold realization below that seam, but the
  scaffold-to-topology bridge is no longer a separate broad live surface

Phase 5 admitted-scaffold helper-subtree slice:

- kernel no longer teaches geometry admission and scaffold realization as peer
  construction modules in `construction/mod.rs`; those helpers now live under
  the admitted-scaffold subtree as subordinate implementation details of the
  admitted-scaffold seam
- `phase_chain/admitted_scaffold/mod.rs` now owns the full remaining internal
  choreography for geometry admission, scaffold realization, and topology
  admitted-handoff preparation, while the helper logic is split into
  `geometry_admission.rs` and `realization.rs` beneath that seam instead of
  reading like separate top-level phases
- machine-checkable boundary proof now fails if `construction/mod.rs`
  reintroduces `phase_chain/admission.rs` or
  `phase_chain/scaffold_realization.rs` as peer construction modules
- this still does not complete phase 5 because kernel still owns the admitted-
  scaffold choreography itself, but the file tree now matches the live
  authority/orchestration seam instead of preserving stale peer-phase structure

Phase 5 common-path wrapper deletion slice:

- the separate internal `common_path.rs` wrapper is gone; the admitted-scaffold
  seam now owns execution preparation as well as geometry admission, scaffold
  realization, and topology admitted-handoff preparation
- `prepare_primitive_construction_result(...)` now enters kernel's internal
  construction choreography through the admitted-scaffold boundary directly
  instead of through a second coordinator module that only repackaged the same
  scaffold and admitted-handoff truth
- machine-checkable proof now fails if `construction/mod.rs` reintroduces
  `phase_chain/common_path.rs` as a peer construction module, and if the
  result surface reintroduces the deleted common-path helper/error lane
- this still does not complete phase 5 because kernel still owns the admitted-
  scaffold choreography itself, but it removes another unnecessary local phase
  wrapper and leaves one narrower internal construction owner instead of two

Phase 5 execution subtree rehome slice:

- kernel no longer teaches execution preparation as a peer construction phase in
  `construction/mod.rs`; the live execution boundary now lives under
  `phase_chain/admitted_scaffold/execution.rs` as a subordinate part of the
  admitted-scaffold seam
- the old `PreparedPrimitiveConstructionExecution::from_phase_chain(...)` entry
  is gone from live code; execution now enters through
  `PreparedPrimitiveConstructionExecution::from_admitted_scaffold(...)`
- internal report, artifact, result, certification, and test callers now depend
  on the admitted-scaffold subtree instead of `crate::construction::execution`
- machine-checkable boundary proof now fails if live admitted-scaffold
  production files reintroduce either `phase_chain/execution.rs` or the deleted
  `from_phase_chain(...)` constructor lane
- this still does not complete phase 5 because kernel still owns the admitted-
  scaffold choreography and downstream report/artifact contracts, but it
  removes another stale peer-phase surface and keeps the file tree aligned with
  the live construction authority seam

Phase 5 admitted-result assembly seam slice:

- the stale peer `phase_chain/phase_report.rs` surface is gone; result assembly
  no longer teaches a separate phase-chain report lane after the admitted-
  scaffold seam became the live construction owner
- result evidence now lives under `result_surface/evidence.rs`, where the
  admitted common path lowers into `PrimitiveConstructionResultAssemblyReport`
  and `PrimitiveConstructionResultEvidence` instead of reconstructing a spread
  `request + scaffold + admitted handoff + execution` tuple across separate
  downstream files
- canonical artifact construction no longer uses a standalone tuple-style
  `build_*_with_admitted_handoff(...)` production helper; the live path now
  enters through `CanonicalPrimitiveConstructionArtifact::from_admitted_common_path(...)`
- machine-checkable boundary proof now fails if production code reintroduces
  `phase_chain/phase_report.rs` as a peer module or reintroduces the deleted
  `PrimitiveConstructionPhaseChainReport` lane in live result-assembly files
- dead tuple residue was removed while landing the slice: scaffold no longer
  stores an unused request digest, execution no longer carries unused duplicated
  request/scaffold/birth fields, and the new result-assembly/report products
  retain only the truth they actually expose
- this still does not complete phase 5 because kernel still owns the admitted-
  scaffold choreography and the broader prepared-result authority story, but it
  removes another stale downstream spread and makes the result-assembly lane
  structurally honest

Phase 5 admitted-result input seam slice:

- the stale admitted common-path vocabulary is now gone from the live
  result-assembly lane; result evidence, canonical artifact assembly, and
  prepared result assembly all enter through
  `PreparedPrimitiveConstructionAdmittedResultInput`
- the admitted-scaffold boundary now owns the last retained request truth
  needed downstream, so result assembly no longer carries a raw
  `PrimitiveConstructionRequest` beside the admitted result seam just to recover
  family and request digest
- `PrimitiveConstructionResultAssemblyReport`,
  `PrimitiveConstructionResultEvidence`, and
  `CanonicalPrimitiveConstructionArtifact` now lower directly from admitted
  result input instead of from an extra `(request, common path)` pair
- machine-checkable boundary proof now fails if production code reintroduces
  `PreparedPrimitiveConstructionAdmittedCommonPath`,
  `prepare_primitive_construction_admitted_common_path(...)`,
  `PrimitiveConstructionAdmittedCommonPathError`, or
  `from_admitted_common_path(...)` in the live admitted result-assembly files
- this still does not complete phase 5 because kernel still owns the admitted-
  scaffold choreography itself, but it removes the last downstream raw request
  leak from result assembly and makes the surviving seam more explicit

Phase 5 execution-wrapper deletion slice:

- the redundant admitted-scaffold execution wrapper is gone; kernel no longer
  keeps `PreparedPrimitiveConstructionExecution` or
  `PrimitiveConstructionExecutionError` as a second local staging product over
  the same admitted handoff
- the one real behavior that wrapper owned, the compose-graph mutation posture
  check, now happens directly inside
  `prepare_primitive_construction_admitted_result_input(...)`, so result input
  fails through `PrimitiveConstructionPhaseError::TopologyQueryEnvelope(...)`
  instead of through a separate execution error lane
- `result_surface/result.rs` no longer has a dedicated `Execution(...)`
  variant on `PrimitiveConstructionResultError`; rejected topology-execution
  outcomes are now classified through the existing phase/topology-query
  envelope boundary
- the empty public `facade::outcome::execution` module was deleted, and
  compile-fail proof now certifies that the module itself is absent instead of
  only certifying that a type inside it is absent
- `PrimitiveConstructionResultAssemblyReport` no longer stores a duplicate
  `execution_digest()` sidecar once the execution wrapper is gone; the report
  keeps only family, admitted-handoff digest, mutation surface, and report
  digest
- machine-checkable boundary proof now fails if live admitted-scaffold or
  result-assembly files reintroduce the deleted execution wrapper names or the
  deleted admitted-scaffold execution helper file
- this still does not complete phase 5 because kernel still owns the admitted-
  scaffold choreography itself, but it deletes another false lifecycle layer
  and leaves the surviving construction authority seam more honest

Phase 5 topology-envelope error-lane collapse slice:

- kernel no longer performs a second local compose-graph mutation-posture
  check after crossing the topology admitted-handoff boundary; admitted result
  input now trusts the topology admitted handoff as the authority for that
  construction Query posture
- the dead kernel-local `PrimitiveConstructionPhaseError::TopologyQueryEnvelope`
  lane is deleted from live production code, and synthetic topology-execution
  rejection witnesses now flow through the real
  `TopologyConstructionQueryAdmittedHandoffError::Handoff(...)` corridor
  instead of a separate local envelope failure variant
- the remaining scaffold-to-topology bridge helpers now live under
  `phase_chain/admitted_scaffold/topology_handoff.rs`, so the root admitted-
  scaffold seam stops mixing orchestration with raw birth-input projection and
  topology admitted-handoff preparation
- topology-side proof now certifies that the admitted construction handoff
  itself retains compose-graph mutation posture, and kernel boundary proof now
  fails if live admitted-scaffold/result files reintroduce kernel-local
  topology envelope posture checking
- this still does not complete phase 5 because kernel still owns the surviving
  admitted-scaffold choreography and prepared-result authority story, but it
  removes another false local execution/error corridor and leaves the topology
  admitted-handoff boundary as the one honest owner of that posture

Phase 5 public queryless happy-path quarantine slice:

- `worth-kernel` no longer exports the local
  `prepare_primitive_construction_result(...)` or
  `prepare_primitive_construction_outcome(...)` helpers from the root facade,
  `facade::outcome`, `facade::outcome::prepared`, or the public prelude
- public construction and spatial-intent contract proof now enters prepared
  result and prepared outcome through the query-backed authoring session instead
  of teaching the local admitted-scaffold happy path as a caller-facing public
  construction front door
- internal `construction/mod.rs` also stops re-exporting those local happy-path
  helpers as a broad convenience alias lane; internal callers that still need
  them now depend on the narrower `result` or `outcome` boundary directly
- compile-fail proof now rejects all three former public entry paths:
  root facade imports, `facade::outcome::prepared::*`, and prelude imports
- machine-checkable boundary proof now fails if public facade files
  reintroduce those queryless happy-path helper exports
- this still does not complete phase 5 because kernel still owns the internal
  admitted-scaffold choreography itself, but it removes another false public
  construction front door and keeps query-backed authoring as the sanctioned
  public entry precedent

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

Phase 6 public projection lane quarantine slice:

- the root topology facade no longer exports raw projection declarations,
  computed-declaration builders, maintainer types, query-surface helper inputs,
  or projection metadata carriers as public API
- the surviving projection assembly story is now explicitly internal:
  `declared_query_surfaces` owns query-surface declaration, historical fallback
  synthesis, and snapshot decoding through direct submodule seams instead of a
  broad `projection` bucket or public facade exports
- machine-checkable cleanup closeout now fails if the public facade reintroduces
  low-level projection declarations or maintainer types, and compile-fail proof
  rejects callers that try to import the removed projection-entry lane from the
  public facade
- this does not complete phase 6 because `worth-topo` still owns internal
  projection/truth assembly folders under `projection/`, but it removes the old
  public projection-first entry story in one substantial batch

Phase 6 declared-query-surfaces ownership rehome slice:

- the low-level topology query-surface declaration builders no longer live in
  top-level `projection/truth_surfaces`, `projection/derived_surfaces`, or
  `projection/diagnostic_surfaces/query_diagnostics`
- those internal live/computed/query-diagnostics builders now live under
  `projection/runtime_boundary/declared_query_surfaces/`, so the on-disk module
  topology matches the actual runtime-boundary ownership story
- public derived-diagnostics vocabulary remains in
  `projection/diagnostic_surfaces/`; only the internal declaration-builder lane
  moved, which keeps public report surfaces distinct from the runtime-boundary
  assembly seam
- machine-checkable boundary and cleanup-closeout proof now fail if projection
  root reintroduces the displaced declaration buckets or if
  `diagnostic_surfaces` starts owning the internal query-diagnostics declaration
  lane again
- this still does not complete phase 6, but it removes another false internal
  projection boundary and makes the remaining projection runtime seams much more
  obviously query-runtime-owned

Phase 6 public derived-diagnostics helper demotion slice:

- the public topology facade no longer exports manual derived-diagnostics
  builders like `build_derived_read_diagnostics(...)`,
  `build_derived_invalidation_report(...)`,
  `build_derived_rebuild_report(...)`, or
  `build_derived_fallback_report(...)`
- `projection/diagnostic_surfaces/` now keeps public report vocabulary while
  the manual helper lane lives explicitly under
  `projection/diagnostic_surfaces/derived_read_diagnostics.rs`; internal
  runtime-boundary and certification callers now import that named helper seam
  directly instead of leaning on a `pub(crate)` re-export bucket
- compile-fail proof now rejects callers that try to import the removed manual
  derived-diagnostics builders from the topology facade, and cleanup-closeout
  proof now fails if the facade reintroduces that helper lane as public API
- this still does not complete phase 6, but it removes the remaining public
  manual diagnostics-assembly shortcut and makes the surviving
  `diagnostic_surfaces` boundary read more clearly as derived-diagnostics
  vocabulary instead of a mixed public helper bucket

Phase 6 read-proof domain rehome slice:

- the public read-proof, closeout, parity, and no-n-plus-one vocabulary no
  longer lives under `projection/diagnostic_surfaces/read_proof/`
- that whole subtree now lives under `projection/read_views/domain/read_proof/`,
  which matches the real ownership story: read-proof is the public read-view
  domain surface, not a generic diagnostics bucket
- internal runtime-boundary, read-lowering, certification, and harness callers
  now depend on the read-view-domain proof seam directly, while
  `projection/diagnostic_surfaces/` is reduced to derived-diagnostics ownership
  only
- cleanup-closeout and structure-guard proof now fail if `diagnostic_surfaces`
  reclaims the read-proof subtree or if the read-view domain stops owning that
  proof boundary
- this still does not complete phase 6, but it removes another false folder
  owner and makes the surviving public read boundary materially more Query-style

Phase 6 historical snapshot seam collapse slice:

- `projection/runtime_boundary/declared_query_surfaces/mod.rs` no longer stages
  historical snapshot materialization through a local rows-first protocol before
  decoding; the root boundary now asks `historical_rows.rs` for a ready
  historical snapshot directly and only verifies read-basis alignment afterward
- the old `snapshot_rows.rs` pass-through wrapper is deleted; historical row
  collection and the one legitimate low-level row bag,
  `TopologyQuerySnapshotRows`, now live together in `historical_rows.rs`
- machine-checkable boundary proof now fails if the root seam starts routing
  historical snapshot assembly through a `snapshot_rows::` bucket again, which
  keeps the root runtime-boundary surface focused on declaration and
  read-basis-checked snapshot entry instead of restaging an internal fallback
  workflow
- this still does not complete phase 6, but it removes another false internal
  historical snapshot boundary and makes the surviving declared-query-surfaces
  split read more honestly as low-level row ownership plus decode ownership
  rather than root-local choreography

Phase 6 historical snapshot child-boundary rehome slice:

- the surviving historical snapshot reconstruction seam no longer sits flat
  beside declaration-builder peers under
  `projection/runtime_boundary/declared_query_surfaces/`
- that responsibility now lives under
  `projection/runtime_boundary/declared_query_surfaces/historical_snapshot/`,
  with explicit `rows.rs` and `decode.rs` children plus local tests, so the
  parent boundary clearly separates declaration ownership from historical
  snapshot reconstruction ownership
- machine-checkable cleanup and boundary proof now fail if
  `declared_query_surfaces/mod.rs` flattens `historical_rows` or
  `snapshot_decode` back into peer modules instead of preserving the named
  historical-snapshot child boundary
- this still does not complete phase 6, but it removes another folder-level
  ownership mismatch and makes the remaining historical reconstruction seam
  easier to judge honestly against the upcoming phase-7 retained-artifact work

Phase 6 closeout:

- complete once `certify_topology_query_boundary_cleanup_closeout()` proves all
  cleanup areas closed and the topology-facing public surface is reduced to
  Query-domain entry, handle-bound reads, runtime support, and report
  vocabulary rather than raw projection declarations or maintainer assembly
- `projection` remains internal-only, `query_domain` remains the designated
  survivor for public topology read entry, and compile-fail proof continues to
  reject the removed raw projection helper and declaration lanes from the
  public boundary
- the remaining historical/materialization truth work under
  `declared_query_surfaces/historical_snapshot/` is therefore not a phase-6
  public projection-boundary gap; it is phase-7 retained-artifact and
  historical-materialization work

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

Phase 7 opening slice:

- configured topology read handles now choose current-vs-historical execution
  explicitly instead of letting read execution infer historical posture by
  inspecting workspace runtime support contracts
- the historical basis selection story for topology reads now starts from the
  admitted handle context and snapshot token already owned by the session
  boundary, not from public runtime-contract archaeology
- this advances phase 7 by deleting one topo-local historical-entry guess lane,
  but it does not yet close the phase because historical snapshot
  materialization still reconstructs topology-owned derived truth locally under
  `declared_query_surfaces/historical_snapshot`

Phase 7 historical retained-truth seam slice:

- historical snapshot assembly no longer mixes retained-truth reconstruction
  and diagnostics/equivalence backfill in one flat `rows.rs` file
- the surviving local fallback for materialized/interpreted/validation truth is
  now isolated behind `declared_query_surfaces/historical_snapshot/retained_truth.rs`
  as an explicit retained-truth artifact seam
- `declared_query_surfaces/historical_snapshot/rows.rs` now consumes that seam
  and only owns the remaining historical diagnostics/equivalence row assembly
- this is a real phase-7 narrowing step, but it still does not close the
  phase because `worth-topo` continues to reconstruct historical retained truth
  locally instead of crossing a Query-owned retained-artifact or historical
  materialization surface

Phase 7 certification-owned historical snapshot slice:

- production `declared_query_surfaces` no longer owns a historical snapshot
  artifact type, `snapshot_for_read_basis(...)`, or the
  `historical_snapshot/` subtree at all
- historical snapshot reconstruction now lives under
  `certification/support/historical_query_snapshot/`, and all live callers of
  that reconstruction seam are certification or proof-only surfaces instead of
  production declared-query-surface entry
- the production declared-query-surface boundary is now limited to live and
  computed declaration seams, while machine-checkable closeout proof fails if
  production code reintroduces historical retained-truth reconstruction there
- this is a real phase-7 narrowing step because it deletes live production
  historical assembly, but it still does not close the phase: the surviving
  certification-side helper still reconstructs historical retained truth
  locally instead of crossing a genuine Query-owned retained-artifact or
  historical materialization surface

Phase 7 explicit certification historical execution target slice:

- certification-side topology read proof no longer infers historical posture by
  inspecting workspace runtime contracts or the
  `topology-snapshot-historical-basis` evidence string
- `TopologyReadProofHarness` now exposes explicit `current_head()` and
  `historical_from_workspace_token()` constructors, so replay and snapshot proof
  callers declare their basis posture directly instead of relying on workspace
  contract archaeology hidden inside the harness
- projection closeout parity/readiness/closeout proof and milestone-three side
  quest parity now use the explicit historical harness mode for snapshot-backed
  workspaces, while mutation, scenario, scale-pressure, and ordinary
  certification reads use the explicit current-head mode
- machine-checkable cleanup closeout now fails if
  `certification/support/read_proof_harness.rs` reintroduces
  `SNAPSHOT_HISTORICAL_BASIS_EVIDENCE` or `public_api_contract()` inspection,
  which keeps the historical execution choice aligned with Query's basis-owned
  posture instead of local support heuristics
- this is a real phase-7 narrowing step because it deletes one more topo-local
  historical-entry guess lane from certification support, but it still does not
  close the phase: `historical_query_snapshot/` continues to reconstruct
  historical retained truth locally instead of crossing a genuine Query-owned
  retained-artifact or historical materialization surface

Phase 7 current-head baseline materialization quarantine slice:

- certification-side hostile baseline and materialization proofs no longer call
  `historical_query_snapshot_for_read_basis(...)` when they only need the live
  current-head materialized topology before mutation or comparison
- that narrower current-head seam now lives on
  `certification/support/current_head_materialized_topology.rs`, which first
  consumes the retained computed `materialized` surface when present and only
  falls back to live entity/relation materialization when the runtime has not
  retained the computed row yet
- milestone-three scenario programs and the projection materialization proof now
  use that current-head helper directly, while the historical snapshot helper
  remains reserved for the true read-basis certification lanes that still need
  historical retained-truth reconstruction
- machine-checkable cleanup closeout now fails if those current-head baseline
  files drift back to `historical_query_snapshot_for_read_basis(...)`, which
  keeps the remaining local historical reconstruction quarantined to the
  actual historical-read problem instead of letting ordinary current-head
  baselines overreach
- this is a real phase-7 narrowing step because it shrinks the live caller set
  of the historical reconstruction helper and aligns ordinary hostile baselines
  with Query's current-head retained-materialization posture, but it still does
  not close the phase: `historical_query_snapshot/` continues to reconstruct
  true historical retained truth locally instead of crossing a genuine
  Query-owned retained-artifact or historical materialization surface

Phase 7 historical snapshot support subtree slice:

- certification historical snapshot support no longer lives as one flat
  `historical_query_snapshot.rs` bucket beside a same-named test folder; it now
  lives as a real subtree under `certification/support/historical_query_snapshot/`
  with explicit `truth_bundle.rs`, `derived_snapshot.rs`, and `full_snapshot.rs`
  seams
- the surviving local historical reconstruction is now structurally honest:
  `truth_bundle.rs` owns the historical topology-truth bundle,
  `derived_snapshot.rs` owns diagnostics/equivalence backfill over that retained
  truth, and `full_snapshot.rs` owns the extra naming-attachment layer for the
  few certification callers that truly need the whole snapshot
- `derived_topology_closeout/read_basis.rs` now uses the narrower
  `historical_derived_surface_snapshot_for_read_basis(...)` lane instead of
  rebuilding naming attachments it never reads, while the broader
  `historical_query_snapshot_for_read_basis(...)` helper remains reserved for
  the authority/read-view proof lanes that actually need the full naming plus
  derived snapshot package
- machine-checkable boundary proof now fails if `derived_topology_closeout`
  falls back to the broad full-snapshot helper or if production declared-query
  surfaces start re-owning any part of the historical truth bundle,
  diagnostics, or naming reconstruction seams
- this is a real phase-7 narrowing step because it deletes one more false
  caller dependency on the full historical snapshot and turns the remaining
  certification-side historical reconstruction into a named subtree instead of
  a flat support bucket, but it still does not close the phase: the surviving
  `historical_query_snapshot/truth_bundle.rs` seam continues to reconstruct
  true historical retained truth locally instead of crossing a genuine
  Query-owned retained-artifact or historical materialization surface

Phase 7 staged historical truth source slice:

- certification-side historical truth assembly no longer silently mixes
  staged historical truth with retained downstream derived rows
- `certification/support/historical_query_snapshot/truth_source.rs` now owns
  one explicit source choice for the whole historical truth trio:
  `RetainedRows` or `StagedReadView`
- certification historical truth no longer reconstructs
  `materialized` / `interpreted` / `validation` one component at a time; if the
  retained trio is incomplete, `truth_bundle.rs` now falls back to the shared
  `read_stage` boundary and stages the whole truth package from the snapshot
  read view instead
- downstream diagnostics and equivalence rows are now only trusted when the
  truth bundle stayed on the retained-rows lane; staged truth can no longer
  quietly pair with retained diagnostics or retained equivalence rows
- `HistoricalDerivedSurfaceSnapshot` and `HistoricalTopologyQuerySnapshot` now
  carry `truth_source_path`, which makes historical provenance explicit in the
  returned certification artifacts instead of burying it inside local helper
  behavior
- this narrows phase 7 by turning the surviving local historical reconstruction
  seam into an auditable provenance-carrying boundary, but it still does not
  close the phase: `historical_query_snapshot/truth_bundle.rs` continues to
  reconstruct true historical retained truth locally instead of crossing a
  genuine Query-owned retained-artifact or historical materialization surface

Phase 7 certification read-basis query-runtime seam slice:

- certification-side historical query assembly no longer gets rebuilt inline in
  the authority-closeout and derived-topology read-basis callers through local
  `read_snapshot -> stage -> snapshot_read_only workspace -> declare query
  surfaces` setup blobs
- that shared assembly now lives behind one explicit
  `certification/support/read_basis_query_runtime.rs` seam, which owns
  historical snapshot read-view opening, read staging, query workspace setup,
  declared query-surface installation, and validation/equivalence readiness
  evidence for certification callers
- `historical_query_snapshot_for_read_basis(...)` and
  `historical_derived_surface_snapshot_for_read_basis(...)` now consume that
  shared read-basis query-runtime boundary instead of taking four parallel
  local ingredients from every caller
- this is a real phase-7 narrowing step because it deletes one more duplicated
  local assembly-owned historical entry and makes the remaining historical
  reconstruction seam easier to audit, but it still does not close the phase:
  the shared runtime boundary still feeds local truth reconstruction in
  `historical_query_snapshot/truth_bundle.rs` instead of crossing a genuine
  Query-owned retained-artifact or historical materialization surface

Phase 7 historical truth retained-artifact seam slice:

- certification historical truth no longer crosses the derived/full snapshot
  lanes as three loose `materialized` / `interpreted` / `validation` fields
  assembled behind a flat `truth_bundle.rs` helper
- the surviving local historical truth reconstruction now lives behind one
  explicit retained-artifact seam at
  `certification/support/historical_query_snapshot/truth_artifact.rs`
- `HistoricalReadBasisQueryRuntime` now owns
  `historical_truth_artifact()` directly, so the shared historical query-runtime
  boundary produces one retained historical truth artifact before downstream
  derived or full snapshot orchestration
- `historical_derived_surface_snapshot_for_read_basis(...)` now consumes that
  retained historical truth artifact instead of reopening low-level
  `workspace/materialized + staged_truth + source_path` assembly inputs locally
- this is a real phase-7 narrowing step because it moves the remaining local
  historical truth reconstruction onto an explicit retained-artifact step that
  matches the Query workflow direction more honestly, but it still does not
  close the phase: that retained artifact is still assembled by `worth-topo`
  locally instead of crossing a genuine Query-owned retained-artifact or
  historical materialization surface

Phase 7 basis-aware declaration-time retained refresh slice:

- `forge-query` runtime whole-refresh declaration seeding no longer reuses a
  fake mutation-only refresh story; the runtime now owns one explicit retained
  refresh context that distinguishes mutation refresh from declaration-time
  initialization while still carrying refresh identity, snapshot token, touched
  aspects, and metadata through the same maintainer lane
- runtime backends now expose an explicit declaration-initialization metadata
  seam, so historical or basis-aware runtimes can attach read-basis evidence at
  declaration time instead of forcing maintainers or certification callers to
  rediscover that posture from local fallback logic
- `worth-topo` historical query runtimes now use that seam through
  `TopologyRuntimeAdapters::snapshot_historical_basis(...)`, which injects
  `.topology.read_basis` metadata during declaration-time seeding for retained
  diagnostics and equivalence surfaces
- certification historical derived snapshots no longer rebuild diagnostics or
  equivalence locally once the runtime has declared those surfaces; they now
  require retained rows seeded through the basis-aware runtime boundary, and
  the closeout proof fails if that local compensation returns
- this materially narrows phase 7 because the framework gap is now filled where
  the architecture laws say it belongs: at the Query runtime boundary rather
  than in topo-side snapshot archaeology. The remaining historical truth seam is
  narrower and more honest, but phase 7 still stays open while
  the historical retained-artifact assembly still lives on a topo-owned
  boundary instead of crossing a richer Query-owned retained artifact surface

Phase 7 Query-owned typed derived-materialization decode slice:

- `forge-query` runtime now owns typed single-row retained computed decode on
  two real runtime floors: `ForgeQueryDerivedMaterializationResult` exposes
  `decode_single_row::<T>()`, and `ForgeQueryRetainedUpstreamInputs` exposes
  `decode_single_computed_row(...)` for whole-refresh maintainer use
- `worth-topo` no longer owns the generic `workspace.materialize(...) +
  decode_single_computed_row(...)` archaeology pattern for historical truth,
  diagnostics, equivalence, or current-head hostile baselines; those callers
  now cross either `materialize_declared_query_surface_row(...)` on the
  admitted materialization-intent floor or the Query-owned retained-upstream
  decode seam
- machine-checkable topo cleanup proof now fails if certification historical
  snapshot support falls back to raw retained-row materialization for declared
  computed surfaces instead of the Query-owned materialization/decode floor
- this materially advances phase 7 because the remaining topo-local historical
  truth artifact is now narrower and higher-level: the surviving topo-owned
  retained-artifact seam still assembles the topology-specific retained truth
  package, but it no longer owns the generic retained-row decode substrate that
  Query should provide

Phase 7 Query-owned retained derived-artifact bundle slice:

- `forge-query` runtime now exposes one explicit retained-artifact bundle over
  multiple derived surfaces through `materialize_derived_artifact_bundle(...)`
  plus `ForgeQueryDerivedMaterializationBundle`; downstream code can retain one
  coherent multi-surface materialization artifact instead of rebuilding that
  pack with repeated per-view materialization entry
- `worth-topo` historical truth and derived-snapshot certification support now
  cross that Query-owned bundle boundary before decoding
  `materialized` / `interpreted` / `validation` or
  `diagnostics` / `equivalence_contract`; those files no longer teach the
  caller-owned loop that reissues three or two separate retained-materialization
  entry calls as a local product story
- machine-checkable cleanup proof now fails if the surviving historical truth
  helpers drift back to repeated per-surface retained materialization instead of
  the new Query-owned retained-artifact bundle seam
- this materially advances phase 7 because the remaining topo-local historical
  truth seam is now narrower again: the surviving topo-owned retained-artifact
  seam still assembles the topology-specific truth package, but Query now owns
  both the single-row retained decode floor and the coherent multi-surface
  retained artifact pack that historical topology truth crosses first

Phase 7 retained historical artifact boundary rehome slice:

- certification support no longer owns a dedicated
  `historical_query_snapshot/truth_artifact.rs` wrapper for retained historical
  truth assembly; that file is deleted from live code
- the declared-query-surfaces runtime boundary now owns one explicit retained
  historical artifact seam at
  `projection/runtime_boundary/declared_query_surfaces/retained_artifacts.rs`,
  which assembles both the topology truth trio
  (`materialized` / `interpreted` / `validation`) and the retained derived
  snapshot package (`diagnostics` / `equivalence_contract`) above the Query-
  owned retained-artifact bundle floor
- `HistoricalReadBasisQueryRuntime` now exposes the retained derived-surface
  snapshot directly from that production boundary, and certification-side
  `historical_derived_surface_snapshot_for_read_basis(...)` is reduced to
  read-basis proof over the runtime-owned artifact instead of reopening
  retained-artifact bundle assembly locally
- machine-checkable cleanup proof now fails if certification support drifts
  back to owning retained-artifact bundle assembly for historical derived
  surfaces, while the historical truth assembly proof now points at the
  production `retained_artifacts.rs` seam instead of a certification wrapper
- this materially advances phase 7 because the remaining topo-owned historical
  truth package is now on the production declared-query-surface boundary
  rather than in certification support, but phase 7 still remains open because
  that production boundary is still topology-owned instead of a richer
  Query-owned historical truth artifact surface

Phase 7 Query-owned retained artifact binding slice:

- `forge-query` runtime no longer stops at a naked multi-surface retained
  bundle when the next step needs one exact historical artifact contract.
  `ForgeQueryDerivedMaterializationBundle` now binds through one explicit
  `bind_retained_artifact(...)` seam into `ForgeQueryDerivedArtifactBinding`,
  which owns exact target-set validation plus one retained artifact digest over
  that bundle
- `worth-topo` declared-query-surface retained artifacts now consume that
  stronger binding type directly for both
  `topology.historical.truth` and
  `topology.historical.derived_snapshot`; the topo seam no longer teaches that
  a naked `ForgeQueryDerivedMaterializationBundle` is already a final retained
  artifact identity
- Query docs now describe the stronger floor explicitly: callers that need one
  coherent multi-surface retained pack should cross the bundle seam, and
  callers that also need exact artifact identity should bind that bundle
  through `bind_retained_artifact(...)` instead of inventing local target-set
  folklore
- machine-checkable topo cleanup proof now fails if
  `retained_artifacts.rs` reverts to treating
  `ForgeQueryDerivedMaterializationBundle` as the final retained artifact type
  instead of consuming `ForgeQueryDerivedArtifactBinding`
- this materially advances phase 7 because Query now owns not only typed
  retained-row decode and coherent multi-surface materialization, but also the
  explicit retained artifact binding step over that pack. The remaining topo
  seam is therefore narrower again: topology still owns historical truth
  package semantics, but it no longer owns generic retained artifact identity
  checking or digest construction

Phase 7 Query-owned retained scalar evidence slice:

- `forge-query` runtime no longer makes product or certification code decode a
  whole retained derived row just to prove a few historical or parity-bearing
  scalar fields. `ForgeQueryDerivedArtifactBinding` now exposes
  `consume_scalar_fields(...)`, which turns one named retained derived row into
  one retained scalar fact set with dotted-path extraction and one fact-set
  digest
- `worth-topo` historical retained-artifact proof now consumes that runtime
  seam in two places: the production `retained_artifacts.rs` boundary compares
  diagnostics-carried equivalence evidence against the retained equivalence row
  through Query-owned scalar facts instead of rebuilding nested equivalence
  report meaning locally, and historical read-basis certification now verifies
  snapshot identity, branch identity, mutation origins, truth-basis digest, and
  touched-aspect count through retained scalar facts instead of spelunking
  decoded struct fields
- Query docs now name the new floor explicitly: bundle binding is the right
  seam for exact retained artifact identity, and retained scalar fact
  consumption is the right seam when the next step only needs stable scalar
  evidence from that named retained artifact
- machine-checkable topo boundary proof now fails if the retained-artifact seam
  drifts back to `equivalence_contract_from_diagnostics_rows(...)` for nested
  equivalence comparison or if historical read-basis proof drifts back to raw
  decoded equivalence field access
- this materially advances phase 7 because the remaining topology-owned
  historical seam is narrower again: topo still owns the final topology meaning
  of the historical truth package, but Query now owns generic named retained
  artifact scalar extraction instead of leaving that archaeology to callers

Phase 7 Query-owned retained artifact typed-pack decode slice:

- `forge-query` runtime no longer leaves callers to repeat separate
  `decode_single_row(...)` choreography when one named retained artifact
  already carries the exact multi-surface pack they need. `ForgeQueryDerivedArtifactBinding`
  now exposes `decode_row_pair(...)` and `decode_row_triple(...)` as the
  runtime-owned typed decode floor above exact artifact binding
- `worth-topo` historical retained-artifact assembly now consumes that floor
  directly: the historical truth artifact decodes its retained
  materialized/interpreted/validation trio through one Query-owned triple
  decode, and the historical derived-surface snapshot decodes its retained
  diagnostics/equivalence pair through one Query-owned pair decode
- Query docs now name this seam explicitly as the honest next step when a
  retained artifact needs a small typed pack but is still not yet a full
  projection-consumption source family
- machine-checkable topo proof now fails if the retained-artifact seam drifts
  back to repeated per-row decode choreography instead of consuming the
  Query-owned pair/triple retained-artifact decode floor
- this materially advances phase 7 because the remaining topology-owned
  historical seam is narrower again: topo still owns topology-specific
  historical truth package meaning, but Query now owns the generic typed
  multi-row decode pattern for a named retained artifact

Phase 7 Query-owned retained scalar alignment slice:

- `forge-query` runtime no longer leaves callers to extract two retained scalar
  fact sets and compare them locally when one named retained artifact already
  owns both rows. `ForgeQueryDerivedArtifactBinding` now exposes
  `verify_scalar_alignment(...)`, which proves correspondence across one
  declared set of scalar field pairs and returns one retained alignment
  artifact with its own digest
- `worth-topo` historical retained-artifact assembly now consumes that floor
  directly when proving diagnostics/equivalence contract correspondence. The
  production retained-artifact seam no longer owns the generic scalar
  extraction and pairwise comparison loop; it only supplies the topology-
  specific field map that should stay aligned
- Query docs now name this seam explicitly as the honest next step when one
  retained artifact needs cross-row scalar correspondence proof but is still
  not yet a full projection-consumption source family
- machine-checkable topo proof now fails if the retained-artifact seam drifts
  back to local scalar extraction and comparison folklore instead of consuming
  the Query-owned retained scalar alignment floor
- this materially advances phase 7 because the remaining topology-owned
  historical seam is narrower again: topo still owns topology-specific
  historical truth package meaning, but Query now owns generic retained
  scalar correspondence checking instead of leaving that proof logic to
  callers

Phase 7 closeout:

- complete once historical correctness no longer depends on `worth-topo`
  reconstructing missing basis or materialization truth from staged read
  authority, raw live reads, or repeated local retained-materialization entry
- the surviving topology-owned seam is now the thin
  `declared_query_surfaces/retained_artifacts.rs` projection over Query-owned
  retained and live artifact floors; it decodes and names topology-specific
  historical packages, but it does not rebuild missing truth from lower
  authority
- certification historical snapshot callers now cross one shared
  `HistoricalReadBasisQueryRuntime` seam plus Query-owned retained/live artifact
  bindings instead of reopening staged read, declaration, materialization, or
  direct live-read archaeology inline
- Query runtime now owns the generic historical floors this phase required:
  basis-aware declaration-time retained refresh, exact retained materialize-and-
  bind, exact live read-and-bind, retained typed-pack decode, retained scalar
  evidence, and retained scalar alignment
- Phase 7 is therefore closed: `worth-topo` no longer compensates for missing
  historical basis or retained materialization truth locally, and the next
  unfinished migration target is Phase 8, Bridge Registration

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

Phase 8 closeout:

- complete once hostile closeout proof shows that bridge wiring is no longer a
  topology-facing entry workflow and instead survives only as lower runtime
  adapter infrastructure beneath the Query-native topology boundary
- the public topology surface now omits bridge builders, bridge mapping packs,
  and bridge aspect registrations from both `facade.rs` and
  `runtime_support.rs`, while compile-fail proof rejects those removed bridge
  entry exports from the public API
- the surviving bridge machinery is now internal-only: bridge mapping and
  aspect registration packs are crate-local, `build_milestone_one_bridge(...)`
  is crate-local for certification and bridge regression proof, and the live
  production runtime keeps its bridge wiring below `query_runtime` instead of
  teaching bridge registration as a caller concern
- Phase 8 is therefore closed: bridge registration no longer competes with the
  Query-native topology front door, and the remaining migration work is
  concentrated in Phase 9, Committed Artifact Alignment

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

Phase 9 in-flight slice:

- Query runtime now exposes `materialize_batch_write_artifact_binding(...)` as
  the retained mutation-aftermath seam for one batch-write receipt plus the
  matching exact retained derived artifact, so downstream crates no longer need
  caller-owned `inspect(...)` plus retained materialization choreography just
  to assemble one post-write package
- `worth-topo` post-write mutation closeout now crosses that Query-owned seam
  before projecting topology-specific materialized truth, which narrows the
  remaining local committed-artifact story to the declaration synopsis and
  topology-specific aftermath projection rather than raw receipt/inspection/
  retained-artifact reopening

Phase 9 declared-artifact proof-payload demotion slice:

- the remaining declared mutation artifact no longer retains Query lineage,
  mutation evidence, or execution-shape proof payload as part of its live
  production contract; those generic aftermath proof seams now survive only
  behind test-only access on the local artifact
- live production code therefore keeps only the topology-facing contract that
  actually drives downstream behavior: semantic family key, declaration
  synopsis, accepted semantic projection, and topology-specific materialized
  aftermath
- the constructor still fails closed against Query declaration-family mismatch,
  but that validation no longer forces the live artifact to keep a second
  generic Query lineage sidecar after closeout

Phase 9 declared-synopsis semantic-family ownership slice:

- the remaining declared mutation artifact no longer keeps semantic family
  identity as a free-floating committed-artifact field; that identity now
  lives inside `TopologyDeclaredMutationSynopsis` as part of the topology
  declaration synopsis boundary
- certification and regression proof now read semantic family identity through
  the declaration synopsis seam instead of teaching one extra top-level
  committed-artifact accessor
- live production contract is therefore narrower again: one topology-owned
  declaration synopsis boundary, one accepted semantic projection seam, and
  one topology-specific materialized aftermath seam

Phase 9 direct semantic-projection retention slice:

- the remaining declared mutation artifact no longer retains raw Query
  contribution composition in live production code once validation has already
  succeeded; it now stores the topology semantic projection directly
- retained-application handoff therefore exposes one validated semantic
  projection seam instead of teaching downstream callers to keep one extra
  retained Query composition object alive after closeout
- live committed-artifact truth is narrower again: declaration synopsis,
  validated topology semantic projection, and topology-specific materialized
  aftermath

Phase 9 synopsis and semantic projection boundary audit:

- hostile verification proved that declaration synopsis and validated topology
  semantic projection cannot yet collapse behind test-only seams because
  certification and closeout consumers still compile in the normal library
  build
- the declared mutation artifact therefore still keeps those two seams live, but
  the failed demotion was useful: it showed the remaining dependency is
  structural and auditable instead of being hidden inside a broader generic
  Query aftermath payload
- the remaining committed-artifact contract is now explicit and narrow:
  declaration synopsis, validated topology semantic projection, and
  topology-specific materialized aftermath

Phase 9 accepted mutation projection slice:

- the remaining declared mutation artifact no longer teaches declaration
  synopsis and validated topology semantic projection as two adjacent live
  seams; it now carries one `TopologyAcceptedMutationProjection` that packages
  semantic family identity, mutation families, mutation digest, naming
  continuity, and derived fallback posture together
- certification and runtime proof consumers now read accepted mutation meaning
  through that single projection seam instead of pairing separate
  `declared_mutation_synopsis()` and
  `accepted_query_contribution_semantic_projection()` lookups
- the remaining committed-artifact contract is therefore narrower again and now
  resolves to one accepted mutation projection plus topology-specific
  materialized aftermath

Phase 9 closeout:

- complete once the hostile closeout proof shows that downstream topology
  workflows no longer need to choose between a local committed-artifact story
  and a Query-owned receipt, outcome, recovery, or retained-artifact story
- that closeout now rests on three aligned facts:
  Query owns the retained post-write artifact floor through
  `materialize_batch_write_artifact_binding(...)`, the live declared mutation
  artifact resolves committed mutation meaning through one
  `TopologyAcceptedMutationProjection`, and generic receipt/inspection proof
  access remains test-only instead of surviving as a production competing
  artifact lane
- the surviving `TopologyDeclaredMutationArtifact` therefore reads as the final
  honest topo-owned projection over Query-owned post-write truth: one accepted
  mutation projection plus topology-specific materialized aftermath
- Phase 9 is therefore closed: committed-artifact alignment no longer leaves
  downstream topology workflows split across two artifact stories, and this
  migration plan is complete

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

## Phase Log

Phase 4 branch-workflow helper narrowing slice:

- the explicit schema authoring boundary now exposes operation-shaped helpers
  for common branch-local certification flows instead of forcing ordinary
  callers to manually compose `open_schema_topology_authoring_branch(...)`
  with follow-on primitive seeding or empty branch-local certification input
- primitive corpus branch-local sweeps, topology-read parity/readiness proof,
  milestone-three side-quest parity, and certification branch-local empty-input
  proof now enter schema-backed branch authoring through those workflow-shaped
  helpers
- raw `open_schema_topology_authoring_branch(...)` usage is now reduced to the
  boundary file itself plus the one remaining rejected-branch-local parity
  proof seam that still needs direct branch head unchanged checks
- a machine-checkable guard now fails if raw branch-session helper usage grows
  again outside the explicit boundary and that one intentionally retained proof
  seam
- this is still a quarantine slice, not a fake Query migration: the underlying
  branch-local authoring lane is still schema-owned, but topo no longer teaches
  broad manual schema branch-session assembly as the ordinary support pattern

Phase 4 rejected-branch parity witness slice:

- the last external raw `open_schema_topology_authoring_branch(...)` caller is
  gone from certification code; rejected branch-local hostile parity now enters
  through one explicit schema-boundary witness instead of performing branch
  archaeology inline
- `test_support/schema_topology_authoring_boundary/` now owns a named
  rejected-branch-local parity witness product, including the branch label,
  branch identity, and the proof that the branch head stayed unchanged across
  the rejected branch-local path
- the raw branch-opening primitive is narrowed back to boundary-internal use,
  and machine-checkable structure guard proof now fails if it leaks back out of
  the schema authoring boundary at all
- this is still a quarantine slice, not a fake Query migration: the surviving
  branch-local rejection witness is still schema-owned, but the schema session
  mechanics are now fully boundary-owned rather than partially reconstructed in
  topology certification code

Phase 4 schema-boundary structural split slice:

- the surviving schema-owned topology authoring lane no longer lives in one
  mixed bucket file; it is now split into responsibility-predictive submodules
  for mainline execution, primitive seeding, and branch execution/witness
  flows under `test_support/schema_topology_authoring_boundary/`
- the machine-checkable schema-authoring structure guard now treats the whole
  boundary subtree as the allowed quarantine surface instead of hard-coding one
  file path, so the quarantine proof stayed honest through the structural split
- this is a structural phase-4 slice, not a fake Query migration: the
  remaining schema-owned authoring authority is still real, but it is now
  arranged as deletion-ready sub-boundaries instead of one mixed responsibility
  container

Phase 4 schema-seed contract narrowing slice:

- the explicit schema authoring boundary now owns topo-local primitive seeding
  failures and a topo-local minimal-topology seed witness instead of exporting
  raw schema `MilestoneOnePrimitiveAuthoringError` and `MinimalTopologySeed`
  types as the ordinary `worth-topo` support contract
- the remaining boundary seeding helpers now use execution-shaped names
  (`seed_milestone_one_primitive_through_schema_execution(...)` and
  `seed_minimal_topology_through_schema_execution(...)`) so the live precedent
  no longer teaches authority-shaped helper naming for the quarantined schema
  lane
- certification, runtime proof, validation, bridge proof, derived-topology
  proof, and primitive-corpus support callers now consume the topo-owned
  boundary contract instead of schema seed/result vocabulary directly
- `certification/structure_guard_schema_authoring.rs` now machine-checks both
  that the old authority-shaped helper names stay gone from live code and that
  raw schema primitive seed types stay confined to the schema authoring
  boundary subtree
- this is still a quarantine slice, not a fake Query migration: the surviving
  schema-backed seed and authoring lane still exists, but the contract it
  teaches to the rest of `worth-topo` is now boundary-owned rather than raw
  schema-shaped

Phase 4 accepted semantic-summary seam slice:

- `TopologyDeclaredMutationArtifact` no longer teaches a second
  `TopologyAcceptedMutationCloseoutProjection` bag on top of its retained
  Query-derived semantic aftermath; accepted closeout/report code now crosses
  one shared `TopologyAcceptedMutationSemanticSummary` seam instead
- the same accepted semantic-summary product is now used both for live
  accepted execution artifacts and for hostile scenario re-aggregation from
  replay step rows, so naming continuity and fallback explanation truth are no
  longer split across two differently named topo-local products
- hostile closeout proof and runtime declaration proof now certify against that
  one summary seam rather than the removed closeout-projection wrapper
- this still does not close phase 4 because the accepted aftermath is still
  topo-owned rather than fully Query-owned, but it removes another duplicate
  local artifact layer from the post-write semantic closeout story

Phase 4 retained semantic aftermath lifecycle split slice:

- `topology_operators/application/declared_mutation_artifact.rs` no longer
  acts like a private aftermath universe; query anchor, mutation evidence, and
  semantic aftermath now live in a responsibility-predictive subtree under
  `declared_mutation_artifact/`
- the declaration-entry handoff now retains
  `TopologyQuerySemanticAftermathEvidence` before sequence validation, while
  `TopologyDeclaredMutationArtifact` stores
  `TopologyRetainedSemanticAftermath` only after the declared mutation sequence
  has been checked and the accepted semantic summary has been retained
- accepted semantic summary is now a retained product, not a value rebuilt on
  every artifact read; artifact accessors for mutation families, digest,
  continuity, and fallback posture all flow through that retained summary seam
- this still does not close phase 4 because the retained semantic aftermath is
  still topo-owned and still sits above Query-owned contribution evidence, but
  it makes the phase distinction and structure honest instead of collapsing
  pre-sequence evidence, retained aftermath, and artifact container into one
  file and one pseudo-lifecycle

Phase 4 single retained accepted-aftermath product slice:

- the extra retained-aftermath wrapper is now gone from live production code;
  `TopologyDeclaredMutationArtifact` retains one
  `TopologyAcceptedMutationSemanticSummary` product directly instead of
  carrying a second wrapper that separately owned accepted summary access
- that retained accepted summary now owns the accepted naming report together
  with mutation families, digest, continuity matrix, and fallback posture, so
  accepted aftermath truth no longer splits across a wrapper field and a nested
  summary field
- hostile closeout scenario aggregation now enters through
  `scenario_programs/accepted_semantic_summary.rs` instead of the stale
  `accepted_projection.rs` seam, so the certification tree also teaches the new
  accepted semantic-summary story honestly
- this still does not close phase 4 because the retained accepted aftermath is
  still topo-owned rather than Query-owned, but it deletes another local
  layering seam and makes the retained accepted-aftermath product singular in
  both production and hostile proof code

Phase 4 hostile closeout semantic-summary report slice:

- hostile closeout scenario reports now retain accepted and rejected mutation
  semantics through one certification-owned
  `MilestoneThreeScenarioMutationSemanticSummary` product instead of splatting
  mutation families, digest, continuity matrix, fallback posture, and
  continuity classification across loose top-level report fields
- `MilestoneThreeHostileScenarioReport` now exposes that semantic truth through
  narrow accessors, and the hostile closeout tests plus branch-local parity
  proof use the same accessor seam instead of reaching into the report as a
  field bag
- the first shared-summary attempt in `topology_operator_closeout/shared.rs`
  was deleted when it turned out to be decorative rather than real reused
  structure, so the slice finishes with one honest report seam instead of one
  live seam plus one dead helper claim
- this still does not close phase 4 because the accepted semantic aftermath is
  still topo-owned rather than Query-owned, but it aligns the hostile
  certification report boundary with the same semantic-summary precedent the
  production application boundary already teaches

Phase 4 retained-summary thinning and hostile lowering reuse slice:

- `TopologyAcceptedMutationSemanticSummary` no longer stores a second retained
  naming-report field alongside the retained continuity matrix; accepted
  naming-report access is now derived from the retained matrix rows instead of
  being stored twice inside the same topo-owned aftermath product
- `TopologyDeclaredMutationArtifact` no longer teaches a separate artifact-level
  naming-report accessor; runtime proof now reaches naming-report evidence
  through the retained accepted summary seam itself
- hostile scenario programs no longer hand-build accepted and rejected
  `MilestoneThreeScenarioMutationSemanticSummary` values case by case; they now
  lower through shared accepted/rejected semantic-summary helpers, so the
  certification tree stops re-declaring the same field mapping across each
  hostile family
- this still does not close phase 4 because the retained accepted aftermath is
  still topo-owned rather than Query-owned, but it removes another duplicate
  retained truth layer and makes hostile closeout lowering reuse a real seam
  instead of five parallel struct literal islands

Phase 4 real rejected branch-local parity witness slice:

- rejected branch-local parity rows no longer certify a no-op branch session;
  they now cross the schema-authoring boundary through an actual rejected
  branch-local intent execution attempt and only pass if the branch head stays
  unchanged after that rejection
- the hostile closeout branch-local parity proof now reuses the real hostile
  scenario declaration builders for the rejected bowtie-adjacent and broken
  radial cases instead of inventing a separate branch-local placeholder path
- the old fake rejected-branch witness helper is deleted from the live schema
  boundary, and the row digest now records that rejected execution was actually
  attempted
- this still does not close phase 4 because the surviving branch-local write
  authority is still schema-owned, but it removes a proof weakness in the
  quarantined branch-local lane instead of merely renaming it

Phase 4 stop-sidecar public-boundary cleanup slice:

- topo-local declaration-entry stop and refusal classes no longer leak through
  `topology_operators` or the root facade as part of the public operator
  boundary
- those classes still exist as internal application support vocabulary for the
  local runtime/application seam, but public operator callers are pushed back
  onto Query ordinary outcomes and Query recovery instead of being taught a
  second topo-owned stop taxonomy
- compile-fail proof now rejects public imports of
  `TopologyDeclarationEntryStopClass` and
  `TopologyDeclarationEntryRefusalClass`
- this still does not close phase 4 because the retained accepted aftermath is
  still topo-owned and the surviving schema-backed branch write lane still
  exists, but it removes another public sidecar that the phase said should not
  remain part of the Query-facing precedent

Phase 4 certification-aftermath decoupling and pre-sequence thinning slice:

- hostile closeout certification no longer reconstructs the production
  `TopologyAcceptedMutationSemanticSummary` type from replay step rows just to
  lower it again into certification reporting; the hostile scenario-program
  seam now rebuilds `MilestoneThreeScenarioMutationSemanticSummary` directly
- the accepted hostile scenario program families now retain certification-owned
  accepted semantic summaries instead of storing topo production aftermath
  products in their replay/certification structs
- `TopologyQuerySemanticAftermathEvidence` no longer stores a duplicate
  `TopologyMutationNamingReport` bag just to validate sequence alignment;
  retained Query aftermath now keeps one continuity-matrix seam plus fallback
  posture and validates the declared sequence against that thinner retained
  evidence
- the now-dead cached naming-report field on
  `TopologyDeclaredMutationSequence` is gone from live production code, and the
  declaration-runtime plus hostile direct-acceptance proof now certify against
  the thinner continuity-matrix seam instead of the removed duplicate report
  path
- this still does not close phase 4 because the accepted semantic aftermath is
  still topo-owned and still shapes the post-write closeout story, but it
  removes another certification dependency on topo-local aftermath products and
  thins the retained pre-sequence evidence instead of just renaming it

Phase 4 single accepted-aftermath access seam slice:

- `TopologyDeclaredMutationArtifact` no longer re-teaches accepted aftermath
  through duplicate artifact-level accessors for mutation families, mutation
  digest, naming continuity matrix, fallback policy, or fallback explanation
  detail
- runtime proof and declaration-entry certification callers now reach accepted
  aftermath truth only through `accepted_semantic_summary()` instead of
  splitting across both the retained accepted-summary seam and a second set of
  artifact pass-through helpers
- this still does not close phase 4 because the accepted semantic aftermath is
  still topo-owned rather than Query-owned, but it removes a parallel local
  access contract and makes the retained accepted semantic-summary seam the one
  honest production and proof-facing entry for accepted aftermath data

Phase 4 retained-handoff sidecar deletion slice:

- `TopologyRetainedApplicationHandoff` no longer caches a second topo-owned
  semantic-aftermath sidecar alongside the retained Query contribution artifact
- contribution digest anchoring now comes directly from the retained Query
  contribution composition, and retained semantic aftermath is derived on
  demand from that same contribution composition instead of being stored as a
  parallel handoff field
- `TopologyQuerySemanticAftermathEvidence` no longer mixes contribution-digest
  anchoring with its actual semantic responsibility; it now carries only the
  retained continuity and fallback evidence needed to validate the declared
  mutation sequence
- machine-checkable application boundary proof now rejects reintroduction of a
  cached `semantic_aftermath` handoff field
- this still does not close phase 4 because the accepted semantic aftermath is
  still topo-owned rather than Query-owned, but it removes another duplicate
  post-contribution sidecar and makes the retained Query contribution artifact
  the one authoritative source for handoff-stage contribution truth

Phase 4 direct accepted-summary retention slice:

- `TopologyQuerySemanticAftermathEvidence` is no longer a live
  topo-application boundary type; retained application handoff now crosses
  directly from Query contribution composition to
  `TopologyAcceptedMutationSemanticSummary` retention
- the handoff boundary now exposes one
  `retain_accepted_semantic_summary(...)` step plus direct contribution-digest
  access, instead of teaching a named intermediate topo-owned aftermath-evidence
  product
- the accepted-summary retention logic now lives as one direct semantic
  validation/projection seam over Query contribution composition rather than as
  a two-step `from_query_contribution_composition(...).retain_for_sequence(...)`
  local object protocol
- machine-checkable application boundary proof now rejects re-exporting the old
  `TopologyQuerySemanticAftermathEvidence` type from the application surface
- this still does not close phase 4 because the retained accepted semantic
  summary itself is still topo-owned rather than Query-owned, but it removes a
  second topo-local post-contribution protocol and narrows the handoff to one
  direct retained-summary step over Query contribution truth

Phase 4 declaration-synopsis / aftermath split slice:

- `TopologyAcceptedMutationSemanticSummary` no longer carries declaration-owned
  mutation families or topology-mutation digest
- `TopologyDeclaredMutationArtifact` now retains one separate
  `TopologyDeclaredMutationSynopsis` built from the declared mutation sequence,
  while accepted semantic summary keeps only Query-validated naming continuity
  and fallback aftermath truth
- hostile closeout replay-step projection now combines declaration synopsis with
  accepted semantic aftermath explicitly instead of pretending both belong to
  one accepted-aftermath bag
- the over-cap closeout helper bucket was split by moving replay-step
  projection/aggregation into its own `replay_step_rows` module
- machine-checkable application proof now rejects reintroducing declaration
  synopsis fields into the accepted semantic-aftermath seam
- this still does not close phase 4 because the accepted semantic aftermath is
  still topo-owned, but it removes another false “aftermath owns everything”
  precedent and makes the production lifecycle boundary more honest

Phase 4 deterministic fallback-detail thinning slice:

- `TopologyAcceptedMutationSemanticSummary` no longer stores fallback
  explanation text as retained production aftermath state
- the production accepted-aftermath seam now retains only fallback policy and
  re-derives its explanation text from that policy when callers need it
- accepted aftermath retention no longer reads or carries fallback explanation
  payload from Query contribution composition during production closeout
- machine-checkable application proof now rejects reintroducing a retained
  `fallback_explanation_detail` field into the accepted semantic-aftermath seam
- this still does not close phase 4 because the accepted semantic aftermath is
  still topo-owned, but it removes another deterministic sidecar from the live
  runtime artifact instead of treating explanation text as production truth

Phase 4 certification fallback-detail thinning slice:

- `MilestoneThreeScenarioMutationSemanticSummary` and
  `MilestoneThreeMutationReplayStepRow` no longer store fallback explanation
  text as retained certification state
- the hostile closeout certification seam now retains fallback policy and
  derives the explanation detail on demand instead of caching deterministic
  report prose in replay rows and summary bags
- accepted hostile scenario aggregation still proves fallback posture, but it
  now reconstructs only the actual semantic state and not a second retained
  explanation sidecar
- `certification/structure_guard.rs` now enforces that the closeout report seam
  does not reintroduce stored `fallback_explanation_detail` fields
- this still does not close phase 4 because the accepted semantic aftermath is
  still topo-owned, but it aligns the certification/report lowering seam with
  the thinner production precedent

Phase 4 retained Query semantic-evidence rehome slice:

- `TopologyAcceptedMutationSemanticSummary` is gone from live production code;
  retained accepted aftermath now crosses the declaration-entry to application
  seam as `TopologyOperatorRetainedSemanticEvidence` owned by the topology
  Query workflow boundary
- `TopologyDeclaredMutationArtifact` now retains Query-workflow-owned semantic
  evidence plus a separate declared mutation synopsis, instead of rebuilding or
  storing an application-owned accepted semantic-summary product
- the old application-owned `semantic_aftermath.rs` boundary is deleted, and
  the retained semantic decode/validation logic now lives in
  `query_workflow/workflow_artifacts.rs` alongside the other topology-named
  Query workflow retained products
- runtime proof and closeout replay consumers now read naming continuity and
  fallback posture through `accepted_query_semantic_evidence()` instead of an
  application-owned accepted-summary seam
- `query_workflow/mod.rs` was split so the workflow tree stays under the file
  cap while taking on the new retained semantic evidence responsibility
- this still does not close phase 4 because the retained semantic aftermath is
  still topo-retained rather than natively Query-owned end-to-end, but it
  rehomes the live retained semantic evidence boundary onto the topology Query
  workflow seam and deletes the parallel application-owned aftermath product

Phase 4 retained contribution-composition deletion slice:

- the topo-owned `TopologyOperatorRetainedSemanticEvidence` wrapper is gone;
  retained accepted aftermath now stays as the underlying
  `ForgeQueryDeclarationEntryContributionComposition`, held through the
  topology-named alias `TopologyOperatorRetainedContributionComposition`
- `TopologyDeclaredMutationArtifact` now retains accepted Query contribution
  composition directly, and runtime/certification consumers derive naming
  continuity and fallback posture through topology-named helper functions
  instead of a second topo-owned retained semantic product
- retained sequence-alignment validation now clones the accepted Query
  contribution composition only after proving it matches the declared mutation
  sequence, so the accepted post-write seam keeps the real Query product
  instead of a wrapper projection
- machine-checkable application boundary proof now rejects reintroducing the
  removed retained semantic wrapper
- this still does not close phase 4 because the retained accepted aftermath is
  still consumed through topo helpers rather than a fully Query-native
  end-to-end product surface, but it removes another topo-owned retained bag
  and makes the accepted post-write seam more directly Query-owned

Phase 4 certification synopsis / semantic-summary split slice:

- hostile closeout certification no longer mixes declared mutation synopsis
  with accepted semantic aftermath in one semantic-summary product
- `MilestoneThreeHostileScenarioReport` now carries family and digest truth
  through `declared_mutation_synopsis`, while
  `MilestoneThreeScenarioMutationSemanticSummary` keeps only naming continuity
  and fallback aftermath semantics
- accepted and rejected hostile scenario builders now lower both certification
  seams explicitly, mirroring the production split between declared synopsis
  and accepted aftermath instead of preserving a broader certification-owned
  summary bag
- hostile proof that reconstructs accepted replay semantics from step rows now
  rebuilds synopsis and semantic aftermath separately, so certification no
  longer teaches that digest/family truth belongs inside the accepted semantic
  aftermath contract
- hostile report types were then split into a dedicated child module under the
  closeout report boundary so the touched report surface stays under the
  workspace line cap instead of preserving a single oversized certification
  bucket file
- this still does not close phase 4 because retained Query contribution
  composition is still interpreted by topo-side certification/report helpers,
  but it removes another false aftermath contract from the proof surface

Phase 4 retained contribution semantic projection slice:

- retained Query contribution composition is now interpreted through one
  topology-named projection seam,
  `topology_retained_contribution_semantic_projection(...)`, instead of
  scattered direct helper calls for continuity matrix, fallback policy, and
  fallback explanation detail
- the retained interpretation logic moved out of the generic workflow-artifacts
  bucket and into a dedicated `retained_contribution_semantics` boundary, so
  the remaining topo-side interpretation seam is structurally explicit
- declaration-runtime proof, mutation-application runtime proof, declaration-
  entry runtime certification proof, and hostile replay-step lowering now all
  consume that shared projection seam rather than independently decoding the
  retained Query contribution composition
- a machine-checkable boundary test now proves those proof-bearing consumers
  stay on the shared projection seam instead of sliding back to ad hoc retained
  helper interpretation
- this still does not close phase 4 because topo still owns the interpretation
  layer over retained Query contribution composition, but it narrows that debt
  to one explicit seam instead of a spread of local helper calls

Phase 4 retained contribution artifact-lane narrowing slice:

- `TopologyDeclaredMutationArtifact` no longer exports the raw retained Query
  contribution composition to downstream runtime and certification callers
- the application artifact boundary now exposes
  `accepted_query_contribution_semantic_projection()` as the one honest
  post-write retained semantic read lane, so callers do not have a second
  convenient path for reinterpreting the retained Query contribution bag
- hostile replay-step lowering, declaration-runtime proof, mutation-application
  runtime proof, and declaration-entry runtime certification proof now consume
  that artifact-level semantic projection seam instead of reaching through to
  the raw retained contribution composition and then re-projecting it locally
- machine-checkable application boundary proof now rejects reintroducing the
  raw artifact accessor once the narrow semantic lane exists
- this still does not close phase 4 because topo still owns the retained
  interpretation layer after admission, but it removes the last dual-lane
  artifact boundary and makes the next correct edit more obvious than the next
  convenient edit

Phase 4 closeout:

- complete as of the retained contribution-composed and retained contribution
  semantic-projection boundary
- every migrated topology declaration family now enters runtime through the
  contribution-composed Query lane, so topology continuity and fallback posture
  are carried on Query contribution surfaces rather than local execution
  sidecars
- `TopologyDeclaredMutationArtifact` no longer exposes the raw retained Query
  contribution bag and no longer retains any parallel topo-owned continuity,
  fallback, or explanation sidecar products
- public compile-fail proof rejects topo-local declaration-entry stop and
  refusal taxonomy imports, preserving Query ordinary outcomes and Query
  recovery as the only public stop/recovery story
- machine-checkable application boundary proof now treats reintroduction of
  local accepted-aftermath sidecars, raw retained contribution export, or
  contribution-bypassing declaration entry as a phase-4 regression

Phase 5 query-ready authoring-session entry slice:

- workspace-backed kernel construction proof and public contract surfaces no
  longer open a real `ForgeQueryWorkspace` and then bypass it by calling the
  purely local `prepare_primitive_construction_result(...)` or
  `prepare_primitive_construction_outcome(...)` lane
- `PrimitiveConstructionAuthoringSession` now owns sanctioned query-backed
  `prepare_result(...)` and `prepare_outcome(...)` entry helpers, so live
  runtime-proof and public-construction consumers can enter through the same
  authoring front door they already use for authority-chain proof
- branch-preview runtime proof, graph-composition parity, inspection parity,
  projection-consumption receipt, and the public construction contract now all
  prepare results or outcomes through the authoring session instead of mixing a
  workspace-backed setup story with a local result-preparation shortcut
- machine-checkable audit proof now fails if those query-ready runtime and
  public-contract files reintroduce direct local result or outcome preparation
- phase 5 still remains open because kernel still owns the internal
  admitted-scaffold choreography itself; this slice closes a real query-entry
  bypass at the live proof and public-precedent boundary, not the remaining
  internal construction authority seam

Phase 5 scaffold-result seam narrowing slice:

- kernel result assembly no longer retains or passes the broad
  `PrimitiveConstructionScaffold` bag beyond the admitted-scaffold subtree just
  to recover intent digest, scaffold digest, or realization report truth
- `PreparedPrimitiveConstructionAdmittedResultInput` now retains only the
  narrowed result facts downstream result surfaces actually need:
  family, request digest, intent digest, scaffold digest, realization report,
  and the topology admitted handoff
- `CanonicalPrimitiveConstructionArtifact` and
  `PrimitiveConstructionResultAssemblyReport` now bind their digests and
  retained evidence from that narrowed admitted-result seam instead of reaching
  back through `.scaffold()`
- scaffold-only modules now live under the admitted-scaffold subtree rather
  than as peer construction modules, and the boundary proof is split across
  dedicated files so the proof layer stays under the workspace line cap while
  still rejecting peer scaffold declarations or broad scaffold dependencies in
  result/evidence/artifact code
- phase 5 still remains open because kernel still owns the admitted-scaffold
  choreography itself; this slice narrows the downstream result lane and file
  topology, but it does not yet delete the remaining request-to-scaffold
  orchestration authority

Phase 5 move-based admitted result input slice:

- the admitted-scaffold root no longer owns the
  `PreparedPrimitiveConstructionAdmittedResultInput` struct definition or the
  clone-heavy handoff assembly inline; result-input ownership now lives in the
  dedicated `phase_chain/admitted_scaffold/result_input.rs` boundary file
- `PreparedPrimitiveConstructionAdmittedScaffold` now converts into the
  admitted result-input seam by moving owned truth forward, so the topology
  admitted handoff no longer gets cloned merely to cross an internal
  construction phase boundary
- `PrimitiveConstructionScaffold` now yields just the retained
  result-input facts the downstream lane needs instead of preserving dead
  borrow-style accessors from the older clone-oriented flow
- machine-checkable boundary proof now fails if the admitted-scaffold root
  reintroduces the result-input struct definition or the old inline
  `realization_report().clone()` / `topology_query_admitted_handoff().clone()`
  assembly patterns
- phase 5 still remains open because kernel still owns the request-to-scaffold
  choreography itself; this slice makes the surviving admitted-scaffold seam
  more lifecycle-honest and cheaper, but it does not yet delete that remaining
  authority lane

Phase 5 admitted result-input single-output slice:

- the admitted-scaffold root no longer teaches a transient
  `PreparedPrimitiveConstructionAdmittedScaffold` wrapper phase or a separate
  `prepare_primitive_construction_admitted_scaffold(...)` lane; its one honest
  production output is now admitted result input
- geometry admission, scaffold realization, topology admitted-handoff
  preparation, and result-input assembly now collapse directly into
  `prepare_primitive_construction_admitted_result_input(...)`, deleting a fake
  intermediate product that had no live production consumers outside its own
  file
- construction regression proof now fails if the admitted-scaffold root
  reintroduces that deleted wrapper product or its helper lane, and the
  construction tests now prove typed local denial through the surviving
  admitted result-input boundary instead of depending on the broader deleted
  helper
- `PrimitiveConstructionScaffold` is further narrowed to admitted-scaffold
  subtree visibility, so the surviving scaffold bag remains an internal helper
  detail rather than a broader construction phase surface
- phase 5 still remains open because kernel still owns the surviving
  request-to-scaffold choreography itself; this slice removes one more false
  internal phase boundary, but it does not yet rehome the underlying admitted-
  scaffold authority lane

Phase 5 birth-input bridge rehome slice:

- the admitted-scaffold subtree no longer keeps a kernel-local
  `PrimitiveConstructionScaffold` bag or a separate `topology_handoff.rs`
  helper just to re-project realized geometry into lower-layer birth-input
  truth after the fact
- admitted geometry realization now builds
  `PrimitiveConstructionBirthScaffoldInput` directly, so the surviving bridge
  from kernel realization into topology admitted handoff crosses the lower-
  layer birth-input seam rather than a second kernel-owned scaffold product
- `PreparedPrimitiveConstructionAdmittedResultInput` now retains the birth
  scaffold input directly and exposes result truth through that seam, removing
  the duplicate kernel-owned `scaffold_digest` and `realization_report`
  storage that used to sit beside the admitted handoff
- machine-checkable boundary proof now fails if the deleted `scaffold.rs` or
  `topology_handoff.rs` helper files are reintroduced as peer construction
  boundary files, and the surviving construction tests now certify result-input
  realization truth through the retained birth-input seam
- phase 5 still remains open because kernel still owns the request-to-admitted-
  geometry and realization choreography itself; this slice rehomes the live
  bridge product onto the lower-layer birth-input contract, but it does not
  yet delete the remaining kernel-owned orchestration lane

Phase 5 dead local failure-lane collapse slice:

- the live kernel construction path no longer teaches a dead local
  `PrimitiveConstructionPhaseError::SpatialBirth(...)` corridor; rejected birth
  completeness and impossible-attachment truth now survive only through the
  topology admitted-handoff error seam that actually emits them in production
- the live result lane no longer teaches `PrimitiveConstructionArtifactError`
  or `PrimitiveConstructionResultError::Artifact(...)`; canonical artifact
  assembly is direct from admitted result input, and the deleted local
  artifact-assembly failure story now survives nowhere in production because it
  was synthetic-only
- rejection classification, blocking-boundary diagnostics, replay-siege witness
  coverage, and public corpus proof now agree on the narrowed five-boundary
  failure story instead of preserving a sixth artifact-assembly precedent that
  no live construction path could reach
- machine-checkable boundary proof now fails if production construction files
  reintroduce the deleted local spatial-birth or artifact failure lanes, and
  the public compile-fail fixture has been refreshed so the public root export
  quarantine no longer mentions deleted artifact-sidecar names
- phase 5 still remains open because kernel still owns the admitted-scaffold
  choreography itself; this slice deletes dead local failure taxonomy and
  synthetic-only artifact denial precedent, but it does not yet rehome the
  surviving request-to-admitted-geometry orchestration lane

Phase 5 public authoring-session happy-path honesty quarantine slice:

- `PrimitiveConstructionAuthoringSession` no longer teaches
  `prepare_result(...)` or `prepare_outcome(...)` as public query-backed
  construction entry lanes; those local happy-path methods are now crate-
  private because they were still thin wrappers over kernel-local preparation
  rather than genuine `ForgeQueryWorkspace`-owned construction workflow
- the public authoring-session contract is now limited to the honest
  query-readiness surface it actually owns: front-door identity,
  authority-chain reporting, and family admission posture through the
  supported Query runtime families
- the public facade contract files no longer certify query-backed prepared
  result or prepared outcome production through the authoring session, and a
  new compile-fail fixture now proves external callers cannot reach the
  demoted happy-path methods
- machine-checkable construction boundary proof now fails if public contract
  surfaces reintroduce `.prepare_result(` or `.prepare_outcome(` as sanctioned
  public authoring-session precedent
- phase 5 still remains open because kernel still owns the internal admitted-
  scaffold choreography and the crate-private local preparation seam; this
  slice closes a false public precedent, but it does not yet rehome the
  surviving request-to-admitted-geometry authority lane

Phase 5 internal authoring-session wrapper deletion slice:

- `PrimitiveConstructionAuthoringSession` no longer owns crate-private
  `prepare_result(...)` or `prepare_outcome(...)` helpers at all; the session
  now teaches only the honest Query-readiness surface it actually owns:
  workspace identity, authority-chain reporting, and family admission posture
- query-runtime proof files that still need prepared result or outcome truth
  now call the real local kernel preparation seam directly instead of
  instantiating an authoring session just to forward into those same local
  functions through a second wrapper layer
- the construction no-local-runtime-workaround audit now rejects reintroduced
  authoring-session happy-path wrappers in the query-ready runtime files, and
  machine-checkable phase-five boundary proof now fails if either the deleted
  methods or their downstream `.prepare_result(`/`.prepare_outcome(` callsites
  come back in authoring or query-runtime proof files
- the public compile-fail fixture for the authoring session is now stronger:
  external callers fail because the happy-path methods do not exist on the
  session at all, not merely because they were demoted to crate privacy
- phase 5 still remains open because kernel still owns the admitted-scaffold
  choreography and the real local preparation seam itself; this slice deletes a
  second fake internal entry layer, but it does not yet rehome the surviving
  request-to-admitted-geometry authority lane

Phase 5 admitted result-input birth-witness narrowing slice:

- `PreparedPrimitiveConstructionAdmittedResultInput` no longer retains the full
  lower-layer `PrimitiveConstructionBirthScaffoldInput` bag beside the
  topology admitted handoff; after the topology handoff lands, kernel now keeps
  only the birth-side facts it still genuinely needs downstream: scaffold
  digest plus realization report
- result evidence, canonical artifact assembly, and the surviving construction
  tests now read their birth-side truth through that narrowed witness instead
  of reaching back into a retained lower-layer birth-input object after
  topology already owns the birth plan, completeness, and mapping story
- machine-checkable phase-five boundary proof now fails if the admitted
  result-input seam reintroduces `PrimitiveConstructionBirthScaffoldInput` or
  a `birth_input()` accessor as a retained post-handoff kernel bag
- this still does not complete phase 5 because kernel still owns the
  admitted-scaffold choreography and the local birth realization/orchestration
  lane itself; this slice narrows another retained kernel sidecar, but it does
  not yet rehome the surviving request-to-admitted-geometry authority lane

Phase 5 admitted result-input digest-sidecar collapse slice:

- `PreparedPrimitiveConstructionAdmittedResultInput` no longer retains
  `request_digest` or `intent_digest` beside the topology admitted handoff;
  once admitted handoff truth exists, kernel now keeps only the remaining
  post-handoff birth witness facts plus the admitted topology handoff itself
- canonical artifact assembly and result-assembly evidence no longer read
  request or intent digest sidecars from the admitted result-input seam; their
  retained digest story now derives from family, scaffold digest, realization
  truth, and topology admitted-handoff facts instead of preserving local
  pre-handoff identity bags after authority crossing
- machine-checkable phase-five boundary proof now fails if the admitted
  result-input seam reintroduces retained `request_digest` / `intent_digest`
  fields or if result/evidence/artifact code starts depending on
  `.request_digest()` / `.intent_digest()` again
- this still does not complete phase 5 because kernel still owns the
  admitted-scaffold choreography and the request-to-admitted-geometry /
  realization authority lane itself; this slice removes another retained local
  post-handoff identity sidecar, but it does not yet rehome the surviving
  orchestration seam

Phase 5 admitted result-input birth-plan reanchor slice:

- `PreparedPrimitiveConstructionAdmittedResultInput` no longer stores local
  `family` or `scaffold_digest` sidecars after topology admitted handoff;
  those accessors now read through the admitted handoff's retained birth plan
  instead of preserving a second kernel-owned post-crossing copy
- the admitted result-input seam is now down to one genuinely kernel-local
  retained birth witness product, the full realization report, plus the
  topology admitted handoff; family and scaffold identity are treated as
  lower-layer birth-plan truth that kernel reads rather than re-stores
- machine-checkable phase-five boundary proof now fails if the admitted
  result-input seam reintroduces retained `family` / `scaffold_digest`
  sidecars alongside the admitted handoff
- this still does not complete phase 5 because kernel still owns the
  admitted-scaffold choreography and the request-to-admitted-geometry /
  realization authority lane itself; this slice reanchors more surviving truth
  onto the lower-layer birth-plan boundary, but it does not yet delete that
  remaining orchestration seam

Phase 5 transient admitted-geometry deletion slice:

- the admitted-scaffold subtree no longer teaches `AdmittedPrimitiveConstructionGeometry`
  as a local intermediate phase between request admission and lower-layer birth
  input construction; the live path now takes one direct bridge from
  `PrimitiveConstructionRequest` to `PrimitiveConstructionBirthScaffoldInput`
- family-specific request-to-birth-input authority is now split into predictive
  helper files by construction class:
  `closed_solids.rs` for simplex/orthotope/prism/pyramid and
  `planar_constructions.rs` for wire-body / shell-with-hole, with shared scalar,
  placement, and mapping support living in `support.rs`
- the admitted-scaffold root no longer clones request geometry into a transient
  admitted-geometry bag before realization; it computes the admitted intent
  digest, delegates to the direct birth-input bridge, and then crosses the
  topology admitted-handoff boundary
- machine-checkable phase-five boundary proof now fails if the deleted
  transient admitted-geometry type, helper name, or the old
  `request.geometry().clone() -> admitted_geometry -> build_admitted_birth_input(...)`
  protocol reappears
- this still does not complete phase 5 because kernel still owns the
  admitted-scaffold choreography itself, especially the request-to-placement /
  realization authority under that subtree; this slice deletes the fake
  intermediate seam and makes the surviving orchestration boundary more honest,
  but it does not yet rehome that remaining authority lane

Phase 5 request-family sidecar collapse slice:

- `PrimitiveConstructionRequest` no longer stores `family` beside a geometry
  enum whose variant already implies the same family; request family truth now
  derives directly from geometry instead of preserving a parallel request-side
  classifier that downstream admission code had to trust
- request digest assembly now derives family from geometry as well, so the
  request identity lane no longer depends on a separate family field staying in
  lockstep with the geometry variant during placement rewriting or future
  request-shape changes
- the admitted-scaffold birth-input dispatcher still performs the surviving
  request-to-birth-input orchestration, but it now reads one derived family fact
  from the request instead of carrying two independent request-side
  classifications through that lane
- machine-checkable phase-five boundary proof now fails if
  `PrimitiveConstructionRequest` reintroduces a stored `family` sidecar or if
  request placement rewriting starts rebuilding digests from a separate
  `self.family` field again
- this still does not complete phase 5 because kernel still owns the admitted-
  scaffold choreography itself, especially the request-to-placement admission
  and realization authority under that subtree; this slice removes another
  duplicated local classifier, but it does not yet rehome the remaining
  orchestration boundary

Phase 5 admitted-scaffold family-parameter collapse slice:

- the admitted-scaffold birth-input helpers no longer accept a parallel
  `PrimitiveConstructionFamily` parameter beside geometry-specific builder
  roles that already imply the same family; each builder now derives family
  from its own construction-class responsibility instead of trusting an extra
  caller-supplied classifier token
- the birth-input dispatcher no longer computes `let family = request.family()`
  just to thread that same value through every branch; it now dispatches on
  request geometry alone, which makes the surviving request-to-birth-input lane
  more honest about what fact actually selects each lower-layer bridge
- machine-checkable phase-five boundary proof now fails if the admitted-
  scaffold birth-input lane reintroduces the old parallel family-parameter
  protocol instead of deriving family locally from each geometry-specific
  builder role
- this still does not complete phase 5 because kernel still owns the admitted-
  scaffold choreography itself, especially request-to-placement admission and
  realization authority under that subtree; this slice removes another
  duplicated classifier lane, but it does not yet rehome the remaining
  orchestration boundary

Phase 5 realized-birth bridge consolidation slice:

- the admitted-scaffold subtree no longer duplicates lower-layer birth-input
  assembly across `closed_solids.rs` and `planar_constructions.rs`; both
  geometry-family branches now lower into one shared `realized_birth.rs`
  bridge that owns scaffold digest assembly and
  `PrimitiveConstructionBirthScaffoldInput::new_with_realization(...)`
- closed-solid and planar helpers now own only request admission, family-local
  realization, placement embedding, and topology counts; the post-realization
  bridge into the lower-layer birth scaffold contract lives in one explicit
  module instead of two sibling copies
- machine-checkable phase-five boundary proof now fails if sibling admitted-
  scaffold helper files reintroduce a local `build_birth_input(...)` helper or
  call `PrimitiveConstructionBirthScaffoldInput::new_with_realization(...)`
  directly instead of going through the shared realized-birth bridge
- this still does not complete phase 5 because kernel still owns the admitted-
  scaffold choreography itself, especially request-to-placement admission and
  realization authority under that subtree; this slice consolidates the
  surviving post-realization bridge, but it does not yet rehome the remaining
  orchestration lane

Phase 5 shared placement-admission dispatcher slice:

- the admitted-scaffold dispatcher now owns request-level placement admission
  once, before geometry-family branching, instead of letting every closed-solid
  and planar helper re-admit the same placement independently
- closed-solid and planar helper files now receive one shared admitted
  placement and focus on family-local parameter validation, realization,
  placement embedding, and topology counts; request-to-placement admission is
  no longer part of each family helper's responsibility
- machine-checkable phase-five boundary proof now fails if sibling family
  helpers reintroduce local `admit_placement(...)` calls or take
  `PrimitiveConstructionPlacement` directly instead of consuming a shared
  admitted placement from the dispatcher
- this still does not complete phase 5 because kernel still owns the admitted-
  scaffold choreography itself, especially the remaining request-to-realization
  authority under that subtree; this slice narrows that choreography by moving
  one shared admission fact onto the dispatcher, but it does not yet rehome the
  surviving realization authority lane

Phase 5 request-parameter admission dispatcher slice:

- the admitted-scaffold dispatcher now owns request-level parameter admission
  and bit decoding before family branching, instead of letting closed-solid and
  planar helpers validate raw request bits and loop counts locally
- closed-solid and planar helpers now receive admitted scalar, extent, side,
  and loop-count inputs plus shared admitted placement, so their remaining live
  responsibility is family-local realization, placement embedding, and topology
  counts rather than request policing
- machine-checkable phase-five boundary proof now fails if sibling family
  helpers reintroduce raw `f64::from_bits(...)`, scalar/side rejection helpers,
  or shell hole-loop emptiness checks instead of consuming admitted parameters
  from the dispatcher
- this still does not complete phase 5 because kernel still owns the admitted-
  scaffold choreography itself, especially the surviving family-local
  realization authority under that subtree; this slice moves more request
  policing onto the shared dispatcher, but it does not yet rehome the
  remaining realization lane

Phase 5 shared embedding choreography slice:

- the admitted-scaffold subtree no longer lets closed-solid and planar family
  helpers each coordinate their own placement embedding and realized-birth
  assembly; that choreography now lives in one shared `embedded_birth.rs` seam
- closed-solid helpers now choose lower-layer realization source plus topology
  counts, and planar helpers now choose support-plane source, local vertices,
  and topology counts; both families lower through shared embedding helpers
  instead of calling `apply_spatial_placement(...)`, direct planar realization
  reporting, or `RealizedPrimitiveConstructionBirth::new(...)` themselves
- machine-checkable phase-five boundary proof now fails if sibling family
  helpers reintroduce `apply_spatial_placement(...)`,
  `build_direct_realization_report(...)`, or direct
  `RealizedPrimitiveConstructionBirth::new(...)` construction instead of using
  the shared embedding seam
- this still does not complete phase 5 because kernel still owns the admitted-
  scaffold choreography itself, especially the surviving family-local
  realization authority under that subtree; this slice consolidates embedding
  protocol, but it does not yet rehome the remaining realization-choice lane

Phase 5 request-admission seam slice:

- the admitted-scaffold subtree now lowers all remaining request policing
  through one explicit `request_admission.rs` seam instead of smearing it
  across the dispatcher file and the mixed `support.rs` bucket
- `birth_input.rs` is now an honest dispatcher: it admits placement once,
  branches by geometry family, and forwards admitted request facts into the
  surviving realization lane without also defining local scalar/loop/placement
  helper functions
- `support.rs` is now narrowed to cross-layer mapping helpers plus spatial
  family conversion; it no longer mixes request-admission policy with geometry
  failure mapping and lower-layer vocabulary bridging
- machine-checkable phase-five boundary proof now fails if `birth_input.rs`
  reintroduces local request-admission helper definitions or if `support.rs`
  starts carrying request-policing helpers again instead of leaving that work
  on the explicit request-admission seam
- this still does not complete phase 5 because kernel still owns the admitted-
  scaffold realization choreography itself under that subtree; this slice makes
  the surviving request-to-realization boundary structurally honest, but it
  does not yet rehome the remaining realization-choice authority

Phase 5 family-realization subtree slice:

- the admitted-scaffold realization lane no longer hides six family-specific
  strategies inside the category buckets `closed_solids.rs` and
  `planar_constructions.rs`; each family now owns its own realization file
  under `family_realization/`, and the root seam names that subtree explicitly
- this removes a real structural sink: “closed solids” and “planar
  constructions” were grouping by rough similarity rather than one deletable
  responsibility, which made the remaining realization authority harder to
  locate under pressure
- machine-checkable phase-five boundary proof now fails if the admitted-
  scaffold root reintroduces those category-bucket modules instead of the
  explicit family-realization subtree, while the existing family-helper guards
  now audit the per-family files directly
- this still does not complete phase 5 because kernel still owns the remaining
  realization choreography under the admitted-scaffold subtree; this slice
  makes the surviving family-local realization authority structurally honest,
  but it does not yet move that authority off the kernel side

Phase 5 admitted family-request seam slice:

- `birth_input.rs` no longer owns the raw geometry strategy table plus the
  realization dispatch handoff; it now admits placement, lowers one explicit
  admitted family-request product, and passes that to the family-realization
  boundary
- the new `admitted_family_request.rs` seam owns the family-specific request
  admission facts, while `family_realization/mod.rs` owns the realization
  dispatch over those admitted facts; this separates request policing from
  realization strategy choice instead of fusing both into the dispatcher
- machine-checkable phase-five proof now fails if `birth_input.rs`
  reintroduces a raw `PrimitiveConstructionGeometry` match or if the
  family-realization subtree starts redoing request admission instead of
  consuming the admitted family-request seam
- this still does not complete phase 5 because kernel still owns the shared
  embedding and lower-layer birth-bridge choreography under the admitted-
  scaffold subtree; this slice makes the family strategy handoff explicit, but
  it does not yet rehome that remaining realization/bridge authority

Phase 5 birth-scaffold bridge seam slice:

- the admitted-scaffold subtree no longer teaches a transient
  `RealizedPrimitiveConstructionBirth` product or the split
  `embedded_birth.rs` / `realized_birth.rs` bridge protocol; that pair is
  deleted from live code
- shared placement embedding, direct planar realization reporting, scaffold
  digest assembly, and lower-layer `PrimitiveConstructionBirthScaffoldInput`
  lowering now live in one explicit
  `family_realization/birth_scaffold.rs` seam, and the family realization
  helpers return birth-scaffold input directly instead of fabricating an
  intermediate product just to lower it again
- `birth_input.rs` is now an honest dispatcher from admitted placement plus
  admitted family request into lower-layer birth scaffold input; it no longer
  receives a transient realized-birth bag and then performs a second lowering
- machine-checkable phase-five boundary proof now fails if the deleted
  embedded-birth or realized-birth module split returns, while the existing
  family-helper guards still reject local embedding choreography and duplicate
  birth-bridge lowering outside the one explicit bridge seam
- this still does not complete phase 5 because kernel still owns the
  admitted-scaffold realization and lower-layer birth-scaffold authority lane
  itself; this slice deletes a fake intermediate product and consolidates the
  surviving bridge, but it does not yet rehome that remaining internal
  orchestration boundary

Phase 5 admitted family-request admission subtree slice:

- the admitted-scaffold subtree no longer teaches one flat mixed request
  bucket where placement admission, scalar/side/loop admission, and family
  request dispatch all lived together; the old `request_admission.rs` and flat
  `admitted_family_request.rs` file shapes are gone from live code
- placement admission now lives on its own
  `placement_admission.rs` seam, while family-specific request admission facts
  now live under the explicit `admitted_family_request/` subtree with one
  root dispatch file plus per-family admission files and shared scalar support
- `birth_input.rs` still owns the surviving request-to-birth-input
  orchestration, but it now crosses two structurally honest internal seams:
  one for placement admission and one for admitted family-request production,
  instead of dropping both concerns into one mixed local bucket
- machine-checkable phase-five boundary proof now fails if production code
  reintroduces the old mixed request bucket shape or if the placement-admission
  seam starts re-owning family parameter admission instead of leaving that work
  inside the admitted-family-request subtree
- this still does not complete phase 5 because kernel still owns the admitted-
  scaffold realization and lower-layer birth-scaffold authority lane itself;
  this slice makes the request-admission side of that lane structurally honest,
  but it does not yet rehome the remaining realization authority

Phase 5 topology-native construction synopsis correction slice:

- the construction boundary in `worth-topo` no longer depends on
  `worth-spatial` or `worth-geom`; the live receipt, envelope, handoff, and
  admitted-handoff seams now lower from one topology-native
  `TopologyPrimitiveConstructionQueryBirthSynopsis` plus topology-owned
  admitted summary facts instead of importing spatial birth plans, spatial
  completeness reports, or geom realization reports directly
- this corrects a real migration drift: earlier Phase 5 slices had made the
  construction seam query-shaped but had still left it spatial-owned in
  dependency direction, which violated the intended topology boundary
- `worth-kernel` now reclaims the spatial completeness, spatial mapping, and
  realization witness truth as kernel-owned post-admission evidence while
  crossing into `worth-topo` only through the topology-native construction
  synopsis and retained query handoff products
- machine-checkable worth-topo structure proof now fails if `worth-topo`
  reintroduces `worth-spatial` as a production dependency, and the phase-five
  construction boundary proof now fails if the live topology construction seam
  stops exposing the topology-native birth synopsis boundary
- this still does not complete phase 5 because kernel still owns the
  admitted-scaffold realization and lower-layer birth-scaffold authority lane
  itself; this slice corrects the target boundary so the remaining work now
  narrows the real kernel-local orchestration seam instead of polishing a
  spatial-shaped topology boundary

Phase 5 unified family birth-input lane slice:

- the remaining kernel-local admitted-scaffold authority is no longer split
  across parallel `admitted_family_request/` and `family_birth_input/`
  boundaries; the deleted admitted-family-request subtree is gone from live
  code
- `birth_input.rs` now crosses one honest family-owned lane after placement
  admission: `family_birth_input/` owns family-specific request admission,
  realization choice, and lowering into lower-layer
  `PrimitiveConstructionBirthScaffoldInput`
- the subtree naming is now structurally honest under pressure:
  `family_birth_input/` names the surviving responsibility, and
  `scalar_admission.rs` names the shared scalar request-policing seam instead
  of hiding it in a vague support bucket
- machine-checkable phase-five boundary proof now fails if the deleted
  admitted-family-request split returns or if the surviving family-owned lane
  starts teaching the old parallel request/realization protocol again
- this still does not complete phase 5 because kernel still owns the family-
  local realization and lower-layer birth-scaffold authority lane itself; this
  slice collapses the internal split and makes that remaining seam more
  deletable, but it does not yet rehome the surviving kernel-local authority

Phase 5 typed family-parameter admission slice:

- the remaining family-local birth-input files no longer mix raw request-bit
  decoding, stringly parameter-name validation, and realization in one unnamed
  flow; each family file now owns an explicit admitted-parameter step before it
  realizes geometry or lowers into birth scaffold input
- the deleted `family_birth_input/parameter_admission.rs` bucket is replaced by
  the narrower `family_birth_input/scalar_admission.rs` seam, which owns only
  shared scalar and polygon-count admission primitives; family-specific request
  policy now lives on the family files that actually own those parameters
- simplex, orthotope, prism, pyramid, wire-body, and shell-with-hole birth-
  input helpers now read as `admit parameters -> realize geometry -> lower
  birth scaffold`, and machine-checkable phase-five boundary proof fails if the
  old generic parameter bucket or raw `f64::from_bits(...)` decoding returns to
  those family files
- this still does not complete phase 5 because kernel still owns the family-
  local realization and lower-layer birth-scaffold authority lane itself; this
  slice makes that surviving authority more structurally honest, but it does not
  yet move the lane off the kernel side

Phase 5 unified birth-scaffold plan lane slice:

- the shared family birth-scaffold bridge no longer teaches two different entry
  protocols, one for pre-realized support and one for direct planar synthesis;
  the deleted dual-helper surface is replaced by one
  `PrimitiveConstructionBirthScaffoldPlan` lane lowered through one shared
  bridge
- family-local birth-input files now choose their realization mode explicitly:
  realized-support families produce `from_realized_support(...)` plans, while
  direct planar families produce `from_direct_planar_support(...)` plans; the
  shared bridge now owns only placement embedding, direct-planar report
  materialization from the chosen plan, scaffold digest assembly, and lower-
  layer birth-scaffold construction
- machine-checkable phase-five boundary proof now fails if the deleted
  `build_lower_layer_birth_scaffold_input(...)` or
  `build_direct_planar_birth_scaffold_input(...)` bridge protocol returns
- this still does not complete phase 5 because kernel still owns the surviving
  family-local realization and lower-layer birth-scaffold authority lane
  itself; this slice deletes another fake protocol split and makes the
  remaining bridge more honest, but it does not yet move that authority off the
  kernel side

Phase 5 family-birth-input subtree rehome slice:

- the helper files that only serve the surviving `family_birth_input/` lane no
  longer live as peer admitted-scaffold modules; the deleted peer
  `scaffold_geometry.rs`, `support.rs`, and `topology_counts.rs` files are
  replaced by subtree-local `geometry.rs`, `support.rs`, and
  `topology_counts.rs` under `family_birth_input/`
- this makes the filesystem match the actual authority boundary: the remaining
  lower-layer birth-scaffold lane now owns its geometry generation, topology
  count vocabulary, and lower-layer error/family mapping support inside the same
  subtree instead of borrowing them from misleading parent-level helpers
- machine-checkable phase-five boundary proof now fails if `admitted_scaffold`
  root reintroduces those helper modules as peer declarations or if the old
  parent-level support bucket returns as the request-policing or lower-layer
  mapping seam
- this still does not complete phase 5 because kernel still owns the surviving
  family-local realization and lower-layer birth-scaffold authority lane
  itself; this slice removes another false parent seam and makes the remaining
  kernel-local boundary more structurally honest, but it does not yet move that
  authority off the kernel side

Phase 5 explicit lower-layer bridge support seams slice:

- the surviving `family_birth_input/` lane no longer hides lower-layer family
  bridging and geometry failure lowering inside a vague `support.rs` bucket;
  that file is deleted from live code
- lower-layer family translation now lives on one explicit
  `spatial_family_bridge.rs` seam, while geometry, realization, support-plane,
  and placement error lowering now live on one explicit `error_mapping.rs`
  seam under the same `family_birth_input/` subtree
- machine-checkable phase-five boundary proof now fails if the deleted support
  bucket returns or if the new spatial-family bridge seam starts re-owning
  geometry error lowering instead of carrying only lower-layer family
  translation
- this still does not complete phase 5 because kernel still owns the surviving
  family-local realization and lower-layer birth-scaffold authority lane
  itself; this slice removes one more structural bucket inside that lane, but
  it does not yet prove that the lane itself can be deleted

Phase 5 synopsis-owned admitted-handoff sequencing slice:

- kernel no longer assembles the topology receipt -> envelope -> handoff ladder
  itself before crossing the admitted-handoff boundary; that sequencing now
  lives on the topology side through one
  `prepare_primitive_construction_query_admitted_handoff_from_synopsis(...)`
  seam
- the public topology construction contract now certifies the direct
  synopsis-to-admitted-handoff lane as the sanctioned authority-crossing helper
  story for live consumers, while receipt, envelope, and raw handoff remain
  explicit subordinate construction products rather than kernel-owned workflow
  steps
- machine-checkable phase-five proof now fails if the admitted-scaffold root
  starts depending on raw topology handoff construction again instead of using
  the topology-owned synopsis-to-admitted-handoff seam
- this still does not complete phase 5 because kernel still owns the surviving
  family-local realization and lower-layer birth-scaffold authority lane
  itself; this slice moves one more piece of authority-crossing workflow onto
  topology, but it does not yet eliminate the local request-to-birth-input
  start

Phase 5 closeout-proof boundary slice:

- the phase-five/six closeout surface now verifies the actual query-native
  construction boundary instead of inferring phase-five readiness only from
  compound closeout, simplex ladder coverage, and policy-pressure evidence
- `worth-kernel` now exposes a dedicated
  `PrimitiveConstructionPhaseFiveBoundaryCloseoutReport` that machine-checks
  the live phase-five boundary posture: `worth-topo` rejects spatial
  dependency direction, topology public proof still certifies the
  synopsis-owned admitted-handoff seam, kernel consumes that synopsis-owned seam
  instead of sequencing raw topology handoff locally, public queryless
  construction helpers stay demoted, authoring/query-runtime files do not
  reintroduce fake session happy-path wrappers, and the remaining kernel-local
  start is structurally localized to the `family_birth_input/` lane
- the phase-five/six closeout verifier now treats that boundary report as a
  required closeout input and fails with an explicit
  `PhaseFiveBoundaryUnverified` mismatch if the current query-native
  construction boundary drifts while the broader milestone closeout evidence
  still passes
- public certification now exposes both the direct phase-five boundary closeout
  report and the strengthened phase-five/six closeout report, so closeout
  readiness is machine-checkable through the public facade rather than relying
  on scattered boundary tests alone
- this still does not complete phase 5 by itself because the remaining
  question is substantive rather than mechanical: whether the surviving
  `family_birth_input/` lane is the final honest kernel-local boundary or still
  hides one more false seam; this slice makes that decision auditable through
  the closeout surface instead of informal judgment alone

Phase 5 topology-ready birth seam slice:

- the admitted-scaffold root no longer hides spatial birth planning,
  completeness/mapping, and topology synopsis assembly inline after the
  `family_birth_input/` handoff; that post-birth bridge now lives in one
  explicit `topology_ready_birth.rs` seam
- `PreparedPrimitiveConstructionAdmittedResultInput` now builds from that
  named topology-ready birth seam instead of forcing `admitted_scaffold/mod.rs`
  to assemble the admitted-handoff package itself
- the phase-five boundary closeout proof now checks two surviving kernel-local
  seams honestly: `family_birth_input/` for request-to-birth-input lowering,
  and `topology_ready_birth.rs` for the post-birth bridge into the topology
  admitted-handoff lane
- this still does not complete phase 5 because kernel still owns both of those
  internal seams, but it corrects a real overclaim in the closeout evidence
  and makes the remaining boundary more auditably explicit

Phase 5 query-backed construction entry slice:

- workspace-backed kernel construction proof and certification surfaces no
  longer begin from direct local `prepare_primitive_construction_result(...)`
  or `prepare_primitive_construction_outcome(...)` calls when they already
  hold a `ForgeQueryWorkspace`; they now cross the explicit
  `PrimitiveConstructionAuthoringSession` query front door instead
- `PrimitiveConstructionAuthoringSession` once again owns the honest
  query-backed entry lane through `prepare_result(...)` and
  `prepare_outcome(...)`, while public queryless happy-path exports remain
  demoted from the facade root, prelude, and outcome buckets
- query-runtime parity reports, runtime basis, and workspace-backed corpus
  certification builders now materialize prepared result/outcome truth through
  that session seam, and machine-checkable boundary audits now fail if those
  workspace-backed files fall back to direct local preparation helpers again
- public contract and closeout proof now treat the session entry lane as the
  sanctioned query-backed precedent instead of a fake wrapper to suppress,
  which makes the phase-five boundary evidence consistent with the actual
  query-native migration goal
- this still does not complete phase 5 because kernel still owns the surviving
  internal construction start under `family_birth_input/` and
  `topology_ready_birth.rs`; this slice removes the remaining queryless
  workspace-backed bypasses rather than deleting that last local start

Phase 5 family-case subtree slice:

- the surviving `family_birth_input/` seam no longer mixes per-family case
  files and shared helper/lowering seams at one flat directory level
- the six family-specific builders now live under
  `family_birth_input/families/`, while the parent `family_birth_input/`
  boundary keeps only the shared dispatch, scalar admission, geometry,
  topology-count, error-mapping, spatial-family-bridge, and birth-scaffold
  lowering seams
- this is a real structural correction, not just a rename: the directory now
  encodes one dominant classification axis at each level instead of mixing
  family variants and shared mechanics in the same flat bucket

Phase 5 closeout:

- complete once the hostile closeout proof shows that the remaining kernel-local
  construction start is no longer a fake topology-owned workflow, but the final
  honest lower-layer preparation seam required by the crate dependency rules
- that closeout now rests on four aligned facts:
  `worth-topo` rejects `worth-spatial` and `worth-geom` dependencies,
  `worth-spatial` rejects `worth-kernel` dependency direction,
  workspace-backed construction entry crosses the query-backed authoring-session
  front door, and topology-owned authority crossing begins at the
  synopsis-owned admitted-handoff seam rather than a local stepwise
  construction pipeline
- the surviving `family_birth_input/` and `topology_ready_birth.rs` seams remain
  in `worth-kernel`, but they now read as the final honest lower-layer birth
  preparation and post-birth bridge, not as a competing `worth-topo`
  construction-first runtime
- Phase 5 is therefore closed: the `worth-topo` construction migration is now
  query-native and dependency-honest, and the next unfinished migration target
  is Phase 6, Projection And Truth Surfaces
- Phase 9 declared-artifact generic-accessor demotion slice:
  - `TopologyDeclaredMutationArtifact` no longer exports generic Query
    `receipt()` or `inspection()` accessors in live production code; those
    generic aftermath seams now survive only behind test proof access so
    downstream operator/runtime code cannot treat the local declared artifact as
    a competing generic Query aftermath product.
  - the surviving live `TopologyDeclaredMutationArtifact` surface is therefore
    narrower and more honest: declaration synopsis, topology-specific
    materialized aftermath, mutation evidence, execution shape, query anchor,
    and accepted semantic projection remain, while raw Query receipt/inspection
    truth stays on the dedicated Query-owned post-write artifact seam
    underneath.
- Phase 7 Query-owned retained artifact binding entry slice:
  - `forge-query` now owns the combined `materialize_derived_artifact_binding(...)`
    seam for callers that already know they want one exact named retained
    artifact, instead of forcing topo to spell bundle materialization followed
    by artifact binding by hand.
  - `worth-topo` historical retained artifact materialization now crosses
    `materialize_declared_query_surface_binding(...)` and no longer teaches the
    two-step bundle-then-bind choreography inside the topo boundary.
- Phase 7 Query-owned live artifact binding entry slice:
  - `forge-query` now owns the combined `read_live_artifact_binding(...)` seam
    for callers that already know they want one exact named live snapshot pack,
    instead of forcing topo to spell repeated `read(...)` calls followed by a
    caller-owned pack/bind sequence.
  - `worth-topo` historical full-snapshot naming attachment assembly now
    crosses `read_declared_query_surface_binding(...)`, so entity rows and
    persistent-name rows arrive as one Query-owned live artifact binding rather
    than a local pair of direct workspace reads.
