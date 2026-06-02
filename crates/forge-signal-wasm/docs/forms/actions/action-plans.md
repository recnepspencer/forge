# Action Plans

## What This Feature Is

An action plan is Forge's preflight report for one form action.

Before you call `executeAction("submit")`, `executeAction("archive")`, or a
step action like `executeAction("nextDetails")`, the runtime can tell you:

- whether that action can run right now
- what would block it
- what form patch it would submit
- whether it needs host capability, route authority, approval, or resource
  state
- what recovery action would help the user move forward
- which proof digest execution must match later

That means a button does not need its own little state machine. It reads the
plan.

## Why You Use It

Use an action plan when UI needs to answer "can this action run, and what
exactly would happen if it did?"

Common uses:

- disable a submit button with a real reason instead of a vague `canSubmit`
- show why one action is blocked while another action is still available
- distinguish "unchanged form" from "invalid form" from "waiting on approval"
- inspect the patch that will be submitted before starting effects
- wire recovery UI from runtime suggestions instead of hand-writing every
  blocked state
- tie execution and history back to the exact plan that was admitted

## Stable Entry Points

```ts
const plan = form.actionPlan("submit");
const actions = form.actions();
```

Use `form.actionPlan(actionId)` when you care about one action.

Use `form.actions()` when you want the action catalog, every current plan,
summary counts, counters, host posture, and digest set for the whole form.

## Core Mental Model

An action declaration says "this action exists."

An action plan says "given the current form state, this action is accepted or
denied for these exact reasons."

Forge recomputes the plan from retained form truth:

- source, draft, and effective values
- patch plan
- validation report
- field/control availability
- admission and approval requirements
- host requirements such as online state or browser capability
- route authority for route-coupled steps
- resource-backed source and recovery posture when the form is resource-backed

If the plan is denied, execution is denied before effects start. If the plan is
accepted, execution records the plan digest so later fulfillment, rejection,
retry, or stale completion can be checked against the same proof.

## How It Executes

Planning is read-only. It does not submit, mutate, or start network work.

The normal flow is:

1. Declare actions on the form.
2. Read `form.actionPlan(actionId)` while rendering UI.
3. Use `plan.status`, `plan.readiness.blockers`, and `plan.recoveryActions` to
   explain the action.
4. Call `form.executeAction(actionId)` only when the user actually runs it.
5. Fulfill, reject, cancel, time out, or retry the resulting operation.
6. Inspect `actionExecutionHistory()` when you need the lifecycle after
   execution.

Planning and execution are intentionally separate. A plan can be accepted
without execution having started.

## Small Example

```ts
const submit = form.actionPlan("submit");

button.disabled = submit.status === "denied";
button.title = submit.readiness.blockers[0]?.reason ?? "Ready to submit";

if (submit.status === "accepted") {
  console.log("Will submit operations:", submit.patch.operations);
}
```

This is the useful replacement for scattered checks like `isDirty &&
isValid && !locked && online && hasApproval`.

## Real Example

This form has three actions:

- `submit` is the default action and requires a non-empty patch
- `saveDraft` can run even when nothing changed
- `archive` ignores the patch but requires approval

```ts
const form = signals.form({
  source: {
    title: "Ship docs",
    archived: false,
    locked: false,
  },
  fields: ({ field }) => ({
    title: field("title"),
    archived: field("archived"),
    locked: field("locked"),
  }),
  availability: ({ action }) => ({
    submitAvailability: action("submit", ["locked"], (values) =>
      values.locked
        ? { state: "blocked", reason: "record is locked" }
        : "enabled",
    ),
  }),
  admission: ({ action }) => ({
    archiveApproval: action("archive", "approval", ["archived"], () => ({
      posture: "requiresApproval",
      actorDigest: "actor:reviewer",
      policyDigest: "policy:archive",
    })),
  }),
  actions: ({ action }) => ({
    saveDraft: action("saveDraft", {
      patchPolicy: "allowEmpty",
      idempotency: "collapse",
      hostEffect: "draft.store",
    }),
    archive: action("archive", {
      patchPolicy: "ignore",
      destructive: true,
      idempotency: "deny",
      admissionCapability: "approval",
      hostEffect: "task.archive",
    }),
  }),
});

form.fields.title.set("Temporarily changed");
form.fields.title.set("Ship docs");

const submit = form.actionPlan("submit");
const saveDraft = form.actionPlan("saveDraft");
const archive = form.actionPlan("archive");

console.log(submit.status);
// "denied"

console.log(submit.readiness.blockers.map((blocker) => blocker.kind));
// ["unchanged"]

console.log(submit.recoveryActions.map((action) => action.kind));
// ["focusFirstActionableBlocker"]

console.log(saveDraft.status);
// "accepted"

console.log(saveDraft.patch.empty);
// true

console.log(archive.status);
// "denied"

console.log(archive.readiness.blockers.map((blocker) => blocker.kind));
// ["admission:requiresApproval"]

console.log(archive.regulatedActionBindings[0].actorDigest);
// "actor:reviewer"
```

The important part is not the amount of data. The important part is that each
action gets its own answer. `submit` is denied because there is no meaningful
patch. `saveDraft` is accepted because it explicitly allows empty patches.
`archive` is denied because approval is required, even though it does not care
about the patch.

## How It Relates To Other Features

Action plans sit between form state and action execution.

- Readiness tells the plan whether the action can run.
- Patch planning tells the plan what value changes the action would consume.
- Validation, availability, admission, and host facts add action-specific
  blockers.
- Step actions use the same plan surface as submit actions.
- Resource-backed forms add resource effect profile and resource recovery
  posture to the same plan.
- Execution consumes the plan; it does not rediscover the action state from
  scratch.

For React, read the plan during render and keep execution in the click handler:

```tsx
const save = form.actionPlan("saveDraft");

return (
  <button
    disabled={save.status === "denied"}
    onClick={() => form.executeAction("saveDraft")}
  >
    {save.status === "denied" ? "Cannot save" : "Save draft"}
  </button>
);
```

## Inspection And Debugging

Start with these fields:

```ts
const plan = form.actionPlan("submit");

console.log(plan.status);
console.log(plan.readiness.canRun);
console.log(plan.readiness.blockers);
console.log(plan.recoveryActions);
```

Then inspect the specific lane that explains the denial:

```ts
console.log(plan.patch);
console.log(plan.validation.summary);
console.log(plan.availability.summary);
console.log(plan.admission.summary);
console.log(plan.host.blockers);
console.log(plan.regulatedActionBindings);
```

When you need proof that execution used the same plan, compare digests:

```ts
const plan = form.actionPlan("submit");
const pending = form.executeAction("submit");

console.log(plan.planDigest === pending.planDigest);
// true when execution admitted this exact plan
```

Useful debugging fields:

- `status`: accepted or denied
- `resultKind`: the execution-style result implied by the plan
- `readiness.blockers`: the concrete reason the action cannot run
- `patch.empty`: whether the action would submit any semantic changes
- `patch.policy`: how the action treats empty or ignored patches
- `recoveryActions`: runtime-suggested next moves
- `diagnostics.deniedBeforeEffects`: whether denial prevents effects from
  starting
- `diagnostics.repeatedAttemptPolicy`: collapse, supersede, queue, deny, or
  allow repeated attempts
- `planDigest`: the stable proof handle for this planned action state

## Anti-Patterns

Do not rebuild the planner in component code:

```ts
const disabled =
  !form.dirty().semanticDirty ||
  !form.validation().summary.valid ||
  locked ||
  !online ||
  needsApproval;
```

That spreads the action rules across the UI and misses action-specific policy.
Read the plan instead:

```ts
const disabled = form.actionPlan("submit").status === "denied";
```

Do not treat an accepted plan as execution:

```ts
if (form.actionPlan("submit").status === "accepted") {
  showSavedToast(); // wrong: nothing has executed yet
}
```

Use `executeAction(...)` and `actionExecutionHistory()` for execution lifecycle.

Do not mutate plan objects. They are derived runtime artifacts, not inputs.
Change the form declaration or form state, then read the next plan.

## Current Limits

- `form.actionPlan(actionId)` throws if the action is not declared.
- Planning is synchronous and read-only; server work starts through execution,
  not planning.
- Route-coupled step actions can report typed deferred posture when route
  authority is not available.
- Resource-backed action details are stable on the plan surface, but the
  resource-specific execution behavior is documented separately.

## Related Docs

- [Action Overview](./action-overview.md)
- [Submit Actions](./submit-actions.md)
- [Action Execution](./action-execution.md)
- [Recovery Actions](./recovery-actions.md)
- [Readiness Blockers](../availability/readiness-blockers.md)
- [Step Actions](../steps/step-actions.md)
- [Resource Action Plans](../resource-backed/resource-action-plans.md)
