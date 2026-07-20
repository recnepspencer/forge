# How Worth Signals Thinks About State

Worth Signals is opinionated about state because ambiguous ownership is where
state systems become expensive.

## One Value, One Owner

Every value should have one authority: the place allowed to decide what is
true. An input can own browser-local state. A resource line can own the local
representation of one server resource. A form can own its draft. A router can
own the admitted visible route.

Do not copy that value into React state "for convenience." The copy immediately
creates a synchronization problem that did not exist before.

## Derive What Can Be Rebuilt

If a value follows from other values, compute it. Derived state should be safe
to destroy and rebuild from its authorities.

```ts
const firstName = signals.input("Ada");
const lastName = signals.input("Lovelace");
const displayName = signals.computed(() => `${firstName()} ${lastName()}`);
```

Storing `displayName` separately would create two writers for the same meaning.
That is not flexibility. It is a future bug with a friendly face.

## Intent Is Not Confirmed Truth

Draft edits, speculative navigation, and optimistic requests express intent.
They should remain distinguishable from confirmed truth until the responsible
authority settles them.

Worth uses explicit drafts, branches, and effect identities because a shared
"before" snapshot cannot honestly represent several concurrent requests.

## Evidence Explains; It Does Not Decide

Diagnostics, histories, receipts, and proofs explain execution. They can show
what changed, why it ran, what branch existed, and whether a restore was exact.
They do not become application truth merely because they are detailed.

Applications may retain diagnostic payloads for presentation or export. That
retention is application-owned unless a documented runtime history surface says
otherwise.

## Boundaries Are Features

The callable local lane uses opaque runtime identities. Add `debugName` for
humans. Publish a graph when names and input rules become an application
contract. Use the explicit spec lane only when structural names are themselves
the contract.

The same principle applies at larger scales:

- React renders Signal state; it does not own another state engine.
- Resources represent browser-local resource state; the server remains durable
  authority.
- TypeScript Local Truth owns process-local branch values; Query and Relational
  own durable platform truth.

## Worker-First Is An Architectural Choice

`await createSignals()` selects worker-first execution. Compatibility is an
explicit deployment, not an automatic fallback. Execution placement affects
which synchronous specialist surfaces are available, so the package reports
that choice rather than hiding it.

## What This Buys You

When authority, derivation, intent, and evidence stay separate:

- derived state can be rebuilt;
- rollback can target the right request;
- debugging can show real causes;
- UI frameworks stay replaceable;
- support limits can be explicit instead of silently approximate.

That is the through-line of the package. The larger APIs are different tools
for preserving the same distinctions.

## Related Docs

- [Choose The Right Surface](./choosing-a-surface.md)
- [Core Signals](../core/README.md)
- [Diagnostics And Explanation](../core/diagnostics.md)
