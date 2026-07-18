# Layout, Inputs, And Accessibility

Worth can describe what a form control means to a renderer: its label, help and
message tracks, row and column, density, responsive hints, focus order, raw
input posture, and required capabilities. It does not render, measure, focus,
or resize the DOM by itself.

```ts
const form = signals.form({
  source: { title: "Draft", seats: 1 },
  fields: ({ field }) => ({
    title: field<string>("title", {
      label: "Title",
      description: "Shown to reviewers",
      row: "summary",
      density: "comfortable",
      accessibility: {
        readingOrder: 1,
        focusOrder: 1,
        describedBy: ["title-help"],
      },
    }),
    seats: field<number, string>("seats", {
      parse: (raw) => Number.parseInt(raw, 10),
      adapter: {
        tier: "externalImperative",
        reportsRawInput: true,
        reportsCommitBoundary: true,
      },
    }),
  }),
});
```

## Three Boundaries

1. **Field declaration** describes stable semantics and layout hints.
2. **Input adapter** reports raw input, composition, commit, focus, blur,
   touch, and visit events it actually supports.
3. **Renderer** turns artifacts into DOM, native controls, or another UI.

Do not mark an adapter as supporting focus, label tracks, or responsive tokens
unless it truly reports or renders them. Capability reports exist so the host
can explain missing behavior instead of assuming it.

Layout measurement is explicit host ingress. A renderer may report observed
sizes and causes; the controller retains the admitted artifacts and counters.
The controller never reaches into browser globals to measure elements.

Attachments and evidence fields follow the same rule: the controller owns
identity and patch posture, while a transfer service owns upload bytes and
progress. Resource-owned transfer truth can be projected back when the binding
is unambiguous.

## Go Deeper

- [Layout Overview](./layout-overview.md)
- [Rows, Sections, And Placement](./rows-sections-and-placement.md)
- [Label, Help, Message, And Control Tracks](./label-help-and-message-tracks.md)
- [Accessibility Artifacts](./accessibility-artifacts.md)
- [Inputs And Controls](../inputs/README.md)
- [Interaction And Host Facts](../interaction/README.md)
- [Attachments And Media](../media/README.md)
- [Label Size And Control Sizing](./label-size-and-control-sizing.md)
