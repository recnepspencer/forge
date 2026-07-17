# Actions And Submission

An action turns form truth into an inspectable execution plan. The controller
checks readiness, captures the patch basis, and records lifecycle evidence. It
does not secretly call an arbitrary endpoint.

There is no `form.submit()` method. Declare a submit action, then execute it by
ID.

## Declare An Action

```ts
const form = signals.form({
  source: { title: "Draft" },
  fields: ({ field }) => ({
    title: field<string>("title"),
  }),
  actions: ({ submit, action }) => ({
    submit: submit({
      hostEffect: "article.update",
      hostRequirements: ["online", "credentials"],
    }),
    saveDraft: action("saveDraft", {
      patchPolicy: "allowEmpty",
      hostEffect: "draft.store",
    }),
  }),
});
```

`actionPlan(id)` is safe to inspect before execution. It exposes the patch,
blockers, host requirements, idempotency posture, recovery actions, and current
basis.

## Execute Host Work Honestly

```ts
const execution = await form.executeAction("submit");

if (execution.resultKind === "pending") {
  try {
    const response = await fetch("/api/articles/42", {
      method: "PATCH",
      body: JSON.stringify(form.patchPlan()),
    });

    if (!response.ok) throw new Error(`save failed: ${response.status}`);
    const saved = await response.json();

    form.fulfillAction(execution.operationId, {
      reason: "server accepted the patch",
      canonicalValue: saved,
    });
  } catch (error) {
    form.rejectAction(execution.operationId, {
      reason: error instanceof Error ? error.message : "save failed",
    });
  }
}
```

`executeAction(...)` can return an artifact immediately or a promise, so
`await` it before reading `resultKind` or `operationId`. Host code owns the
network call and explicitly fulfills, rejects, cancels, or times out the
pending operation. That boundary is what makes stale completion and repeated
attempt behavior inspectable.

## Resource Actions Are Different

When the source is a real resource line, a declared resource action can lower a
patch, refresh, revalidate, replay, restore, or targeted rollback through that
resource's authority. It may settle synchronously or asynchronously. If the
line cannot prove the requested operation, the result is denied or unavailable
rather than approximated.

## Repeated Attempts

Choose `none`, `collapse`, `supersede`, `queue`, or `deny` deliberately. An
idempotency label is not a server guarantee; the server and host effect still
need a real idempotency contract when duplicate external work matters.

## Go Deeper

- [Action Overview](./action-overview.md)
- [Action Plans](./action-plans.md)
- [Submit Actions](./submit-actions.md)
- [Action Execution](./action-execution.md)
- [Recovery Actions](./recovery-actions.md)
- [Repeated Attempts And Idempotency](./repeated-attempts-and-idempotency.md)
- [Resource-Backed Forms](../resource-backed/README.md)
