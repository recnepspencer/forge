# Equivalence And Mismatch Classification

## What This Feature Is

This feature compares two ready canonical basis artifacts under an explicit
equivalence basis. It tells you whether they are equivalent, mismatched, or
unsupported, and it preserves structured mismatch data instead of flattening
the result into true or false.

## Why You Use It

- Use this when canonical basis parity matters and you need an explicit answer.
- Use this when a blind consumer should be able to inspect where two producers
  drifted.
- Use this when some comparison requests are unsupported and that unsupported
  result should remain distinct from a real mismatch.
- Use this when you need the mismatch locus to stay structured and inspectable
  instead of getting buried in generic comparison text.

## Stable Entry Points

Common path:

- `canonicalization().compare().left(...)`
- `.right(...)`
- `.under(...)`
- `canonicalization().compare().evaluate(...)`
- `canonicalization().compare().equivalent_basis(...)`
- `canonicalization().compare().mismatch_basis(...)`
- `canonicalization().compare().unsupported_basis(...)`

Lower lane:

- `prepare_canonical_comparison(...)`
- `compare_canonical_basis(...)`
- `CanonicalComparisonReadyArtifact`
- `CanonicalComparisonOutcome`
- `CanonicalEquivalenceBasis`
- `CanonicalEquivalentBasis`
- `CanonicalMismatchBasis`
- `CanonicalMismatchKind`

Good to know:

- `canonicalization_api::common_path` is the recommended grouped public lane.
- `canonicalization_api::lower_lane::comparison` is the inspectable lower
  lane.

## Core Mental Model

Comparison has three possible outcomes:

- equivalent: the basis artifacts match under the equivalence basis
- mismatched: they do not match, and the mismatch basis explains where
- unsupported: the request itself is outside the supported comparison surface

That distinction matters. Unsupported is not just another mismatch, and a
mismatch is not just a boolean failure.

## How It Executes

The normal flow is:

1. prepare the left and right ready basis artifacts
2. choose the equivalence basis explicitly
3. admit the comparison request
4. evaluate the comparison outcome
5. inspect either equivalent basis, mismatch basis, or unsupported basis

The common path makes left/right/equivalence staging visible so comparison
still looks like a real decision boundary.

The hostile proof bar for this feature also assumes blind consumers can inspect
small mismatch loci directly. That is why mismatch basis and unsupported basis
are first-class outputs instead of just error strings.

## Small Example

```rust
use forge_foundational::{canonicalization, CanonicalEquivalenceBasis};
use forge_proof::TransitionOutcome;

let ready = match canonicalization()
    .compare()
    .left(left_ready)
    .right(right_ready)
    .under(CanonicalEquivalenceBasis::Strict)
{
    TransitionOutcome::Success(ready) => ready,
    other => return Err(format!("comparison admission failed: {other:?}").into()),
};

let outcome = canonicalization().compare().evaluate(&ready);
```

This is the smallest honest example because it keeps readiness admission and
final evaluation separate.

## Real Example

```rust
use forge_foundational::{canonicalization, CanonicalEquivalenceBasis};
use forge_proof::TransitionOutcome;

let ready = match canonicalization()
    .compare()
    .left(left_ready)
    .right(right_ready)
    .under(CanonicalEquivalenceBasis::Strict)
{
    TransitionOutcome::Success(ready) => ready,
    other => return Err(format!("comparison admission failed: {other:?}").into()),
};

let outcome = canonicalization().compare().evaluate(&ready);

if let Some(mismatch) = canonicalization().compare().mismatch_basis(&outcome) {
    println!("mismatch kind: {:?}", mismatch.kind());
}
```

What is authoritative here is the structured comparison outcome, not a local
guess based on two digests or two producer-specific records.

## How It Relates To Other Features

- [Canonical Basis Sequences And Entry Grammar](./canonical-basis-sequences-and-entry-grammar.md)
  explains where the ready basis artifacts come from.
- [Export Bundles And Producer Shape](./export-bundles-and-producer-shape.md)
  uses the same equivalence ideas when comparing canonical exports.
- [Canonical Production Readiness](./canonical-production-readiness.md)
  freezes mismatch handling and grouped surface inventory as certified Milestone
  2 behavior.

## Inspection And Debugging

Inspect these first:

- `canonicalization_api::lower_lane::comparison` when you need the exact lower
  comparison vocabulary
- `CanonicalComparisonOutcome` to confirm whether this is equivalent,
  mismatched, or unsupported
- `CanonicalMismatchBasis` when the outcome is mismatched or unsupported
- the chosen `CanonicalEquivalenceBasis` when a result feels stricter or looser
  than expected

If comparison surprises you, check the equivalence basis first. Many surprises
come from the declared comparison scope, not from bad basis preparation.

## Anti-Patterns

- Do not collapse unsupported comparison into generic mismatch text.
- Do not compare raw digests as a substitute for canonical comparison.
- Do not hide equivalence-basis choice in ambient configuration.

## Current Limits

- Comparison explains structured parity and drift. It does not publish bundles
  for other consumers.
- Unsupported outcomes remain explicit even when two producers feel "close
  enough" locally.

## Related Docs

- [Canonical Basis Sequences And Entry Grammar](./canonical-basis-sequences-and-entry-grammar.md)
- [Export Bundles And Producer Shape](./export-bundles-and-producer-shape.md)
