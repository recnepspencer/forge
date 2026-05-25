# Comments And Presence

## What This Feature Is

This page covers the advisory collaboration data behind:

- `supportsPresence`
- `supportsComments`
- `presence`
- `comments`

## Why You Use It

- show who is actively viewing or editing
- attach lightweight reviewer comments to fields
- keep advisory collaboration data on the same report as lock or lease truth

## Stable Entry Points

- `collaboration: { supportsPresence: true, supportsComments: true }`
- `form.reportCollaboration({ presence, comments, ... })`
- `form.collaboration().presence`
- `form.collaboration().comments`

## Core Mental Model

Presence and comments are opt-in advisory signals. They are not available by
default because not every collaboration mode can provide them honestly.

## How It Executes

1. opt into presence and/or comments on the declaration
2. report the latest advisory state
3. the runtime retains it on the collaboration report, events, diagnostics, and
   verification package

## Small Example

```ts
form.reportCollaboration({
  presence: [{ actorId: "peer-1", status: "active" }],
  comments: [{ id: "comment-1", authorId: "peer-1", target: "title" }],
  reason: "review state updated",
});

console.log(form.collaboration().presence);
console.log(form.collaboration().comments);
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
});

form.reportCollaboration({
  posture: "blocked",
  lockOwnerId: "peer-2",
  leasedFields: [{ field: "notes", ownerId: "peer-2" }],
  presence: [
    { actorId: "peer-1", status: "idle" },
    { actorId: "peer-2", status: "viewing" },
  ],
  comments: [
    { id: "comment-1", authorId: "peer-1", target: "title" },
    { id: "comment-2", authorId: "peer-2", target: "notes" },
  ],
  reason: "lease and review advanced",
});

console.log(form.collaboration().events);
```

## How It Relates To Other Features

- Read [Locks And Leases](./locks-and-leases.md) for blocking collaboration.
- Read [Diagnostics Summary](../diagnostics/diagnostics-summary.md) if you need
  the summary surfaces that include collaboration digests.

## Inspection And Debugging

- `form.collaboration().counters.presenceActors` shows how many actors are
  currently present
- `form.collaboration().counters.commentArtifacts` shows how many comments are
  retained
- `form.collaboration().events` shows presence and comment changes over time

## Anti-Patterns

- reporting presence without `supportsPresence: true`
- reporting comments without `supportsComments: true`
- treating comments as if they were a field validation message surface

## Current Limits

- comments are lightweight collaboration records, not a full threaded comment
  system

## Related Docs

- [Collaboration Overview](./collaboration-overview.md)
- [Diagnostics Summary](../diagnostics/diagnostics-summary.md)
