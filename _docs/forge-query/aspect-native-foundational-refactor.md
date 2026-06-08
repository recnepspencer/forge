# Forge Query Aspect-Native Foundational Refactor

## Purpose

Define the actual end-state for making `forge-query` aspect-native in the same foundational sense that now governs `forge-runtime-bridge` and `forge-relational`, while preserving Query's role as the daily-driver authoring and read surface.

## Why this doc exists

The current public Query mutation facade is already safer than the old payload-first surface. `workspace.insert(...)`, `workspace.update(...)`, `workspace.delete(...)`, batch builders, and preview-facing authoring all teach aspect-oriented authoring instead of raw write commands. That work was the right stabilization move, but it did not finish the deeper migration.

Today, many core Query internals still treat authority, mutation lowering, materialization, and projection consumption as string-path plus `serde_json::Value` problems:

- `crates/forge-query/src/runtime/surface/mutation/command.rs` still exposes `ForgeQueryWriteCommand::UpdateAspect { aspect_path: String, value: serde_json::Value }`.
- `crates/forge-query/src/runtime/mutation/aspect.rs` still centers `ForgeQueryAspectValue` on string paths and JSON lowering.
- `crates/forge-query/src/runtime/workspace_queries.rs` still returns `Vec<serde_json::Value>` for derived materialization.
- `crates/forge-query/src/runtime/computed/surface.rs` still stores computed rows and patch payloads as `serde_json::Value`.
- `crates/forge-query/src/projection_consumption/extraction/row_like.rs` and `aspect_value_projection.rs` still translate authoritative aspect values into consumption JSON as a normal internal representation.
- `crates/forge-query/src/memory_workspace/mod.rs` still stores external row projections as `serde_json::Value` beside aspect maps and exposes dotted-path JSON lookup.

So the open question is no longer "does Query mention aspects?" The open question is "which concrete subfolders and files still treat JSON or path strings as authority carriers, and what is the correct foundational target for each migration zone?" This doc answers that question by zone.

## Governing summaries

`MENTALITY.md`

Solve the hard substrate mismatch instead of polishing the comfortable facade. The migration must remove authority leakage and false-native seams even when the public API already feels nicer.

`arch_laws.md`

Authority must travel through explicit proof-bearing types. Construction, validation, lowering, and mutation admission cannot rely on ambient strings or loosely shaped payload carriers.

`composition_laws.md`

The plan must match the code's responsibility boundaries. We should not declare "remove JSON from Query" as one monolithic task; we should rewrite each migration zone according to its real authority role.

`domain_structure_laws.md`

Folder topology should expose domain boundaries. Query is large enough that top-level folders hide important authority differences, so this spec must name subfolders and representative files.

`perf_laws.md`

The rewrite should lower once, validate once, and avoid repeated encode/decode churn between foundational values and JSON. Cheap-looking APIs must not hide expensive re-materialization or re-validation loops.

`forge_query_vision.md`

Query is supposed to be the platform's daily-driver authoring and read layer. Its read, write, preview, inspection, and live surfaces should share one coherent ontology instead of mixing aspect truth with ad hoc JSON carriers.

`forge_query_roadmap.md`

Query should declare intent once, lower once, and execute against canonical truth without stealing authority from relational, bridge, or foundational subsystems. That requires a stricter internal authority model than the current public DX freeze delivered.

`test-requirements.md`

This migration is not done when the happy-path facade compiles. It is done when certification, replay, parity, and adversarial harnesses all prove that foundational-native carriers are the actual substrate.

`aspect-api-finalization-plan.md` and `aspect-api-finalization-closeout.md`

These docs intentionally froze the public mutation API before the deeper substrate rewrite. They are prerequisites, not proof of completion.

## Adversarial constraint

Assume the current Query API is only "aspect-shaped" unless a surface can prove that:

1. authority enters through foundational contracts, locators, validated values, and authoritative patches;
2. internal mutation and materialization steps keep that truth in foundational carriers instead of re-expanding it into JSON;
3. production Query contains no JSON authority or projection shim except for a small, explicitly named allowlist that is justified by a hard external contract.

If a migration zone fails any of those three tests, it is not done.

## What "foundational-native" means for Forge Query

Forge Query does not need to become a thin alias over raw `forge-foundational`, but its authority-bearing core must.

For this refactor, "foundational-native" means:

- aspect identity is represented by foundational keys, locators, field locators, or canonical field paths, not by free-form dotted strings as the source of truth;
- aspect values are represented by `AspectValue`, `StructAspectValue`, validated aspect artifacts, or authoritative state/patch artifacts, not by `serde_json::Value`;
- mutation intent lowers into authoritative patch/state vocabulary before runtime execution;
- inspection, preview, and read materialization expose clearly separated native truth surfaces versus external projections;
- JSON is banned from production Query by default. Any exception must be named, justified, and treated as an external contract rather than a general compatibility lane.

## JSON ban and exception policy

The migration target is not "JSON at the edges." The migration target is "no production JSON in Query unless an exception is explicitly approved."

The current likely exception candidates are temporary and must be re-validated during implementation:

- `crates/forge-query/src/aspect_field_authoring/external_json_ingress.rs`
- `crates/forge-query/src/aspect_field_authoring/external_json_projection.rs`

Those files may survive only if Query truly owns an external JSON contract that cannot move elsewhere. If they are only legacy convenience shims, they should be deleted or moved out of production Query. Tests may temporarily retain JSON fixtures because the crate is large, but those tests must be marked as legacy coverage or compatibility-debt tests instead of teaching JSON as the normal authoring model.

## Migration zone register

This register intentionally uses subfolders and representative files. Top-level folders such as `runtime/` and `domain_capabilities/` are too broad to be actionable work units.

### Zone 1: `runtime/surface/mutation/`

Representative files:

- `crates/forge-query/src/runtime/surface/mutation/command.rs`
- `crates/forge-query/src/runtime/surface/mutation/write_receipt/`

Current state:

- `ForgeQueryWriteCommand::UpdateAspect` still carries `aspect_path: String` and `value: serde_json::Value`;
- delete variants still carry `touched_aspect_paths: Vec<String>`;
- receipt helpers reflect the same legacy command shape.

Required end state:

- expert write commands carry native mutation targets and foundational values or patches;
- string aspect paths exist only as caller-facing authoring sugar before command construction;
- write receipts expose native aspect operation evidence.

Migration class: first production blocker.

### Zone 2: `runtime/mutation/`

Representative files:

- `crates/forge-query/src/runtime/mutation/aspect.rs`
- `crates/forge-query/src/runtime/mutation/lowering.rs`
- `crates/forge-query/src/runtime/mutation/probe.rs`
- `crates/forge-query/src/runtime/mutation/metadata.rs`
- `crates/forge-query/src/runtime/mutation/assertion.rs`

Current state:

- `ForgeQueryAspectValue` is still a string-path plus JSON carrier;
- mutation lowering and assertion diagnostics serialize JSON values for evidence;
- existing-truth probes still reason in `(String, Value)` field pairs;
- metadata uses JSON as an open-ended bag.

Required end state:

- introduce native desired-aspect and asserted-aspect carriers;
- lower into foundational patch/state vocabulary once;
- represent probe results and assertion evidence in native aspect facts;
- either remove JSON metadata or replace it with typed metadata entries.

Migration class: first production blocker, paired with Zone 1.

### Zone 3: `runtime/backend/`

Representative files:

- `crates/forge-query/src/runtime/backend/contracts.rs`
- `crates/forge-query/src/runtime/backend/bridge_backed.rs`

Current state:

- backend contracts still return `Vec<(String, serde_json::Value)>` for existing-truth probes;
- bridge-backed execution consumes the legacy runtime mutation carrier.

Required end state:

- backend contracts return native existing-truth facts;
- bridge-backed execution accepts native mutation carriers directly;
- no backend trait makes JSON part of the production authority contract.

Migration class: production contract rewrite.

### Zone 4: `runtime/computed/` plus derived materialization root files

Representative files:

- `crates/forge-query/src/runtime/computed/surface.rs`
- `crates/forge-query/src/runtime/computed/routing.rs`
- `crates/forge-query/src/runtime/workspace_queries.rs`
- `crates/forge-query/src/runtime/runtime_reads_programs.rs`
- `crates/forge-query/src/runtime/retained_rows.rs`
- `crates/forge-query/src/runtime/surface/derived_materialization_result.rs`

Current state:

- derived materialization defaults to `serde_json::Value`;
- computed rows and patch payloads are JSON rows;
- retained row decoding still assumes JSON materialization.

Required end state:

- derived views retain native row/value artifacts;
- materialization result types no longer expose JSON rows as the default production shape;
- retained row decoding operates on typed/native rows rather than JSON.

Migration class: second production blocker.

### Zone 5: `runtime/effect/`

Representative files:

- `crates/forge-query/src/runtime/effect/delivery.rs`
- `crates/forge-query/src/runtime/effect/routing.rs`
- `crates/forge-query/src/runtime/effect/declaration.rs`

Current state:

- effect delivery payloads and handles default to `serde_json::Value`;
- routing outcomes synthesize JSON payloads.

Required end state:

- effect delivery uses typed/native effect outputs;
- mutation effects carry native mutation intent, not JSON desired-aspect state;
- any JSON-like delivery value must be test-only or allowlisted with a hard external reason.

Migration class: production delivery rewrite after mutation carriers are fixed.

### Zone 6: `runtime/surface/` retained scalar and live surfaces

Representative files:

- `crates/forge-query/src/runtime/surface/retained_scalar_facts.rs`
- `crates/forge-query/src/runtime/surface/retained_scalar_alignment.rs`
- `crates/forge-query/src/runtime/surface/live.rs`
- `crates/forge-query/src/runtime/surface/derived_artifact_binding.rs`
- `crates/forge-query/src/runtime/surface/read_receipt_support.rs`

Current state:

- retained scalar facts and alignment still store JSON values;
- live view generic defaults still use `serde_json::Value`;
- read receipt support serializes external rows.

Required end state:

- retained facts and alignment use native scalar/aspect facts;
- live defaults stop teaching JSON as the normal row type;
- read receipt support digests native rows or explicitly typed external artifacts.

Migration class: production surface cleanup after Zones 4 and 5.

### Zone 7: `projection_consumption/consumed/`

Representative files:

- `crates/forge-query/src/projection_consumption/consumed/facts.rs`
- `crates/forge-query/src/projection_consumption/consumed/set.rs`

Current state:

- consumed fact values, member identities, and grouping values are `serde_json::Value`;
- fact digests canonicalize JSON.

Required end state:

- consumed facts use native value carriers;
- digesting is based on native canonical value encodings;
- JSON is absent from production consumed fact storage.

Migration class: read-side production blocker.

### Zone 8: `projection_consumption/extraction/`

Representative files:

- `crates/forge-query/src/projection_consumption/extraction/row_like.rs`
- `crates/forge-query/src/projection_consumption/extraction/aspect_value_projection.rs`
- `crates/forge-query/src/projection_consumption/extraction/grouped.rs`
- `crates/forge-query/src/projection_consumption/extraction/query_context.rs`

Current state:

- extraction projects relational, bridge, and Query rows into JSON before fact extraction;
- `aspect_value_projection.rs` is a production JSON shim from native values to JSON;
- grouped and query-context extraction still expect JSON field values.

Required end state:

- extraction accepts native projected-field rows;
- `aspect_value_projection.rs` is deleted, moved out of production Query, or allowlisted with a hard external-contract reason;
- grouped/query-context extraction uses native identity and grouping values.

Migration class: read-side production blocker, paired with Zone 7.

### Zone 9: `projection_consumption/certification/` and `projection_consumption/tests/`

Representative files:

- `crates/forge-query/src/projection_consumption/certification/oracle_comparison_terms.rs`
- `crates/forge-query/src/projection_consumption/tests/phase_four_support.rs`
- `crates/forge-query/src/projection_consumption/tests/phase_four_remaining_sources.rs`
- `crates/forge-query/src/projection_consumption/tests/phase_four_query_context.rs`

Current state:

- certification oracle comparison still canonicalizes JSON facts;
- tests use JSON fixtures for rows and expected values.

Required end state:

- certification compares native canonical terms;
- any remaining JSON fixtures are marked as legacy debt and isolated from native proof tests.

Migration class: certification/test cleanup after Zones 7 and 8.

### Zone 10: `memory_workspace/`

Representative files:

- `crates/forge-query/src/memory_workspace/mod.rs`
- `crates/forge-query/src/memory_workspace/workspace.rs`
- `crates/forge-query/src/memory_workspace/external_projection.rs`
- `crates/forge-query/src/memory_workspace/tests.rs`

Current state:

- `ForgeQueryEntityRow` mixes native aspect maps with JSON external projections;
- `external_projection.rs` exists only to build JSON objects;
- tests author aspect values through `json!`.

Required end state:

- remove production external projection rows or move them behind an approved external-contract allowlist;
- expose native entity rows and native aspect lookup as the primary workspace representation;
- keep JSON tests only as marked legacy coverage while migration volume is being paid down.

Migration class: small but important production cleanup.

### Zone 11: `aspect_field_authoring/`

Representative files:

- `crates/forge-query/src/aspect_field_authoring/external_json_ingress.rs`
- `crates/forge-query/src/aspect_field_authoring/external_json_projection.rs`
- `crates/forge-query/src/aspect_field_authoring/keys.rs`
- `crates/forge-query/src/aspect_field_authoring/declarations.rs`

Current state:

- the key/declaration helpers are useful native authoring support;
- the external JSON ingress/projection files are legacy shim candidates.

Required end state:

- preserve native key and declaration helpers;
- delete, move, or explicitly allowlist the external JSON files;
- prevent other zones from depending on these files as a general migration escape hatch.

Migration class: exception audit with deletion as the default answer.

### Zone 12: `domain_capabilities/authoring/` and `domain_capabilities/payloads/`

Representative files:

- `crates/forge-query/src/domain_capabilities/authoring/workflow.rs`
- `crates/forge-query/src/domain_capabilities/payloads/workflow_semantics.rs`

Current state:

- workflow authoring still accepts `desired_aspect_fields_external_json`;
- workflow semantics digest that JSON state.

Required end state:

- workflow authoring accepts native desired-aspect carriers;
- workflow semantics digest native mutation intent;
- contribution payload vocabulary remains only where it means proof or capability payload, not JSON truth.

Migration class: targeted production workflow cleanup.

### Zone 13: `effect_lifecycle/` tests and certification

Representative files:

- `crates/forge-query/src/effect_lifecycle/tests/support.rs`
- `crates/forge-query/src/effect_lifecycle/tests/execution/`
- `crates/forge-query/src/effect_lifecycle/tests/batch/`
- `crates/forge-query/src/effect_lifecycle/certification/`

Current state:

- most JSON here is test/certification fixture authoring;
- those fixtures mirror the legacy runtime/effect mutation substrate.

Required end state:

- update primary certification fixtures to native mutation intent;
- mark any retained JSON tests as legacy migration debt;
- avoid paying this zone down before Zones 1, 2, and 5 give it native APIs to use.

Migration class: test and certification follow-up.

### Zone 14: `intent_admission/certification/` and `lower_runtime_routing/certification/`

Representative files:

- `crates/forge-query/src/intent_admission/certification/fixtures/runtime.rs`
- `crates/forge-query/src/intent_admission/certification/fixtures/effect.rs`
- `crates/forge-query/src/lower_runtime_routing/certification/surface/fixtures/`

Current state:

- certification fixtures still construct `ForgeQueryWriteCommand::UpdateAspect` and `serde_json::Value` rows;
- runtime fixture probes still return string/JSON field pairs.

Required end state:

- certification fixtures use the native write and probe carriers;
- legacy JSON fixture cases are explicitly marked and isolated.

Migration class: certification follow-up after Zones 1, 2, 3, and 5.

### Zone 15: broad test suites under `runtime/tests`, `domain_capabilities/tests`, and related closeout fixtures

Current state:

- this is the largest JSON volume by count;
- much of it exists because production APIs still force or reward JSON-shaped fixtures.

Required end state:

- only update tests opportunistically until production APIs are replaced;
- once a production zone is migrated, update the tests in that zone to native fixtures;
- keep a known debt list for tolerated JSON tests so the exception does not silently become policy.

Migration class: final cleanup and debt tracking.

## Migration phases

### Phase 1. Lock the target ontology and boundary rules

Define the crate-wide rule set before editing code:

- list the exact foundational carrier families Query will treat as authoritative for mutation, retained state, row materialization, and projection facts;
- define the tiny production `serde_json::Value` allowlist, if any, including owner, reason, and removal condition;
- document whether string aspect paths remain only ergonomic caller inputs or remain part of any authoritative internal representation.

### Phase 2. Rewrite runtime mutation authority carriers

Focus zones:

- Zone 1: `runtime/surface/mutation/`
- Zone 2: `runtime/mutation/`
- Zone 3: `runtime/backend/`

Deliverables:

- replace `UpdateAspect { aspect_path, value }` style authority primitives with foundational-native mutation carriers;
- remove `ForgeQueryAspectValue` from the core mutation path or collapse it into a native desired-aspect carrier;
- make runtime batch writes, backend contracts, and existing-truth verification consume the new native carriers directly.

### Phase 3. Rewrite read, materialization, and projection consumption

Focus zones:

- Zone 4: `runtime/computed/` plus derived materialization root files
- Zone 6: `runtime/surface/` retained scalar and live surfaces
- Zone 7: `projection_consumption/consumed/`
- Zone 8: `projection_consumption/extraction/`

Deliverables:

- replace JSON row materialization as the default internal representation;
- introduce native row/value abstractions for derived views, retained rows, and consumed fact extraction;
- ensure bridge-backed and relational-backed read paths converge on native row contracts before any optional external projection.

### Phase 4. Delete JSON shims or prove the tiny allowlist

Focus zones:

- Zone 10: `memory_workspace/`
- Zone 11: `aspect_field_authoring/`
- any production facade-level compatibility helper that still mentions JSON

Deliverables:

- delete production JSON ingress/projection shims that exist only for legacy convenience;
- produce an explicit allowlist for any production JSON that survives, including owner, reason, and removal condition;
- remove duplicate lowering/projection utilities from runtime and support layers;
- leave test JSON triage to the later test phases unless a test directly blocks production shim deletion.

### Phase 5. Rewrite effect and workflow mutation substrates

Focus zones:

- Zone 5: `runtime/effect/`
- Zone 12: `domain_capabilities/authoring/` and `domain_capabilities/payloads/`

Deliverables:

- replace effect delivery JSON payloads with typed/native outputs;
- replace workflow `desired_aspect_fields_external_json` with native desired-aspect carriers;
- keep benign contribution payload vocabulary where it remains truthful;
- confirm that declarations, admissions, and capability traces digest native substrates instead of legacy JSON intermediates.

### Phase 6. Rewrite certification fixtures by migrated zone

Focus zones:

- Zone 9: `projection_consumption/certification/` and `projection_consumption/tests/`
- Zone 13: `effect_lifecycle/` tests and certification
- Zone 14: `intent_admission/certification/` and `lower_runtime_routing/certification/`

Deliverables:

- align test fixtures with the new authoritative carriers;
- preserve only explicitly marked legacy compatibility tests for JSON, with migration debt called out;
- prove certification, replay, and parity surfaces no longer depend on JSON carriers.

### Phase 7. Retire broad test debt and close the docs

Focus zones:

- Zone 15: broad test suites under `runtime/tests`, `domain_capabilities/tests`, and related closeout fixtures
- `_docs/forge-query/`

Deliverables:

- update tests opportunistically by migrated production zone;
- maintain a visible list for any tolerated JSON tests that remain due to crate size;
- produce closeout docs that distinguish ergonomic facade wins from true substrate completion.

## Must preserve

- Query remains the pleasant day-to-day authoring API and does not force ordinary callers to hand-assemble raw foundational patch machinery.
- Public facade ergonomics added during aspect API finalization remain available unless a specific surface is proven dishonest or unsafe.
- Query does not steal bridge or relational authority; it lowers into their authoritative contracts cleanly.
- Preview, live delivery, inspection, and certification keep their current capability breadth even as their substrates change.

## Must remove

- JSON as an internal mutation authority carrier.
- JSON as the default internal row/materialization carrier for derived and consumed truth.
- production JSON shims whose only purpose is easing migration from legacy payload-shaped APIs.
- string aspect paths as the sole authoritative representation of aspect identity inside mutation/runtime internals.
- test and doc language that implies the facade freeze already completed the foundational-native migration.

## Acceptance evidence

- no runtime mutation authority type uses `serde_json::Value` as its authoritative desired-aspect carrier;
- no read/materialization substrate requires projecting native aspect values into JSON before core fact extraction or derived-view retention;
- production `serde_json::Value` usage in Query is zero except for an approved allowlist with owner, reason, and removal condition;
- tolerated test JSON is explicitly marked as legacy compatibility or migration debt;
- certification, replay, parity, and adversarial tests prove the new substrate instead of only the facade;
- a closeout doc can state, truthfully, that Query is foundational-native internally while still offering ergonomic aspect authoring externally.

## Sequencing notes

- Do not start with global terminology churn. Start with runtime mutation and read/materialization truth.
- Delete compatibility helpers once core consumers are moved off them; do not let them become permanent legacy shims.
- Expect `runtime/` and `projection_consumption/` to dominate the engineering time.
- Treat `application/` as mostly preserved unless a concrete seam proves otherwise.
- Treat `domain_capabilities/` as an audit problem, not an automatic rename problem.
- Finish with tests and closeout docs, not before them.
