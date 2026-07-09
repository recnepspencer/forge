# Turn A Stop Into A Recovery Action

## What This Recipe Covers

This recipe shows the shortest path from a typed Query stop to one recommended
next action.

Use it when your app does not want to hand-roll repair logic for every feature
family.

## When To Use It

Use this when:

- you already have an ordinary outcome, checked result, or proof-visible
  transcript
- the next question is "who owns the fix and what should happen next?"

Do not use this when:

- you still need to inspect the feature-specific result itself first
- you expect recovery to rerun the operation for you

## The Smallest Useful Path

```rust
let outcome = handle.orchestrate_declaration_with_contributions_outcome(input);

if let Some(recovery) = handle.recover_from_outcome(&outcome) {
    let _ = recovery.recommended_action();
    let _ = recovery.authority_surface();
}
```

Use this when you are already on the compact ordinary lane and you need one
typed repair answer.

## A Richer Path For Grouped Stops

```rust
let proof = handle.orchestrate_grouped_proof(declaration);

let recovery = handle
    .recover_from_grouped_orchestration_proof(proof)
    .expect("grouped stop should yield recovery");

let _ = recovery.source_family();
let _ = recovery.recommended_action();
let _ = recovery.explanation().grouped_member_context();
```

Use the stronger grouped recovery lane when one neighborhood member stopped and
you need the retained member-local context.

## What To Read From Recovery

For most app flows, start here:

- `recommended_action()`
- `authority_surface()`
- `stop_kind()`

If the stop is still unclear, then read:

- `basis_posture()`
- `aspect_posture()`
- `evidence_strength()`
- `grouped_member_context()`
- `contribution_intent_descriptor()`

## What This Reuses

Recovery is not a second error system. It is a projection over:

- ordinary outcomes
- checked results
- proof-visible transcripts

That is why the same recovery surface can answer declaration, signal,
continuation, contribution-composed, and grouped stops without flattening them.

## Related Docs

- [Recovery Boundary](../recovery-boundary.md)
- [Stop To Recovery](../workflow/stop-to-recovery.md)
- [Choosing The Right Surface](../choosing/inspection-vs-readiness-vs-recovery.md)
