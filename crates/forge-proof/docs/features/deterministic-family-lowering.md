# Deterministic Family Lowering

## What This Feature Is

Deterministic family lowering turns a fixed-arity set of same-family lifecycle actions into one canonical lowered family program with an explicit canonical-order proof.

## Why You Use It

- same-family actions must converge to one deterministic order
- you need a lowered family program that proves canonical ordering was established
- you want lowering to stay explicit instead of rediscovering family semantics procedurally later

## Stable Entry Points

- `LoweredFamilyProgram2<S, A, P>`
- `LoweredFamilyProgram2::actions()`
- `LoweredFamilyProgram2::proof()`
- `LoweredFamilyProgram2::into_parts()`
- `lower_deterministic_family_pair(actions, canonical_key)`

## Core Mental Model

The feature does two things at once:

- preserves the pair of lifecycle actions
- proves they were put into canonical order

That means downstream code can rely on one deterministic same-family lowering shape instead of ad hoc ordering choices.

The caller is responsible for supplying the canonical key function. The crate is responsible for enforcing that the output is an explicitly ordered lowered program.

## How It Executes

Representative lowering flow:

1. build a `Pair<FamilyLifecycleAction<...>>`
2. provide a canonical key function
3. call `lower_deterministic_family_pair(...)`
4. receive `LoweredFamilyProgram2<...>` carrying:
   - canonicalized actions
   - `Proof<CanonicalOrder>`

## Small Example

```rust
use forge_proof::{FamilyLifecycleAction, LoweredFamilyProgram2};

type Program = LoweredFamilyProgram2<u8, u16, &'static str>;
let _ = std::any::type_name::<Program>();
```

This is the smallest honest example because the lowered program itself is the stable output boundary for the feature.

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

    let _actions = lowered.actions();
    let _proof = lowered.proof();
}
```

What this shows:

- family symbols and authoritative members remain explicit before lowering
- ordering is not guessed; the canonical key is provided directly
- the result carries proof that canonical order was established

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

## Current Limits

- only pairwise lowering is modeled in the stable surface today
- canonical ordering policy is caller-supplied rather than globally imposed
- the feature models deterministic lowering, not execution or support reporting

## Related Docs

- [Family Symbol Resolution](./family-symbol-resolution.md)
- [Family Lifecycle Actions](./family-lifecycle-actions.md)
- [Structural Facts](./structural-facts.md)
