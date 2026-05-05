# Family Lifecycle Actions

## What This Feature Is

Family lifecycle actions are the explicit action vocabulary for same-family composition before deterministic lowering closes the authority boundary.

## Why You Use It

- you need to express create, rewrite, supersede, or retire inside one composition family
- you want lifecycle intent to stay explicit before deterministic lowering
- you need symbolic and authoritative roles to remain distinguishable inside the action set

## Stable Entry Points

- `FamilyLifecycleAction<S, A, P>`
  - `Create { symbol, payload }`
  - `Rewrite { target, payload }`
  - `Supersede { target, replacement, payload }`
  - `Retire { target }`

## Core Mental Model

These actions are the family-local lifecycle vocabulary.

They let one composition say:

- create something new by symbol
- rewrite an existing authoritative target
- supersede an authoritative target with a symbolic replacement
- retire an authoritative target

The value here is not "four enum variants." The value is that same-family identity-transforming intent becomes a stable typed surface.

## How It Executes

Typical use:

1. declare symbolic and authoritative references explicitly
2. build one or more `FamilyLifecycleAction` values
3. put them into a fixed-arity carrier such as `Pair`
4. feed them into deterministic family lowering

## Small Example

```rust
use forge_proof::{CompositionFamilySymbol, FamilyLifecycleAction};

let create = FamilyLifecycleAction::Create {
    symbol: CompositionFamilySymbol::new(2_u8),
    payload: "create",
};

let _ = create;
```

This is the smallest honest example because lifecycle intent begins with choosing the right action variant.

## Real Example

```rust
use forge_proof::{
    AuthoritativeFamilyMember, CompositionFamilySymbol, FamilyLifecycleAction, Pair,
};

fn family_actions() {
    let actions = Pair::new(
        FamilyLifecycleAction::Create {
            symbol: CompositionFamilySymbol::new(2_u8),
            payload: "create",
        },
        FamilyLifecycleAction::Supersede {
            target: AuthoritativeFamilyMember::new(11_u16),
            replacement: CompositionFamilySymbol::new(3_u8),
            payload: "replace",
        },
    );

    let _ = actions;
}
```

What this shows:

- create uses a symbolic lane
- supersede uses both authoritative and symbolic lanes
- the lifecycle surface stays explicit before lowering decides canonical order

## How It Relates To Other Features

- Pair this with [Family Symbol Resolution](./family-symbol-resolution.md) when an action needs a resolved relationship between symbol and authoritative member.
- Pair this with [Deterministic Family Lowering](./deterministic-family-lowering.md) because action sets become lowered family programs there.
- Pair this with [Fork And Join](./fork-and-join.md) when same-family composition is being assembled from earlier fixed-arity structures.

## Inspection And Debugging

- pattern matching on the enum is the clearest way to inspect lifecycle intent
- action types make it obvious whether the code is operating on symbols, authoritative members, or both
- if identity pressure is blurry, inspect which variant was chosen first

## Anti-Patterns

- Do not encode same-family lifecycle intent as untyped strings or flags.
- Do not collapse symbolic creation and authoritative mutation into one generic "update" action.
- Do not treat supersede as just another rewrite; it has a different identity story.

## Current Limits

- the current vocabulary is intentionally small
- the stable surface is pair-oriented in current lowering
- the feature models intent, not persistence, diagnostics, or support reporting

## Related Docs

- [Family Symbol Resolution](./family-symbol-resolution.md)
- [Deterministic Family Lowering](./deterministic-family-lowering.md)
- [Fork And Join](./fork-and-join.md)
