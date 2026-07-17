# Form API

Most forms should be straightforward to construct. Give Worth one source of
truth, declare the fields a person can edit, connect the real input behavior,
and declare a named action. Then let readiness decide whether that action can
run.

Start there even when the final workflow will be large. Do not begin by
configuring every Forms subsystem. Availability, admission, steps, route
authority, collaboration, and recovery are answers to specific requirements,
not boxes every form must check.

## The Construction We Recommend

Keep business rules out of the form declaration. A validator should be an
ordinary function you can read, test, and reuse without constructing a form.
This small adapter turns a failed rule into the message artifact Forms expects:

```ts
// forms/invalidField.ts
export function invalidField(field: string, code: string, message: string) {
  return {
    kind: "invalid",
    field,
    message: {
      code,
      message,
      severity: "error",
      target: field,
      audience: "user",
      visibility: "visible",
    },
  } as const;
}
```

The actual profile rules stay pleasantly boring:

```ts
// profile/profile.validation.ts
import { invalidField } from "../forms/invalidField";

export const profileRules = {
  email(value: string) {
    return value.includes("@")
      ? true
      : invalidField("email", "email.invalid", "Enter a complete email.");
  },
  seats(value: number) {
    return value >= 1
      ? true
      : invalidField("seats", "seats.minimum", "Choose at least one seat.");
  },
};
```

Now the form declaration only answers form questions: where truth comes from,
which fields are editable, which rules apply to them, and which actions exist.
This profile belongs to the server-backed resource lifecycle, so its resource
module exports `profileLine`. See
[Your First Resource](../resources/start-here/your-first-resource.md) for that
declaration.

```ts
// profile/profile.form.ts
import { profileLine } from "../resources/profile";
import { profileRules } from "./profile.validation";

export const profileForm = signals.form.define({
  id: "profile-editor",
  source: signals.form.source.resourceLine(profileLine, { id: "profile" }),
  fields: ({ field }) => ({
    email: field<string>("email", {
      label: "Email",
    }),
    seats: field<number, string>("seats", {
      label: "Seats",
      parse: (raw) => Number.parseInt(raw, 10),
    }),
  }),
  validation: ({ field }) => ({
    email: field<string>("email", profileRules.email),
    seats: field<number>("seats", profileRules.seats),
  }),
  actions: ({ submit }) => ({
    submit: submit(),
  }),
});
```

This is the preferred shape, not a beginner-only shortcut. The source owns the
accepted value. The form owns the draft. Fields describe editable paths and
input behavior. The action captures the current patch and blockers, then lowers
the admitted changes through `profileLine`. The resource family—not the React
component—owns loading, external I/O, reconciliation, and retained evidence.

`signals.form.define(...)` preserves and types the reusable declaration; the
controller is created later by `useSignalsForm(profileForm)` or
`signals.form(profileForm)`. It does not create another state owner.

## Where Client Validation Actually Runs

The two lines under `validation` are where the business rules are applied to
the form. They register `profileRules.email` and `profileRules.seats` against
their declared fields. Return `true` when a value is valid; return an artifact
when it is not.

The runtime path is automatic:

1. `field.onChange` receives the browser value and commits it through the form
   input binding. The `seats` parser runs before the draft is updated.
2. Forms derives synchronous validation from the current effective values.
   The registered `profileRules` functions run as part of that read.
3. An invalid artifact appears in `field.messages`, which the application input
   renders.
4. The same artifact blocks the `submit` action plan, so
   `editor.submit.disabled` becomes `true` and execution is denied.

There is no separate `validate()` call in the component or hook. There is also
no second client-validation layer to keep in sync. The rule is declared once;
messages and action readiness consume the same validation result. A validator
can read its field and declared dependencies, but it cannot mutate the form or
hide network work inside the callback.

## Use The Form Components You Already Have

Assume the application already owns ordinary controls such as `TextInput`,
`NumberInput`, `TextareaInput`, `CheckboxInput`, and `SubmitButton`. Worth does
not prescribe their markup or teach you how to build them.

Those components consume the appropriate Worth binding. The binding carries
the current value, messages, write posture, and interaction handlers. The
component renders that state in the application's design system without
creating another copy of form state. If an existing component does not accept a
Worth binding directly, adapt it once at the design-system boundary.

## Put Form Behavior In A Hook

Assuming the application already has one `ReactSignalsStoreProvider`, keep the
form binding and event boundary in an application hook. The hook does not call
`fetch` or manually settle the controller. `submit.execute()` lowers the form's
patch plan through `profileLine`, which already owns that work.

```tsx
import type { FormEvent } from "react";
import { useSignalsForm } from "worth-signals-wasm/react";

export function useProfileEditor() {
  const form = useSignalsForm(profileForm);
  const submit = form.action("submit");

  async function save(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    await submit.execute();
  }

  return {
    email: form.field<string, string>("email"),
    seats: form.field<number, string>("seats"),
    save,
    submit,
  };
}
```

`save` is deliberately an ordinary event adapter. Do not memoize it with a
React callback hook; callback identity is not form state, and Worth already
owns the action's readiness and execution lifecycle.

## Then Components Stay Small

The component renders the application-owned controls. It does not know how the
profile is loaded, patched, confirmed, or reconciled.

```tsx
import {
  NumberInput,
  SubmitButton,
  TextInput,
} from "../ui/forms";

export function ProfileEditor() {
  const editor = useProfileEditor();

  return (
    <form onSubmit={editor.save}>
      <TextInput
        label="Email"
        field={editor.email}
      />
      <NumberInput
        label="Seats"
        min={1}
        field={editor.seats}
      />
      <SubmitButton action={editor.submit}>Save profile</SubmitButton>
    </form>
  );
}
```

There is no `useState`, syncing effect, component-owned request, form reducer,
or duplicate validation layer. If a screen needs unusual markup, consume the
same binding directly instead of bypassing the form or resource controller.

## First Decision: Who Owns The Value?

Choose the source before adding form behavior. If this choice is vague, the
rest of the form will be vague too.

- Use `signals.form.source.signal(...)` for application state owned by a Worth
  signal. This is the normal choice for local application workflows.
- Use `signals.form.source.resourceLine(...)` when the record belongs to a
  server-backed resource lifecycle and form actions should lower through that
  resource authority.
- Use `signals.form.source.graphPublicInput(...)` when the value enters through
  a published graph contract.
- Use `signals.form.source.external(...)` for a snapshot or foreign readable
  boundary. Do not quietly treat it as durable application truth.

Passing a compatible signal, readable, or plain object directly is supported.
Use the explicit source factories once identity or authority matters. In a real
application, that usually happens sooner than you think.

A field write changes the draft; it does not mutate the source. Read
`form.effective()` for the source with draft edits applied, `form.dirty()` for
semantic change, and `form.patchPlan()` for the change Worth can currently
describe.

## Second Decision: What Can The Control Actually Do?

Declare only the fields the workflow edits:

- `field(...)` for ordinary values;
- `repeated(...)` for identity-bearing collections;
- `attachment(...)` for files or file-like values;
- `evidence(...)` when the attachment is also evidence the workflow must
  inspect.

Repeated items need stable item identity. Attachments and evidence need stable
attachment identity. Do not substitute array position, a label, or a temporary
render key. Worth can only produce precise patches and recovery evidence from
identity that survives editing.

Input behavior belongs with the field declaration. Add `parse` when raw input
differs from the stored value. Declare the adapter capabilities the real
control supports, including raw input, commit boundaries, composition, focus,
label and message tracks, or responsive layout behavior.

```ts
seats: field<number, string>("seats", {
  label: "Seats",
  parse: (raw) => Number.parseInt(raw, 10),
  adapter: {
    tier: "signalBridge",
    reportsRawInput: true,
    reportsCommitBoundary: true,
    reportsFocus: true,
    supportsLabelTrack: true,
    supportsMessageTrack: true,
  },
})
```

Input capabilities are not decorative metadata. They let the form expose parse
barriers, missing focus support, and presentation gaps before those problems
turn into UI folklore. Report only what the adapter really implements. Use
`form.bindInput(fieldId)` to send raw input, composition, focus, blur, touch,
and visit events through the form boundary. Inspect `form.inputCapability(id)`
when a control is not behaving as expected.

## Third Decision: What Must Be True Before Work Runs?

Do not hand-build a separate `canSubmit` boolean. Declare the rule where it
belongs and read the combined result from `form.actionReadiness(actionId)` or
`form.readiness()`.

- Put value correctness in `validation`.
- Put enabled, disabled, hidden, and read-only UI posture in `availability`.
- Put permission, approval, and capability gates in `admission`.
- Put browser facts such as online state or credentials in `host` bindings and
  action requirements.

These are different questions. An invalid email is not a permission denial,
and a missing credential is not a disabled field. Keeping them separate is
what makes the final blocker report useful.

Synchronous validation derives from current form values. Async validation is
an explicit lifecycle: declare it with `asyncField(...)`, start it with
`startAsyncValidation(...)`, and settle it through `fulfillAsyncValidation`,
`rejectAsyncValidation`, `cancelAsyncValidation`, or
`timeoutAsyncValidation`. The validation declaration does not perform network
I/O for you.

## Fourth Decision: How Does Work Leave The Form?

There is no `form.submit()` method. Declare an action, inspect its plan when
useful, and execute it by ID.

```ts
const execution = await form.executeAction("submit");

if (execution.resultKind === "fulfilled") {
  console.log(execution.resourceSubmission);
}
```

Always await `executeAction(...)`; it may return an artifact or a promise. For
the server-backed path shown here, the action lowers patches, confirmation, and
reconciliation through the resource line. If the line lacks the necessary
patch capability, proof, or retained history, the action reports denied or
unavailable. It does not improvise.

A genuinely host-owned effect is a different lane: it returns a pending
operation that an application effect hook or service must settle. Keep that
adapter outside the component and do not use it to bypass an available resource
line. See [Action Execution](../forms/actions/action-execution.md) when the work
really is host-owned.

## Add The Broader Framework When The Requirement Appears

The rest of Forms is powerful, but it should enter through a reason:

- Add `steps` when one draft must be presented across several stages. Do not
  create a second form controller for each screen.
- Add route authority and lifecycle policy when the URL, entry, exit, or
  handoff can change who owns the visible workflow.
- Add resource-backed actions when the server record already belongs to a
  resource line or when concurrent optimistic work needs that machinery.
- Add collaboration when presence, leases, comments, or reviewer posture are
  product requirements. The application still owns the transport.
- Add presentation and measurement policies when the host must coordinate
  busy states, acknowledgement, layout, or external handoff.
- Add verification and retained history when operators need to explain what
  the controller observed. Process-local evidence is not a durable audit log.

This is the governing opinion: use the full framework when the workflow needs
it, but make every added subsystem earn its place by owning a real decision.

## The Five Reads To Reach For First

When something looks wrong, start here:

- `form.effective()` — what the user would submit now;
- `form.actionReadiness(actionId)` — whether the intended action can run and
  why;
- `form.actionPlan(actionId)` — the captured patch, requirements, and proof for
  that action;
- `form.inputCapabilities()` — whether the controls report the behavior the
  form expects;
- `form.diagnosticsSummary()` — the compact cross-cutting explanation.

Use the larger reports and histories after one of these points to the relevant
subsystem. Dumping every report at once is rarely debugging; it is just moving
the confusion into a larger object.

## Avoid These Designs

- Do not mirror the draft in React or another UI state store. Render the form
  state instead of creating a second owner.
- Do not use a plain object as if it were a live server authority.
- Do not infer collection identity from position.
- Do not hide network work inside validation or field callbacks.
- Do not treat an action plan as proof that the external effect succeeded.
- Do not enable every subsystem in anticipation of future complexity.

## Current Boundary

The controller is a TypeScript product-runtime surface exported by
`worth-signals-wasm`. It works with worker-first and compatibility deployments,
but the draft is not a Rust/Wasm-resident form kernel. Worth Forms does not
render DOM controls, navigate the browser, upload files, call arbitrary
endpoints, provide a collaboration transport, or turn process-local history
into durable shared truth.

## Go Deeper

- [Forms Overview](../forms/index.md)
- [Your First Form](../forms/getting-started/your-first-form.md)
- [Inputs And Controls](../forms/inputs/README.md)
- [Validation And Messages](../forms/validation/README.md)
- [Readiness And Permissions](../forms/availability/README.md)
- [Actions And Submission](../forms/actions/README.md)
- [Resource-Backed Forms](../forms/resource-backed/README.md)
- [Diagnostics And Recovery](../forms/diagnostics/README.md)
- [Complete Form Export Catalog](form-export-catalog.md)

The export catalog is the long-tail map. Use this page to choose the design;
use the catalog when you or your AI agent needs the exact declaration behind
that design.
