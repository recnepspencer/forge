# Label, Help, Message, And Control Tracks

## What This Feature Is

This page covers the track model inside the layout report.

## Why You Use It

- understand how label, control, help, and message lanes are represented
- debug why one field can or cannot support a richer track layout
- keep track meaning separate from measured heights

## Stable Entry Points

- `form.layout().fields[*].tracks`
- `form.layoutField(fieldId)`
- field adapter capabilities:
  `supportsLabelTrack`, `supportsHelpTrack`, `supportsMessageTrack`

## Core Mental Model

Tracks are the declared layout lanes around one field:

- label
- control
- help
- message

The layout report shows whether those lanes are declared or omitted. Each
layout field hint also carries the control capability truth for that same
field.

## How It Executes

1. field and adapter declarations are read together
2. the layout report exposes track posture
3. capability posture explains when a track cannot be fully supported

## Small Example

```ts
const title = form.layoutField("title");
console.log(title?.tracks);
```

## Real Example

```ts
const field = form.layoutField("title");

console.log(field?.tracks);
console.log(field?.capabilities.supportsLabelTrack);
console.log(field?.capabilities.supportsMessageTrack);
```

## How It Relates To Other Features

- Read [Control Sizing](./control-sizing.md) for height and growth concerns.
- Read [Input Capability Matrix](../inputs/input-capability-matrix.md) for the
  capability-side explanation.

## Inspection And Debugging

- `tracks` tells you which lanes were declared
- `layoutField(fieldId)` is the shortest read when you want track and
  capability truth together
- accessibility reports tell you what those lanes mean for labels and
  described-by output

## Anti-Patterns

- treating track declaration as if it guaranteed adapter support
- using measured row heights to infer declared track posture

## Current Limits

- tracks are declared structure, not measured geometry

## Related Docs

- [Control Sizing](./control-sizing.md)
- [Accessibility Artifacts](./accessibility-artifacts.md)
