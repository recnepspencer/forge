# Resource Overview

This is the shortest path into the shipped resource surface.

If you are building ordinary app code, the default lane is:

1. `signals.api(...)`
2. `api.scope(...)` when one area shares request defaults
3. `api.url(...)`
4. `.detail(...)`, `.list(...)`, `.paged(...)`, `.create(...)`, `.update(...)`,
   or `.remove(...)`
5. `family.line(...)`
6. `line.summary()`

## Start With The Question You Actually Have

- "How do I get my first resource working?"
  [Your First Resource](./start-here/your-first-resource.md)
- "Should this be one record, a collection, or a paged list?"
  [Choose A Resource Shape](./start-here/choose-a-resource-shape.md)
- "How do I fetch one thing?"
  [Fetch A Single Record](./fetching/fetch-a-single-record.md)
- "How do I fetch a list?"
  [Fetch A Collection](./fetching/fetch-a-collection.md)
- "How do I fetch pages?"
  [Fetch A Paged List](./fetching/fetch-a-paged-list.md)
- "How do I set auth, headers, basis, or correlation ids?"
  [Request Auth And Context](./fetching/request-auth-and-context.md)
- "How do I choose retry or stale behavior?"
  [Request Policies](./fetching/request-policies.md)
- "How do I create, update, or remove?"
  [Write A Resource](./updating/write-a-resource.md)
- "How do I patch a loaded line locally?"
  [Submit Patches And Replacements](./updating/submit-patches-and-replacements.md)
- "Which effect profile should I use?"
  [Choose An Effect Profile](./updating/choose-an-effect-profile.md)
- "What does the runtime expose after a write runs?"
  [What Happens After A Write](./updating/what-happens-after-a-write.md)
- "How do I understand what a write response changed?"
  [Handling Server Responses](./responses/README.md)
- "How do I update one row without replacing the whole list?"
  [Update One Item Without Replacing Everything](./lists/update-one-item-without-replacing-everything.md)
- "Why is this line stale or still showing the previous visible value?"
  [Caching And Refresh](./caching/README.md)
- "How do I update just part of a resource?"
  [Partial Updates And Derived Views](./partial-updates/README.md)
- "How do I prepare uploads or accepted processing work?"
  [Uploads And Transfers](./transfers/README.md)
- "How do I attach downloads to ordinary resource value?"
  [Downloads And Binary Data](./downloads/README.md)
- "How do I inspect one live line or debug why it changed?"
  [Inspecting And Debugging Resources](./debugging/README.md)
- "How do optimistic effects, rollback, or merge/rebase work?"
  [Effects And Recovery](./effects/README.md)
- "How do I use this resource line as a form source?"
  [Using Resources In Forms](./forms/README.md)
- "How do I bind this resource to a route?"
  [Using Resources In Routes](./router/README.md)
- "How do I drop to raw family authoring?"
  [Advanced Resource Modeling](./advanced/README.md)
- "How do I inspect the proof package or support matrices?"
  [Verification And Proof](./verification/README.md)

## Default Mental Model

A resource family is the recipe.
A line is one live member of that recipe.

The family decides:

- how params become stable identity
- what request posture gets admitted
- what kind of value is loaded
- for collections and pages, how items are identified

The line is where you read and act:

- `line.value()`
- `line.summary()`
- `line.request()`
- `line.status()`
- `line.freshness()`
- `line.diagnostics()`

## When To Reach For The Raw Lane

Start with `signals.api(...)` unless you already know you need:

- manual canonical identity control with `resourceParamIdentity(...)`
- direct `signals.resource.detail(...)` / `collection(...)` / `paged(...)`
- compatibility-oriented external definitions

That raw lane is still supported. It is just not the first stop for ordinary
app-facing work.

## Related Docs

- [Resources Docs Home](./README.md)
- [Glossary](./glossary.md)
- [Recipes](./recipes.md)
- [Older Flat Resource Overview](./overview.md)
