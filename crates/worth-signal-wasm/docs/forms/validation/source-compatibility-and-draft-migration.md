# Source Compatibility And Draft Migration

## What This Feature Is

This page covers source schema drift, compatibility posture, and draft
migration behavior.

## Why You Use It

- keep long-lived drafts honest when the source schema changes
- preserve or migrate draft truth explicitly
- inspect why a draft was accepted, migrated, or denied after schema drift

## Stable Entry Points

- source descriptor `schemaVersion`
- source descriptor `migrateDraft(...)`
- `form.sourceCompatibility()`
- `form.sourceCompatibilityHistory()`

## Core Mental Model

Schema drift is not just another validation error. It is a source-authority
compatibility question.

The form can report posture such as:

- `current`
- `compatible`
- `migrated`
- `unavailable`

## How It Executes

1. the source declares a schema version
2. the runtime compares current and draft schema versions
3. the optional `migrateDraft(...)` hook can preserve, migrate, or deny draft
   truth
4. the compatibility result and history are retained

## Small Example

```ts
const form = signals.form({
  source: {
    value: source(),
    schemaVersion: "task-v2",
    migrateDraft(draft, context) {
      return {
        kind: "migrated",
        draft: { ...draft, status: "draft" },
      };
    },
  },
  fields: ({ field }) => ({
    title: field("title"),
    status: field("status"),
  }),
});
```

## Real Example

```ts
console.log(form.sourceCompatibility());
console.log(form.sourceCompatibilityHistory());
```

Those reads tell you whether the form stayed current, accepted compatible
drift, migrated draft truth, or declared the drift unavailable.

## How It Relates To Other Features

- Read [Choosing A Form Source](../getting-started/choosing-a-form-source.md)
  for the source-authority lane underneath this feature.
- Read [Server Canonicalization](./server-canonicalization.md) when the source
  changed because of an action result rather than schema drift.

## Inspection And Debugging

- `sourceCompatibility().posture` is the first read
- `sourceCompatibility().artifact` explains the latest compatibility event
- `sourceCompatibilityHistory()` shows the retained drift trail

## Anti-Patterns

- treating schema drift like ordinary validation
- silently replacing draft truth without a migration or compatibility posture

## Current Limits

- route-authority draft continuity is a separate seam
- resource-backed schema and delivery drift are part of later docs

## Related Docs

- [Choosing A Form Source](../getting-started/choosing-a-form-source.md)
- [Server Canonicalization](./server-canonicalization.md)
