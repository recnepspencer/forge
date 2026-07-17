# Forms

Start with the part every form needs: declare a source and its fields, then let
Worth own the draft. You immediately get effective values, semantic dirty
state, patch plans, submit readiness, and inspectable reasons when the form
cannot proceed—without assembling a second state system beside your inputs.

That small path is the framework, not a reduced mode. As the workflow grows,
the same controller can take on explicit source authority, validation,
permissions, steps, actions, resource-backed execution, route handoff,
collaboration posture, history, and verification. Add those lanes when the
product earns them; the core model does not change underneath you.

```ts
import { createSignals } from "worth-signals-wasm";

const signals = await createSignals();
const task = signals.input({ title: "Ship docs", done: false });

const form = signals.form({
  source: task,
  fields: ({ field }) => ({
    title: field<string>("title"),
    done: field<boolean>("done"),
  }),
});

form.fields.title.set("Publish docs");

console.log(form.effective());
console.log(form.dirty());
console.log(form.patchPlan());
console.log(form.readiness());
```

That source vs draft distinction is the load-bearing boundary. The input still
owns source truth. The form controller owns the draft and
derives the effective value, dirty report, patch plan, validation, and
readiness. A field write does not mutate the source. If the user changes a
field back to its source-equivalent value, the form becomes semantically clean
again.

## The Model In One Minute

```text
declared source -> source value
                       |
field writes ------> draft
                       |
                       v
                  effective value
                       |
       dirty + patch + validation + readiness
```

- **Source** is the value owned by a signal, a public graph input, a resource
  line, or an explicit external boundary.
- **Draft** contains controller-local edits. Clearing the draft does not roll
  back a server mutation.
- **Effective value** overlays the draft on the current source.
- **Patch plan** describes the semantic change Worth can prove from declared
  fields and stable item identity.
- **Readiness** combines change, parsing, validation, availability, admission,
  step, action, host, route, collaboration, and resource blockers that actually
  apply to the declaration.

Worth does not render controls, call arbitrary endpoints, invent permissions,
or provide a collaboration transport. It owns the form model and emits typed
artifacts that your UI and host code can use.

## Choose The Next Guide

- [Your First Form](./getting-started/your-first-form.md) — build the ordinary
  source, field, edit, and readiness loop.
- [State, Fields, And Changes](./state/README.md) — understand source, draft,
  effective value, field families, dirty state, and patch plans.
- [Validation And Messages](./validation/README.md) — parse raw input, derive
  validation, and settle explicit async checks.
- [Readiness And Permissions](./availability/README.md) — explain disabled,
  hidden, read-only, denied, or approval-gated work.
- [Actions And Submission](./actions/README.md) — plan actions, hand host work
  out explicitly, and settle the result.
- [Multi-Step Flows](./steps/README.md) — group fields without creating a second
  form state machine.
- [Layout, Inputs, And Accessibility](./layout/README.md) — declare UI semantics
  without pretending the controller renders the DOM.
- [Resource-Backed Forms](./resource-backed/README.md) — bind source and
  execution to a real resource line.
- [Lifecycle And Route Continuity](./lifecycle/README.md) — make entry, exit,
  handoff, and draft continuity visible.
- [Collaboration](./collaboration/README.md) — project locks, reviewer posture,
  comments, and resource-owned collaboration evidence.
- [Diagnostics And Recovery](./diagnostics/README.md) — inspect current truth,
  retained history, verification, and honestly unavailable recovery.
- [Form API Reference](../api-reference/forms.md) — exact factories, controller
  reads, field methods, action settlement, and limits.

Once the basic model is familiar, use [Common Form Patterns](./getting-started/common-form-patterns.md)
and [Form Recipes](./recipes.md) for complete constructions, [Input Adapters](./state/input-adapters.md)
when a custom control needs a DOM-facing binding, and the [Forms Glossary](./glossary.md)
when a term is doing more work than its name first suggests.
