# Collaboration Overview

## What This Feature Is

This page covers the collaboration surface behind `form.collaboration()`,
`form.reportCollaboration(...)`, and `form.clearCollaboration(...)`.

## Why You Use It

- model lock, lease, branch, or reviewer-only collaboration inside the form
- block writes or submit when another actor owns the relevant truth
- expose presence, comments, and remote-update state without building a second
  collaboration store

## Stable Entry Points

- `collaboration: { ... }`
- `form.collaboration()`
- `form.reportCollaboration(...)`
- `form.clearCollaboration(...)`
- `form.presentationLifecycle("collaboration")`

## Core Mental Model

The collaboration declaration says what kind of shared editing the form
supports. Collaboration reports say what is happening right now.

The report is not draft truth. It is operating posture around the draft truth:

- who can write
- whether submit should be blocked
- whether a remote update is settling
- whether presence or comments are available

## How It Executes

1. declare collaboration mode on the form
2. the runtime derives the starting posture
3. host or app code reports collaboration changes
4. the runtime updates write posture, readiness, action plans, presentation,
   history, and verification

## Small Example

```ts
const form = signals.form({
  source: { title: "Ship docs" },
  collaboration: {
    mode: "singleWriterLock",
    actorId: "me",
  },
  fields: ({ field }) => ({
    title: field("title"),
  }),
});

form.reportCollaboration({
  posture: "blocked",
  lockOwnerId: "reviewer-1",
  reason: "reviewer-1 currently owns the draft lock",
});

console.log(form.collaboration().posture);
console.log(form.fieldWritePosture("title").canWrite);
```

## Real Example

```ts
const form = signals.form({
  source: { title: "Ship docs", notes: "Ready" },
  collaboration: {
    mode: "fieldLease",
    actorId: "me",
    supportsPresence: true,
    supportsComments: true,
  },
  fields: ({ field }) => ({
    title: field("title"),
    notes: field("notes"),
  }),
  presentation: {
    collaboration: { scope: "wholeForm", settlementAcknowledgement: "required" },
  },
});

form.reportCollaboration({
  posture: "settling",
  leasedFields: [{ field: "title", ownerId: "peer-1" }],
  remoteUpdateDigest: "remote:delta-1",
  presence: [{ actorId: "peer-1", status: "active" }],
  comments: [{ id: "comment-1", authorId: "peer-1", target: "title" }],
  reason: "remote title update is settling",
});

console.log(form.collaboration().presence);
console.log(form.presentationLifecycle("collaboration"));
console.log(form.actionPlan("submit").status);
```

## How It Relates To Other Features

- Read [Locks And Leases](./locks-and-leases.md) for write blocking rules.
- Read [Read-Only And Advisory Posture](./read-only-and-advisory-posture.md)
  for reviewer lanes and unavailable posture.
- Read [Resource-Backed Collaboration](./resource-backed-collaboration.md) for
  branch-backed collaboration.

## Inspection And Debugging

- `form.collaboration()` shows current posture, history, and events
- `form.fieldWritePosture(fieldId)` shows whether a field can still be edited
- `form.readiness()` and `form.actionPlan(...)` show submit impact
- `form.presentationLifecycle("collaboration")` shows busy, settling, ready,
  or unavailable collaboration presentation state

## Anti-Patterns

- storing collaboration truth in a second ad hoc store beside the form
- treating presence or comments as available when the declaration did not opt
  into them
- using generic presentation-lane updates for collaboration when the dedicated
  collaboration surface exists

## Current Limits

- collaboration does not merge draft truth for you
- resource-backed branch collaboration still depends on the underlying resource
  line exposing visible branch truth

## Related Docs

- [Locks And Leases](./locks-and-leases.md)
- [Comments And Presence](./comments-and-presence.md)
- [Resource-Backed Collaboration](./resource-backed-collaboration.md)
