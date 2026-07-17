# Lifecycle And Route Continuity

Form lifecycle makes entry, visible work, handoff, and exit inspectable. It is
useful when a form must wait for source admission, draft restoration,
validation, layout, a route transition, or another external lane.

## Entry Is A Checklist, Not A Spinner

```ts
const form = signals.form({
  source: { title: "Draft" },
  fields: ({ field }) => ({ title: field<string>("title") }),
  presentation: {
    entry: {
      bootstrap: {
        sourceAdmission: true,
        draftRestore: true,
        validation: true,
        readiness: true,
      },
    },
  },
});
```

The entry report says which declared prerequisites are ready, pending,
blocked, or unavailable. Presentation lifecycle does not fetch data, restore a
draft, or show a loading screen by itself; the owning host reports those facts.

## Route Handoff Preserves Authority

Route-coupled forms can prepare a handoff that describes current draft,
readiness, step, and exit posture. The router decides whether a transition is
admitted and applies browser history. The form does not call `pushState`.

Continuity policy can preserve, freeze, discard, defer, or clear draft
authority. These are explicit transitions, not side effects of a component
unmounting. A historical continuity artifact is evidence of what happened; it
is not authority to perform a later transition.

## External Lanes Stay External

Layout, messages, navigation, uploads, and host effects can contribute visible
lifecycle facts. Reporting one of those facts does not mutate source or draft
truth. This separation keeps a delayed renderer or stale route completion from
silently editing the form.

## Go Deeper

- [Entry Bootstrap](./entry-bootstrap.md)
- [Visible Lifecycle](./visible-lifecycle.md)
- [Handoffs](./handoffs.md)
- [Exit Posture](./exit-posture.md)
- [External Lanes](./external-lanes.md)
- [Route-Coupled Forms](../route-coupling/README.md)
