# Accessibility Artifacts

## What This Feature Is

This page covers the controller's accessibility report for fields, messages,
sections, focus targets, and ordering hints.

## Why You Use It

- read labels, descriptions, and described-by relationships from one place
- inspect focus-target availability and ordering hints
- keep accessibility state tied to the same form truth as validation and
  availability

## Stable Entry Points

- field options such as `label`, `description`, `summaryLabel`,
  `describedBy`, `readingOrder`, `focusOrder`, `summaryOrder`
- field option `accessibility`
- `form.accessibility()`

## Core Mental Model

Accessibility is not just a bag of strings. The controller derives one report
that combines declarations with current validation, message, and availability
state.

## How It Executes

1. field accessibility hints are declared
2. validation and messages add current error/summary state
3. the runtime derives field, message, and section reads plus the current
   focus-target state

## Small Example

```ts
console.log(form.accessibility());
```

## Real Example

```ts
const accessibility = form.accessibility();

console.log(accessibility.fields);
console.log(accessibility.messages);
console.log(accessibility.focusTarget);
console.log(accessibility.orderHints);
```

## How It Relates To Other Features

- Read [Visible Messages](../validation/visible-messages.md) for the user
  message list beneath accessibility messages.
- Read [Layout Hints](./layout-hints.md) when you need the layout side rather
  than the accessibility side.

## Inspection And Debugging

- `fields[*].describedBy` shows label/help/message relationships
- `messages[*].focusTarget` shows where a validation message wants focus
- `focusTarget` shows whether focus handoff is ready, unavailable, or absent
- `orderHints` shows reading, focus, section, and summary order

## Anti-Patterns

- duplicating accessibility ordering in app code when the form already derives
  it
- assuming focus targets are always available on every adapter

## Current Limits

- richer focus and host interaction posture live in the interaction docs rather
  than the layout section

## Related Docs

- [Layout Hints](./layout-hints.md)
- [Visible Messages](../validation/visible-messages.md)
