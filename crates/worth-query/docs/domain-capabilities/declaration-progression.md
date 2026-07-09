# Declaration Progression

## What This Feature Is

Declaration progression is the Query-owned proof-bearing strengthening step that
happens after legality review.

The important split is:

- family admission proves the declaration family is admitted here
- legality review proves the canonical declaration is structurally legal here
- declaration progression proves that legality-cleared declaration can advance
  into one admitted declaration progression artifact, or into a typed
  deferred/denied/stale/rebind/failed outcome

This is where declaration-side work becomes a real `worth-proof` progression
instead of stopping at legality evidence.

## Why You Use It

- strengthen legality-cleared declarations into proof-bearing admitted
  progression artifacts
- preserve `Deferred`, `Denied`, `Stale`, `RebindRequired`, and `Failed` as
  first-class typed outcomes
- keep progression anchored on retained legality evidence instead of reopening
  family meaning or legality
- hand later Query features one admitted declaration proof instead of a loose
  "already checked earlier" assumption

## Stable Entry Points

- `WorthQueryDeclarationProgressionContract`
- `WorthQueryDeclarationProgressionContractClass`
- `WorthQueryAdmittedConfiguredDomainHandle::declaration_progression_recipe(...)`
- `WorthQueryAdmittedConfiguredDomainHandle::progress_declaration(...)`
- `WorthQueryAdmittedConfiguredDomainHandle::progress_declaration_checked(...)`
- `WorthQueryAdmittedConfiguredDomainHandle::progress_declaration_recipe(...)`
- `WorthQueryAdmittedConfiguredDomainHandle::progress_declaration_recipe_checked(...)`
- `WorthQueryAdmittedConfiguredDomainHandle::declare_review_and_progress(...)`
- `WorthQueryAdmittedConfiguredDomainHandle::describe_foundational(...)`
- `WorthQueryAdmittedConfiguredDomainHandle::plan_routes_from_progressed(...)`
- `WorthQueryAdmittedConfiguredDomainHandle::plan_routes_from_progressed_with_intent(...)`
- `WorthQueryDeclarationProgressionRecipe`
- `WorthQueryDeclarationProgressionChecked`
- `WorthQueryDeclarationProgressionTerminalError`
- `WorthQueryAdmittedDeclarationProgression`
- `WorthQueryDeclarationProgressionDeferred`
- `WorthQueryDeclarationProgressionDenied`
- `WorthQueryDeclarationProgressionStale`
- `WorthQueryDeclarationProgressionRebindRequired`
- `WorthQueryDeclarationProgressionFailed`
- `WorthQueryDeclarationProgressionOutcomeView`

Good to know:

- progression starts from legality evidence, not from canonical declarations
- the handle stays the entry surface because the admitted world still matters
- progression reuses `worth-proof` stage and outcome vocabulary instead of
  inventing a separate Query stage dialect
- progression contracts can depend on both retained handle identity and
  retained admitted operating-context identity

## API Reference

Family marker contract:

- `progression_contract(handle_identity_digest, operating_context_identity_digest) -> WorthQueryDeclarationProgressionContract`

Admitted-handle progression entry points:

- `declaration_progression_recipe(legal) -> WorthQueryDeclarationProgressionRecipe<D, I>`
- `progress_declaration(legal) -> Result<WorthQueryAdmittedDeclarationProgression<D, I>, WorthQueryDeclarationProgressionTerminalError<D, I>>`
- `progress_declaration_checked(legal) -> WorthQueryDeclarationProgressionChecked<D, I>`
- `progress_declaration_recipe(recipe) -> Result<WorthQueryAdmittedDeclarationProgression<D, I>, WorthQueryDeclarationProgressionTerminalError<D, I>>`
- `progress_declaration_recipe_checked(recipe) -> WorthQueryDeclarationProgressionChecked<D, I>`
- `declare_review_and_progress(input) -> Result<WorthQueryAdmittedDeclarationProgression<D, I>, WorthQueryDeclarationEntryProgressionError<D, I>>`

Relevant admitted-handle identity inspection:

- `handle_identity_digest() -> &str`
- `operating_context_identity_digest() -> &str`

Foundational-evidence entry that can consume progression truth:

- `describe_foundational(subject) -> Result<WorthQueryDeclarationFoundationalEvidence<D, I>, WorthQueryDeclarationFoundationalEvidenceDenial<D, I>>`

Route-planning entries that can consume admitted progression truth:

- `plan_routes_from_progressed(progressed) -> Result<WorthQueryDeclarationRoutePlan<D, I>, WorthQueryDeclarationRoutePlanTerminalError<D, I>>`
- `plan_routes_from_progressed_with_intent(progressed, intent) -> Result<WorthQueryDeclarationRoutePlan<D, I>, WorthQueryDeclarationRoutePlanTerminalError<D, I>>`
- `bind_route_request_from_context(request) -> WorthQueryBindingOutcome<WorthQueryDeclarationRoutePlanInput<D, I>>`
- `bind_route_request_from_context_checked(request) -> WorthQueryBindingChecked<WorthQueryDeclarationRoutePlanInput<D, I>>`
- `bind_route_request_from_context_proof(request) -> WorthQueryBindingTranscript<WorthQueryDeclarationRoutePlanInput<D, I>>`
- `bind_route_from_target(request) -> WorthQueryBindingOutcome<WorthQueryDeclarationRoutePlanInput<D, I>>`
- `bind_route_from_target_checked(request) -> WorthQueryBindingChecked<WorthQueryDeclarationRoutePlanInput<D, I>>`
- `bind_route_from_target_proof(request) -> WorthQueryBindingTranscript<WorthQueryDeclarationRoutePlanInput<D, I>>`
- `bind_receipt_from_target(request) -> WorthQueryBindingOutcome<WorthQueryDeclarationReceiptInput<D, I>>`
- `bind_receipt_from_target_checked(request) -> WorthQueryBindingChecked<WorthQueryDeclarationReceiptInput<D, I>>`
- `bind_receipt_from_target_proof(request) -> WorthQueryBindingTranscript<WorthQueryDeclarationReceiptInput<D, I>>`
- `bind_envelope_from_target(request) -> WorthQueryBindingOutcome<WorthQueryDeclarationEnvelopeInput<D, I>>`
- `bind_envelope_from_target_checked(request) -> WorthQueryBindingChecked<WorthQueryDeclarationEnvelopeInput<D, I>>`
- `bind_envelope_from_target_proof(request) -> WorthQueryBindingTranscript<WorthQueryDeclarationEnvelopeInput<D, I>>`

Recipe inspection:

- `stage() -> RecipeStageKind`
- `declaration_family_key() -> &'static str`

Checked progression outcomes:

- `WorthQueryDeclarationProgressionChecked::Admitted(WorthQueryAdmittedDeclarationProgression<D, I>)`
- `WorthQueryDeclarationProgressionChecked::Deferred(WorthQueryDeclarationProgressionDeferred<D, I>)`
- `WorthQueryDeclarationProgressionChecked::Denied(WorthQueryDeclarationProgressionDenied<D, I>)`
- `WorthQueryDeclarationProgressionChecked::Stale(WorthQueryDeclarationProgressionStale<D, I>)`
- `WorthQueryDeclarationProgressionChecked::RebindRequired(WorthQueryDeclarationProgressionRebindRequired<D, I>)`
- `WorthQueryDeclarationProgressionChecked::Failed(WorthQueryDeclarationProgressionFailed<D, I>)`

Terminal progression errors:

- `WorthQueryDeclarationProgressionTerminalError::Deferred(...)`
- `WorthQueryDeclarationProgressionTerminalError::Denied(...)`
- `WorthQueryDeclarationProgressionTerminalError::Stale(...)`
- `WorthQueryDeclarationProgressionTerminalError::RebindRequired(...)`
- `WorthQueryDeclarationProgressionTerminalError::Failed(...)`

Combined entry outcome:

- `WorthQueryDeclarationEntryProgressionError::Entry(WorthQueryDeclarationAdmissionOrLegalityError<D, I>)`
- `WorthQueryDeclarationEntryProgressionError::Progression(WorthQueryDeclarationProgressionTerminalError<D, I>)`

Admitted progression inspection:

- `legality_evidence() -> &WorthQueryDeclarationLegalityEvidence<D, I>`
- `canonical_declaration() -> &WorthQueryCanonicalDeclarationArtifact<D, I>`
- `support_report() -> &WorthQueryDeclarationFamilySupportReport<D, I::Family>`
- `legality_contract() -> WorthQueryDeclarationLegalityContract`
- `aspect_contract() -> &WorthQueryDeclarationAspectContract`
- `reviewed_aspect_coverage() -> &WorthQueryDeclarationAspectCoverage`
- `declaration_family_key() -> &'static str`
- `progression_digest() -> &str`
- `outcome() -> WorthQueryDeclarationProgressionOutcomeView`
- `stage() -> RecipeStageKind`
- `binding_target() -> WorthQueryAdmittedDeclarationProgressionBindingTarget`

Deferred, denied, and failed inspection:

- `legality_evidence() -> &WorthQueryDeclarationLegalityEvidence<D, I>`
- `support_report() -> &WorthQueryDeclarationFamilySupportReport<D, I::Family>`
- `legality_contract() -> WorthQueryDeclarationLegalityContract`
- `progression_contract() -> WorthQueryDeclarationProgressionContract`
- `declaration_family_key() -> &'static str`
- `progression_digest() -> &str`
- `outcome() -> WorthQueryDeclarationProgressionOutcomeView`

Stale and rebind inspection:

- `legality_evidence() -> &WorthQueryDeclarationLegalityEvidence<D, I>`
- `support_report() -> &WorthQueryDeclarationFamilySupportReport<D, I::Family>`
- `legality_contract() -> WorthQueryDeclarationLegalityContract`
- `declaration_family_key() -> &'static str`
- `progression_digest() -> &str`
- `outcome() -> WorthQueryDeclarationProgressionOutcomeView`
- `stage() -> RecipeStageKind`

Progression outcome inspection:

- `kind() -> ProofOutcomeKind`

Proof vocabulary reused directly:

- `ProofOutcomeKind`
- `RecipeStageKind`

Progression contract presets:

- `admitted_current() -> WorthQueryDeclarationProgressionContract`
- `deferred_support() -> WorthQueryDeclarationProgressionContract`
- `denied_boundary() -> WorthQueryDeclarationProgressionContract`
- `stale_readable() -> WorthQueryDeclarationProgressionContract`
- `rebind_required() -> WorthQueryDeclarationProgressionContract`
- `failed_transition() -> WorthQueryDeclarationProgressionContract`

Progression contract inspection:

- `class() -> WorthQueryDeclarationProgressionContractClass`
- `reason() -> &'static str`

## Core Mental Model

Think of progression as the first declaration-side proof flow after legality:

1. the admitted handle proves the operating world
2. legality evidence proves canonical declaration identity, family admission,
   and structural legality
3. the family marker contributes one explicit progression contract
4. Query resolves, lowers, and admits the declaration through `worth-proof`
   vocabulary

If progression succeeds, you get one admitted declaration progression artifact.
If it does not, you still get a typed progression truth rather than a generic
failure.

That admitted progression artifact is also now one shared retained binding
target. Route, receipt, envelope, and progressed-entry orchestration surfaces
can bind from it without inventing a second local binding story, and that same
retained target seam is the one later continuation and grouped-authoring work
must extend.

That shared binding target now carries retained aspect contract and reviewed
aspect coverage alongside progression identity so later consumers can prefer
semantic fit before broader artifact precedence.

Reviewed coverage is carried forward exactly as legality proved it. If a slice
was masked at support or legality time, progression preserves that masking in
its retained binding semantics instead of widening it into visible coverage.

## How It Executes

1. define `progression_contract(...)` on the family marker when the default
   admitted progression is not enough
2. produce legality evidence through `review_legality(...)` or
   `declare_and_review(...)`
3. call one of the progression entry points on the admitted handle
4. Query builds one progression payload from:
   - legality evidence
   - retained handle identity
   - retained admitted operating-context identity
   - declaration digest
   - support digest
   - legality digest
5. Query resolves the progression recipe
6. Query lowers and admits it, or returns a typed deferred/denied/stale/rebind
   or failed outcome

The ordinary convenience lane `declare_review_and_progress(...)` preserves the
same structure. It still performs:

1. family admission
2. canonicalization
3. legality review
4. progression

## Small Example

```rust
use worth_proof::ProofOutcomeKind;
use worth_query::facade::WorthQueryDeclarationProgressionChecked;

match handle.progress_declaration_checked(handle.declare_and_review(
    AttachMaterialForActiveFaceSelection,
)?) {
    WorthQueryDeclarationProgressionChecked::Admitted(progressed) => {
        assert_eq!(progressed.outcome().kind(), ProofOutcomeKind::Success);
    }
    WorthQueryDeclarationProgressionChecked::RebindRequired(progress) => {
        assert_eq!(progress.outcome().kind(), ProofOutcomeKind::RebindRequired);
    }
    other => panic!("unexpected progression outcome: {:?}", std::mem::discriminant(&other)),
}
```

## Real Example

```rust
use worth_proof::{ProofOutcomeKind, RecipeStageKind};
use worth_query::facade::{
    WorthQueryApplicationFacade, WorthQueryCapabilityFamily,
    WorthQueryConfigSectionFamily, WorthQueryDeclarationCanonicalEntry,
    WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationInput,
    WorthQueryDeclarationLegalityContract, WorthQueryDeclarationProgressionChecked,
    WorthQueryDeclarationProgressionContract, WorthQueryDomainEntryMarker,
    WorthQueryDomainOperatingContext, WorthQueryNeighborhoodCapableGrouping,
    WorthQueryRelationalTruthAuthority, WorthQuerySignalCompatiblePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomain;

impl WorthQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str { "worth.geometry" }
    fn display_name(&self) -> &'static str { "Worth Geometry" }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CollaborativeWorld;

impl WorthQueryDomainOperatingContext<GeometryDomain> for CollaborativeWorld {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::HistoricalEvaluation]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[WorthQueryConfigSectionFamily::Relational]
    }

    fn context_identity_digest(&self) -> String {
        "geometry.collaborative".to_string()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct AttachFaceMaterial;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for AttachFaceMaterial {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "attach-face-material"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn progression_contract(
        _handle_identity_digest: &str,
        operating_context_identity_digest: &str,
    ) -> WorthQueryDeclarationProgressionContract {
        if operating_context_identity_digest.contains("restricted") {
            WorthQueryDeclarationProgressionContract::rebind_required()
        } else {
            WorthQueryDeclarationProgressionContract::admitted_current()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AttachMaterialForActiveFaceSelection;

impl WorthQueryDeclarationInput<GeometryDomain> for AttachMaterialForActiveFaceSelection {
    type Family = AttachFaceMaterial;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![
            WorthQueryDeclarationCanonicalEntry::text(
                "selection_scope",
                "active-face-selection",
            ),
            WorthQueryDeclarationCanonicalEntry::text(
                "progression_intent",
                "material-attachment-from-current-selection",
            ),
        ]
    }
}

let query = WorthQueryApplicationFacade::runtime_backed_default();
let handle = query
    .domain(GeometryDomain)
    .with_operating_context(CollaborativeWorld)
    .validate()?
    .admit()?;

let legal = handle.declare_and_review(AttachMaterialForActiveFaceSelection)?;
let recipe = handle.declaration_progression_recipe(legal);
assert_eq!(recipe.stage(), RecipeStageKind::Unresolved);

let admitted = handle.progress_declaration_recipe(recipe)?;
assert_eq!(admitted.outcome().kind(), ProofOutcomeKind::Success);
assert_eq!(admitted.stage(), RecipeStageKind::Admitted);
```

What this example is showing:

- progression starts from legality evidence, not from canonical declarations
- the lower-level recipe lane and the convenience lane describe the same
  proof-bearing boundary
- world-sensitive progression posture is carried through an explicit contract
- the public geometry story is current selection and current context, not raw
  identifier passing

## Aspect Semantics

Progression is the first retained aspect-aware declaration-entry
artifact later product binding can trust. `binding_target()` is no longer just
about progression digest and family identity. It must carry aspect-qualified
admissible truth so route, receipt, envelope, and orchestration surfaces can
narrow semantically instead of reconstructing the same granularity locally.

## How It Relates To Other Features

- [Canonical Domain Declarations](./canonical-domain-declarations.md) produce
  the canonical declaration artifact carried into legality
- [Declaration Family Capability Matrix](./declaration-family-capability-matrix.md)
  admits the family before progression can begin
- [Declaration Legality](./declaration-legality.md) produces the legality
  evidence progression consumes
- [Declaration Foundational Evidence](./declaration-foundational-evidence.md)
  describes admitted, deferred, denied, stale, rebind, and failed progression
  truths through shared foundational artifacts
- [Declaration Route Plans](./declaration-route-plan.md) consumes admitted
  progression proof plus matching foundational evidence and turns that retained
  declaration truth into one explicit lower-authority route set
- [Configured Domain Handles](./configured-domain-handles.md) own the admitted
  world that scopes progression binding targets and later retained artifact
  binding
- [Typed Binding Pipeline](./typed-binding-pipeline.md) turns current
  progression context or a retained progression target into the next explicit
  route/receipt/envelope input without reopening progression truth by hand

## Inspection And Debugging

Use these surfaces when reviewing progression:

- `declaration_progression_recipe(...)`
- `progress_declaration_checked(...)`
- `progress_declaration_recipe_checked(...)`
- `declare_review_and_progress(...)`
- `progressed.progression_digest()`
- `progressed.outcome().kind()`
- `progressed.stage()`
- `progressed.legality_evidence()`
- `progressed.binding_target()`

Use them to answer:

- whether a declaration progressed, deferred, denied, went stale, or requires
  rebind
- whether two equivalent legality-cleared declarations converged to the same
  progression digest
- whether a world-sensitive family changed progression posture because the
  admitted operating world changed
- which shared retained target identity later route/receipt/envelope and
  orchestration consumers should bind from

## Anti-Patterns

- attempting progression from canonical declarations instead of legality
  evidence
- treating stale or rebind outcomes as generic denial
- rebuilding progression meaning from family labels or legality folklore
- constructing progression artifacts directly instead of entering through the
  admitted handle

## Current Limits

Declaration progression now gives other Query declaration features a proof
artifact over legality-cleared declarations. It still does not decide:

- lower-authority route planning
- Query boundary receipts
- public Query boundary envelopes
- public Query relational truth routing
- public Query bridge continuation routing
- public Query signal compatibility classification
- grouped execution semantics
- continuation execution

## Related Docs

- [Canonical Domain Declarations](./canonical-domain-declarations.md)
- [Declaration Family Capability Matrix](./declaration-family-capability-matrix.md)
- [Declaration Legality](./declaration-legality.md)
- [Declaration Foundational Evidence](./declaration-foundational-evidence.md)
- [Declaration Route Plans](./declaration-route-plan.md)
- [Declaration Boundary Receipts](./declaration-boundary-receipts.md)
- [Declaration Boundary Envelopes](./declaration-boundary-envelopes.md)
- [Declaration Relational Truth Routing](./declaration-relational-truth-routing.md)
- [Declaration Bridge Continuation Routing](./declaration-bridge-continuation-routing.md)
- [Declaration Signal Compatibility](./declaration-signal-compatibility.md)
- [Configured Domain Handles](./configured-domain-handles.md)
- [Typed Binding Pipeline](./typed-binding-pipeline.md)
- [Domain Capabilities](./README.md)
