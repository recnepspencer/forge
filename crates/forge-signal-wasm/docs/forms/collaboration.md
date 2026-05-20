# Collaboration

## What This Feature Is

This feature projects multi-actor posture onto a form through typed lock,
lease, branch, presence, comment, and unavailable artifacts.

## Why You Use It

- block writes or submit when another actor owns the relevant authority
- expose advisory presence and reviewer comments without inventing a second
  collaboration engine
- make branch-per-actor or lease-based coordination inspectable

## Stable Entry Points

- collaboration declaration in `signals.form({... collaboration: ... })`
- `collaboration()`
- `reportCollaboration(...)`
- `clearCollaboration(...)`

## Core Mental Model

Collaboration posture is derived and typed. Advisory presence and comments do
not become semantic truth on their own. Blocking authority still flows through
lock, lease, branch, review, or admission artifacts.

## How It Executes

The runtime admits one collaboration mode, accepts posture updates through
typed reports, records collaboration history and event artifacts, then projects
blocking or advisory posture back into the form surface.

## Small Example

```ts
const form = signals.form({
  source: { title: "Ship docs" },
  collaboration: {
    mode: "fieldLease",
    actorId: "me",
    supportsPresence: true,
  },
  fields: ({ field }) => ({
    title: field("title"),
  }),
});
```

This is the smallest honest example because collaboration starts from a
declared mode, not from ad hoc peer messages.

## Real Example

```ts
form.reportCollaboration({
  posture: "blocked",
  leasedFields: [{ field: "title", ownerId: "peer-1" }],
  presence: [{ actorId: "peer-1", status: "active" }],
  comments: [{ id: "comment-1", authorId: "peer-1", target: "title" }],
  reason: "peer-1 owns the title lease",
});

console.log(form.collaboration());
```

The runtime keeps the blocking lease posture distinct from advisory presence
and comments, while retaining a structured event history for inspection.

## How It Relates To Other Features

- Pair it with [Availability, Admission, And Steps](./availability-admission-and-steps.md)
  because collaboration often blocks edit or action admission.
- Pair it with [Resource-Line Forms](./resource-line-forms.md) when branch proof
  or remote source drift comes from the backing resource line.
- Pair it with [Diagnostics, History, And Verification](./diagnostics-history-and-verification.md)
  when you need collaboration digests and event history.

## Inspection And Debugging

- `collaboration()` shows mode, posture, reason, lock owner, leased fields,
  branch id, advisory presence, comments, and event history.
- `verification().digests.collaborationDigest` and
  `verification().digests.collaborationEventDigest` certify the current and
  historical posture.

## Anti-Patterns

- treating presence or comments as if they were blocking authority
- inventing field locks in UI code without reporting them through the typed
  collaboration lane
- coercing branch ids into strings when the source authority carries numeric
  branch identity

## Current Limits

- unavailable collaboration posture stays explicit when the required proof is
  absent
- branch ownership still belongs to the resource/branch subsystem when the form
  is resource-backed
- reviewer comments are advisory unless admitted through another gate

## Related Docs

- [Availability, Admission, And Steps](./availability-admission-and-steps.md)
- [Resource-Line Forms](./resource-line-forms.md)
- [Diagnostics, History, And Verification](./diagnostics-history-and-verification.md)
