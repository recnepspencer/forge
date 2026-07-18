# Debugging And Recovery

Start with the line you already use. Its grouped summary, diagnostics, effects,
and history are different depths of the same resource lifecycle—not separate
debug models.

## Read From Shallow To Deep

```ts
const summary = line.summary();
const diagnostics = line.diagnosticsSummary();
const history = line.history();
```

Use them in this order:

1. `line.summary()` — current lifecycle, request, processing, transfer,
   diagnostic, and history-availability posture.
2. `line.status()` and `line.freshness()` — precise current loading and cache
   state.
3. `line.diagnostics()` or `diagnosticsSummary()` — why the latest operation
   was admitted, changed, denied, or failed.
4. `line.effects()` — concurrent optimistic identities, dependencies, and
   canonical-versus-projected value.
5. `line.history()` — retained entries, exact recovery availability, and
   verification proof.

Going straight to the verification package is usually noise. Stopping at
`value()` is usually not enough.

## Answer Common Questions

### Why is the old value still visible?

Check `status()` and `freshness()` together. A refresh may be pending while the
previous fulfilled value remains visible. That is different from a fresh cache
hit and different again from a rejected first load with no value.

### Why did one optimistic change disappear?

```ts
console.log(line.effects().get(effectId));
console.log(line.effects().projection());
```

Inspect the exact effect identity. A rejected sibling should retire without
erasing confirmed siblings; a dependent child may cancel when its required
parent rejects.

### Why did a response update only part of the UI?

Read `line.mutationResponse()` and reconciliation diagnostics. The response may
have proved one declared target while leaving another stale, partial, awaiting
delivery, or requiring refetch.

### Why did a partial patch not update a view?

Inspect the declared locus and reconciliation report. The family may lack item
identity, a field/region/path declaration, or the aspect used by that view.

### Why is external delivery rejected?

Read delivery and compatibility diagnostics. External definitions and pushed
packets carry basis and capability posture; stale or incompatible evidence does
not silently overwrite current line truth.

## Recovery Is Capability-Gated

```ts
const availability = line.history().availability;

if (availability.restoreExact.kind === "available") {
  const result = line.history().restoreExact();
  console.log(result);
}
```

Exact method names and result variants depend on the history operation; consult
the line reference before wiring UI actions. The important rule is stable:
inspect availability and handle the typed unavailable result. Do not promise a
restore merely because the line has some diagnostic history.

Retained lifecycle evidence, exact replay payloads, exact snapshots, and compact
inverse rollback are different artifacts. One does not imply the others.

## Verification Packages

Use `line.history().verificationPackage()` when a test, support tool, or
regulated workflow needs a compact statement of admitted capabilities,
denials, compatibility, lifecycle, and recovery posture. It explains the
browser runtime boundary. It is not durable server audit history.

## Common Mistakes

- Logging only `value()` and losing the lifecycle that explains it.
- Choosing concurrent settlement from `diagnostics().lastEffect`.
- Treating “history exists” as proof that exact replay is available.
- Retrying with a new response identity after an interrupted settlement.
- Reconstructing canonical value in UI code from diagnostics or snapshots.
- Treating a verification package as durable business truth.

## Go Deeper

- [Inspect A Resource Line](./inspect-a-resource-line.md)
- [Check Status, Freshness, And History](./check-status-settlement-and-history.md)
- [Why Did This View Update?](./why-did-this-view-update.md)
- [Why Didn't This View Update?](./why-didnt-this-view-update.md)
- [Read Delivery And Compatibility](./read-delivery-and-compatibility.md)
- [Restore, Replay, And Recover](./restore-replay-and-recover.md)
- [Resource Line Reference](../../api-reference/resource-line.md)
