# Dirty Exit

## What This Feature Is

Dirty exit is the explicit gate that evaluates whether speculative branch
work can commit cleanly or requires explicit confirmation.

## Why You Use It

- stop speculative commit from hiding dirty graph work
- require an explicit confirmation witness when a branch exit is dirty
- keep commit correctness visible instead of magical

## Stable Entry Points

- `session.dirtyExit(specialist)`
- `dirtyExit.confirm()`
- `session.commit(preview, dirtyExit, confirmation)`

## Core Mental Model

Dirty exit is a separate artifact because the router treats "branch is dirty"
as meaningful authority, not as a boolean the app can quietly skip.

If confirmation is required, commit must receive the matching confirmation
witness returned by that dirty-exit artifact.

## How It Executes

1. ask a specialist to evaluate dirty work
2. capture the run summary
3. classify the result as clean or confirmation-required
4. optionally create a confirmation witness
5. require the matching artifacts during commit

## Small Example

```ts
const dirtyExit = await session.dirtyExit(specialist);

if (dirtyExit.confirmationRequired) {
  const confirmation = dirtyExit.confirm();
}
```

## Real Example

```ts
const dirtyExit = await session.dirtyExit(specialist);
const preview = await session.commitPreview();

if (dirtyExit.confirmationRequired) {
  const confirmation = dirtyExit.confirm();
  await session.commit(preview, dirtyExit, confirmation);
} else {
  await session.commit(preview, dirtyExit);
}
```

## How It Relates To Other Features

- dirty exit belongs to a live [Speculative Session](./speculative_sessions.md)
- commit preview remains separate in [Commit Preview](./commit_preview.md)

## Inspection And Debugging

- `dirtyExit.runSummary`
- `dirtyExit.disposition`
- `dirtyExit.confirmationRequired`
- `dirtyExit.verification()`

## Anti-Patterns

- committing without a dirty-exit artifact
- reusing confirmation from another dirty-exit run
- flattening dirty exit into generic "unsaved changes" UI state

## Current Limits

- the specialist must expose `evaluateDirty()`
- dirty confirmation only proves that a specific dirty-exit artifact was
  acknowledged
- commit fails closed when confirmation is missing or mismatched

## Related Docs

- [Speculative Sessions](./speculative_sessions.md)
- [Commit Preview](./commit_preview.md)
