# Presentation And External Lanes

## What This Feature Is

This feature owns visible settlement posture for attachments, media, handoff,
exit, action busy state, message reveal, and other presentation-only lanes that
must stay separate from semantic form truth.

## Why You Use It

- keep upload, cropper, share, exit-confirmation, and visible-settlement work
  explicit
- let UI settlement lag semantic fulfillment honestly
- update presentation posture without forging source, draft, or readiness
  changes

## Stable Entry Points

- presentation declaration in `signals.form({... presentation: ... })`
- `attachments()`
- `attachmentTransfers()`
- `media()`
- `handoff()`
- `exit()`
- `messages()`
- `presentation()`
- `presentationLifecycle(laneId)`
- `reportAttachments(...)`
- `clearAttachments(...)`
- `reportMedia(...)`
- `clearMedia(...)`
- `reportHandoff(...)`
- `clearHandoff(...)`
- `reportExit(...)`
- `clearExit(...)`
- `reportPresentationLane(...)`
- `clearPresentationLane(...)`
- `reportMessages(...)`
- `clearMessages(...)`
- `acknowledgePresentation(...)`
- `timeoutPresentation(...)`
- `presentationHistory()`

## Core Mental Model

Presentation lanes are UI-facing state, not semantic authority. The form may
already be semantically fulfilled while the visible lane is still busy,
settling, failed, unavailable, or waiting for acknowledgement.

There are two important sub-lanes to keep straight:

- the `entry` lane, which can wait for declared bootstrap prerequisites such as
  source admission, draft restore, validation, focus targeting, or layout
  measurement before the form is visually ready
- external lanes such as attachments, media, handoff, exit, collaboration, and
  messages, which track visible settlement without changing semantic form truth

## How It Executes

The runtime accepts typed lane reports, derives presentation summaries, and
retains per-lane update and settlement history. Semantic fulfillment and
visible settlement stay separate until declared dependencies settle, time out,
or are acknowledged.

## Small Example

```ts
const form = signals.form({
  source: { title: "Ship docs" },
  fields: ({ field }) => ({
    title: field("title", { row: "main" }),
  }),
  presentation: {
    entry: {
      bootstrap: {
        layoutMeasurement: true,
      },
    },
  },
});

console.log(form.presentationLifecycle("entry"));
```

This is the smallest honest example because entry bootstrap is part of the
shipped presentation surface. A form can be semantically ready while the entry
lane is still waiting on a declared visible prerequisite.

## Real Example

```ts
form.reportAttachments({
  status: "busy",
  reason: "uploading evidence",
  section: "evidence",
});

form.reportHandoff({
  status: "pending",
  reason: "opening share handoff",
  scopeKind: "modal",
  surfaceId: "share-modal",
});

form.reportExit({
  status: "pending",
  reason: "confirming route exit",
  scopeKind: "route",
  surfaceId: "browser-history",
});

console.log(form.presentation());
```

The runtime keeps each lane scoped and typed. Those updates are inspectable,
but they do not mutate source, draft, patch, validation, or readiness truth.

## How It Relates To Other Features

- Pair it with [Host, Interaction, Accessibility, And Layout](./host-interaction-accessibility-and-layout.md)
  for focus and layout-dependent settlement.
- Pair it with [Actions And Submit](./actions-and-submit.md) when visible busy
  or post-fulfillment acknowledgement matters.
- Pair it with [Diagnostics, History, And Verification](./diagnostics-history-and-verification.md)
  when you need retained presentation history.

## Inspection And Debugging

- `attachments()`, `media()`, `handoff()`, and `exit()` show feature-specific
  posture.
- `presentationLifecycle("entry")` is where you inspect bootstrap posture,
  required dependencies, blocking dependencies, and explicit unavailable entry
  prerequisites.
- `attachmentTransfers()` covers transfer-specific upload or handoff state
  rather than the broader attachment lane.
- `messages()` and `reportMessages(...)` cover externally managed message
  visibility without rewriting validation truth.
- `presentation()` shows the whole lane summary.
- `presentationLifecycle(laneId)` shows the current lane state and whether
  acknowledgement is still required.
- `clear*`, `clearPresentationLane(...)`, `acknowledgePresentation(...)`, and
  `timeoutPresentation(...)` are the supported ways to settle or dismiss a
  visible lane.
- `presentationHistory()` retains lane updates and settlement artifacts.

## Anti-Patterns

- using presentation lanes as if they were validation or readiness truth
- updating generic lane posture without required scope metadata
- assuming semantic submit fulfillment means the user-visible flow is already
  settled

## Current Limits

- route authority and browser-history remain external; the form lane only
  reports typed posture
- unavailable host or adapter dependencies stay explicit
- entry bootstrap only waits on declared prerequisites; it does not invent its
  own router, host, or async lifecycle rules
- presentation lanes do not own resource, collaboration, or source authority

## Related Docs

- [Host, Interaction, Accessibility, And Layout](./host-interaction-accessibility-and-layout.md)
- [Actions And Submit](./actions-and-submit.md)
- [Diagnostics, History, And Verification](./diagnostics-history-and-verification.md)
