# Family Symbol Resolution

## What This Feature Is

Family symbol resolution is the surface that keeps temporary same-family symbolic references distinct from authoritative family members and makes the resolution between them explicit.

## Why You Use It

- a same-family composition flow needs temporary symbolic handles
- you must keep symbolic references distinct from authoritative identity
- later lifecycle lowering needs both the symbol and the authoritative target explicitly

## Stable Entry Points

- `CompositionFamilySymbol<S>::new(symbol)`
- `CompositionFamilySymbol::value()`
- `CompositionFamilySymbol::into_value()`
- `AuthoritativeFamilyMember<A>::new(member)`
- `AuthoritativeFamilyMember::value()`
- `AuthoritativeFamilyMember::into_value()`
- `FamilyResolvedReference<S, A>`
- `FamilyResolvedReference::symbol()`
- `FamilyResolvedReference::authoritative()`
- `FamilyResolvedReference::into_authoritative()`
- `resolve_family_symbol(symbol, authoritative)`

## Core Mental Model

This feature exists to prevent one of the nastiest same-commit mistakes:

- treating a temporary symbolic sibling reference as though it were already authoritative identity

The law is:

- symbols are local temporary references
- authoritative members are real target identities
- resolution between them must be explicit

## How It Executes

Typical usage:

1. create a `CompositionFamilySymbol<S>`
2. create or obtain an `AuthoritativeFamilyMember<A>`
3. resolve them with `resolve_family_symbol(...)`
4. carry the resulting `FamilyResolvedReference<S, A>` into later lifecycle steps

## Small Example

```rust
use forge_proof::{AuthoritativeFamilyMember, CompositionFamilySymbol};

let symbol = CompositionFamilySymbol::new(2_u8);
let member = AuthoritativeFamilyMember::new(11_u16);

assert_eq!(symbol.value(), &2_u8);
assert_eq!(member.value(), &11_u16);
```

This is the smallest honest example because the distinction between the two types is the whole reason the feature exists.

## Real Example

```rust
use forge_proof::{
    resolve_family_symbol, AuthoritativeFamilyMember, CompositionFamilySymbol,
};

fn resolve() {
    let symbol = CompositionFamilySymbol::new(2_u8);
    let authoritative = AuthoritativeFamilyMember::new(11_u16);
    let resolved = resolve_family_symbol(symbol, authoritative);

    assert_eq!(resolved.symbol().value(), &2_u8);
    assert_eq!(resolved.authoritative().value(), &11_u16);
}
```

What this shows:

- the symbolic and authoritative lanes remain distinct
- resolution is not ambient
- later lifecycle code can still extract the authoritative member explicitly

## How It Relates To Other Features

- Pair this with [Family Lifecycle Actions](./family-lifecycle-actions.md) because lifecycle actions use symbols and authoritative members in different roles.
- Pair this with [Deterministic Family Lowering](./deterministic-family-lowering.md) because resolved and unresolved family references both feed later lowering.
- Pair this with [Fixed-Shape Collections](./fixed-shape-collections.md) because family lowering currently operates over fixed pair structure.

## Inspection And Debugging

- type differences are the first debugging tool here; if you erased them, you lost the point of the feature
- `symbol()` and `authoritative()` make it easy to inspect both sides of the resolved reference
- `into_authoritative()` is the explicit extraction point for later authoritative-only actions

## Anti-Patterns

- Do not treat `CompositionFamilySymbol<S>` as a stable identity.
- Do not use raw primitive ids for both symbolic and authoritative lanes.
- Do not skip explicit resolution when later code needs to know that both lanes were present.

## Current Limits

- the feature models resolution, not a full registry or namespace engine
- it preserves explicit distinction, but it does not invent authoritative identity for you
- current stable lowering is pair-based rather than arbitrary-arity

## Related Docs

- [Family Lifecycle Actions](./family-lifecycle-actions.md)
- [Deterministic Family Lowering](./deterministic-family-lowering.md)
- [Fixed-Shape Collections](./fixed-shape-collections.md)
