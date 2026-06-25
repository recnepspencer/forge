# Milestone 9.1: Query-Native Runtime Boundary Rollover

## Goal

Move `worth-topo` off stale terminal Query APIs and onto the current
Query-native authority carriers before later touched-graph milestones consume
runtime rows, writes, probes, receipts, validators, invalidation, replay,
conflict, cache, or diagnostics.

## Why This Milestone Exists

Milestone 9 proved the validator and invariant catalog direction, but full
`worth-topo` compilation remains blocked by older Query integration surfaces.
Those surfaces are not harmless rename debt. They preserve the exact authority
model Query removed: terminal JSON rows, raw string aspect paths, caller-built
write commands, local live-view names, and local truth probes.

Milestone 9.1 exists so the next milestones do not build touched-graph routing
on a compatibility layer that undermines Query's native aspect, touch, read,
write, and receipt authority.

## Governing Summaries

- `MENTALITY.md`: protects adversarial-constraint-first design. This milestone
  treats the compile break as a useful hard break, not as friction to smooth
  over with compatibility shims.
- `arch_laws.md`: protects proof-bearing phase transitions and identity
  authority. Query-native carriers must move through Worth as admitted proof
  products, not be reconstructed from strings or JSON.
- `composition_laws.md`: protects named responsibility boundaries. The rollover
  must create narrow native boundary lanes instead of one broad migration file
  full of conversion helpers.
- `domain_structure_laws.md`: protects visible ownership and deletion. Old
  terminal Query surfaces must have a named place to die, and new native
  boundary surfaces must be locatable without grep.
- `perf_laws.md`: protects semantic-delta-bounded execution. Runtime rows,
  writes, probes, and live targets must carry enough native authority and
  counters for later phases to avoid broad rediscovery.
- `touched-graph-roadmap.md`: protects declare-once touched graph routing.
  Milestone 9.1 belongs after validator/invariant catalog routing because that
  work exposed the stale Query boundary, and before invalidation because every
  later registered catalog consumes the same runtime proof chain.

## Adversarial Constraint

Worth must survive the `forge-query` aspect-native hard break without
recreating the removed terminal API under Worth names.

If any ordinary `worth-topo` production path can still execute, certify, probe,
decode, write, or route live state through `external_row`, JSON projection,
raw string aspect paths, caller-constructed Query write commands, local
live-view names, compatibility adapters, or hand-copied probe fields, then the
milestone has failed even if the crate compiles.

## Product Decision Lock

- This milestone is a hard migration, not a shim milestone.
- `forge-query` remains the owner of aspect touches, admitted aspect values,
  backend-admissible mutations, live artifact targets, retained field paths,
  graph-read access plans, receipts, and existing-truth probe fields.
- `worth-topo` may own domain vocabulary that lowers topology meaning into
  native Query/Foundation carriers.
- `worth-topo` may not reintroduce Query's removed terminal surfaces under
  local helper names.
- Terminal JSON is allowed only in named support-document/report codecs, never
  as runtime boundary state.

## Phase Plan

### Phase 1: Terminal Query Residue Inventory And Deletion Ledger

Freeze the current breakage as a typed migration inventory before any
replacement code is written.

**Relevant subsystems**
- `worth-topo::projection::runtime_boundary`
- `worth-topo::certification`
- `worth-topo::derived_topology`
- `worth-topo::topology_operators`
- `forge-query` runtime, memory workspace, mutation, and projection
  consumption facades

**Relevant APIs**
- Removed/stale: `ForgeQueryEntity::external_row`,
  `ForgeQueryEntity::from_external_projection`, `ForgeQueryAspectValue`,
  string aspect-path mutation helpers, caller-constructed
  `ForgeQueryWriteCommand` variants.
- Current/native: `ForgeQueryEntity::from_native_field_values`,
  `ForgeQueryEntity::scalar_value_at`, `AspectValue`,
  `ForgeQueryAspectTouch`, `ForgeQueryAdmittedAspectValue`,
  `ForgeQueryBackendAdmissibleMutation`, `ForgeQueryRetainedFieldPath`.

**Warnings**
- Do not classify stale call sites as "mechanical compile errors." Each one is
  a possible authority leak.
- Do not allow one broad "compat" module to satisfy the inventory. The
  inventory must name the old caller surface, new owner, migration status, and
  deletion trigger.

**Test requirements**
- `Milestone 9.1 Terminal Query Residue Inventory Completeness Test`: scans
  production `worth-topo` source for every removed Query symbol and proves each
  occurrence is classified as migrate, delete, support-codec-only, or
  explicitly blocked by a named upstream gap.
- `Milestone 9.1 Compatibility Shim Rejection Test`: fails if production
  `worth-topo` adds methods or helpers whose names or behavior recreate
  `external_row`, `from_external_projection`, raw string aspect mutation, or
  caller-constructed write command authority.

**Engineering decisions**
- The deletion ledger is a production closeout artifact, not a test-only grep
  script.
- The ledger must distinguish ordinary runtime paths from terminal support
  codecs so future QA can permit reports without permitting runtime JSON.

**Open questions**
- None.

### Phase 2: Topology Vocabulary To Native Query Carrier Boundary

Freeze the only place where Worth topology vocabulary lowers into
Query/Foundation carriers.

**Relevant subsystems**
- `worth-topo::projection::runtime_boundary`
- `worth-topo::topology_operators::touched_graph_basis`
- `forge-foundational` aspect, field, canonical path, and value vocabulary
- `forge-query` aspect touch and mutation authoring vocabulary

**Relevant APIs**
- `AspectKey`
- `FieldKey`
- `CanonicalFieldPath`
- `AspectValue`
- `ForgeQueryAspectTouch`
- `ForgeQueryAuthoredAspectValue`
- `ForgeQueryAdmittedAspectValue`

**Warnings**
- The boundary may parse legacy terminal names only as an ingress from old
  Worth-owned source truth being deleted. It may not expose string paths as
  authority.
- Do not place this under a generic `helpers`, `support`, or `compat` folder.
  The folder name must say it owns topology-to-native Query carrier lowering.

**Test requirements**
- `Milestone 9.1 Native Touch Lowering Parity Test`: proves each topology
  aspect/field formerly represented as terminal text lowers to the exact native
  `ForgeQueryAspectTouch` and `CanonicalFieldPath` consumed by Query.
- `Milestone 9.1 Raw String Authority Denial Test`: proves external callers
  and ordinary operator code cannot satisfy a Query-native boundary by passing
  raw `"topology.kind"` or copied field strings.

**Engineering decisions**
- Worth topology enums and domain vocabulary are allowed to map into native
  Query carriers; they are not allowed to store native Query authority as
  strings.
- Any temporary parser used to migrate old data must be private to the
  migration lane and carry a deletion ledger row.

**Open questions**
- None.

### Phase 3: Native Entity Row Production And Read Decode

Replace terminal row projection with native entity construction and typed field
access.

**Relevant subsystems**
- `worth-topo::projection::runtime_boundary::query_runtime`
- `worth-topo::projection::runtime_boundary::query_support`
- `worth-topo::projection::runtime_boundary::read_execution`
- `worth-topo::derived_topology::materialized_graph`

**Relevant APIs**
- `ForgeQueryEntity::from_native_field_values`
- `ForgeQueryEntity::scalar_value_at`
- `ForgeQueryRetainedFieldPath`
- `AspectValue`
- `CanonicalFieldPath`

**Warnings**
- A local `row_to_json` or `external_projection` helper is a failed migration
  unless it is isolated to terminal report/document codecs.
- Relation and relation-identity decode must become native field/path decode,
  not JSON map traversal hidden behind a new name.

**Test requirements**
- `Milestone 9.1 Native Entity Row Round Trip Test`: builds topology entity,
  relation, and relation-identity rows through native field maps and proves the
  existing read/decode semantics are preserved without terminal JSON.
- `Milestone 9.1 Terminal Row Decode Leakage Test`: source-firewalls ordinary
  production decode paths against `external_row`, `into_external_row`,
  `from_external_projection`, and serde JSON row traversal.

**Engineering decisions**
- Row production and row decode are separate responsibilities. Production owns
  native field construction; decode owns typed observation from native field
  paths.
- `serde_json` may appear only in named terminal support-document/report codec
  lanes, not runtime boundary decode.

**Open questions**
- None.

### Phase 4: Backend-Admissible Write Authority Cutover

Replace Worth's caller-owned write command lowering with Query's
backend-admissible mutation authority.

**Relevant subsystems**
- `worth-topo::projection::runtime_boundary::query_runtime::adapters`
- `worth-topo::projection::runtime_boundary::query_runtime::adapters::write_authority`
- `forge-query::runtime::mutation`
- `forge-query::runtime::backend`

**Relevant APIs**
- `ForgeQueryRuntimeWriteAuthorityAdapter`
- `ForgeQueryBackendAdmissibleMutation`
- `ForgeQueryAdmittedAspectValue`
- `ForgeQueryAspectTouch`
- `ForgeQueryMutationReceipt`
- Query write authority execution receipt helpers exposed by the facade

**Warnings**
- Worth must not construct Query write command variants directly.
- Worth must not infer mutation family or touched aspects from local strings
  when Query has already admitted the backend mutation.

**Test requirements**
- `Milestone 9.1 Backend-Admissible Mutation Parity Test`: executes
  representative insert, update, relation composition, symbolic reference, and
  delete flows from admitted Query mutations and proves the resulting topology
  patch/receipt semantics match the old certified behavior.
- `Milestone 9.1 Raw Write Command Construction Denial Test`: compile-fail or
  source-firewall proof that `worth-topo` cannot construct non-exhaustive Query
  write command variants or lower raw command records as authority.

**Engineering decisions**
- The adapter consumes Query's admitted mutation product as authority. Worth
  may translate admitted aspects into topology storage effects, but may not
  re-admit or reinterpret the command shape.
- Batch writes must preserve Query admission identity and receipt association
  instead of flattening into local command loops without authority.

**Open questions**
- None.

### Phase 5: Existing Truth Probe And Retained Fact Cutover

Replace local probe tuples and retained scalar string lookup with Query-native
probe and retained-field carriers.

**Relevant subsystems**
- `worth-topo::projection::runtime_boundary::query_runtime::adapters::existing_truth_verification`
- `worth-topo::projection::runtime_boundary::query_support`
- `forge-query` existing truth verification and projection consumption
  surfaces

**Relevant APIs**
- `ForgeQueryRuntimeExistingTruthVerificationAdapter`
- `ForgeQueryExistingTruthProbeField`
- `ForgeQueryAdmittedAspectValue`
- `ForgeQueryRetainedScalarFactSet`
- `ForgeQueryRetainedFieldPath`

**Warnings**
- Existing truth verification must compare admitted native aspects against
  native rows. It must not compare terminal JSON values.
- Retained scalar facts must not regain string lookup helpers under Worth-owned
  names.

**Test requirements**
- `Milestone 9.1 Native Existing Truth Probe Test`: proves matching and
  mismatching existing topology truth are reported through
  `ForgeQueryExistingTruthProbeField` with native touch/field identity.
- `Milestone 9.1 Retained String Lookup Rejection Test`: fails if production
  Worth retained-fact consumption uses raw field strings or a recreated
  `field_value("...")` helper.

**Engineering decisions**
- Probe output is a Query proof product. Worth may attach topology diagnostics
  to it, but may not downgrade it to `(String, Value)` tuples.
- Retained fact field paths must be constructed through the same native
  topology vocabulary boundary as runtime rows.

**Open questions**
- None.

### Phase 6: Live Artifact Target And Runtime Source Cutover

Replace view-name string routing with Query-native live artifact targets and
source adapter contracts.

**Relevant subsystems**
- `worth-topo::projection::runtime_boundary::query_runtime::adapters`
- `forge-query` runtime source adapter contracts
- `forge-query` live artifact target vocabulary

**Relevant APIs**
- `ForgeQueryRuntimeSourceAdapter`
- `ForgeQueryLiveArtifactTarget`
- `live_entities_for_target`
- `drain_live_patches_for_target`
- `affected_live_view_targets`
- `ForgeQueryMutationReceipt`

**Warnings**
- Live target routing must not preserve a hidden `BTreeMap<String, String>` as
  the authority for live view identity.
- A live view name may be display metadata. It is not the target authority once
  Query admits `ForgeQueryLiveArtifactTarget`.

**Test requirements**
- `Milestone 9.1 Live Target Routing Parity Test`: declares representative
  topology live views, routes mutation receipts to native live artifact
  targets, and proves entity/patch delivery matches the old intended view
  behavior.
- `Milestone 9.1 Live View Name Authority Denial Test`: fails if ordinary
  production source adapter logic resolves affected views from raw strings
  instead of `ForgeQueryLiveArtifactTarget`.

**Engineering decisions**
- Target declarations are runtime source-adapter state; display names are
  derived metadata.
- Affected-target computation must consume mutation receipt authority and
  declared live targets, not rediscover live routing from local naming
  convention.

**Open questions**
- None.

### Phase 7: Certification And Operator Closeout Cutover

Move certification, topology operator closeout, and scale-pressure proof code
onto the native runtime boundary so tests certify the production path that now
exists.

**Relevant subsystems**
- `worth-topo::certification`
- `worth-topo::topology_operators`
- `worth-topo::validation`
- `worth-topo::validator_invariant_catalog`
- Native runtime boundary lanes created in earlier phases

**Relevant APIs**
- Native row production/decode surfaces from Phase 3
- Backend-admissible write surfaces from Phase 4
- Existing truth probe surfaces from Phase 5
- Live target surfaces from Phase 6

**Warnings**
- Certification may not keep old terminal APIs alive because they are
  "test-only." Tests prove architecture; they cannot rely on the architecture
  being deleted.
- Operator closeout must consume the same native products ordinary runtime
  paths consume.

**Test requirements**
- `Milestone 9.1 Certification Uses Native Runtime Boundary Test`: proves
  representative topology operator, validator catalog, projection closeout,
  and scale-pressure tests execute through the new native lanes.
- `Milestone 9.1 Test-Only Terminal API Leakage Test`: scans certification and
  test-support source for revived terminal row/write/probe helpers unless they
  are isolated in named support-document codecs.

**Engineering decisions**
- Certification fixtures should use domain builders that produce native Query
  carriers, not raw JSON fixture rows.
- Phase 7 is the cutover point where old certification helper paths become
  deletion targets rather than acceptable support scaffolding.

**Open questions**
- None.

### Phase 8: Old Runtime Boundary Deletion And Source Firewalls

Delete or mechanically cap the old terminal Query runtime boundary.

**Relevant subsystems**
- Old `worth-topo` runtime boundary modules that used terminal Query APIs
- New native runtime boundary modules created by this milestone
- Public facade exports
- Compile-fail and source-firewall certification

**Relevant APIs**
- Removed/stale symbol list from Phase 1
- New native carrier facades from Phases 2 through 7

**Warnings**
- Leaving both old and new paths in production is not a safe transition. It is
  a split authority system.
- A capped residue row must name the blocker, owner, count, and removal
  trigger. "Large migration" is not a blocker.

**Test requirements**
- `Milestone 9.1 Terminal Boundary Hard Deletion Test`: fails if old terminal
  runtime boundary modules remain exported, imported by production callers, or
  reachable through public facades.
- `Milestone 9.1 Removed Query Symbol Source Firewall`: scans ordinary
  production and certification source for `external_row`,
  `from_external_projection`, `ForgeQueryAspectValue`, raw aspect-path mutation
  helpers, and local write command construction.

**Engineering decisions**
- Deletion beats compatibility. Any residue that cannot be deleted must be
  isolated by named blocker, owner, exact count, and removal trigger.
- The source firewall belongs to this milestone's closeout and to later
  milestone runner checkpoints so the old boundary cannot quietly return.

**Open questions**
- None.

### Phase 9: Full Crate Compile And Native Boundary Closeout

Close the milestone only when full `worth-topo` verification reaches the new
native boundary instead of stopping at stale Query APIs.

**Relevant subsystems**
- Entire `worth-topo` crate
- Cross-crate `worth-kernel` callers blocked by `worth-topo`
- Touched-graph Milestone 10 and later seeds

**Relevant APIs**
- All Query-native carriers adopted in Phases 2 through 8
- Milestone 9 validator/invariant catalog closeout seed
- Milestone 10 derived invalidation seed

**Warnings**
- Focused line-cap, formatting, and source scans are not enough for this
  milestone. The failure we are closing is full crate compile breakage.
- Do not mark closeout complete while `worth-kernel` focused tests still stop
  before reaching their target because stale `worth-topo` Query usage remains.

**Test requirements**
- `cargo check -p worth-topo --lib`
- `cargo check -p worth-topo --tests`
- representative focused tests for runtime boundary, topology operator
  closeout, validator/invariant catalog, and projection closeout
- `Milestone 9.1 Native Boundary Closeout Test`: asserts zero uncapped removed
  Query API occurrences and proves the closeout seed is consumable by Milestone
  10 without terminal boundary residue.

**Engineering decisions**
- Full crate compile is an acceptance gate, not an optional broad check.
- Later touched-graph milestones may not begin until this closeout either
  passes or records a genuinely external blocker with an owner and follow-on
  milestone.

**Open questions**
- None.

## Must Ship

- A production inventory and deletion ledger for stale Query terminal API
  residue in `worth-topo`.
- A new Query-native topology carrier boundary that lowers Worth topology
  meaning into `AspectKey`, `FieldKey`, `CanonicalFieldPath`, `AspectValue`,
  `ForgeQueryAspectTouch`, and admitted Query mutation/read/probe carriers.
- Native entity row production and read decode without terminal JSON runtime
  state.
- Backend-admissible write authority consumption without caller-constructed
  Query write command variants.
- Existing truth verification and retained fact consumption through native
  Query probe/field carriers.
- Live source adapter routing through `ForgeQueryLiveArtifactTarget`.
- Certification and test support migrated to the same native production path.
- Hard deletion or exact capped residue for old terminal runtime boundary code.
- Full `worth-topo` compile verification.

## Must Preserve

- Milestone 9 validator and invariant catalog semantics.
- Query ownership of admission, aspect touch authority, backend mutation
  authority, live target authority, graph-read access plans, receipts, and
  probe fields.
- Worth ownership of topology truth, topology vocabulary, operator intent,
  topology touched basis, validation legality, and topology diagnostics.
- Terminal JSON support only at true report/document codecs.
- Semantic-delta-bounded execution and counters needed by later invalidation,
  evidence, replay, conflict, cache, public proof, and diagnostic milestones.

## Acceptance Evidence

- Full `cargo check -p worth-topo --lib` passes.
- Full `cargo check -p worth-topo --tests` passes or reaches only unrelated
  explicitly recorded external blockers.
- Source firewalls report zero uncapped production occurrences of removed Query
  terminal APIs.
- Compile-fail tests prove raw strings, terminal rows, caller-built write
  commands, local probe tuples, and local live-view names cannot satisfy the
  native boundary.
- Closeout rows name every deleted, migrated, support-codec-only, or capped
  residue item with owner and removal trigger.

## Sequencing Notes

Milestone 9.1 belongs between Milestone 9 and Milestone 10. Milestone 9 exposed
the stale Query boundary while building validator/invariant catalog routing.
Milestone 10 and later milestones would multiply that defect across
invalidation, evidence lookup, replay, undo, conflict, cache, diagnostics, and
public proof if they began before the native boundary rollover.

Do not skip this milestone by adding compatibility methods back to
`forge-query`. The hard break is the point: downstream Worth code must learn to
consume Query-native authority instead of rebuilding the removed terminal API.
