# Deterministic Family Lowering

## What This Feature Is

Deterministic family lowering turns a fixed-arity set of same-family lifecycle actions into one canonical lowered family program with an explicit canonical-order proof.

## Why You Use It

- same-family actions must converge to one deterministic order
- you need a lowered family program that proves canonical ordering was established
- you want lowering to stay explicit instead of rediscovering family semantics procedurally later

## Stable Entry Points

- pleasant lane:
  - `use worth_proof::prelude::*;`
  - `create(...)`
  - `rewrite(...)`
  - `supersede(...)`
  - `retire(...)`
  - `family_pair(left, right)`
  - `.lower_by(...)`
- raw lane:
  - `use worth_proof::raw::*;`
  - `LoweredFamilyProgram2<S, A, P>`
  - `LoweredFamilyProgram2::actions()`
  - `LoweredFamilyProgram2::proof()`
  - `LoweredFamilyProgram2::into_parts()`
  - `lower_deterministic_family_pair(actions, canonical_key)`

## Core Mental Model

The feature does two things at once:

- preserves the pair of lifecycle actions
- proves they were put into canonical order

The caller is responsible for supplying the canonical key function. The crate is responsible for enforcing that the output is an explicitly ordered lowered program.

## Pleasant Lane First

```rust
use worth_proof::prelude::*;

fn family_action_key(
    action: &worth_proof::FamilyLifecycleAction<u8, u16, &'static str>,
) -> (u8, Option<u8>, Option<u16>) {
    match action {
        worth_proof::FamilyLifecycleAction::Retire { target } => {
            (0, None, Some(*target.value()))
        }
        worth_proof::FamilyLifecycleAction::Rewrite { target, .. } => {
            (1, None, Some(*target.value()))
        }
        worth_proof::FamilyLifecycleAction::Supersede { target, .. } => {
            (2, None, Some(*target.value()))
        }
        worth_proof::FamilyLifecycleAction::Create { symbol, .. } => {
            (3, Some(*symbol.value()), None)
        }
    }
}

fn lower() {
    let lowered = family_pair(
        create(sym(2_u8), "create"),
        supersede(member(11_u16), sym(3_u8), "replace"),
    )
    .lower_by(family_action_key);

    let _ = lowered.proof();
}
```

## Equivalent Raw Surface

```rust
use worth_proof::raw::*;

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

fn lower() {
    let lowered = lower_deterministic_family_pair(
        Pair::new(
            FamilyLifecycleAction::Create {
                symbol: CompositionFamilySymbol::new(2_u8),
                payload: "create",
            },
            FamilyLifecycleAction::Supersede {
                target: AuthoritativeFamilyMember::new(11_u16),
                replacement: CompositionFamilySymbol::new(3_u8),
                payload: "replace",
            },
        ),
        family_action_key,
    );

    let _ = lowered.proof();
}
```

## How It Relates To Other Features

- Pair this with [Family Symbol Resolution](./family-symbol-resolution.md) when symbolic references must be resolved explicitly before later authoritative use.
- Pair this with [Family Lifecycle Actions](./family-lifecycle-actions.md) because those actions are the inputs to lowering.
- Pair this with [Structural Facts](./structural-facts.md) because the lowered program carries canonical-order proof.

## Inspection And Debugging

- inspect `actions()` to see the canonicalized pair
- inspect `proof()` to confirm the output is explicitly proof-bearing
- if ordering is surprising, inspect the supplied canonical key function before anything else

## Anti-Patterns

- Do not lower same-family action sets without an explicit canonical ordering rule.
- Do not treat lowered family programs as just a pair of actions without noticing the canonical-order proof.
- Do not erase the symbolic-versus-authoritative distinction before lowering.

## Related Docs

- [Family Symbol Resolution](./family-symbol-resolution.md)
- [Family Lifecycle Actions](./family-lifecycle-actions.md)
- [Structural Facts](./structural-facts.md)
