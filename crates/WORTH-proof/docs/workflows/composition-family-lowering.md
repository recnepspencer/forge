# Composition-Family Lowering

## What This Feature Is

This workflow shows how to build a same-family composition with symbolic and authoritative references, express lifecycle intent, and lower the family deterministically into one canonical lowered program.

## Why You Use It

- one same-family composition contains creates, rewrites, supersedes, or retires
- symbolic and authoritative identity must stay distinct
- deterministic canonical ordering must be established before the authority boundary closes

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
  - `CompositionFamilySymbol::new(...)`
  - `AuthoritativeFamilyMember::new(...)`
  - `resolve_family_symbol(...)`
  - `FamilyLifecycleAction`
  - `Pair`
  - `lower_deterministic_family_pair(...)`

## Core Mental Model

This workflow is about one family-local program, not one bag of writes.

The central laws are:

- symbolic family references are temporary and local
- authoritative members are stable targets
- lifecycle actions stay explicit
- deterministic lowering happens before later authority closes over the family

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

    let _ = lowered.actions();
}
```

What this keeps visible:

- helper constructors create intent, not lowered truth
- deterministic lowering still requires an explicit canonical key
- the pleasant lane stays pair-shaped instead of inventing a dynamic family engine

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

    let _ = lowered.actions();
}
```

Use the raw lane when:

- you need direct symbolic resolution before intent construction
- you are building a domain-facing same-family helper
- the pleasant pair grammar stops being semantically obvious

## How It Relates To Other Features

- Use [Family Symbol Resolution](../features/family-symbol-resolution.md) to understand the identity split this workflow depends on.
- Use [Family Lifecycle Actions](../features/family-lifecycle-actions.md) for the action vocabulary.
- Use [Fixed-Arity Join](./fixed-arity-join.md) when earlier static composition is how the family action set gets assembled.

## Inspection And Debugging

- inspect symbol and authoritative types first if identity pressure is getting blurry
- inspect the lifecycle action variants before the canonical key if the wrong semantic action seems to be happening
- inspect the canonical key function first if ordering is surprising

## Anti-Patterns

- Do not treat symbolic handles as authoritative identities.
- Do not lower same-family lifecycle intent without an explicit canonical ordering rule.
- Do not collapse create, rewrite, supersede, and retire into one generic update abstraction.

## Related Docs

- [Family Symbol Resolution](../features/family-symbol-resolution.md)
- [Family Lifecycle Actions](../features/family-lifecycle-actions.md)
- [Deterministic Family Lowering](../features/deterministic-family-lowering.md)
