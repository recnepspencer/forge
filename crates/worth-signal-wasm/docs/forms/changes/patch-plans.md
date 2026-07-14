# Patch Plans

## What This Feature Is

Patch plans are the form's derived write operations from source truth to
effective truth.

## Why You Use It

- inspect what the form would write
- see whether a change stayed narrow or widened
- understand why readiness or action planning may block a submit

## Stable Entry Points

- `form.patchPlan()`
- `form.readiness()`

## Core Mental Model

The patch plan is derived. You do not author it manually. It is the runtime's
best honest lowering of current form changes.

## How It Executes

1. compare source and effective values
2. derive field or attachment operations where a narrow write is honest
3. widen to broad replacement when a narrow patch would be misleading
4. expose blockers when a patch cannot safely proceed yet

## Small Example

```ts
form.fields.title.set("Ready to ship");

console.log(form.patchPlan().operations);
```

## Real Example

```ts
form.fields.count.input("2");
console.log(form.patchPlan().blocked);

form.fields.count.commitInput();
console.log(form.patchPlan().operations);
```

Before commit, the patch plan stays blocked by uncommitted raw input. After
commit, the runtime lowers a real patch operation.

## How It Relates To Other Features

- Read [Dirty State](./dirty-state.md) for the semantic equality lane beneath
  the patch plan.
- Read [Broad Replacement Vs Narrow Patches](./broad-replacement-vs-narrow-patches.md)
  for widening behavior.

## Inspection And Debugging

- `operations` shows the current patch operations
- `blocked` shows why the patch cannot proceed yet
- `broadReplacement` and `replacement` show widened patch posture
- `breadth` and `equivalenceDigest` help explain surprising results

## Anti-Patterns

- treating the patch plan as mutable authoring input
- assuming every change can lower to one narrow field write

## Current Limits

- resource-backed patch execution belongs to a later resource-backed forms
  section

## Related Docs

- [Dirty State](./dirty-state.md)
- [Patching Complex Edit Forms](./patching-complex-edit-forms.md)
- [Broad Replacement Vs Narrow Patches](./broad-replacement-vs-narrow-patches.md)
