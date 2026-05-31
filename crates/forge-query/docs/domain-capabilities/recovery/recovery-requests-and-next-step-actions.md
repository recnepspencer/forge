# Recovery Requests And Next-Step Actions

The recovery boundary gives you two related surfaces:

- `recommended_action()`: the compact answer
- `recovery_request()`: the typed request object that carries the same
  explanation

## Common Actions

- `RefreshBasis`
  - use when retained basis is stale or mismatched
- `RebindContext`
  - use when the retained target context is no longer the right one
- `RepairDeclarationMeaning`
  - use when declaration meaning or required aspect fit is the real problem
- `ReviewContributionIntent`
  - use when a contribution-owned denial needs caller attention
- `CorrectWorld`
  - use when the wrong admitted operating world is attached
- `CorrectHandle`
  - use when the wrong handle identity is attached
- `CheckSupport`
  - use when the feature is unsupported rather than invalid
- `UseExplicitHandoff`
  - use when automation should stop and a stronger or more explicit caller step
    is required

## Good Pattern

Use the action for app flow, and the explanation for UI or logs:

```rust
let brief = handle
    .recover_from_signal_compatibility_checked(checked)
    .expect("non-success should recover");

match brief.recommended_action() {
    ForgeQueryRecoveryAction::RefreshBasis => {
        // reload the current basis, then retry from the appropriate feature
    }
    ForgeQueryRecoveryAction::RepairDeclarationMeaning => {
        // send the user back to the declaration-shaping step
    }
    _ => {}
}

let explanation = brief.explanation();
let _ = (
    explanation.source_family(),
    explanation.aspect_posture(),
    explanation.evidence_strength(),
);
```

This keeps your app from hard-coding stop parsing logic in multiple places.
