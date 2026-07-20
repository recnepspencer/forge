# Multi-Step Flows

Steps are projections over one form controller. They group fields, readiness,
patches, validation, messages, and navigation posture without creating one
draft store per screen.

```ts
const application = signals.form({
  source: { name: "", role: "", accepted: false },
  fields: ({ field }) => ({
    name: field<string>("name"),
    role: field<string>("role"),
    accepted: field<boolean>("accepted"),
  }),
  steps: ({ step }) => ({
    identity: step("identity", ["name", "role"], { order: 1 }),
    consent: step("consent", ["accepted"], {
      order: 2,
      dependencies: ["role"],
    }),
  }),
  actions: ({ step }) => ({
    continue: step("continue", "identity", "next"),
    back: step("back", "consent", "back"),
  }),
});
```

`application.steps()` returns each step's posture, readiness, dirty fields,
patch projection, validation artifacts, messages, and progress. The controller
can perform local `next`, `back`, `jump`, `skip`, and `revisit` commands through
declared step actions.

Step navigation is controller-local unless the declaration is explicitly
route-coupled. A route-coupled step still does not push browser history by
itself; the route handoff boundary must admit and apply that transition.

Steps are derived full reports with `notIncremental` posture. For very large
workflows, inspect their counters and dependency breadth rather than assuming a
step read touches only the active screen.

## Go Deeper

- [Controller-Local Steps](./controller-local-steps.md)
- [Step Groups](./step-groups.md)
- [Step Readiness](./step-readiness.md)
- [Step Actions](./step-actions.md)
- [Controller-Local Step Navigation](./controller-local-step-navigation.md)
- [Route-Coupled Forms](../route-coupling/README.md)
