# Validation And Messages

Validation in Worth is derived form truth. Synchronous validators read the
current form and return typed artifacts. Raw-input parsing and asynchronous
work have explicit lifecycle boundaries so a stale response cannot quietly
become current validation truth.

## Synchronous Validation

```ts
const form = signals.form({
  source: { email: "" },
  fields: ({ field }) => ({
    email: field<string>("email"),
  }),
  validation: ({ field }) => ({
    emailRequired: field<string>("email", (value) =>
      value.includes("@")
        ? { kind: "valid", field: "email", digest: value }
        : {
            kind: "invalid",
            field: "email",
            message: {
              code: "email.invalid",
              message: "Enter a complete email address.",
              severity: "error",
              audience: "user",
              visibility: "visible",
            },
          },
    ),
  }),
});

console.log(form.validation());
console.log(form.readiness());
```

Validators are read-only callbacks. They derive artifacts from current values;
they do not mutate draft state or perform host effects.

## Parse Before You Validate

Use a typed raw-input boundary when a control produces a different type from
the form field:

```ts
const form = signals.form({
  source: { seats: 1 },
  fields: ({ field }) => ({
    seats: field<number, string>("seats", {
      parse: (raw) => Number.parseInt(raw, 10),
    }),
  }),
});

form.fields.seats.input("4").commitInput();
```

Uncommitted raw input and parse failures remain visible readiness blockers.
Worth does not pretend an invalid string is the last valid numeric value.

## Async Validation Is Explicit Host Work

`asyncField(...)` declares identity, triggers, and debounce metadata. It does
not call your server and it does not schedule a uniqueness request by itself.

```ts
const account = signals.form({
  source: { handle: "ada" },
  fields: ({ field }) => ({
    handle: field<string>("handle"),
  }),
  validation: ({ asyncField }) => ({
    handleAvailable: asyncField("handle", {
      triggers: ["blur", "submit"],
      debounceMs: 250,
    }),
  }),
});

const pending = account.startAsyncValidation("handleAvailable");
const response = await fetch(`/api/handles/${account.fields.handle.value()}`);

if (response.ok) {
  account.fulfillAsyncValidation(pending.operationId, {
    reason: "handle is available",
  });
} else {
  account.rejectAsyncValidation(pending.operationId, {
    reason: "handle is already taken",
  });
}
```

The operation ID binds settlement to the validation basis that started it.
Superseded, cancelled, timed-out, and stale completions remain distinguishable
in history.

## Messages Are Artifacts, Not Toasts

Validation messages carry severity, audience, visibility, target, and optional
accessibility posture. A renderer decides whether that becomes inline text, a
summary, an announcement, or nothing. The controller does not render or focus
the DOM.

## Go Deeper

- [Validation Overview](./validation-overview.md)
- [Parse Failures](./parse-failures.md)
- [Visible Messages](./visible-messages.md)
- [Async Validation](./async-validation.md)
- [Server Canonicalization](./server-canonicalization.md)
- [Source Compatibility And Draft Migration](./source-compatibility-and-draft-migration.md)
