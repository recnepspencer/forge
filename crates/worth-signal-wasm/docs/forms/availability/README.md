# Readiness And Permissions

Use availability when the UI posture changes. Use admission when authority or
policy decides whether work may proceed. Readiness brings those results
together with dirty state, parsing, validation, steps, actions, host facts,
routes, collaboration, and resources.

## Availability Describes UI Posture

```ts
const form = signals.form({
  source: { status: "draft", owner: "Ada" },
  fields: ({ field }) => ({
    status: field<string>("status"),
    owner: field<string>("owner"),
  }),
  availability: ({ field }) => ({
    ownerPosture: field("owner", ["status"], (values) =>
      values.status === "published"
        ? { state: "readonly", draftPolicy: "freeze" }
        : "enabled",
    ),
  }),
});
```

Availability can be enabled, disabled, hidden, read-only, required, omitted,
blocked, or unavailable. Its draft policy says whether an existing draft is
preserved, cleared, frozen, or omitted when posture changes.

## Admission Describes Authority

```ts
const governed = signals.form({
  source: { status: "draft", owner: "Ada" },
  fields: ({ field }) => ({
    status: field<string>("status"),
    owner: field<string>("owner"),
  }),
  admission: ({ action }) => ({
    publishApproval: action("publish", "approval", ["status"], () => ({
      posture: "requiresApproval",
      actorDigest: "reviewer-42",
      reason: "publication requires independent review",
    })),
  }),
  actions: ({ action }) => ({
    publish: action("publish", { hostEffect: "article.publish" }),
  }),
});
```

Admission does not invent authentication, roles, signatures, or review policy.
Your application supplies an admitted posture and, for regulated actions,
binding evidence tied to current source, patch, schema, actor, and policy
digests. Stale evidence is not reusable authority.

## Readiness Is The Answer, Not A Boolean Guess

```ts
const readiness = governed.readiness();

if (!readiness.canSubmit) {
  showBlockers(readiness.blockers);
}
```

Changing availability or admission can block a field or action without erasing
the user's draft. The report tells you which layer blocked progress and why.
Availability and admission reports use declared dependency regions but their
aggregate reports are explicitly full scans (`notIncremental`).

## Go Deeper

- [Field And Control Availability](./field-and-control-availability.md)
- [Admission Rules](./admission-rules.md)
- [Readiness Blockers](./readiness-blockers.md)
- [Approval, Signature, And Review Requirements](./approval-signature-and-review-requirements.md)
