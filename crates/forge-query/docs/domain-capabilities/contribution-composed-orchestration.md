# Contribution-Composed Orchestration

## What This Feature Is

Contribution-composed orchestration is the public Query lane that lets you run
one declaration-entry operation and attach declaration-scoped domain-capability
contributions in the same call.

Use it when your app already knows the declaration it wants to make and also
knows that the run should carry support, explanation, admission, or workflow
contribution posture at the same time.

This feature does not replace declaration entry, and it does not replace the
underlying contribution-authoring system. It composes them into one Query-owned
front door while keeping the two proof chains separate and inspectable.

## Why You Use It

- keep declaration entry and declaration-scoped contribution authoring on one
  admitted-handle surface
- preserve declaration success plus contribution denial as a real typed result
- materialize contribution summaries without rebuilding the explicit
  declaration-bound authoring pipeline by hand
- inspect retained declaration and contribution evidence together
- keep the compact ordinary outcome lane without flattening contribution
  posture into generic declaration success

## Stable Entry Points

Admitted-handle entry points:

- `orchestrate_declaration_with_contributions(...)`
- `orchestrate_declaration_with_contributions_outcome(...)`
- `orchestrate_declaration_with_contributions_checked(...)`
- `orchestrate_declaration_with_contributions_proof(...)`

Input and intent types:

- `ForgeQueryContributionComposedOrchestrationInput<D, I>`
- `ForgeQueryContributionIntent`
- `ForgeQueryContributionComposedMaterializationPolicy`

Result and proof types:

- `ForgeQueryContributionComposedOrchestration<D, I>`
- `ForgeQueryContributionComposedContribution`
- `ForgeQueryContributionComposedSummary`
- `ForgeQueryContributionComposedOrchestrationOutcome<D, I>`
- `ForgeQueryContributionComposedOrchestrationChecked<D, I>`
- `ForgeQueryContributionComposedOrchestrationPosture<D, I>`
- `ForgeQueryContributionComposedOrchestrationTranscript<D, I>`

## Core Mental Model

Think of this feature as one composed run with two retained stories inside it:

1. Query lowers the declaration through the normal declaration-entry path until
   it has an envelope.
2. Query binds each contribution intent to the declaration-scoped contribution
   target for that retained declaration.
3. Query evaluates, admits, and optionally materializes contribution summaries.

That means one call can still stop in more than one honest place:

- the declaration can defer, deny, go stale, or require rebind before any
  contribution work happens
- the declaration can succeed and the contribution can still deny, fail, or
  require rebind
- both declaration and contributions can succeed and produce one composed
  artifact

The important rule is:

- declaration success does not automatically mean contribution success

This surface keeps that distinction typed.

## How It Executes

`ForgeQueryContributionComposedOrchestrationInput<D, I>` owns three things:

- one declaration input
- zero or more typed contribution intents
- one materialization policy

The current contribution intents are:

- `ForgeQueryContributionIntent::admission(...)`
- `ForgeQueryContributionIntent::support(...)`
- `ForgeQueryContributionIntent::explanation(...)`
- `ForgeQueryContributionIntent::workflow(...)`

The execution shape is:

1. `declare_review_and_progress(...)`
2. `orchestrate_envelope_from_progressed(...)`
3. bind each contribution intent to the declaration-bound target
4. evaluate and admit each contribution
5. optionally materialize contribution summaries
6. return one bound composed artifact or one typed non-success posture

This feature does not silently continue into continuation preparation, signal
execution, or runtime execution. It stops at declaration envelope plus
contribution posture.

## Small Example

```rust
let composed = handle.orchestrate_declaration_with_contributions(
    ForgeQueryContributionComposedOrchestrationInput::new(
        geometry_session.attach_material_for_active_face_selection()?,
    )
    .with_contribution(ForgeQueryContributionIntent::support(
        ForgeQuerySupportContributionAuthoring::declaration_traceability(
            "domain.traceability.face",
            "face selection remains traceable through declaration entry",
        ),
    )),
)?;

let _ = composed.envelope().envelope_digest();
let _ = composed.contributions()[0].evidence().evidence_digest();
```

## Real Example

```rust
let outcome = handle.orchestrate_declaration_with_contributions_outcome(
    ForgeQueryContributionComposedOrchestrationInput::new(
        geometry_session.publish_boundary_change_for_active_face_selection()?,
    )
    .with_contribution(ForgeQueryContributionIntent::workflow(
        ForgeQueryWorkflowContributionAuthoring::preview_only(
            "domain.workflow.face",
            "preview workflow remains read-only at declaration entry",
        ),
    ))
    .with_contribution(ForgeQueryContributionIntent::support(
        ForgeQuerySupportContributionAuthoring::declaration_traceability(
            "domain.traceability.face",
            "face selection remains traceable through declaration entry",
        ),
    ))
    .materialize_summaries_with_profile(profile),
);

match outcome {
    ForgeQueryOrdinaryOutcome::Bound(composed) => {
        let _ = composed.envelope().envelope_digest();
        let _ = composed.contribution_composition().contribution_digest();
        let _ = composed.contributions()[0].contribution_category();
        let _ = composed.contributions()[0].semantic_posture();
        let _ = composed.materialized_artifacts();
    }
    ForgeQueryOrdinaryOutcome::Denied(posture) => {
        let _ = posture.reason();
        let _ = posture.checked_topology().contribution_composed_kind();
        let _ = posture
            .checked_topology()
            .contribution_composed_linked_artifacts();
    }
    ForgeQueryOrdinaryOutcome::Deferred(posture) => {
        let _ = posture.reason();
        let _ = posture.next_step();
    }
    other => {
        let _ = other;
    }
}
```

Good to know:

- if the declaration succeeds and the contribution denies, the composed lane
  returns `ContributionDenied`, not `Bound`
- if the declaration support posture is deferred before declaration entry can
  start, the composed lane returns `Deferred`, not `DeclarationDenied`

## How It Relates To Other Features

- Use [Declaration Entry Orchestration](./declaration-entry-orchestration.md)
  when you only need the declaration-side envelope-ceiling run.
- Use [Ordinary Outcomes](./ordinary-outcomes.md) when you want the compact
  public result lane over this composed surface.
- Use [Configured Domain Handles](./configured-domain-handles.md) for the
  admitted-handle entry points that own this run.
- Use the `support/`, `workflow/`, `admission/`, and `explanation/` docs in
  this tree when you need the standalone contribution-authoring model behind
  the composed lane.
- Use [Continuation Pipeline](./continuation-pipeline.md) or
  [Signal Compatibility Orchestration](./signal-compatibility-orchestration.md)
  only after you actually need continuation or signal-facing follow-up. This
  feature does not execute either one.

## Inspection And Debugging

Use the bound artifact when you need the success surface:

- `envelope()`
- `contribution_composition()`
- `contributions()`
- `materialized_artifacts()`
- `composed_digest()`

Each bound contribution exposes:

- `evidence()`
- `category_family()`
- `contribution_category()`
- `semantic_posture()`
- `support_outcome_kind()`
- `request_digest()`
- `summary()`

Use the checked lane when you need typed stop posture:

- `Deferred`
- `DeclarationDenied`
- `ContributionDenied`
- `Stale`
- `RebindRequired`
- `Unsupported`
- `Failed`

The posture surface exposes:

- `kind()`
- `stop_stage()`
- `reason()`
- `linked_artifacts()`
- `contribution_digest()`

Use the proof lane when you need retained proof over the whole composed run:

- `request_digest()`
- `outcome()`
- `linked_artifacts()`
- `contribution_digest()`
- `into_checked()`

## Anti-Patterns

- treating declaration success as if it implies contribution success
- using this feature when you really need contribution authoring on a
  non-declaration target
- teaching this as if it replaced the underlying contribution-authoring model
- assuming materialized summaries imply continuation preparation or lower
  runtime execution
- flattening `DeclarationDenied` and `ContributionDenied` into the same app
  behavior
- parsing `reason()` when the checked or ordinary posture kind already carries
  the decision you need

## Current Limits

- the current shipped surface binds contributions only to declaration-scoped
  targets derived from the retained canonical declaration
- the current materialization policy is summary-only or none
- this feature stops at declaration envelope plus contribution posture
- this feature does not prepare continuation, execute continuation, or execute
  Signal work
- grouped contribution-composed authoring is not part of this surface yet

## Related Docs

- [Configured Domain Handles](./configured-domain-handles.md)
- [Ordinary Outcomes](./ordinary-outcomes.md)
- [Declaration Entry Orchestration](./declaration-entry-orchestration.md)
- [Continuation Pipeline](./continuation-pipeline.md)
- [Signal Compatibility Orchestration](./signal-compatibility-orchestration.md)
- [Declaration-Scoped Support And Traceability](./support/declaration-scoped-support-and-traceability.md)
- [Advisory And Violation Contributions](./admission/advisory-and-violation-contributions.md)
- [Preview Inspection And Mutation Planning](./workflow/preview-inspection-and-mutation-planning.md)
- [Lower-Runtime Explanation Contributions](./explanation/lower-runtime-explanation-contributions.md)
