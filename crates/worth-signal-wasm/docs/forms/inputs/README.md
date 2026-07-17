# Inputs And Controls

Field handles accept committed values directly. Input adapters add an honest
boundary for controls that have raw text, composition, search, focus, or an
external imperative lifecycle.

```ts
const seats = form.fields.seats;

seats.input("12");
seats.compose({ active: false });
seats.commitInput();
seats.touch();
seats.blur();
```

Only report capabilities the adapter really supports. An adapter declaration
does not implement a dropdown, debounce a search request, focus the DOM, or
perform accessibility announcements. It gives the controller enough semantic
information to expose parse barriers, interaction state, and missing
capabilities.

Read next:

- [Input Adapter Overview](./input-adapter-overview.md)
- [Raw Input, Compose, And Commit](./raw-input-compose-and-commit.md)
- [Dropdowns, Comboboxes, And Search](./dropdowns-comboboxes-and-search.md)
- [External Imperative Inputs](./external-imperative-inputs.md)
- [Input Capability Matrix](./input-capability-matrix.md)
- [Focus, Blur, Touch, And Visit Reporting](./focus-blur-touch-and-visit-reporting.md)
