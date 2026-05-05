# Composition-Family Lowering

## What This Feature Is

This workflow shows how to build a same-family composition with symbolic and authoritative references, express lifecycle intent, and lower the family deterministically into one canonical lowered program.

## Why You Use It

- one same-family composition contains creates, rewrites, supersedes, or retires
- symbolic and authoritative identity must stay distinct
- deterministic canonical ordering must be established before the authority boundary closes

## Stable Entry Points

- `CompositionFamilySymbol::new(...)`
- `AuthoritativeFamilyMember::new(...)`
- `resolve_family_symbol(...)`
- `FamilyLifecycleAction`
- `Pair`
- `lower_deterministic_family_pair(...)`
- `LoweredFamilyProgram2`

## Core Mental Model

This workflow is about one family-local program, not one bag of writes.

The central laws are:

- symbolic family references are temporary and local
- authoritative members are stable targets
- lifecycle actions stay explicit
- deterministic lowering happens before later authority closes over the family

## How It Executes

1. create one or more family symbols
2. create or obtain authoritative family members
3. resolve symbol-to-authoritative relationships when needed
4. build lifecycle actions
5. place them into a fixed-arity carrier
6. lower them with a canonical key into `LoweredFamilyProgram2`

## Small Example

```rust
use forge_proof::{AuthoritativeFamilyMember, CompositionFamilySymbol};

let symbol = CompositionFamilySymbol::new(2_u8);
let member = AuthoritativeFamilyMember::new(11_u16);

assert_eq!(symbol.value(), &2_u8);
assert_eq!(member.value(), &11_u16);
```

This is the smallest honest example because the entire workflow depends on keeping those two identity lanes distinct from the start.

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

- symbolic references are preserved until they must become authoritative
- lifecycle intent is explicit before lowering
- canonical ordering is caller-defined and then proven

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

## Current Limits

- the stable lowering surface is pair-based
- canonical ordering policy is caller-supplied
- this workflow models deterministic lowering, not later execution or descriptive reporting

## Related Docs

- [Family Symbol Resolution](../features/family-symbol-resolution.md)
- [Family Lifecycle Actions](../features/family-lifecycle-actions.md)
- [Deterministic Family Lowering](../features/deterministic-family-lowering.md)
