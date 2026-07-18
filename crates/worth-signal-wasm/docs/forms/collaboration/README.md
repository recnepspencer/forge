# Collaboration

The form collaboration surface projects collaboration posture into field
writes, readiness, diagnostics, and history. It can explain that an editor is
read-only, a lease is stale, a reviewer is advisory, or a resource branch is
required before work proceeds.

It is not a transport, presence service, lock server, or automatic draft-merge
engine.

## What The Controller Can Own

- declared collaboration mode and participant posture;
- admitted lock or lease evidence;
- read-only and advisory blockers;
- comments and presence events reported by the host;
- resource-backed branch and collaboration proof;
- history and verification artifacts tied to the current form basis.

## What Stays Elsewhere

- your backend grants and renews locks or leases;
- your transport delivers presence and comments;
- the resource or application truth layer owns durable branches;
- your product chooses conflict and manual-resolution policy.

Reporting that another user is present does not mutate the form. A lock or
lease can block a write without erasing the local draft. Independent drafts are
not merged merely because both controllers report the same collaborators.

For local-only multi-actor work, use the product's explicit branch or local
truth layer and project its posture into the form. For durable regulated
collaboration, bind the form to real resource/platform authority rather than
treating a browser history array as an audit log.

## Go Deeper

- [Collaboration Overview](./collaboration-overview.md)
- [Locks And Leases](./locks-and-leases.md)
- [Read-Only And Advisory Posture](./read-only-and-advisory-posture.md)
- [Comments And Presence](./comments-and-presence.md)
- [Resource-Backed Collaboration](./resource-backed-collaboration.md)
- [Resource-Backed Forms](../resource-backed/README.md)
