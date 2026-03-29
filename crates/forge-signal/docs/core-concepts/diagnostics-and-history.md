# Diagnostics And History

This section covers two things:

- why the current result looks the way it does
- what happened over time

## Diagnostics

Use diagnostics when the runtime surprises you.

Main door:

- `runtime.diagnostics()`

Typical questions:

- why did this node run again?
- why is this result different now?
- why is this runtime retaining so much recent detail?

Diagnostics are not an afterthought here.
They are part of how you operate the runtime.
If you cannot explain why something reran, the job is not done yet.

## History

Use history when the latest state is not enough.

Main door:

- `runtime.history()`

Typical questions:

- what happened on this branch?
- what changed between two points in time?
- how did this result evolve across restores or retries?

History is not just the latest state.
It keeps the sequence of changes that got you there.
If you need to answer "what happened before this?" the runtime should already
have that answer.

## Practical Rule

Think of diagnostics as "why did this happen?" and history as "what happened
over time?"
