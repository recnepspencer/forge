# Compatibility Lowering And JSON Bridges

## What This Feature Is

This feature is the explicit bridge from legacy JSON-shaped input into native
Milestone 1 meaning. It lets you lower JSON through contracts and source
locators without making JSON itself authoritative.

## Why You Use It

- Use this when an adopting runtime still receives JSON payloads at a boundary.
- Use this when you want compatibility lowering to land on the same validated
  or authoritative native meaning as the common native path.
- Use this when JSON-originated input must stay visibly transitional instead of
  masquerading as native authoring.

## Stable Entry Points

Common path:

- `compatibility().json()`
- `.input(...)`
- `.lower_value(...)`
- `.lower_state(...)`

Lower lane:

- `JsonCompatibilityAspectInput`
- `lower_json_aspect_value(...)`
- `lower_json_record_aspect_state(...)`
- `JsonCompatibilityLoweringOutcome<_>`
- `JsonCompatibilityLoweringDenial`
- `JsonCompatibilityLoweringDeferred`
- `JsonCompatibilityRebindRequired`

Good to know:

- `compatibility()` is a separate common lane from `aspects()` on purpose.
- the bridge still relies on real aspect contracts and typed source locators.
- JSON state lowering inherits the native state-admission law, including the
  empty-state denial.

## Core Mental Model

Compatibility lowering is not another native constructor. It is an explicit
bridge from transitional JSON input into native aspect meaning.

That distinction matters because:

- native authoring uses `aspects()` directly
- compatibility lowering uses `compatibility().json()`
- the output should match native meaning when the JSON is valid
- the JSON itself never becomes the authority lane

## How It Executes

The normal flow is:

1. define or obtain the real aspect contract
2. construct a typed boundary source locator
3. either build `JsonCompatibilityAspectInput` values or call the direct
   lowering helpers
4. lower one JSON value or a whole JSON-backed state
5. inspect the structured lowering outcome

State lowering does not bypass the native authority boundary:

- duplicate lowered aspect keys are still denied by state admission
- empty lowering requests are denied instead of minting an empty admitted state

## Small Example

```rust
use forge_foundational::{compatibility, BoundarySourceLocator};

let json_lane = compatibility().json();
let input = json_lane.input(contract, source_locator, serde_json::json!(3));
```

This is the smallest honest example because the explicit input bridge is part
of the feature, not just setup noise.

## Real Example

```rust
use forge_foundational::{
    aspects, compatibility, AspectLocator, BoundarySourceLocator, LocatorAuthority,
};
use forge_proof::TransitionOutcome;
use serde_json::json;

let contract = task_summary_contract();
let source = BoundarySourceLocator::Aspect(AspectLocator::new(
    LocatorAuthority::SupportOnly,
    aspects().vocabulary().key("task.summary")?,
));

let json_lane = compatibility().json();
let TransitionOutcome::Success(state) = json_lane.lower_state([json_lane.input(
    contract,
    source,
    json!({
        "title": "Ship it",
        "done": true,
        "note": "compatibility"
    }),
)]) else {
    panic!("expected lowered state");
};
```

What is authoritative here is the lowered native state artifact, not the JSON
payload that crossed the bridge.

## How It Relates To Other Features

- [Validation And Authoritative State Admission](./validation-and-authoritative-state-admission.md)
  describes the native authority lane compatibility lowering is trying to reach.
- [Identities, Locators, And Blind-Consumer Addressing](./identities-locators-and-blind-consumer-addressing.md)
  covers the source-locator surface this bridge uses.
- [Grouped Public Lanes And Common-Path Usage](./grouped-public-lanes-and-common-path-usage.md)
  explains when to stay on the native aspect lane and when to use the
  compatibility bridge instead.

## Inspection And Debugging

Inspect these first:

- the `BoundarySourceLocator` when you need to know where a lowered JSON value
  came from
- the structured `JsonCompatibilityLoweringOutcome<_>` when lowering fails or
  defers
- parity with the native validated or admitted state when the JSON lane seems
  suspicious

If compatibility lowering behaves differently than the native path, the first
thing to check is contract parity, not the JSON parser.

## Anti-Patterns

- Do not treat JSON payloads as if they were native authoritative values.
- Do not route native authoring through the compatibility bridge out of
  convenience.
- Do not hide JSON-originated lowering inside generic validation helpers.

## Current Limits

- This bridge exists to support migration and transitional boundaries.
- The readiness artifact still names JSON lowering as explicit compatibility
  debt to be retired by adopting crates over time.

## Related Docs

- [Validation And Authoritative State Admission](./validation-and-authoritative-state-admission.md)
- [Grouped Public Lanes And Common-Path Usage](./grouped-public-lanes-and-common-path-usage.md)
