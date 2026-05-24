# Locks And Leases

## What This Feature Is

This page covers the blocking collaboration modes:

- `singleWriterLock`
- `fieldLease`

## Why You Use It

- prevent edits while another actor owns the whole draft
- block only the fields another actor has leased
- keep local semantic truth unchanged while collaboration posture changes

## Stable Entry Points

- `collaboration: { mode: "singleWriterLock" | "fieldLease" }`
- `form.reportCollaboration(...)`
- `form.fieldWritePosture(fieldId)`
- `form.readiness()`
- `form.actionPlan(actionId)`

## Core Mental Model

`singleWriterLock` is whole-form ownership. If the draft lock belongs to
someone else, the form becomes blocked.

`fieldLease` is narrower. The runtime only blocks the fields named in
`leasedFields`, and non-leased fields can keep moving.

## How It Executes

1. declare a blocking collaboration mode
2. report either a lock owner or leased fields
3. the runtime recomputes field write posture and submit readiness
4. the draft value stays the same unless an actual field write succeeds

## Small Example

```ts
form.reportCollaboration({
  posture: "blocked",
  lockOwnerId: "reviewer-1",
  reason: "reviewer-1 currently owns the draft lock",
});

console.log(form.fieldWritePosture("title"));
console.log(form.actionPlan("submit").status);
```

## Real Example

```ts
const form = signals.form({
  source: { title: "Ship docs", notes: "Ready" },
  collaboration: {
    mode: "fieldLease",
    actorId: "me",
  },
  fields: ({ field }) => ({
    title: field("title"),
    notes: field("notes"),
  }),
  actions: ({ submit }) => ({
    submit: submit(),
  }),
});

form.reportCollaboration({
  posture: "settling",
  leasedFields: [{ field: "title", ownerId: "peer-1" }],
  reason: "remote title update is settling",
});

console.log(form.fieldWritePosture("title").canWrite); // false
console.log(form.fieldWritePosture("notes").canWrite); // true
console.log(form.actionPlan("submit").status); // still accepted
```

## How It Relates To Other Features

- Read [Read-Only And Advisory Posture](./read-only-and-advisory-posture.md)
  for reviewer-only or unavailable collaboration.
- Read [Comments And Presence](./comments-and-presence.md) for the advisory
  signals that can travel with a lease-backed form.

## Inspection And Debugging

- `form.collaboration().lockOwnerId` shows current whole-form ownership
- `form.collaboration().leasedFields` shows field-level ownership
- `form.collaboration().events` shows when lock or lease truth changed
- `form.verification().digests.collaborationDigest` changes when collaboration
  posture changes, even if semantic draft truth did not

## Anti-Patterns

- reporting leases for undeclared fields
- assuming a leased field means every field is blocked
- treating a blocked collaboration update as if it had mutated the draft

## Current Limits

- the runtime enforces write posture and readiness, but it does not discover
  leases on its own

## Related Docs

- [Collaboration Overview](./collaboration-overview.md)
- [Read-Only And Advisory Posture](./read-only-and-advisory-posture.md)
