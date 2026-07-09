# Feature Index

This page exists so every shipped product feature has one obvious canonical
home.

## Start Here

- quickest entrypoint into the feature-first resource docs:
  [start_here.md](../start_here.md)

## Authoring Features

- rewritten resource docs home:
  [Resources Docs Home](../resources/README.md)
- first resource, shape choice, and common resource patterns:
  [Start Here](../resources/start-here/README.md),
  [Your First Resource](../resources/start-here/your-first-resource.md),
  [Choose A Resource Shape](../resources/start-here/choose-a-resource-shape.md), and
  [Common Resource Recipes](../resources/start-here/common-resource-recipes.md)
- fetch one record, one collection snapshot, or one paged line:
  [Fetching Data](../resources/fetching/README.md),
  [Fetch A Single Record](../resources/fetching/fetch-a-single-record.md),
  [Fetch A Collection](../resources/fetching/fetch-a-collection.md), and
  [Fetch A Paged List](../resources/fetching/fetch-a-paged-list.md)
- auth, request context, and request policy:
  [Request Auth And Context](../resources/fetching/request-auth-and-context.md)
  and [Request Policies](../resources/fetching/request-policies.md)
- create, update, remove, local patches, and effect profile choice:
  [Updating Data](../resources/updating/README.md),
  [Write A Resource](../resources/updating/write-a-resource.md),
  [Submit Patches And Replacements](../resources/updating/submit-patches-and-replacements.md),
  [Choose An Effect Profile](../resources/updating/choose-an-effect-profile.md), and
  [What Happens After A Write](../resources/updating/what-happens-after-a-write.md)
- mutation responses, partial canonical truth, fallback reconciliation, and
  response-owned read updates:
  [Handling Server Responses](../resources/responses/README.md),
  [Understand Mutation Responses](../resources/responses/understand-mutation-responses.md),
  [Handle Partial Canonical Truth](../resources/responses/handle-partial-canonical-truth.md), and
  [Handle Fallback Reconciliation](../resources/responses/handle-fallback-reconciliation.md)
- list identity, visible list truth, narrow row updates, and list summaries:
  [Working With Lists](../resources/lists/README.md),
  [List Shapes And Item Identity](../resources/lists/list-shapes-and-item-identity.md),
  [Visible Selection](../resources/lists/visible-selection.md), and
  [Update One Item Without Replacing Everything](../resources/lists/update-one-item-without-replacing-everything.md)
- line reuse, staleness, refresh, revalidation, and derived resource truth:
  [Caching And Refresh](../resources/caching/README.md),
  [How Resource Caching Works](../resources/caching/how-resource-caching-works.md),
  [Stale, Pending, And Settled State](../resources/caching/stale-pending-and-settled-state.md), and
  [Invalidation And Refresh](../resources/caching/invalidation-and-refresh.md)
- narrow patches, derived item views, summaries, and explicit aspect
  declarations:
  [Partial Updates And Derived Views](../resources/partial-updates/README.md),
  [How Partial Resource Updates Work](../resources/partial-updates/how-partial-resource-updates-work.md),
  [Automatic Derived Views](../resources/partial-updates/automatic-derived-views.md), and
  [Update One Region, Field, Or Item](../resources/partial-updates/update-one-region-field-or-item.md)
- upload preparation, processing jobs, and transfer result reads:
  [Uploads And Transfers](../resources/transfers/README.md),
  [Upload Files](../resources/transfers/upload-files.md),
  [Track Processing Jobs](../resources/transfers/track-processing-jobs.md), and
  [Understand Transfer Results](../resources/transfers/understand-transfer-results.md)
- builder-owned binary descriptors, descriptor kinds, multipart download
  handoff, and unavailable/incompatible download truth:
  [Downloads And Binary Data](../resources/downloads/README.md),
  [Offer Downloads](../resources/downloads/offer-downloads.md),
  [File, Media, And Export Downloads](../resources/downloads/file-media-and-export-downloads.md), and
  [Why A Download Is Unavailable](../resources/downloads/why-a-download-is-unavailable.md)
- grouped line reads, retained history, delivery/compatibility debugging, and
  exact recovery:
  [Inspecting And Debugging Resources](../resources/debugging/README.md),
  [Inspect A Resource Line](../resources/debugging/inspect-a-resource-line.md),
  [Check Status, Freshness, And History](../resources/debugging/check-status-settlement-and-history.md), and
  [Restore, Replay, And Recover](../resources/debugging/restore-replay-and-recover.md)
- effect profiles, effect envelopes, merge/rebase, and rollback:
  [Effects And Recovery](../resources/effects/README.md),
  [Branch-Native Effects](../resources/effects/branch-native-effects.md),
  [Effect Envelopes And Closeout](../resources/effects/effect-envelopes-and-closeout.md), and
  [Rollback And Recovery](../resources/effects/rollback-and-recovery.md)
- resource-backed form sources, settlement, drift, merge, mutation-response
  readback, and replay/restore:
  [Using Resources In Forms](../resources/forms/README.md),
  [Use A Resource As Form Source](../resources/forms/use-a-resource-as-form-source.md),
  [Handle Resource Drift And Merge](../resources/forms/handle-resource-drift-and-merge.md), and
  [Replay, Restore, And Reset Resource-Backed Forms](../resources/forms/replay-restore-and-reset-resource-backed-forms.md)
- route-resource declarations, prefetch/warmup, and projected/admitted route
  capability reads:
  [Using Resources In Routes](../resources/router/README.md),
  [Declare Route Resources](../resources/router/declare-route-resources.md),
  [Prefetch And Warmup Route Resources](../resources/router/prefetch-and-warmup-route-resources.md), and
  [Read Projected And Admitted Resource Capabilities](../resources/router/read-projected-and-admitted-resource-capabilities.md)
- raw family declarations, canonical identity, detail field/region/json-path
  declarations, and item/summary declarations:
  [Advanced Resource Modeling](../resources/advanced/README.md),
  [Resource Family Identity](../resources/advanced/resource-family-identity.md),
  [Detail Fields, Regions, And Json Paths](../resources/advanced/detail-fields-regions-and-json-paths.md), and
  [Item Aspects And Value Summaries](../resources/advanced/item-aspects-and-value-summaries.md)
- verification packages, topology proof, mutation-response support, and
  delivery/compatibility proof digests:
  [Verification And Proof](../resources/verification/README.md),
  [Verification Packages](../resources/verification/verification-packages.md),
  [Response Topology Proof](../resources/verification/response-topology-proof.md), and
  [Delivery And Compatibility Digests](../resources/verification/delivery-and-compatibility-digests.md)
- shared API roots and nested scopes:
  [Fetch And Write Resources](../resources/fetch-and-write.md)
- detail, list, paged, create, update, remove:
  [Fetch And Write Resources](../resources/fetch-and-write.md)
- request params:
  [Fetch And Write Resources](../resources/fetch-and-write.md)
- advanced `verb(...)`, `body<T>()`, `headers(...)`:
  [Fetch And Write Resources](../resources/fetch-and-write.md)
- auth, request context, retry policy, and continuation:
  [Request Posture And Policy](../resources/request-posture-and-policy.md)
- direct-array collections:
  [Collections And Delivery](../resources/collections-and-delivery.md)
- reconcile collections:
  [Collections And Delivery](../resources/collections-and-delivery.md)
- mutation response reconciliation, identity migration, and fallback posture:
  [Mutation Response Reconciliation](../resources/mutation-response-reconciliation.md)
- mutation response support categories and closeout honesty:
  [Mutation Response Closeout Matrix](../resource-contracts/mutation-response-closeout-matrix.md)

## Forms Features

- forms feature router:
  [Forms Overview](../forms/index.md)
- starting a form and choosing a source:
  [Your First Form](../forms/getting-started/your-first-form.md) and
  [Choosing A Form Source](../forms/getting-started/choosing-a-form-source.md)
- source truth, draft truth, effective values, fields, repeated items,
  attachment/evidence fields, and adapters:
  [Form State](../forms/state/README.md)
- semantic dirty truth, patch planning, complex edit forms, unchanged submit
  denial, and broad replacement posture:
  [Changes And Patching](../forms/changes/README.md),
  [Patching Complex Edit Forms](../forms/changes/patching-complex-edit-forms.md), and
  [Unchanged Forms And Submit Readiness](../forms/changes/unchanged-forms-and-submit-readiness.md)
- validation artifacts, parse failures, visible messages, async validation,
  server canonicalization, and source compatibility:
  [Validation](../forms/validation/README.md),
  [Async Validation](../forms/validation/async-validation.md), and
  [Server Canonicalization](../forms/validation/server-canonicalization.md)
- field, control, group, and section availability plus submit blockers and
  regulated requirements:
  [Availability And Permissions](../forms/availability/README.md),
  [Field And Control Availability](../forms/availability/field-and-control-availability.md), and
  [Readiness Blockers](../forms/availability/readiness-blockers.md)
- controller-local steps, step progress, and step navigation:
  [Steps And Multi-Step Forms](../forms/steps/README.md),
  [Controller-Local Steps](../forms/steps/controller-local-steps.md), and
  [Controller-Local Step Navigation](../forms/steps/controller-local-step-navigation.md)
- action catalogs, submit plans, action execution, and repeated-attempt
  posture:
  [Actions And Submit](../forms/actions/README.md),
  [Action Plans](../forms/actions/action-plans.md), and
  [Submit Actions](../forms/actions/submit-actions.md)
- label sizing, layout hints, measured row heights, and accessibility reads:
  [Layout And Accessibility](../forms/layout/README.md),
  [Label Size And Control Sizing](../forms/layout/label-size-and-control-sizing.md), and
  [Accessibility Artifacts](../forms/layout/accessibility-artifacts.md)
- dropdowns, comboboxes, search inputs, control availability, and external
  widget integration:
  [Inputs And Controls](../forms/inputs/README.md),
  [Dropdowns, Comboboxes, And Search](../forms/inputs/dropdowns-comboboxes-and-search.md), and
  [Control-Level Availability](../forms/inputs/control-level-availability.md)
- touched/visited/focused state, input-adapter support, and host facts:
  [Interaction And Host Facts](../forms/interaction/README.md),
  [Focus, Touch, And Visited State](../forms/interaction/focus-touch-and-visited-state.md), and
  [Host Facts](../forms/interaction/host-facts.md)
- entry bootstrap, visible lifecycle state, handoffs, exit posture, and
  external presentation:
  [Lifecycle](../forms/lifecycle/README.md),
  [Entry Bootstrap](../forms/lifecycle/entry-bootstrap.md), and
  [Exit Posture](../forms/lifecycle/exit-posture.md)
- attachment presentation, transfer readback, media visibility, and
  evidence-field media behavior:
  [Attachments And Media](../forms/media/README.md),
  [Attachment Transfers](../forms/media/attachment-transfers.md), and
  [Media Visibility](../forms/media/media-visibility.md)
- resource-line source truth, visible selection, settlement, drift, merge,
  reset, replay/restore, mutation-response readback, and resource-backed action
  execution:
  [Resource-Backed Forms](../forms/resource-backed/README.md),
  [Resource Line Source](../forms/resource-backed/resource-line-source.md), and
  [Resource Action Execution](../forms/resource-backed/resource-action-execution.md)
- collaboration modes, lock and lease posture, presence/comments, and
  branch-backed collaboration:
  [Collaboration](../forms/collaboration/README.md),
  [Locks And Leases](../forms/collaboration/locks-and-leases.md), and
  [Resource-Backed Collaboration](../forms/collaboration/resource-backed-collaboration.md)
- route-authority handoff, draft continuity, and route-coupled action denial:
  [Route-Coupled Forms](../forms/route-coupling/README.md),
  [Route Authority Handoff](../forms/route-coupling/route-authority-handoff.md), and
  [Continuity Audit](../forms/route-coupling/continuity-audit.md)
- current-state form debugging, retained histories, and proof-bearing closeout:
  [Diagnostics And History](../forms/diagnostics/README.md),
  [Diagnostics Summary](../forms/diagnostics/diagnostics-summary.md), and
  [Verification](../forms/verification/README.md)

## Patch / Delivery Features

- family-owned `patch` helpers:
  [Collections And Delivery](../resources/collections-and-delivery.md)
- family-owned `delivery` helpers:
  [Collections And Delivery](../resources/collections-and-delivery.md)

## Router Features

- router overview:
  [Router Overview](../router/index.md)
- canonical route and URL authority:
  [Canonical URL Authority](../router/authority/canonical_url_authority.md)
- route schema authoring:
  [Route Schema Authoring](../router/projection/route_schema_authoring.md)
- route projection:
  [Projected Candidates](../router/projection/projected_candidates.md)
- route admission and route outcomes:
  [Admit](../router/admission/admit.md) and
  [Route Outcomes](../router/admission/route_outcomes.md)
- browser-history truth, retained entries, and auditability:
  [Browser History Story](../router/history/browser_history_story.md),
  [History Inspection](../router/history/history_inspection.md), and
  [Navigation Auditability](../router/history/navigation_auditability.md)
- stale-link fallback and nearest-valid route truth:
  [Stale Deep Link Recovery](../router/recovery/stale_deep_link_recovery.md)
  and [Nearest Valid Truth](../router/recovery/nearest_valid_truth.md)
- speculative route branches, dirty exit, and pending visibility:
  [Speculative Branch Plans](../router/speculation/speculative_branch_plans.md),
  [Dirty Exit](../router/speculation/dirty_exit.md), and
  [Visible Projection](../router/speculation/visible_projection.md)
- breadcrumb declarations, carried ancestry, and restore-backed crumb return:
  [Breadcrumb Declarations](../router/breadcrumbs/breadcrumb_declarations.md),
  [Carried Provenance](../router/breadcrumbs/carried_provenance.md), and
  [Breadcrumb Replay And Restore](../router/breadcrumbs/breadcrumb_replay_restore.md)
- exact route restore, replay, and outlet composition preservation:
  [Restore Boundaries](../router/restore/restore_boundaries.md),
  [Replay History](../router/restore/replay_history.md), and
  [Outlet Composition Restore](../router/restore/outlet_composition_restore.md)
- route-local resources, prefetch, warmup, and host warmup ingress:
  [Route Resource Declarations](../router/resources/route_resource_declarations.md),
  [Resource Prefetch](../router/resources/resource_prefetch.md), and
  [Warmup Ingress](../router/resources/warmup_ingress.md)
- route transitions, pending visibility, and continuity-preserved visible
  truth:
  [Transition Artifacts](../router/transitions/transition_artifacts.md),
  [Pending Visibility](../router/transitions/pending_visibility.md), and
  [Continuity Preservation](../router/transitions/continuity_preservation.md)
- hydration handoff and browser-authority ownership:
  [Hydration Handoff](../router/boundaries/hydration_handoff.md) and
  [Browser Authority Coherence](../router/boundaries/browser_authority_coherence.md)
- worker-first default placement, host-worker route truth, and worker fallback:
  [Worker-First Default](../router/runtime_placement/worker_first_default.md),
  [Host And Worker Boundary](../router/runtime_placement/host_worker_boundary.md), and
  [Worker Navigation Auditability](../router/runtime_placement/worker_navigation_auditability.md)
- route-authority handoff, draft continuity, and public continuity audit for
  route-coupled forms:
  [Route Authority Handoff](../router/forms/route_authority_handoff.md),
  [Draft Continuity](../router/forms/draft_continuity.md), and
  [Continuity Audit](../router/forms/continuity_audit.md)
- router diagnostics, proof packages, provenance vocabulary, and browser
  boundary artifact meaning:
  [Diagnostics Surfaces](../router/diagnostics/diagnostics_surfaces.md),
  [Verification Packages](../router/diagnostics/verification_packages.md),
  [Provenance Statuses](../router/diagnostics/provenance_statuses.md), and
  [Coherence And Boundary Artifacts](../router/diagnostics/coherence_and_boundary_artifacts.md)

## Transfer Features

- signed upload:
  [Upload Files](../resources/transfers/upload-files.md)
- multipart upload:
  [Upload Files](../resources/transfers/upload-files.md)
- deferred processing:
  [Track Processing Jobs](../resources/transfers/track-processing-jobs.md)

## Download Features

- builder-owned `.downloads(...)`:
  [Offer Downloads](../resources/downloads/offer-downloads.md)
- binary descriptors:
  [Describe Binary Values](../resources/downloads/describe-binary-values.md)
- multipart downloads:
  [File, Media, And Export Downloads](../resources/downloads/file-media-and-export-downloads.md)

## Read / Debug Features

- `line.summary()`:
  [Line Inspection](../resources/line-inspection.md)
- request inspection:
  [Line Inspection](../resources/line-inspection.md)
- diagnostics and history:
  [Line Inspection](../resources/line-inspection.md)
- upload, processing, and download reads:
  [Line Inspection](../resources/line-inspection.md)
- retained history, exact restore, replay availability, and verification:
  [History And Restore](../resource-contracts/history-and-restore.md)
- branch-native optimistic effects, response-lens topology declarations, JSON
  effects, advanced topology effects, and UI lifecycle events:
  [Branch-Native Resource Effects](../resources/branch-native-effects.md)
- sealed effect envelopes:
  [Effect Envelope Contract](../resource-contracts/effect-envelope.md)
- resource effect merge and rebase:
  [Effect Merge And Rebase](../resources/merge-and-rebase.md)
- resource effect rollback:
  [History And Restore](../resource-contracts/history-and-restore.md)
- response topology proof:
  [Response Topology Proof](../resource-contracts/response-topology-proof.md)
- JSON path item-aspect effects:
  [JSON Path Effects](../resources/json-effects.md)
- effect profile closeout matrices:
  [Effect Closeout Matrix](../resource-contracts/closeout-matrix.md)
- mutation-response closeout matrix:
  [Mutation Response Closeout Matrix](../resource-contracts/mutation-response-closeout-matrix.md)

## External / Compatibility Features

- external resource definitions:
  [External Delivery And Compatibility](../resources/external-delivery-and-compatibility.md)
- basis refresh and compatibility delivery:
  [External Delivery And Compatibility](../resources/external-delivery-and-compatibility.md)

## Escape Hatch

- raw `signals.resource.*(...)` family declarations:
  [Raw Escape Hatch](../resources/raw-escape-hatch.md)

## Task-First Companion

- [Resource Recipes](./recipes.md)
