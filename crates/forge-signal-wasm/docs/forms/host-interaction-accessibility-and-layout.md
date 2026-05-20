# Host, Interaction, Accessibility, And Layout

## What This Feature Is

This feature admits browser-local facts, interaction posture, accessibility
artifacts, generated layout hints, and measured layout snapshots without
turning them into semantic form truth.

## Why You Use It

- gate readiness or actions on online, visibility, viewport, or persistence
  facts
- expose touched, visited, focus, and submit-intent posture honestly
- drive renderer accessibility and layout from derived artifacts instead of
  handwritten DOM rules

## Stable Entry Points

- host bindings in `signals.form({... host: ... })`
- `host()`
- `interaction()`
- `reportFieldInteraction(...)`
- `reportSubmitIntent(...)`
- `clearSubmitIntent(...)`
- `inputCapabilities()`
- `navigation()`
- `accessibility()`
- `layout()`
- `layoutMeasurement()`
- `recordLayoutMeasurement(...)`

## Core Mental Model

Host and renderer facts matter, but they are not source truth. The runtime
derives accessibility, navigation, and layout artifacts from canonical form
state, then lets the renderer report measurement and interaction posture back
through typed lanes.

## How It Executes

The runtime reads declared host capabilities, derives interaction and
accessibility surfaces, computes layout hints, then accepts coalesced
measurement snapshots and interaction reports without reclassifying them as
source or draft writes.

## Small Example

```ts
const report = form.host();
const interaction = form.interaction();
const accessibility = form.accessibility();

console.log(report.summary);
console.log(interaction.summary);
console.log(accessibility.summary);
```

This is the smallest honest example because these are the read surfaces a
renderer or controller should actually consume.

## Real Example

```ts
form.reportFieldInteraction("title", {
  kind: "focus",
  source: "keyboard",
});

form.recordLayoutMeasurement([
  {
    rowId: "title-row",
    labelHeight: 24,
    controlHeight: 40,
    messageHeight: 18,
  },
], {
  cause: "contentGrowth",
  frameToken: "frame-1",
});

console.log(form.layout());
console.log(form.layoutMeasurement());
```

Interaction and measurement become typed artifacts the runtime can inspect and
retain without rewriting semantic form truth.

## How It Relates To Other Features

- Pair it with [Presentation And External Lanes](./presentation-and-external-lanes.md)
  when visible settlement depends on layout, focus, or handoff timing.
- Pair it with [Validation And Messages](./validation-and-messages.md) because
  accessibility summaries are derived from validation and message artifacts.
- Pair it with [Diagnostics, History, And Verification](./diagnostics-history-and-verification.md)
  when you need digests and counters for these derived lanes.

## Inspection And Debugging

- `host()` shows supported and unavailable host facts.
- `interaction()` shows touched, visited, focus, raw input, composition, and
  submit-intent history.
- `clearSubmitIntent(...)` lets you clear an admitted intent instead of leaving
  stale keyboard or pointer submit posture behind.
- `inputCapabilities()` shows unavailable adapter capability explicitly.
- `navigation()` shows controller-local navigation posture and blockers.
- `accessibility()` and `layout()` show what the renderer should consume.
- `layoutMeasurement()` shows retained layout snapshots and counters.

## Anti-Patterns

- reading the DOM directly inside semantic validators or patch planning
- treating host fact absence as `false` instead of explicit unavailability
- using visual position to decide semantic validation or dirty truth

## Current Limits

- DOM measurement remains outside the semantic signals graph
- unavailable input, focus, or layout capabilities stay typed unavailable; the
  runtime does not fake them
- route ownership, browser-history, and external handoff remain outside this
  surface

## Related Docs

- [Presentation And External Lanes](./presentation-and-external-lanes.md)
- [Validation And Messages](./validation-and-messages.md)
- [Diagnostics, History, And Verification](./diagnostics-history-and-verification.md)
