# Milestone 12: Canonical Semantic Graph Contract For Replay And Undo

## Goal

Make replay scope, undo scope, and transaction scope first-class consumers of
the touched-graph architecture by lowering them from one canonical
semantic-graph contract instead of family-local replay folklore.

## Why This Milestone Exists

Milestone 10 made derived invalidation real. Milestone 11 made evidence lookup
real. Those two milestones already left operational proof products behind:
selected plans, execution receipts, index products, public closeout rows,
workload receipts, stage-index identity, and lookup-consumed workload handoff.

If Milestone 12 treats replay and undo as a fresh subsystem that can rediscover
scope from command names, retained UI intent, broad topology reads, or local
ledger conventions, the touched-graph roadmap breaks exactly where it needs to
be strongest: after multiple proof families already exist.

Milestone 12 therefore belongs here because it is the first post-lookup
milestone that must prove the remaining architecture can still converge into
one semantic graph language instead of becoming "whatever replay needs."

## Governing Summaries

- `MENTALITY.md`: protects hard-problem-first design. The milestone must solve
  replay/undo scope authority before feature-local replay work spreads.
- `arch_laws.md`: protects proof-bearing lowering. Replay and undo must consume
  typed prior receipts and emit typed scope products; they may not rediscover
  meaning during execution.
- `composition_laws.md`: protects lifecycle decomposition. Inventory,
  vocabulary, family catalog, admission, planning, scope product, execution,
  and closeout may not collapse into a single replay helper.
- `domain_structure_laws.md`: protects visible authority boundaries. The tree
  must show shared semantic-graph vocabulary, topology replay scope,
  spatial replay scope, and kernel composition pressure as distinct
  responsibilities.
- `perf_laws.md`: protects semantic-delta-bounded replay breadth. Replay,
  rollback, and transaction proof must scale with touched closure and prior
  receipts, not broad evidence or topology rescans.
- `touched-graph-roadmap.md`: protects the declare-once routing target. The
  roadmap already names Milestone 12 as canonical semantic graph contract work
  over replay scope, undo scope, transaction scope, and transaction receipts.

## Adversarial Constraint

Worth must survive long boolean and future curved-operation chains where a
replay or rollback request arrives after multiple prior proof families already
exist: touched closure, validator receipts, invalidation receipts, evidence
lookup receipts, retained replay workload receipts, diagnostics receipts, and
stage-index identity.

If replay or undo can recover its scope by rescanning broad topology, broad
evidence, retained artifact rows, command names, UI request classes, or local
reconstruction helpers instead of consuming those prior proof products through a
canonical semantic-graph contract, this milestone has failed.

## Product Decision Lock

- Milestone 12 is a cross-crate architecture milestone. It is not a
  `worth-spatial` replay feature and not a `worth-kernel` workflow wrapper.
- Build parallel replay/undo lanes beside old replay helpers, rollback helpers,
  retained replay shortcuts, transaction-scope helpers, and public closeout
  seams before cutover.
- Use parallel migration plus hard deletion. In-place refactoring is allowed
  only inside the new lane after its authority shape exists.
- The milestone must follow the target lifecycle shape:
  `family_catalog -> admitted_input -> selected_plan -> scope_product ->
  execution -> cutover/public_closeout/source_firewall`.
- `worth-schema` owns any new shared semantic-graph vocabulary or identity
  distinctions required for replay and undo.
- `worth-topo` owns topology replay-scope and topology undo-scope lowering from
  topology touched closure plus prior receipts.
- `worth-spatial` owns spatial replay-scope and spatial undo-scope lowering
  from spatial touch authority plus evidence lookup receipts and workload-stage
  evidence.
- `worth-kernel` owns workload composition pressure, transaction boundary
  packets, public closeout pressure, and cross-family proof that no lower
  authority substitute can pass.
- Retained replay workload, diagnostics workload, and public closeout rows are
  inputs or derived products. They are not authority shortcuts.
- Replay scope products, undo scope products, transaction packets, retained
  replay workload receipts, diagnostics receipts, invalidation receipts,
  evidence lookup receipts, and public closeout rows are distinct proof
  products. A later product may consume an earlier product; it may not
  reconstruct it from strings, display labels, broad scans, or local helpers.
- Deletion is part of the milestone. Broad topology replay scans, broad
  evidence replay scans, local rollback shortcuts, replay/undo compatibility
  wrappers, raw scope constructors, and retained-replay authority shortcuts
  must be deleted, capped, or denied before closeout.

## Implicit Requirements Made Explicit

- Covered replay/undo means every ordinary production topology, spatial,
  boolean-chain, retained-replay, transaction-boundary, workload-composition,
  and public-closeout path that can currently recover replay or rollback scope
  from touched proof, invalidation receipts, evidence lookup receipts, retained
  workload receipts, stage identities, or local helper folklore.
- Non-covered replay/undo must be explicitly named as certification-only,
  report/document codec support, test fixture support, or non-ordinary residue.
  It cannot be omitted from inventory because it is "not on the hot path."
- Replay family declarations, admitted replay inputs, selected replay plans,
  replay scope products, undo family declarations, admitted undo inputs,
  selected undo plans, transaction boundary packets, execution receipts,
  diagnostic projections, deletion rows, residue rows, and Milestone 13 seeds
  are separate proof products.
- Density and breadth policy for replay scope belongs in selected plans and
  scope products, not in replay execution.
- Workload composition and public closeout are in-scope consumers. Milestone 12
  is not done if the new scope products exist but those consumers still teach
  older replay semantics.

## Phase Plan

### Phase 1: Replay And Undo Folklore Inventory And Cut Line

Freeze the exact current replay-, undo-, and transaction-scope seed surfaces so
Milestone 12 cannot narrow itself to a fresh lane while old semantics keep
teaching alongside it.

**Relevant subsystems**
- `worth-kernel` workload composition and lookup-consumed workload handoff
- `worth-spatial` evidence lookup public closeout and workload vocabulary
- `worth-topo` invalidation selected plan and execution receipts

**Relevant APIs**
- `WorthWorkload`
- `LookupConsumedWorkloadComposition`
- `current_evidence_lookup_public_closeout()`
- invalidation selected-plan and execution-receipt public closeout surfaces

**Warnings**
- This phase is not a grep-only audit. It must produce typed inventory rows
  with migrate/delete/cap/Query-gap disposition.
- Do not treat retained replay workload and diagnostics workload as "later UX
  concerns." They are already operational seed surfaces in the ordinary path.

**Test requirements**
- Parity test: the same current workload receipt set and lookup handoff produce
  the same inventory classification across reruns, proving the inventory is not
  prose or author memory.
- Leakage test: any current replay-, rollback-, or transaction-scope producer
  omitted from the inventory causes closeout failure.
- Boundary test: a helper that consumes retained replay or diagnostics receipts
  without appearing in the inventory is rejected by source-firewall proof.

**Engineering decisions**
- Inventory rows must classify by responsibility, not by file provenance. Use
  categories such as topology replay scope, spatial replay scope, undo scope,
  transaction boundary, residue, or Query-gap.
- The inventory must explicitly distinguish current authority inputs from
  derived observability inputs. Receipt-backed diagnostics are never a replay
  authority input.

**Open questions**
- Which currently shipped retained replay cases remain certification-only until
  Milestone 12 finishes the ordinary scope product lane?

### Phase 2: Parallel Replay And Undo Family Catalogs

Build the new replay and undo family catalogs beside the old replay helpers and
rollback helpers before any replay execution path is migrated.

**Relevant subsystems**
- new `worth-topo` replay/undo family lanes
- new `worth-spatial` replay/undo family lanes
- `worth-kernel` workload composition and public closeout consumers

**Relevant APIs**
- touched closure proof products
- spatial touch authority and evidence lookup receipt identity
- workload receipt identities and stage-index identity

**Warnings**
- A replay or undo family catalog is source truth for applicability and required
  authority. It is not a callback list.
- Do not let family identity come from command names, stage labels, or old
  replay report names.

**Test requirements**
- Declared-once test: one replay family declaration applies to at least two
  matching replay consumers without editing those consumers.
- Boundary test: a family declaration missing touched-locality applicability,
  required prior-receipt posture, or scope-product posture cannot enter the
  catalog.
- Identity test: raw strings and copied receipt digests cannot mint replay or
  undo family identity.

**Engineering decisions**
- Replay families and undo families are distinct catalogs even where they share
  some admitted inputs.
- Family records must expose required prior-proof classes, locality posture,
  stage-index posture, and whether retained replay workload is required.

**Open questions**
- None.

### Phase 3: Shared Semantic Graph Replay And Undo Vocabulary

Freeze the canonical vocabulary that replay and undo consume so topology and
spatial families lower from the same semantic-graph language.

**Relevant subsystems**
- `worth-schema` shared touched-graph and identity vocabulary
- `worth-topo` touched closure and invalidation receipt identity surfaces
- `worth-spatial` spatial touch authority and evidence lookup receipt identity

**Relevant APIs**
- topology touched basis / touched closure proof types
- spatial touch authority and evidence lookup execution receipt identity
- workload evidence stage and stage-index identity surfaces

**Warnings**
- Do not let this phase become a generic naming cleanup. Every new type must
  carry a proof distinction needed by later lowering.
- Do not reuse a representation-identical digest string where the authority,
  lifecycle, or trust boundary differs.

**Test requirements**
- Equivalence test: replay-equivalent topology and spatial operations emit
  stable replay/undo vocabulary identities across reruns and benign ordering
  noise.
- Denial test: a value with the right bytes but the wrong authority class
  cannot enter replay or undo vocabulary admission.
- Drift test: changing touched locality, required receipt family, or stage-index
  identity changes the admitted replay/undo vocabulary identity.

**Engineering decisions**
- Put shared replay/undo identity distinctions in `worth-schema` under the
  semantic-graph vocabulary target, not in crate-local helper modules.
- The vocabulary must name touched entities, relations, aspects, locality
  scope, prior-receipt identity, transaction-scope claim, and equivalence basis
  separately.
- Distinguish replay scope from undo scope. They may share lifecycle, but they
  are not the same semantic claim.

**Open questions**
- Do transaction-scope claims need one shared basis type with phantom-tagged
  scope kind, or separate proof wrappers for replay and undo?

### Phase 4: Topology Replay Input Admission

Freeze topology replay admission as a proof-bearing lane that admits only
topology touched closure plus declared prior-receipt classes.

**Relevant subsystems**
- `worth-topo` touched closure and invalidation execution
- `worth-topo` replay family catalog

**Relevant APIs**
- topology touched closure proof products
- invalidation execution receipts
- topology query-native runtime boundary receipt identity

**Warnings**
- Do not let topology replay admission quietly accept spatial evidence receipts
  or public closeout rows as substitutes.

**Test requirements**
- Admission parity test: equivalent topology closure and prior receipts admit to
  the same replay input identity.
- Denial test: wrong receipt family, wrong stage-index identity, or foreign
  authority is rejected before plan selection.

**Engineering decisions**
- Admission and planning must be separate phases. Admitted input is its own
  proof product.

**Open questions**
- None.

### Phase 5: Topology Replay Plan Lowering And Scope Product

Freeze topology replay scope as a family-catalog-driven lane that lowers from
topology touched closure plus prior topology-owned receipts.

**Relevant subsystems**
- `worth-topo` touched closure and invalidation selection/execution
- `worth-topo` semantic-graph routing target for replay scope
- `worth-kernel` workload composition pressure for topology-backed replay

**Relevant APIs**
- topology touched closure proof products
- invalidation selected-plan and execution-receipt surfaces
- topology query-native runtime boundary receipt surfaces

**Warnings**
- Do not let topology replay rediscover scope from broad topology reads.
- Do not treat invalidation receipts as sufficient replay scope by themselves;
  replay still needs its own family catalog and selected scope product.

**Test requirements**
- Parity test: the same topology touched closure, invalidation receipt set, and
  stage-index identity produce identical topology replay scope products across
  reruns.
- Denial test: a topology replay request with a mismatched invalidation receipt,
  mismatched stage-index identity, or foreign spatial receipt is rejected
  before scope construction.
- Localization test: replay denial localizes to the exact missing or mismatched
  topology proof input instead of failing as a broad "cannot replay" error.

**Engineering decisions**
- Create a responsibility-named topology replay lane rather than burying replay
  under `derived_topology` or a generic `workload` helper.
- Use the lifecycle shape from Milestones 10 and 11: family catalog, admitted
  input, selected plan, scope product, execution, and closeout.
- Topology replay execution must consume lowered scope products only. No
  executor-side strategy rediscovery is allowed.

**Open questions**
- Which topology replay families can close on ordinary path in Milestone 12,
  and which must remain capped due to missing lower-authority writeback or
  patch application infrastructure?

### Phase 6: Spatial Replay Input Admission

Freeze spatial replay admission as a proof-bearing lane that admits only
spatial touch authority, lookup receipts, and declared retained-workload
dependencies.

**Relevant subsystems**
- `worth-spatial` spatial touch authority and evidence lookup execution
- `worth-spatial` replay family catalog
- `worth-kernel` lookup-consumed workload composition

**Relevant APIs**
- spatial touch authority
- evidence lookup execution receipt identity
- retained replay workload receipt
- workload stage-index identity

**Warnings**
- Do not admit replay from retained replay workload alone.
- Do not reopen raw-row or broad-receipt fallback during admission.

**Test requirements**
- Admission parity test: equivalent spatial touch authority, lookup receipt,
  and stage-index identity admit to the same replay input identity.
- Fallback-denial test: raw evidence rows, broad receipt scans, and
  caller-owned scan fallback are rejected before plan selection.

**Engineering decisions**
- Spatial replay admission must consume the current lookup-consumed workload
  guarantees rather than rebuilding them.

**Open questions**
- None.

### Phase 7: Spatial Replay Plan Lowering And Scope Product

Freeze spatial replay scope as a lane that consumes spatial touch authority,
evidence lookup receipts, workload-stage evidence, and retained replay workload
surfaces without reopening evidence scans.

**Relevant subsystems**
- `worth-spatial` spatial touch authority and evidence lookup execution
- `worth-spatial` workload vocabulary, retained replay workload, and diagnostics
- `worth-kernel` lookup-consumed workload composition

**Relevant APIs**
- spatial touch authority and evidence lookup execution receipt surfaces
- `RetainedReplayWorkloadReceipt`
- `WorthWorkload` and `LookupConsumedWorkloadComposition`
- evidence lookup public closeout seed and source-firewall surfaces

**Warnings**
- Do not let retained replay become an authority shortcut over evidence lookup
  receipts.
- Do not let replay read scope be recovered by broad evidence or broad receipt
  scans after Milestone 11 already denied those fallbacks.

**Test requirements**
- Replay-honesty test: the same spatial touch authority, lookup execution
  receipt, retained replay workload receipt, and stage-index identity produce
  identical spatial replay scope products across reruns.
- Fallback-denial test: any path that tries to admit replay from raw evidence
  rows, broad receipt scans, or caller-owned scan fallback is rejected with the
  same zero-scan guarantees already enforced by lookup-consumed workload
  composition.
- Drift test: retained replay workload drift against lookup receipt identity or
  stage-index identity is rejected before spatial replay execution begins.

**Engineering decisions**
- Spatial replay scope must be modeled as a sibling lifecycle lane to evidence
  lookup, not as an extension method on evidence lookup execution receipt.
- Retained replay workload is an admitted input to spatial replay families only
  where the family catalog declares that dependency explicitly.
- Diagnostics workload may project replay explanation later, but it cannot
  widen replay authority in Milestone 12.

**Open questions**
- Which spatial replay families require both retained replay workload and
  lookup-consumed workload handoff, and which can remain lookup-receipt-only?

### Phase 8: Undo Family Admission, Planning, And Scope Product

Freeze undo scope and transaction boundary packets as their own proof products
so rollback and transaction closeout consume the same semantic-graph contract
as replay without collapsing into it.

**Relevant subsystems**
- `worth-topo` truth-adjacent effect and invalidation proof surfaces
- `worth-spatial` workload evidence ledger and stage receipts
- `worth-kernel` workload composition and transaction-facing public proof

**Relevant APIs**
- workload evidence ledger stage-link surfaces
- invalidation execution receipts
- evidence lookup execution receipts
- workload stage support and stage-index identity surfaces

**Warnings**
- Undo is not a UI command reversal and not a re-run of replay in reverse.
- Transaction boundary packets must expose enough proof for downstream
  inspection without requiring producer re-query.

**Test requirements**
- Convergence test: replay scope and undo scope derived from the same admitted
  semantic-graph inputs agree on touched locality and prior-receipt basis where
  the domain semantics require parity.
- Hidden-mutation test: a mutation outside the admitted undo scope fails
  transaction closeout with a localized proof gap.
- Envelope test: transaction boundary packets remain self-describing and can be
  interpreted without reopening topology or evidence authority.

**Engineering decisions**
- Model undo-family catalog entries separately from replay-family entries even
  where they share admitted input or selected-plan structure.
- Transaction boundary packets must bind touched digest, stage-index identity,
  invalidation receipt identity, evidence lookup receipt identity, replay scope
  identity, undo scope identity, and support posture into one ordinary packet.
- If a rollback path cannot be made ordinary in Milestone 12, it must become
  capped residue with owner, blocker, and removal trigger rather than being
  silently routed through a certification helper.

**Open questions**
- Which rollback cases require a later lower-authority patch-application
  milestone before they can become ordinary?

### Phase 9: Transaction Boundary Packet

Freeze the transaction packet that downstream consumers use to understand one
replay/undo boundary without reopening producers.

**Relevant subsystems**
- `worth-kernel` workload composition and public proof pressure
- `worth-topo` replay/undo proof products
- `worth-spatial` replay/undo proof products

**Relevant APIs**
- workload evidence ledger stage-link surfaces
- replay scope identities
- undo scope identities
- invalidation and lookup receipt identities

**Warnings**
- A transaction packet is not a debug report.
- Do not let the packet omit counters or identity fields that later conflict or
  cache milestones would need.

**Test requirements**
- Envelope test: packet interpretation requires no producer re-query.
- Drift test: mismatched replay/undo scope or receipt identities reject packet
  assembly.

**Engineering decisions**
- Packet must bind touched digest, stage-index identity, replay scope,
  undo scope, invalidation receipt identity, evidence lookup receipt identity,
  and support posture into one ordinary artifact.

**Open questions**
- None.

### Phase 10: First Replay And Undo Migration Slice

Migrate one ordinary vertical replay/undo slice through the new lanes before
attempting broad sweep cutover.

**Relevant subsystems**
- one covered topology or spatial replay consumer
- one covered undo or transaction consumer
- `worth-kernel` workload composition

**Relevant APIs**
- admitted replay/undo scope products
- transaction boundary packet
- selected current ordinary consumers from Phase 1 inventory

**Warnings**
- Do not sweep broadly before one slice proves parity or stronger denial.
- Do not pick a certification-only slice just because it is easy.

**Test requirements**
- Vertical parity test: migrated slice produces the same or stronger replay
  boundary proof than the old path.
- Authority test: old helper cannot satisfy the new consumer once cut over.

**Engineering decisions**
- Pick the first migrated slice from a real ordinary consumer using current
  seed surfaces, not a synthetic harness-only path.

**Open questions**
- None.

### Phase 11: Workload Composition And Consumer Sweep

Freeze the ordinary Milestone 12 cutover so replay and undo can only enter
through the new semantic-graph lanes and current seed surfaces cannot continue
teaching old semantics beside them.

**Relevant subsystems**
- `worth-kernel` workload composition and public closeout pressure
- `worth-spatial` public closeout and source firewall
- `worth-topo` replay-scope and undo-scope closeout products

**Relevant APIs**
- `WorthWorkload`
- `LookupConsumedWorkloadComposition`
- evidence lookup public closeout and source firewall reports
- new replay/undo closeout and milestone-thirteen seed surfaces

**Warnings**
- This phase must cut over ordinary entry. It is not enough to leave the new
  lanes unused but well tested.
- Do not let public proof or diagnostics masquerade as replay or undo authority
  because the ordinary lane remains inconvenient.

**Test requirements**
- Cutover parity test: an ordinary covered replay/undo path routes through the
  new scope products and produces the same or stronger boundary proof as the
  pre-cutover path without reviving broad scans.
- Firewall test: old replay helpers, broad topology rediscovery, broad evidence
  rediscovery, raw receipt admission, and local rollback shortcuts fail
  compile-fail or source-firewall checks.
- Residue-honesty test: every non-ordinary replay/undo remainder is counted,
  owned, blocked, and attached to a removal trigger.

**Engineering decisions**
- Emit a Milestone 13 seed that names admitted replay/undo scope products,
  transaction boundary packets, residue ledger, and source-firewall proof
  without claiming conflict or cache completion.
- Make the ordinary public closeout path consume the same replay/undo proof
  products the workload composition lane consumes.
- Closeout must classify all remaining replay/undo consumers as migrated,
  deleted, capped residue, or Query-gap. "Future cleanup" is not a class.

**Open questions**
- None.

### Phase 12: Source Firewalls, Hard Deletion, And Residue Caps

Delete or cap the displaced replay/undo folklore and deny reintroduction.

**Relevant subsystems**
- old replay helpers and rollback helpers
- public raw constructor surfaces
- source-firewall and compile-fail certification bands

**Relevant APIs**
- source-firewall reports
- compile-fail contracts
- residue and deletion ledgers

**Warnings**
- Residue is not permission to keep convenient old helpers.
- Do not leave "temporary" compatibility wrappers uncapped.

**Test requirements**
- Reintroduction test: adding a broad replay scan, broad evidence scan, raw
  scope constructor, or rollback shortcut fails compile-fail or firewall proof.
- Residue-honesty test: every remaining non-ordinary path has owner, blocker,
  cap, and removal trigger.

**Engineering decisions**
- Old-path deletion is part of the milestone closeout, not a follow-up cleanup.

**Open questions**
- None.

### Phase 13: Public Closeout And Milestone 13 Seed

Publish the Milestone 12 closeout only after ordinary replay/undo consumers use
the new scope products and the old replay/undo authority paths are deleted,
capped, or denied.

**Relevant subsystems**
- `worth-kernel` public closeout pressure
- `worth-topo` replay/undo closeout
- `worth-spatial` replay/undo closeout

**Relevant APIs**
- replay/undo closeout products
- transaction boundary packet
- source-firewall reports
- Milestone 13 seed surfaces

**Warnings**
- Do not claim conflict, cache, or public explainer completion here.
- Do not let the seed claim more than admitted replay/undo proof.

**Test requirements**
- Closeout proof test: ordinary public closeout is built only from real replay
  and undo proof products plus residue/firewall rows.
- Seed test: Milestone 13 seed carries enough replay/undo and transaction
  identity to start conflict work without topology or evidence rescans.

**Engineering decisions**
- Emit a Milestone 13 seed with admitted replay/undo scope products,
  transaction packet identity, residue ledger, and firewall proof.

**Open questions**
- None.

## Must Ship

- typed inventory of current replay-, retained-replay-, rollback-, and
  transaction consumers
- shared semantic-graph replay/undo vocabulary and identity distinctions
- topology replay-scope family lane
- spatial replay-scope family lane
- undo-scope family lane and transaction boundary packet
- workload-composition cutover, source firewall, and Milestone 13 seed

## Must Preserve

- touched-graph authority remains the only source of locality and changed
  meaning
- Milestone 10 invalidation receipts and Milestone 11 evidence lookup receipts
  remain prior-proof inputs, not recertified or substituted authority
- stage-index identity, zero raw-row scan fallback, zero broad-receipt scan
  fallback, and zero caller-owned scan fallback remain ordinary-path guards
- diagnostics and public proof remain derived observability products, not
  execution authority

## Acceptance Evidence

- compile-fail tests proving wrong-authority digests, raw receipts, or local
  helpers cannot enter replay or undo admission
- replay-equivalence tests proving stable scope identity under rerun and benign
  ordering noise
- hidden-mutation tests proving undo scope localizes missing authority
- source-firewall tests proving broad topology/evidence rediscovery does not
  survive the cutover
- closeout inventory showing every in-scope seed surface is migrated, deleted,
  capped, or Query-gap
- milestone-thirteen seed showing conflict work can start from admitted replay
  and undo proof products without rescanning topology or evidence

## Sequencing Notes

- This milestone belongs immediately after Milestone 11 because evidence lookup
  receipts are now real and replay/undo is the first remaining family that can
  either honor or destroy that architecture.
- It belongs before Milestone 13 because conflict and batch admission must
  reason over admitted replay/undo and transaction-scope products, not over
  pre-scope local conventions.
- It should not attempt Milestone 13 conflict posture, Milestone 14
  cache/equivalence closure, or Milestone 15 public explainer unification,
  except where Milestone 12 must emit the typed seeds those later milestones
  consume.
