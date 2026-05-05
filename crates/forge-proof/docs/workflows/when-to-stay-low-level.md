# When To Stay Low-Level

## What This Feature Is

This workflow explains when a caller should keep using the raw `forge-proof` substrate directly instead of trying to compress everything into a higher-level helper or wrapper.

## Why You Use It

- the domain flow is unusual enough that a "nice" helper would hide real law
- you need to preserve authority, basis, or freshness distinctions explicitly
- you are composing multiple adversarial boundaries in one place

## Stable Entry Points

- raw transition types such as:
  - `ResolveRecipeTransition`
  - `LowerRecipeTransition`
  - `AdmitRecipeTransition`
  - `CheckedAdmitExecutionReadyRecipeTransition`
  - `CheckedReadmitLoweredForExecutionReadyTransition`
- raw gate and outcome types such as:
  - `PreConstructionGate`
  - `TransitionReadiness`
  - `TransitionOutcome`
- raw family and composition types such as:
  - `Pair`
  - `JoinInputs2`
  - `FamilyLifecycleAction`
  - `LoweredFamilyProgram2`

## Core Mental Model

Stay low-level when the "nice" abstraction would hide the exact thing the compiler and the reviewer most need to see.

That usually means:

- trust-boundary crossing
- basis replacement
- stale vs rebind vs revalidation distinctions
- checked divergence topology
- symbolic vs authoritative family identity
- explicit proof redistribution on composition

These are not places to be cute.

## How It Executes

1. start from the domain need
2. ask whether a compressed helper would erase a meaningful adversarial boundary
3. if yes, keep the raw substrate surface visible
4. if no, a higher-level helper may be safe

## Small Example

```rust
use forge_proof::{PreConstructionGate, TransitionOutcome};

type Gate = PreConstructionGate<u8, &'static str, &'static str>;
type Outcome = TransitionOutcome<u8, &'static str, &'static str>;

let _ = std::any::type_name::<Gate>();
let _ = std::any::type_name::<Outcome>();
```

This is the smallest honest example because gates and outcomes are common places where over-compression becomes dishonest.

## Real Example

```rust
use forge_proof::{
    lower_deterministic_family_pair, resolve_family_symbol, AuthoritativeFamilyMember,
    CompositionFamilySymbol, FamilyLifecycleAction, Pair,
};

fn family_action_key(
    action: &FamilyLifecycleAction<u8, u16, &'static str>,
) -> (u8, Option<u8>, Option<u16>) {
    match action {
        FamilyLifecycleAction::Retire { target } => (0, None, Some(*target.value())),
        FamilyLifecycleAction::Rewrite { target, .. } => (1, None, Some(*target.value())),
        FamilyLifecycleAction::Supersede { target, .. } => (2, None, Some(*target.value())),
        FamilyLifecycleAction::Create { symbol, .. } => (3, Some(*symbol.value()), None),
    }
}

fn stay_explicit() {
    let symbol = CompositionFamilySymbol::new(2_u8);
    let authoritative = AuthoritativeFamilyMember::new(11_u16);
    let resolved = resolve_family_symbol(symbol.clone(), authoritative);

    let lowered = lower_deterministic_family_pair(
        Pair::new(
            FamilyLifecycleAction::Create {
                symbol,
                payload: "create",
            },
            FamilyLifecycleAction::Supersede {
                target: resolved.into_authoritative(),
                replacement: CompositionFamilySymbol::new(3_u8),
                payload: "replace",
            },
        ),
        family_action_key,
    );

    let _ = lowered;
}
```

This should stay low-level because:

- symbolic vs authoritative identity is adversarially important
- lifecycle intent is adversarially important
- canonical ordering responsibility is adversarially important

Hiding that behind one convenience helper would likely make the architecture worse, not better.

## How It Relates To Other Features

- Use [Authoring A New Proof Flow](./authoring-a-new-proof-flow.md) when deciding whether a new domain helper is warranted at all.
- Use [Checked Recipe Progression](./checked-recipe-progression.md) when low-level checked surfaces are the honest choice.
- Use [Composition-Family Lowering](./composition-family-lowering.md) when same-family identity pressure is the reason to stay explicit.

## Inspection And Debugging

- if a helper name sounds nicer than the law it is hiding, that is a warning sign
- if code review would be harder after adding the abstraction, keep the raw surface
- if an AI would infer the wrong semantics from the abstraction name, keep the raw surface

## Anti-Patterns

- Do not build a "universal convenience layer" that erases the crate's adversarial boundaries.
- Do not compress witness, basis, or trust-boundary handling just to save lines.
- Do not wrap raw surfaces when the wrapper adds no invariant and only renames things.

## Current Limits

- the crate is still more explicit than ideal in some happy-path flows
- the future DX pass may safely compress some mechanics
- the adversarial boundaries named here should still remain explicit even after that DX pass

## Related Docs

- [Authoring A New Proof Flow](./authoring-a-new-proof-flow.md)
- [Checked Recipe Progression](./checked-recipe-progression.md)
- [Composition-Family Lowering](./composition-family-lowering.md)
