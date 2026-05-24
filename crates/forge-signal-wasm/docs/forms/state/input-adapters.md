# Input Adapters

## What This Feature Is

Input adapters declare what one field input can and cannot report to the form
runtime.

## Why You Use It

- keep non-native input capability explicit
- inspect why one field cannot support composition, focus, or richer UI sync
- avoid pretending every external control behaves like a signal-native input

## Stable Entry Points

- field option `adapter` or `inputAdapter`
- `form.inputAdapters()`
- `form.inputCapabilities()`
- `fieldHandle.diagnostics().inputAdapter`

## Core Mental Model

Input adapters are capability declarations, not decoration. They tell the form
what an input can report, such as raw input, commit boundaries, composition, or
focus.

## How It Executes

1. a field declares adapter posture
2. the runtime admits capability presence or absence
3. field diagnostics expose unavailable capabilities explicitly
4. `inputCapabilities()` summarizes adapter posture across the form

## Small Example

```ts
const form = signals.form({
  source: { title: "Ship docs" },
  fields: ({ field }) => ({
    title: field("title", {
      adapter: {
        tier: "externalImperative",
        reportsComposition: false,
        reportsFocus: false,
      },
    }),
  }),
});

console.log(form.inputCapabilities());
```

## Real Example

```ts
console.log(form.inputAdapters());
console.log(form.fields.title.diagnostics().inputAdapter);
console.log(form.inputCapabilities().fields[0]);
```

Those reads tell you exactly which capabilities are unavailable and why.

## How It Relates To Other Features

- Read [Input Adapter Overview](../inputs/input-adapter-overview.md) for the
  dedicated input integration section.
- Read [Raw Input, Compose, And Commit](../inputs/raw-input-compose-and-commit.md)
  when the control has explicit raw-input and commit phases.
- Read [Fields And Field Paths](./fields-and-field-paths.md) for the underlying
  field contract.
- Read [Parse Failures](../validation/parse-failures.md) when the input must
  cross a parse boundary into admitted draft truth.

## Inspection And Debugging

- `inputAdapters()` shows field-by-field admitted capability sets
- `inputCapabilities()` shows the aggregate unavailable posture
- field diagnostics show the most local explanation

## Anti-Patterns

- assuming an imperative external input reports composition or focus unless it
  declared those capabilities
- using adapter options as if they were layout or presentation controls

## Current Limits

- richer layout configuration now lives in the layout section, and richer input
  integration guidance now lives in the dedicated inputs section

## Related Docs

- [Input Adapter Overview](../inputs/input-adapter-overview.md)
- [Input Capability Matrix](../inputs/input-capability-matrix.md)
- [Fields And Field Paths](./fields-and-field-paths.md)
- [Parse Failures](../validation/parse-failures.md)
