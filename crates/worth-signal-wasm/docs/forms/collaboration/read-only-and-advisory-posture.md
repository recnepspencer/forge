# Read-Only And Advisory Posture

## What This Feature Is

This page covers collaboration modes and postures where the form is not a
normal writable draft:

- `reviewerCommentOnly`
- `unavailable`
- explicit `readOnly: true`

## Why You Use It

- let reviewers comment without mutating the draft
- show that collaboration infrastructure is unavailable without pretending the
  form is locally broken
- expose a first-class reason for blocked or advisory collaboration posture

## Stable Entry Points

- `collaboration: { mode: "reviewerCommentOnly" | "unavailable" }`
- `form.reportCollaboration({ readOnly: true, ... })`
- `form.collaboration()`
- `form.readiness()`
- `form.presentationLifecycle("collaboration")`

## Core Mental Model

Read-only collaboration is not the same as a field lease.

- a lease says "someone else owns part of the editable draft"
- reviewer-only says "this actor is not in an editing role"
- unavailable says "collaboration truth cannot be provided right now"
- `readOnly: true` is the explicit update lane when your collaboration source
  needs to mark the current posture as non-editable

## How It Executes

1. declare the collaboration mode
2. the runtime derives the starting read-only or unavailable posture
3. writes and submit are blocked only when that posture semantically requires
   it

## Small Example

```ts
const form = signals.form({
  source: { title: "Ship docs" },
  collaboration: {
    mode: "reviewerCommentOnly",
    actorId: "reviewer-1",
    supportsComments: true,
  },
  fields: ({ field }) => ({
    title: field("title"),
  }),
});

console.log(form.collaboration().readOnly);
console.log(form.fieldWritePosture("title").canWrite);
```

## Real Example

```ts
const form = signals.form({
  source: { title: "Ship docs" },
  collaboration: {
    mode: "unavailable",
  },
  fields: ({ field }) => ({
    title: field("title"),
  }),
  presentation: {
    collaboration: { unavailableAcknowledgement: "required" },
  },
});

console.log(form.collaboration().posture); // unavailable
console.log(form.presentationLifecycle("collaboration"));

form.fields.title.set("Local edit still allowed");
```

## How It Relates To Other Features

- Read [Comments And Presence](./comments-and-presence.md) when reviewer-only
  posture still needs advisory data.
- Read [Route Authority Handoff](../route-coupling/route-authority-handoff.md)
  if your form can also become read-only for route-owned reasons.

## Inspection And Debugging

- `form.collaboration().posture` tells you whether the form is active,
  blocked, settling, or unavailable
- `form.collaboration().readOnly` tells you whether mutation is allowed
- `form.readiness().blockers` shows submit impact

## Anti-Patterns

- treating collaboration unavailable as if it always blocks local editing
- using reviewer-comment-only when the actor should still be able to edit
- hiding the reason for blocked writes from the UI

## Current Limits

- the runtime does not infer reviewer roles from your auth system

## Related Docs

- [Collaboration Overview](./collaboration-overview.md)
- [Comments And Presence](./comments-and-presence.md)
