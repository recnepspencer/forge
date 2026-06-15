# Milestone 4: Interactive Workbench Validation App And Shell Acceptance

## Goal

Build the first real Worth UI workbench validation app: an interactive, replayable,
native desktop diagnostic UI that runs through Worth UI's Rust/egui platform
path and exercises platform shell behavior, mosaic layout, command projection,
theme posture, persistence/restore, and hot-reload survival as composed product
behavior rather than isolated primitive tests.

The default validation app theme should intentionally resemble the clarity and
density of VS Code dark mode: dark editor canvas, slightly differentiated
sidebars and panels, quiet borders, blue command/accent focus, readable neutral
text, and restrained runtime-state colors.

## Why This Milestone Exists

Milestone 3 closed the active runtime, execution-plan, durable-state,
reconciliation, Query-binding, lane, reload, diagnostics, and frame-cost
foundation. The remaining risk is not that Worth UI lacks individual primitive
tests. The risk is that those primitives can pass in isolation while a real
shell still drifts, cheats, looks bad, hides runtime evidence, or requires
app-local glue when composed.

This milestone turns the original shell acceptance evidence into a hostile
interactive desktop validation app. It must let a human see the native UI, run
scenarios, inspect the same receipts and counters the automated tests assert,
and catch composed failures that primitive tests miss.

The validation app must consume the foundations from Milestones 1 through 3:
facade/capability registries, canonical source/artifact lowering, active
runtime launch, execution-plan inspection, durable-state reconciliation,
command projection, diagnostics, replacement candidates, reload preservation,
and frame-cost evidence. It must not introduce a browser, DOM, React, Vite,
HTML/CSS, or web-view substitute for the product surface being validated.

## Governing Summaries

- `MENTALITY.md`: protects Worth UI from MVP/demo instincts. The validation app must
  be built for the UI platform Worth UI is becoming, not as a comforting toy
  dashboard.
- `arch_laws.md`: protects proof-bearing boundaries. Validation app operations must
  consume typed source, artifact, runtime, state, command, and diagnostic
  envelopes rather than re-deciding platform truth.
- `composition_laws.md`: protects the validation app from becoming a god demo. Scenario
  registry, operation runner, evidence capture, visual shell, theme surface,
  manual validation, and QA probes must be separate responsibilities.
- `domain_structure_laws.md`: protects authority and derivation in the tree.
  Scenario scripts, runtime evidence, visible UI, manual observations, and
  hostile validation app probes need distinct structural homes.
- `perf_laws.md`: protects hot-path honesty. Validation app visual polish and evidence
  projection must not introduce source parsing, broad artifact scans, registry
  string lookup, rich diagnostics, or allocation-heavy behavior into steady
  frame execution.
- `worth_ui_roadmap.md`: protects the sequence. M4 belongs immediately after M3
  because the active runtime foundation now needs composed product evidence
  before command, focus, Query-bound view, form, component, accessibility, and
  native breadth expand.

## Adversarial Constraint

A real workbench validation app must run hostile shell scenarios where
open/close/dock/split/tab/pin/overlay/persist/restore/restart/hot-reload
operations replay deterministically, expose the same runtime receipts as
automated tests, preserve stable-ID state only where eligible, deny invalid
edits without corrupting the active shell, keep visual theme and density changes
on declared platform token paths, and make any app-local shell workaround or
visual-only success mechanically visible as a failure.

## Product Decision Lock

- The validation app is product infrastructure, not a demo gallery.
- The validation app is a native Rust desktop app over Worth UI and egui. Browser,
  web-view, Vite, React, DOM, or HTML/CSS implementations are not acceptable
  milestone implementation paths.
- The validation app is an ordinary Worth UI consumer and receives no privileged
  internal active-plan, state, command, or diagnostic authority.
- Manual validation observes typed evidence; it cannot replace mechanical
  proof.
- The sample workbench must look intentionally designed. Theme, typography,
  spacing, density, focus, overlay, and runtime-state visuals are part of the
  acceptance surface.
- The default visual target is a VS Code-like dark workbench palette expressed
  through named Worth UI theme tokens, not copied as raw colors in component or
  validation app code.
- Scenario success requires both visible behavior and attached runtime evidence.
- The validation app must be hostile enough to expose production weakness instead of
  smoothing over it with helpers.
- Every major acceptance claim gets one explicit end-to-end scenario phase.
- Future milestones may add scenarios to the validation app, but may not rewrite the
  runner to hide weaker proof.
- Per-phase `Relevant APIs` sections cite already-existing public Worth UI
  surfaces or existing `worth-ui-harness` test-support surfaces for
  orientation. They are reference surfaces, not construction instructions, and
  they do not authorize invented APIs, placeholder type names, or
  implementation outside the current facade.

## Phase Plan

### Phase 1: Workbench App Shell And Navigation

Freeze the first real validation app shell: a VS Code-like validation
desktop workbench with pages, persistent navigation, command surfaces, run
status, and room for future scenario families without changing the app frame.

**Relevant subsystems**

- native validation workbench app
- native Rust/egui app entrypoint
- page registry
- activity rail and page navigation
- menu bar, toolbar, and command palette shell
- status bar and run summary surface
- theme token consumption

**Relevant APIs**

- `WorthUi`
- `WorthUiAppBuilder`
- `WorthUiApp`
- `WorthUiRuntimeLaunchBuilder`
- `WorthUiRuntimeSourceModule`
- `WorthUiRuntimeLaunch`
- `WorthUiRuntimeHost`
- `WorthUiActiveRuntimeObservation`
- `WorthUiRuntimeDiagnosticsProjection`
- `WorthUiExecutionPlanInspection`
- `CommandProjectionDescriptor`
- `ThemeTokenDescriptor`

**Warnings**

- Do not make a landing page. The first screen is the usable validation
  workbench.
- Do not implement the workbench as a web app, web-view, Vite app, React app,
  DOM shell, or HTML/CSS prototype.
- Do not build a separate demo shell that later needs to be replaced by the
  validation app.
- Do not hide menu, toolbar, palette, context, status, inspector, or bottom
  panel surfaces behind future work; the first page must reserve and render
  them.
- Do not let app-local page state masquerade as scenario or runtime truth.

**Test requirements**

- `validation_app_workbench_app_loads_first_page_with_all_shell_surfaces`: the app
  opens directly to a workbench page with activity rail, scenario nav, menu,
  toolbar, command palette affordance, editor tabs, inspector, bottom timeline,
  status bar, and overlay surface present.
- `validation_app_page_navigation_preserves_active_run_context`: switching between
  validation app pages preserves the selected scenario, latest run receipt reference,
  theme, and density state without rebuilding the shell frame.
- `validation_app_shell_surface_ids_are_stable_across_reload`: hot reload or native
  app restart preserves stable IDs for the activity rail, page host,
  inspector, bottom panel, command palette, and status bar.
- `validation_app_workbench_cannot_be_implemented_through_web_tooling`: compile or
  repository guard coverage rejects browser/Vite/React/web-view validation app
  surfaces as milestone implementation artifacts.
- `validation_app_shell_does_not_use_marketing_or_demo_only_routes`: no app route may
  replace the workbench with a hero, landing page, or disconnected showcase.

**Engineering decisions**

- The validation app shell is product infrastructure and should feel like a real
  desktop tool from the first slice.
- Pages are registered as workbench surfaces; the app frame owns navigation,
  while scenario pages own their local presentation.
- All visible shell surfaces use the default VS Code-like Worth UI dark theme
  tokens.

**Open questions**

- None.

### Phase 2: Surface Atlas Workbench Page

Build the first loaded page as a surface atlas: one dense, inspectable
workbench page that shows every major validation app surface before the runner is
fully wired to all scenario families.

**Relevant subsystems**

- surface atlas page
- sample workbench canvas
- pinned sidebar and stacked panes
- tabbed editor region
- evidence inspector
- bottom run timeline
- overlay preview
- theme and density controls

**Relevant APIs**

- `SurfaceDescriptor`
- `SurfaceId`
- `SurfacePlacementClass`
- `ComponentDescriptor`
- `CommandProjectionDescriptor`
- `ThemeTokenDescriptor`
- `IconDescriptor`
- `RuntimeOutcomeProjectionDescriptor`
- `HarnessEvidenceBundle`
- `HarnessRunReceipt`
- `HarnessExpectedObservation`
- `HarnessVisualFoundationReceipt`
- `PreparedHarnessVisualFoundation`

**Warnings**

- Do not wait for every backend scenario before the first page exists.
- Do not implement the surface atlas outside the native Rust/egui/Worth UI
  desktop path.
- Do not fake success; placeholder data must be visibly labeled as fixture or
  sample evidence until a live runner surface owns it.
- Do not let the page collapse into decorative cards. It must expose real
  shell surfaces: panes, tabs, overlays, command projections, inspectors,
  scroll regions, and status surfaces.
- Do not make the first page sparse. The purpose is to see the platform
  surface density early.

**Test requirements**

- `surface_atlas_renders_all_required_surface_families`: the first page
  renders activity/navigation, scenario list, command projection surfaces,
  tabbed editor, pinned sidebar, stacked scroll panes, evidence inspector,
  bottom timeline, overlay, and status surfaces.
- `surface_atlas_fixture_data_is_labeled_and_cannot_mark_success`: fixture
  evidence may populate the first page visually but cannot produce a completed
  validation record.
- `surface_atlas_theme_controls_update_tokens_without_layout_drift`: theme and
  density controls update token/density displays without moving the workbench
  region topology.
- `surface_atlas_mobile_layout_preserves_surface_access`: narrower viewports
  keep the scenario list, workbench canvas, inspector, and run timeline
  reachable without text overlap or hidden critical controls.

**Engineering decisions**

- The first page is intentionally ambitious: it is the visual contract for the
  validation app.
- Fixture evidence is allowed only as sample projection data and must remain
  distinguishable from live runner receipts.
- The page should make later scenario wiring obvious by naming the exact
  surface and evidence slots the runner will fill.

**Open questions**

- None.

### Phase 3: Validation App Honesty Boundary

Freeze the rule that the validation app is a normal Worth UI application consuming
public platform contracts, not a privileged internal test runtime.

**Relevant subsystems**

- validation app crate or example topology
- Worth UI facade construction
- source-to-artifact path
- runtime host launch path
- validation app QA probes

**Relevant APIs**

- `WorthUi`
- `WorthUiAppBuilder`
- `WorthUiApp`
- `WorthUiRuntimeLaunchBuilder`
- `WorthUiRuntimeSourceModule`
- `WorthUiRuntimeLaunch`
- `WorthUiRuntimeHost`
- `WorthUiRuntimeLaunchDenial`
- `WorthUiActiveRuntimeObservation`
- `WorthUiReplacementCandidate`
- `WorthUiExecutionPlan`
- `WorthUiExecutionPlanInspection`

**Warnings**

- Do not build the validation app by importing runtime internals to make scenarios
  convenient.
- Do not let validation app helpers inject state, commands, active plans, or receipts
  that real product code could not earn.
- Do not certify a scenario whose setup path has already solved the hard
  authority, identity, state, or command routing conditions.
- Do not hide a production gap behind an app-local fallback in the validation app.

**Test requirements**

- `validation_app_launch_uses_only_public_worth_ui_facade`: compile-pass coverage
  proves the validation app can construct and launch its workbench through public
  Worth UI surfaces.
- `validation_app_cannot_import_internal_runtime_or_registry_modules`: compile-fail
  coverage rejects deep imports into runtime, registry, active-plan, or
  diagnostic internals.
- `validation_app_rejects_app_local_shell_state_injection`: an attempted scenario that
  provides pre-minted shell state or active-plan evidence fails before scenario
  execution.
- `validation_app_result_requires_runtime_receipts_not_visual_success`: a scenario
  cannot be marked complete with only a visible frame or user note.

**Engineering decisions**

- The validation app owns scenario orchestration and presentation; Worth UI owns shell
  runtime truth.
- Validation app evidence is derived from runtime receipts and counters, not from
  local assumptions.
- Validation app test support must obey the same file and function composition bar as
  production code.

**Open questions**

- None.

### Phase 4: Theme And Visual Foundation

Freeze the validation app visual substrate so manual validation happens inside a
beautiful, token-driven workbench rather than an ugly diagnostic wrapper.

The initial palette should be close to VS Code dark mode while remaining a
Worth UI theme: near-black editor canvas, darker activity/sidebar chrome,
slightly raised panel surfaces, low-contrast separators, bright blue accent,
subtle selection/focus treatment, and clear warning/error/success colors that
do not overwhelm the shell.

**Relevant subsystems**

- theme token registry
- icon registry
- mosaic sizing contracts
- runtime-state visual tokens
- density and typography posture
- focus, selection, overlay, and diagnostic visual treatment

**Relevant APIs**

- `ThemeTokenDescriptor`
- `ThemeTokenId`
- `IconDescriptor`
- `MosaicSizingContractDescriptor`
- `NamedMeasurementToken`
- `RuntimeOutcomeProjectionDescriptor`
- `CommandProjectionDescriptor`

**Warnings**

- Do not use raw colors, raw spacing numbers, or one-off focus visuals in the
  validation app shell.
- Do not hard-code VS Code-like colors directly in widgets; the palette must
  enter through named Worth UI theme token descriptors.
- Do not make the validation app theme prettier by bypassing declared token or sizing
  surfaces.
- Do not let density changes mutate shell meaning, stable IDs, or persisted
  state.
- Do not delay visual quality until after scenario proof; this validation app is a UI
  platform artifact.

**Test requirements**

- `theme_density_change_preserves_shell_artifact_meaning`: changing theme or
  density updates visual token evidence without changing shell structure,
  stable IDs, command identity, or active plan equivalence where layout posture
  is unchanged.
- `raw_visual_values_are_rejected_from_validation_app_shell_source`: source or facade
  declarations that try to use raw color/spacing/overlay values fail with typed
  diagnostics.
- `validation_app_theme_tokens_cover_focus_selection_overlay_and_runtime_states`:
  accepted validation app theme includes explicit tokens for focus, selection,
  overlays, diagnostics, success, warning, danger, disabled, and active states.
- `vscode_like_dark_theme_token_palette_is_complete_and_inspectable`: the
  default dark theme exposes named tokens for editor canvas, sidebar, panel,
  elevated overlay, border, text, muted text, accent, focus, selection, command
  highlight, warning, danger, and success.
- `theme_reload_does_not_create_layout_drift`: visual-only theme reloads do not
  change persisted mosaic state, splitter position, tab identity, or scroll
  ownership receipts.

**Engineering decisions**

- The validation app should ship with a polished VS Code-like dark default theme and
  one density variant.
- Theme and sizing are platform declarations consumed by the validation app, not
  renderer-local constants.
- Manual validation panels display both visual expectation and the token or
  sizing evidence that produced it.

**Open questions**

- None.

### Phase 5: Scenario Runner And Evidence Model

Freeze the typed scenario model: scenario identity, ordered steps, expected
observations, runtime evidence capture, failure localization, and replay
records.

**Relevant subsystems**

- scenario registry
- operation script model
- evidence envelope
- scenario result ledger
- deterministic replay basis
- failure localization

**Relevant APIs**

- `HarnessScenarioId`
- `HarnessScenario`
- `HarnessScenarioOperation`
- `HarnessScenarioStep`
- `HarnessExpectedObservation`
- `HarnessRunReceipt`
- `HarnessReplayRecord`
- `HarnessEvidenceBundle`
- `HarnessEvidenceLedger`
- `HarnessEvidenceRequirement`
- `HarnessFailureLocation`
- `HarnessRunner`
- `WorthUiRuntimeDiagnosticsProjection`
- `WorthUiReloadStormCertification`

**Warnings**

- Do not let scenarios assert on arbitrary strings where structured evidence
  exists.
- Do not let expected digests be hard-coded without proving they derive from
  current scenario inputs.
- Do not merge setup, execution, assertion, and UI reporting into one helper.
- Do not let manual observations become the only source of pass/fail truth.

**Test requirements**

- `scenario_replay_produces_equivalent_evidence_bundle`: replaying the same
  scenario from the same fixture produces equivalent operation receipts,
  artifact/plan digests, state receipts, command identities, counters, and
  denials.
- `scenario_result_rejects_missing_required_evidence_family`: a result missing
  a required runtime receipt, counter family, command evidence, state receipt,
  or visual observation is denied.
- `scenario_expected_digest_must_be_derived_from_run_inputs`: hard-coded
  expected digests that do not match the current run basis fail.
- `scenario_failure_localizes_to_operation_and_evidence_family`: failure output
  identifies the scenario step and evidence family rather than only final
  mismatch.

**Engineering decisions**

- Scenario definitions are declarative and replayable.
- Scenario results carry evidence; UI panels derive from those results.
- The runner owns orchestration but never owns platform truth.

**Open questions**

- None.

### Phase 6: Open And Close Workbench Surface Scenario

Prove that the sample workbench can open and close document or panel surfaces
through platform shell operations without app-local shell mutation.

**Relevant subsystems**

- workbench shell fixture
- document/panel surface declaration
- active shell structure
- stable surface identity
- durable panel visibility state
- runtime diagnostics projection

**Relevant APIs**

- `SurfaceDescriptor`
- `ComponentDescriptor`
- `MosaicPlacementPolicyDescriptor`
- `MosaicStateSlotDescriptor`
- `WorthUiReplacementCandidate`
- `WorthUiDurableStateReconciliationPlan`
- `WorthUiNodeReplacementPlan`

**Warnings**

- Do not implement open/close as local boolean widget state.
- Do not let a closed panel leave orphan durable state unless a receipt explains
  why it is intentionally retained.
- Do not let duplicate surface IDs collapse into one visible panel.
- Do not treat visual disappearance as proof of correct shell state.

**Test requirements**

- `workbench_open_close_surface_round_trip_preserves_unrelated_state`: opening
  and closing a panel changes only the intended shell surface and preserves
  unrelated splitter, tab, scroll, and command state.
- `duplicate_or_stale_panel_identity_rejected_before_activation`: duplicate or
  stale panel IDs fail before active shell mutation.
- `closed_surface_state_has_explicit_drop_or_retention_receipt`: closing a
  surface either drops or retains durable state with a typed receipt.
- `open_close_visible_state_matches_active_plan_inspection`: visible panel
  presence agrees with active execution-plan inspection and diagnostics.

**Engineering decisions**

- Open/close is represented as shell structure replacement through platform
  runtime paths.
- Panel visibility is a durable UI state family, not domain truth.
- The validation app displays the surface identity and receipt that explains the
  visible shell result.

**Open questions**

- None.

### Phase 7: Dock, Split, And Tab Workbench Scenario

Prove that dock, split, and tab operations compose in one workbench through the
mosaic and runtime substrate rather than through app-owned geometry mutation.

**Relevant subsystems**

- mosaic placement policy
- mosaic region topology
- tab state family
- splitter state family
- execution-plan topology
- plan equivalence and replay

**Relevant APIs**

- `MosaicPlacementAction`
- `MosaicPlacementPolicyDescriptor`
- `MosaicStableIdentityBehavior`
- `MosaicPlacementReloadReconciliation`
- `MosaicStateSlotDescriptor`
- `WorthUiTabStateReconciliation`
- `WorthUiSplitterPositionReconciliation`
- `WorthUiExecutionPlanInspection`
- `WorthUiExecutionPlanEquivalence`
- `WorthUiPlanTopology`

**Warnings**

- Do not represent dock or split placement as anonymous geometry.
- Do not let tab order or active tab be inferred from display order without a
  stable state slot.
- Do not allow split ratios to become raw local numbers detached from mosaic
  sizing contracts.
- Do not make equivalent replay depend on declaration order accidents.

**Test requirements**

- `dock_split_tab_replay_produces_equivalent_shell_plan`: replaying the same
  dock/split/tab operation script produces equivalent active plan digests,
  topology inspection, and state receipts.
- `stale_splitter_or_tab_receipt_rejected_before_restore`: stale tab or
  splitter receipts cannot certify a later layout restore.
- `tab_reorder_preserves_state_only_for_stable_tab_identity`: reordering tabs
  preserves eligible tab state only when stable identity matches.
- `dock_target_mismatch_denied_without_mutating_active_shell`: docking a
  surface into an incompatible target region fails before activation.

**Engineering decisions**

- Dock, split, and tab operations are scenario operations over platform layout
  artifacts.
- Tab and splitter state are durable UI state, not ad hoc egui memory.
- The validation app must show active region IDs, tab IDs, split receipts, and plan
  topology for each operation.

**Open questions**

- None.

### Phase 8: Pinned Sidebar And Stacked Scroll Scenario

Prove that a nested mosaic shell can express a pinned sidebar and stacked
scroll regions without DOM-style percentage-height, overflow, or implicit parent
measurement hacks.

**Relevant subsystems**

- mosaic region kind registry
- mosaic sizing contracts
- named measurement definitions
- scroll ownership
- pinned region posture
- stacked region topology

**Relevant APIs**

- `MosaicRegionKindDescriptor`
- `MosaicRegionRole`
- `MosaicSizingBehavior`
- `MosaicScrollOwnership`
- `MosaicChildRule`
- `MosaicClippingPosture`
- `MosaicSizingContractDescriptor`
- `NamedMeasurementDefinition`

**Warnings**

- Do not model the sidebar as a special-case widget wrapper.
- Do not make scroll ownership implicit from widget nesting.
- Do not allow raw height or overflow settings to masquerade as mosaic sizing.
- Do not let resize behavior mutate stable identity or restore state.

**Test requirements**

- `pinned_sidebar_and_stacked_scroll_regions_restore_without_drift`: sidebar,
  stacked scroll regions, and resize/restart restore to equivalent layout
  evidence.
- `raw_height_or_overflow_hack_rejected_from_nested_mosaic_shell`: DOM-style
  raw sizing or overflow declarations fail validation.
- `scroll_position_preserved_only_for_declared_scroll_owner`: scroll state
  carries forward only for regions with declared scroll ownership and stable
  identity.
- `sidebar_resize_uses_named_measurement_contract`: sidebar width changes route
  through named sizing contracts and produce explicit state receipts.

**Engineering decisions**

- Pinned sidebar is a region/topology declaration, not a separate layout system.
- Stacked scroll regions must name scroll ownership at the region boundary.
- The validation app should make scroll owner, sizing token, and region identity
  visible in the evidence panel.

**Open questions**

- None.

### Phase 9: Overlay Surface Scenario

Prove that overlay surfaces can appear above the shell without corrupting
layout, focus posture, command routing, hit testing, or underlying durable
state.

**Relevant subsystems**

- overlay placement policy
- overlay region kind
- focus and command routing posture
- hit-test posture
- real-time overlay or HUD lane where applicable
- active-plan inspection

**Relevant APIs**

- `MosaicPlacementAction`
- `SurfacePlacementClass`
- `MosaicRegionRole`
- `MosaicClippingPosture`
- `MosaicHitTestPosture`
- `WorthUiRealtimeOverlayHook`
- `WorthUiRealtimeOverlayLane`
- `WorthUiHudPlan`
- `WorthUiCanvasOverlayPlan`
- `WorthUiExecutionPlanInspection`
- `CommandProjectionDescriptor`

**Warnings**

- Do not let an overlay become ordinary layout geometry just because that is
  easier to draw.
- Do not let overlay focus steal command routing without explicit posture.
- Do not allow overlay close to drop unrelated underlying shell state.
- Do not certify overlays visually without active-plan and hit-test evidence.

**Test requirements**

- `overlay_open_close_preserves_underlying_shell_state`: opening and closing an
  overlay preserves underlying sidebar, tab, splitter, scroll, and command
  state.
- `overlay_as_ordinary_layout_region_is_rejected`: an overlay declaration that
  enters ordinary layout placement instead of overlay placement fails.
- `overlay_command_routing_uses_declared_focus_posture`: overlay commands route
  according to declared focus and command projection posture.
- `overlay_hit_test_and_z_order_evidence_match_visible_surface`: overlay
  visible position agrees with hit-test/z-order evidence and plan inspection.

**Engineering decisions**

- Overlays are platform shell surfaces with explicit placement, focus, and
  clipping posture.
- The validation app should include at least one normal modal-like overlay and one
  diagnostic/inspector overlay.
- Overlay visuals must consume theme tokens for scrim, border, elevation, and
  focus.

**Open questions**

- None.

### Phase 10: Command Backbone Scenario

Prove that menu bar, toolbar, command palette, and context surfaces project the
same command registry meaning instead of each surface inventing local command
behavior.

**Relevant subsystems**

- command registry
- command projection registry
- command readiness posture
- menu, toolbar, palette, and context projection surfaces
- active shell routing
- command evidence display

**Relevant APIs**

- `CommandDescriptor`
- `CommandId`
- `CommandProjectionDescriptor`
- `CommandProjectionId`
- `CommandReadinessBinding`
- `CommandProjectionSurface`
- `WorthUiCommandHandle`
- `WorthUiCommandBindingInvalidation`
- `WorthUiPlanLookupIndex`
- `WorthUiOrdinaryFrameTarget`

**Warnings**

- Do not let menu entries, toolbar buttons, palette rows, or context actions
  own separate labels, readiness, icons, or routing.
- Do not flatten structured readiness into a boolean in the validation app UI.
- Do not assert command correctness by matching visible text only.
- Do not let context surfaces smuggle app-local command identities.

**Test requirements**

- `same_command_identity_projects_to_menu_toolbar_palette_and_context`:
  one command appears across all projection surfaces with the same command ID,
  label/icon posture, readiness evidence, and routing target.
- `projection_drift_between_command_surfaces_is_rejected`: a surface that
  changes label, readiness, icon, grouping, shortcut visibility, or command
  identity outside the command backbone fails.
- `context_command_requires_declared_mosaic_scope`: region/context commands
  require declared mosaic scope and cannot attach to arbitrary visible widgets.
- `command_invocation_receipt_matches_projection_identity`: invoking a command
  from any projection records the same command identity and route basis.

**Engineering decisions**

- Command projections are views over canonical command meaning.
- The validation app command palette is useful UI, but also an acceptance evidence
  surface.
- The manual validation panel must show command ID, projection surface, routing
  scope, readiness, and invocation evidence.

**Open questions**

- None.

### Phase 11: Persist And Restore Scenario

Prove that shell state can persist and restore after runtime recreation without
inventing layout drift or promoting persisted UI state into authoritative truth.

**Relevant subsystems**

- persisted shell state envelope
- durable state inventory
- durable state reconciliation
- runtime launch/recreate path
- artifact and plan equivalence
- restore diagnostics

**Relevant APIs**

- `WorthUiDurableStateInventory`
- `WorthUiDurableStateInventoryBuilder`
- `WorthUiDurableStateFamily`
- `WorthUiDurableStateFamilyId`
- `WorthUiDurableStateReconciliationPlan`
- `WorthUiDurableStateReconciliationReceipt`
- `WorthUiStateCarryForwardReceipt`
- `WorthUiLastValidObservation`
- `WorthUiExecutionPlanEquivalence`
- `WorthUiRuntimeDiagnosticsProjection`

**Warnings**

- Do not restore by replaying local egui memory.
- Do not let persisted shell state claim authoritative domain truth.
- Do not accept best-effort restore without explicit preserve, replace, drop,
  or create receipts.
- Do not compare only final visuals when pre/post evidence can be compared.

**Test requirements**

- `persist_restore_recreates_equivalent_shell_state_after_runtime_restart`:
  saving shell state, recreating the runtime, and restoring produces equivalent
  eligible state receipts, active plan digest, and visible shell structure.
- `restore_rejects_state_from_mismatched_artifact_or_snapshot`: persisted state
  from a different artifact, snapshot, or runtime basis cannot silently restore.
- `restore_drops_ineligible_state_with_explicit_receipts`: stale, orphaned, or
  ineligible state is dropped or replaced with typed receipts.
- `manual_restore_validation_requires_pre_and_post_evidence`: manual completion
  requires both pre-save and post-restore evidence bundles.

**Engineering decisions**

- Persisted shell state is a replayable input to runtime reconciliation, not an
  active runtime object.
- Restore is a scenario operation with visible before/after evidence.
- The validation app should include a restart/recreate button that runs the real
  restore path, not a UI reset shortcut.

**Open questions**

- None.

### Phase 12: Hot Reload Layout Edit Scenario

Prove that workspace layout edits survive hot reload when stable IDs remain
intact, and invalid edits preserve the previous active shell with typed
diagnostics.

**Relevant subsystems**

- file-authored source edit path
- Rust-authored replacement path where relevant
- replacement candidate admission
- impact narrowing
- identity matching
- state reconciliation
- activation gate and atomic swap
- reload diagnostics

**Relevant APIs**

- `WorthUiReplacementCandidate`
- `WorthUiCandidateAdmission`
- `WorthUiRuntimeArtifactComparison`
- `WorthUiRuntimeImpactNarrowing`
- `WorthUiIdentityMatchGraph`
- `WorthUiDurableStateReconciliationPlan`
- `WorthUiReadyActivation`
- `WorthUiPlanSwapReceipt`
- `WorthUiReloadStormCertification`

**Warnings**

- Do not make hot reload a validation app-local patch of visible widgets.
- Do not allow equivalent edits to force unnecessary swaps.
- Do not let invalid source blank or partially mutate the active shell.
- Do not infer state preservation from matching layout position alone.

**Test requirements**

- `valid_layout_reload_preserves_stable_id_shell_state`: valid layout edits
  with stable IDs activate and carry forward eligible sidebar, tab, splitter,
  scroll, and overlay state.
- `invalid_layout_reload_preserves_previous_active_shell`: malformed,
  unsupported, or denied layout edits leave the previous active plan and shell
  evidence intact.
- `equivalent_layout_reload_noops_without_plan_swap`: equivalent source edits
  classify as no-op and avoid unnecessary activation.
- `layout_reload_with_identity_change_replaces_or_drops_state_explicitly`:
  identity-changing edits do not preserve state by accident.

**Engineering decisions**

- Hot reload scenarios must enter through the same candidate-to-active-plan
  pipeline M3 certified.
- The validation app UI shows candidate classification, denial, state receipts, and
  swap/no-op evidence for each edit.
- Layout edit fixtures should include valid, invalid, equivalent, stable-ID,
  and identity-changing variants.

**Open questions**

- None.

### Phase 13: Hostile Replay And Recovery Scenario

Run interleaved shell operations under interruption pressure to prove replay,
restart, invalid reload, overlay, command, and restore behavior converge to the
same active shell evidence.

**Relevant subsystems**

- scenario replay engine
- operation interruption and recovery
- reload failure preservation
- command routing during recovery
- overlay and durable state residue scan
- diagnostics projection

**Relevant APIs**

- `HarnessScenarioOperation`
- `HarnessReplayRecord`
- `HarnessReplayDenial`
- `HarnessRunReceipt`
- `WorthUiReloadFailure`
- `WorthUiReloadPreservationReceipt`
- `WorthUiIdentityStateCertification`
- `WorthUiQueryDriftCertification`
- `WorthUiRuntimeDiagnosticsProjection`
- `WorthUiPlanSwapRollback`

**Warnings**

- Do not curate only clean operation sequences.
- Do not let recovery repair state through hidden cleanup.
- Do not let stale receipts from one operation certify a later operation.
- Do not accept final convergence if residue remains in diagnostics, live
  bindings, state inventory, command routing, or overlay surfaces.

**Test requirements**

- `interleaved_shell_replay_converges_after_invalid_reload_and_restart`:
  dock/split/tab/overlay/command/invalid-reload/restart sequences replay to the
  same active shell evidence.
- `stale_operation_receipts_cannot_certify_later_recovery`: receipts from
  earlier operations cannot be reused after source, artifact, snapshot, or
  runtime basis changes.
- `recovery_residue_scan_finds_no_orphan_shell_state`: mixed failure and
  success flows leave no orphan panel state, stale overlay, stale command
  route, or unpaired diagnostics residue.
- `command_invocation_during_overlay_recovery_uses_current_active_route`:
  commands invoked during recovery bind to current active shell context, not
  stale pre-failure context.

**Engineering decisions**

- Hostile replay is the first full composed acceptance storm for the validation app.
- Recovery evidence must distinguish preservation, rollback, restore, and
  fresh activation.
- The validation app should make the operation timeline and evidence timeline visible
  side by side.

**Open questions**

- None.

### Phase 14: Manual Validation Panel

Build the human-facing validation UI that lets a reviewer run scenarios, inspect
expected visible behavior, compare runtime evidence, record observations, and
mark scenario outcomes without detaching from mechanical proof.

**Relevant subsystems**

- manual validation UI
- expected observation model
- scenario evidence display
- pass/fail record
- reviewer notes
- screenshot or visual capture extension seam

**Relevant APIs**

- `HarnessExpectedObservation`
- `HarnessEvidenceBundle`
- `HarnessEvidenceFamily`
- `HarnessEvidenceRequirement`
- `HarnessRunReceipt`
- `HarnessEvidenceLedger`
- `HarnessOperationReceipt`
- `WorthUiDiagnosticsProjection`
- `WorthUiFrameCostCertification`

**Warnings**

- Do not let a reviewer mark success without attached evidence.
- Do not use manual validation as a substitute for typed denial, receipt, or
  counter assertions.
- Do not make validation notes the source of truth for scenario outcome.
- Do not build screenshot/golden support in a way that snapshots noise instead
  of semantic visual state.

**Test requirements**

- `manual_validation_record_requires_scenario_evidence_bundle`: a manual
  validation record cannot be completed without the run evidence bundle and
  required receipt families.
- `manual_visible_observation_must_attach_to_named_scenario_step`: observations
  must attach to specific scenario steps and expected visible outcomes.
- `manual_success_rejected_when_required_counter_or_receipt_fails`: a reviewer
  cannot override a failed required receipt, digest, denial, or counter family
  with a success note.
- `visual_capture_extension_cannot_change_scenario_result`: screenshot or
  visual capture hooks observe the result but cannot change pass/fail truth.

**Engineering decisions**

- Manual validation is a structured review artifact over scenario evidence.
- Expected visible behavior is named per step rather than freeform prose only.
- Screenshot/golden capture is prepared as an extension seam, but the first
  milestone can close with structured manual validation and runtime evidence.

**Open questions**

- None.

### Phase 15: Validation App Self-QA Scenario

Try to make the validation app lie, and prove those lies fail. This phase closes the
milestone by treating the validation app itself as hostile test infrastructure.

**Relevant subsystems**

- validation app QA probes
- fake scenario fixtures
- forbidden helper paths
- evidence integrity validation
- test-support structure
- line-cap and composition guardrails for validation app code

**Relevant APIs**

- `HarnessEvidenceBundle`
- `HarnessEvidenceRequirement`
- `HarnessRunReceipt`
- `HarnessRunDenial`
- `HarnessHonestyDenial`
- `HarnessRunner`
- `WorthUiRuntimeHost`
- `WorthUiExecutionPlanInspection`
- `WorthUiDiagnosticsProjection`

**Warnings**

- Do not trust the validation app because it looks polished.
- Do not let helpers hide lifecycle edges, pre-solve state, or inject receipts.
- Do not weaken assertions to make visual scenarios pass.
- Do not certify the validation app with synthetic-only tests.

**Test requirements**

- `validation_app_rejects_visual_only_success_path`: a scenario with correct-looking
  visuals and missing runtime evidence fails.
- `validation_app_rejects_hard_coded_expected_digest`: expected digests not derived
  from the current run basis fail.
- `validation_app_rejects_helper_injected_state_or_command_route`: helper-injected
  durable state, command route, or active shell evidence fails integrity
  validation.
- `validation_app_self_qa_runs_against_real_workbench_scenarios`: self-QA probes run
  against the real scenario runner and workbench fixture, not a miniature fake
  validation app.
- `validation_app_test_support_obeys_production_composition_laws`: validation app test
  support stays decomposed by scenario family, operation family, evidence
  family, and assertion family rather than becoming a broad helper bucket.

**Engineering decisions**

- The final M4 proof is not that the validation app can pass scenarios; it is that the
  validation app can expose dishonest scenarios.
- Validation app QA is allowed to include intentionally fake fixtures, but only to
  prove the real runner rejects them.
- Any production weakness exposed by validation app QA must be fixed in production or
  explicitly made visible as a blocked acceptance path.

**Open questions**

- None.

## Must Ship

- public-facade-only validation app target that launches a real Worth UI native
  desktop workbench over the Rust/egui platform path
- repository guard or compile proof that M4 does not introduce browser, Vite,
  React, DOM, HTML/CSS, or web-view validation app implementation artifacts
- workbench app shell with activity rail, page registry, menu bar, toolbar,
  command palette affordance, scenario navigation, inspector, bottom timeline,
  overlay layer, and status bar from the first loaded page
- surface atlas first page that visibly exposes shell surfaces, command
  surfaces, evidence surfaces, theme controls, fixture/sample evidence labels,
  and the future live-runner slots
- polished VS Code-like dark, token-driven theme and density foundation for
  the validation app shell
- typed scenario registry, operation script model, evidence bundle, run
  receipt, and validation record
- canonical workbench shell with pinned sidebar, tabbed editor region, stacked
  scroll regions, bottom/status region, overlays, menus, toolbar, command
  palette, and context surfaces
- scenario operations for open, close, dock, split, tab, pin, overlay, persist,
  restore, restart/recreate, valid reload, invalid reload, equivalent reload,
  and identity-changing reload
- runtime evidence projection UI for artifact/plan digests, state receipts,
  command identities, reload diagnostics, activation/swap receipts, and frame
  counters
- manual validation panel with expected visible observations tied to scenario
  steps and required runtime evidence
- hostile validation app QA suite proving the validation app cannot certify visual-only,
  helper-injected, hard-coded, stale, or privileged-internal success

## Must Preserve

- Worth UI runtime remains the owner of active artifacts, active plans,
  diagnostics, state reconciliation, plan swaps, and frame-cost evidence.
- The validation app remains a consumer of public Worth UI contracts.
- Manual validation observes structured evidence; it does not replace typed
  tests, receipts, counters, denials, or digests.
- Theme and visual quality use declared tokens, sizing contracts, and runtime
  posture surfaces rather than raw local styling.
- The VS Code-like dark palette remains a Worth UI theme contract, not a direct
  dependency on VS Code assets or raw component-local color constants.
- Workspace layout remains a platform artifact, not app-local geometry state.
- Persisted shell state remains distinct from authoritative runtime truth.
- Scenario helpers clarify proof pressure and never pre-solve production
  authority, identity, state, command, restore, or reload conditions.
- Steady frame execution remains free of source parsing, artifact validation,
  registry string lookup, broad artifact scans, and rich diagnostics by
  default.

## Acceptance Evidence

- the validation app launches as a normal Worth UI app through public facade and
  native desktop runtime paths
- the validation app consumes the M1-M3 facade, canonical artifact, active runtime,
  execution-plan, durable-state, command projection, diagnostics, reload, and
  frame-cost foundations rather than creating replacement UI infrastructure
- the first visible screen is a usable workbench validation page, not a
  marketing or demo-only route
- the first page renders all primary validation app surfaces at once: activity rail,
  scenario list, menu, toolbar, command palette affordance, tabbed workbench,
  pinned sidebar, stacked panes, overlay, evidence inspector, bottom run
  timeline, and status bar
- the sample workbench can open, close, dock, split, tab, persist, restore, and
  restart without app-local shell logic
- nested mosaic shell evidence shows pinned sidebar, stacked scroll regions,
  bottom/status region, tab stack, and overlays without DOM-style height or
  overflow hacks
- the default validation app theme resembles a polished VS Code-like dark workbench,
  is fully token-driven, and density/theme changes do not create layout or
  state drift
- menu bar, toolbar, command palette, and context surfaces expose the same
  command identities and routing evidence
- valid layout reloads preserve eligible stable-ID shell state; invalid reloads
  preserve the previous active shell with typed diagnostics
- interleaved hostile replay converges without stale command routes, orphan
  state, stale overlay surfaces, or diagnostics residue
- a human can run each acceptance scenario from the validation app UI and see the
  visible shell result beside the exact runtime evidence required for success
- validation app self-QA proves visual-only success, hard-coded expected values,
  helper-injected state, stale receipts, and privileged internals are rejected

## Sequencing Notes

This milestone belongs immediately after Milestone 3 because the active runtime
and shell-relevant primitives now exist, but the platform still needs composed
product evidence before broader command, focus, Query-bound view, form,
component, accessibility, native, plugin, and tooling milestones depend on the
shell behaving as a real product surface.

It deliberately does not replace later component-system or accessibility
milestones. It creates the workbench and validation validation app those later
milestones can extend.

## Self-Check

- Does the milestone solve a real structural problem or just package work
  cosmetically? Yes: it closes the gap between primitive proof and composed
  product evidence.
- Is the adversarial constraint precise and load-bearing? Yes: the validation app must
  survive hostile replay, restore, reload, visual, command, and self-QA pressure.
- Does the roadmap justify this milestone now? Yes: M3 built the runtime
  substrate and the next risk is invisible primitive-only proof.
- Does the spec preserve crate authority boundaries? Yes: the validation app consumes
  public Worth UI contracts and cannot own active runtime truth.
- Are the phases carrying most of the real design information? Yes.
- Is each phase centered on one conceptual detail or boundary? Yes.
- Does each phase contain at least 2 adversarial tests by default? Yes.
- Could a competent engineer map this spec into honest types, modules, and
  tests? Yes.
- Does the milestone belong in this roadmap sequence, or is it out of order?
  It belongs here, directly after M3 and before broader product-surface growth.
