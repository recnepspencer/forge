# Precision Fallbacks

## What This Feature Is

This feature covers the published precision-escalation and fallback vocabulary
in `worth-schema`.

## Why You Use It

Use this when you need to classify:

- which precision regime resolved the work
- why precision escalated
- what fallback disposition followed
- what kind of proof obligation the fallback created

## Stable Entry Points

- `PrecisionRegime`
- `PrecisionEscalationCause`
- `FallbackDisposition`
- `FallbackProofClass`
- `PrecisionFallbackRecord`
- `PrecisionBudgetFallbackRecord`

## Core Mental Model

These names let schema-owned and runtime-adjacent code talk about precision
events in a typed way.

There are two main record shapes:

- `PrecisionFallbackRecord` for ordinary escalation/fallback outcomes
- `PrecisionBudgetFallbackRecord` for budget-threshold escalation outcomes

## How It Executes

These records can be derived from lower-level precision events through the
published `from_*` helpers.

## Small Example

```rust
use worth_schema::facade::platform::authority::{
    FallbackDisposition,
    FallbackProofClass,
    PrecisionEscalationCause,
    PrecisionRegime,
};

let regime = PrecisionRegime::ExpansionC;
let cause = PrecisionEscalationCause::NearBoundary;
let disposition = FallbackDisposition::EscalatePrecision;
let proof = FallbackProofClass::ReplayRequired;
```

## Real Example

```rust
use math::arithmetic::precision::PrecisionEscalation;
use worth_schema::facade::platform::authority::PrecisionFallbackRecord;

fn lower(escalation: &PrecisionEscalation) -> PrecisionFallbackRecord {
    PrecisionFallbackRecord::from_precision_escalation(escalation)
}
```

## How It Relates To Other Features

- Use [Interpretation Vocabulary](./interpretation-vocabulary.md) when the
  topology meaning and the precision outcome both matter.
- Use [Geometry Binding Vocabulary](./geometry-binding-vocabulary.md) when the
  fallback sits beside geometry-side classification.

## Inspection And Debugging

If a fallback record is not telling the story you need:

- inspect `resolved_regime`
- inspect `escalation_cause`
- inspect `disposition`
- inspect `proof_class`

## Anti-Patterns

- Do not collapse these records into one untyped "precision warning" string.
- Do not assume every fallback means failure; the disposition and proof class
  are the important distinctions.

## Current Limits

- These types name precision outcomes. They do not replace the broader runtime
  workflow that consumes or publishes them.

## Related Docs

- [Authority](./README.md)
- [Interpretation Vocabulary](./interpretation-vocabulary.md)
- [Geometry Binding Vocabulary](./geometry-binding-vocabulary.md)
