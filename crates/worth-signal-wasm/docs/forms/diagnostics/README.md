# Diagnostics And Recovery

Start with current summaries. Reach for retained history only when you need to
explain a transition, and ask for replay or restore only when the owning source
can actually support it.

```ts
const summary = form.diagnosticsSummary();

console.log(form.sourceAuthority());
console.log(form.declaration());
console.log(form.fieldContract());
console.log(form.readiness());
console.log(summary);
```

These reads derive from current form truth. They do not copy the complete
history or create a second authority store.

## Current State Versus Retained History

- `diagnostics()` and `diagnosticsSummary()` explain the current controller.
- `stateHistory()` records retained state transitions.
- `actionHistory()` and `actionExecutionHistory()` separate planning outcomes
  from effect settlement.
- `asyncValidationHistory()` preserves stale, cancelled, timed-out, and
  superseded completions.
- resource, route, presentation, source-compatibility, and collaboration
  histories remain separate because they have different owners.

Process-local history is useful evidence, not a durable audit log. Reload,
disposal, retention limits, or another process can remove it unless a real
platform boundary persists it.

## Verification Checks Agreement

```ts
const package_ = form.verification();
console.log(package_.digests);
```

The verification package lets tests and diagnostic tools compare source,
draft, patch, validation, readiness, action, route, resource, and presentation
surfaces. A digest summarizes evidence; it is not authority to execute an
action or restore state.

## Recovery Can Be Unavailable

`reset()` always means “clear this controller's local draft.” Resource rollback,
replay, and restore are separate operations. They require a resource-line
source, compatible retained history, and the necessary effect or replay
support. Treat an unavailable result as information, not as an exception to
paper over.

## Go Deeper

- [Diagnostics Summary](./diagnostics-summary.md)
- [Diagnostics History](./diagnostics-history.md)
- [State History](./state-history.md)
- [Action History](./action-history.md)
- [Resource History](./resource-history.md)
- [Source Compatibility History](./source-compatibility-history.md)
- [Verification](../verification/README.md)
