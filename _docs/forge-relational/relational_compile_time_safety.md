# forge-relational Safety — Deferred Items

> **Status**: These items do not affect type signatures in Milestones 1–6. They can be implemented after the core structural refactor lands without requiring breaking API changes.
>
> **Relationship to `relational_architecture.md`**: That document contains the compile-time safety patterns that _do_ affect current milestone type signatures (`VersionBound`, branded `SlotView`, `SnapshotGuard`, adjacency deltas as data). This document covers the remaining items that are additive safety layers for future features.

---

## Table of Contents

1. [`ConflictKey` Enum for Merge](#1-conflictkey-enum-for-merge)
2. [`MergeParentList` Smart Constructor](#2-mergeparentlist-smart-constructor)
3. [Intent Match Exhaustiveness Enforcement](#3-intent-match-exhaustiveness-enforcement)
4. [Invariant Registration Completeness Test](#4-invariant-registration-completeness-test)
5. [`Symbol` Stable `Ord` Implementation](#5-symbol-stable-ord-implementation)

---

## 1. `ConflictKey` Enum for Merge

**When it matters**: When branch merge conflict detection is built.

**The Bug It Prevents**: Merge conflict detection compares records by `EntityId`. But if branch A modifies entity E1 and branch B deletes E1 and replaces it with E2 (with lineage E1→E2), the merge doesn't detect a conflict because E1 ≠ E2. Both changes apply, producing overlapping state.

**Design**:

```rust
/// Forces the developer to explicitly choose which identity space
/// to compare in. You cannot accidentally compare storage IDs
/// when you meant lineage IDs.
pub enum ConflictKey {
    /// Compare by storage identity (EntityId / RelationId).
    ByStorageId(RecordRef),
    /// Compare by lineage identity (follows lineage chain).
    ByLineage(LineageId),
}

pub fn detect_conflicts(
    branch_a: &[ConflictKey],
    branch_b: &[ConflictKey],
) -> Vec<MergeConflict> { ... }
```

**Why deferred**: Merge conflict detection doesn't exist yet. This is a new API, not a change to an existing one.

---

## 2. `MergeParentList` Smart Constructor

**When it matters**: When branch merge/history is built.

**The Bug It Prevents**: If merge parents are stored in a `Vec<BranchId>` with no ordering guarantee, replay of the same merge commit on two different machines could process parents in different order, producing different truth state.

**Design**:

```rust
/// A list of merge parents that is guaranteed to be sorted and deduplicated.
/// The inner Vec is private — there is no push() method.
/// The ONLY way to construct this is from_canonical(), which sorts + deduplicates.
pub struct MergeParentList(Vec<BranchId>);

impl MergeParentList {
    pub fn from_canonical(mut parents: Vec<BranchId>) -> Self {
        parents.sort();
        parents.dedup();
        Self(parents)
    }

    pub fn iter(&self) -> impl Iterator<Item = &BranchId> { self.0.iter() }
    pub fn len(&self) -> usize { self.0.len() }
}
```

**Why deferred**: Already implied by the vision doc's "ordered parent commit lists." The smart constructor is additive — the underlying storage field doesn't change shape.

---

## 3. Intent Match Exhaustiveness Enforcement

**When it matters**: After Milestone 4 (Intent Dispatch) lands.

**The Bug It Prevents**: A developer adds a 9th `TransactionIntent` variant. Eight of ten match sites are updated. Two files have `_ => {}` catch-all arms and compile successfully. The new intent is silently ignored in merge conflict detection and snapshot publication.

**Design**:

1. Remove all `_ => {}` catch-all arms on `TransactionIntent` matches.
2. Add a CI lint (or `#[deny(unreachable_patterns)]` at crate level) that fails on wildcard arms for this enum.
3. Milestone 4's self-describing methods already reduce match sites from 10+ to 1, making this largely moot.

**Why deferred**: Milestone 4 addresses the root cause (too many match sites). This is additive polish.

---

## 4. Invariant Registration Completeness Test

**When it matters**: After Milestone 5 (structural cleanup) lands.

**The Bug It Prevents**: A developer writes a new `InvariantRule` variant and implements `evaluate_rule` for it, but forgets to register it in the invariant catalog. The rule exists but is never executed.

**Design**:

```rust
#[test]
fn all_invariant_rules_are_registered() {
    let catalog = default_invariant_catalog();
    // Use strum or a manual method to enumerate all variants.
    let all_rules = InvariantRule::all_variants();
    for rule in all_rules {
        assert!(
            catalog.contains(&rule),
            "InvariantRule::{:?} is not registered in any catalog group. \
             Add it to the appropriate InvariantCategory in the catalog builder.",
            rule
        );
    }
}
```

**Why deferred**: Just a test. No API or type changes.

---

## 5. `Symbol` Stable `Ord` Implementation

**When it matters**: Anytime (independent fix).

**The Bug It Prevents**: `BTreeMap<Symbol, u64>` iteration order depends on `Symbol`'s `Ord` implementation. If `Symbol` wraps an interned string and the interner assigns IDs based on insertion order (which varies between process restarts), `BTreeMap` iteration order changes between runs. Patch emission and diagnostic output become nondeterministic.

**Design**: Implement `Ord` for `Symbol` based on the string content (stable across runs), not the interner ID (unstable across runs).

```rust
impl Ord for Symbol {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare by string content, not by intern ID.
        self.as_str().cmp(other.as_str())
    }
}
```

**Why deferred**: Independent single-type fix. No downstream API changes.

---

## Priority Order

When these items are ready to be implemented, the recommended order is:

1. **Symbol `Ord`** (#5) — trivial fix, zero risk, immediate determinism improvement
2. **Invariant completeness test** (#4) — trivial test, catches future omissions
3. **Intent exhaustiveness** (#3) — trivial lint, catches future match gaps
4. **`MergeParentList`** (#2) — smart constructor, needed before merge ships
5. **`ConflictKey`** (#1) — API design, needed before merge ships
