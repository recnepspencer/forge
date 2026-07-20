# Writing And Server Reconciliation

Resource writes have three separate jobs: describe the request, show any
admitted local intent, and apply what the server actually proved. Keeping those
jobs separate prevents a successful HTTP response from silently becoming
broader client truth than the response earned.

## Declare The Write

Use the route finalizer that matches the endpoint:

- `.create(...)` for a POST-shaped create;
- `.update(...)` for a PUT-shaped update;
- `.remove(...)` for a DELETE-shaped remove;
- `.command(...)` when the operation is not honest CRUD.

```ts
interface ProjectUpdate {
  name: string;
  revision: number;
}

const updateProject = api.url("/projects/:projectId").update<
  Project,
  ProjectUpdate
>({
  async load({ projectId, body }, request) {
    if (!request.target.url) throw new Error("project URL was not admitted");

    const response = await fetch(request.target.url, {
      method: request.method,
      headers: { "content-type": "application/json" },
      body: JSON.stringify(body),
    });
    if (!response.ok) {
      throw new Error(`project ${projectId} failed: ${response.status}`);
    }
    return response.json() as Promise<Project>;
  },
});

const save = updateProject.line({
  projectId: "project-42",
  body: { name: "Launch", revision: 7 },
});

const result = await save.awaitSettlement();
```

The finalizer owns method and request shape. `load(...)` performs the I/O. The
line owns request lifecycle and the result of this resource member; it does not
automatically gain authority over every read line that might contain the same
entity.

## Decide How The Response Maps Back

A write response can prove different things:

- a full detail replacement;
- one collection item or detail field;
- a collection summary;
- a created identity replacing a draft identity;
- a delete or tombstone;
- only partial truth, requiring delivery or refetch for the rest.

Declare those targets on the family. Then inspect `line.mutationResponse()` and
the settlement result instead of assuming “200 OK” updated the entire client.

When the response cannot prove a requested target, the runtime keeps the
fallback typed and visible: partial, stale, awaiting delivery, or requiring a
refetch. It does not manufacture the missing data.

## Local Patches Are A Different Operation

If a loaded line itself needs a local change, build a family-owned patch and
submit it through the line:

```ts
const admission = await tasks.line({ workspaceId: "demo" }).patch(
  tasks.patch.item({
    itemId: "task-42",
    nextItem: { id: "task-42", title: "Reviewed" },
  }),
);
```

The family decides which patch shapes are honest. Broad replacement works only
where broad replacement is admitted; item, field, region, JSON-path, aspect,
and summary patches require the corresponding declared structure.

With a pessimistic or server-canonical profile, a patch has different visible
and recovery behavior than it does with `branchNative()`. Choose the effect
profile from the product guarantee you need, not from which helper is shortest.

## Reconciliation Lifecycle

1. The family admits the request and any declared optimistic patch.
2. Application transport sends the request.
3. Success is reported with the real response identity and server result.
4. The runtime reconciles only declared targets into canonical line truth.
5. Partial or unavailable targets remain explicit.
6. Open intent retires and the projected value is rebuilt.

For concurrent optimistic work, close out the exact runtime-issued `effectId`.
Never choose a request from “the latest effect” when several are open.

## Common Mistakes

- Treating a write line and every related read line as the same authority.
- Applying a response object everywhere it happens to fit structurally.
- Hand-building patch envelopes instead of using `family.patch.*(...)`.
- Issuing an inverse UI patch when a branch-native effect rejects.
- Hiding method, auth, or request context decisions inside transport code.

## Go Deeper

- [Write A Resource](./write-a-resource.md)
- [Submit Patches And Replacements](./submit-patches-and-replacements.md)
- [Choose An Effect Profile](./choose-an-effect-profile.md)
- [What Happens After A Write](./what-happens-after-a-write.md)
- [Handling Server Responses](../responses/README.md)
- [Optimistic Updates](../effects/README.md)
