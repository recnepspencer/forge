# Writes and Intent Boundaries

## What This Feature Is

This is the authority boundary for changing truth or staging future change.
`workspace.insert(...)`, `workspace.update(...)`, `workspace.delete(...)`, and
`workspace.batch(...)` are the preferred direct mutation paths.
`workspace.write(...)` remains a stable lower-level compatibility seam. Intent
surfaces exist in the public vocabulary, but they remain support-gated and
must not be treated as part of the same stable compatibility closure as direct
writes.

The important posture is simple: ordinary runtime code should not need
`workspace.write(...)` or `ForgeQueryWriteCommand::*`. Those exist as expert or
compatibility seams while the substrate is being replaced underneath the facade.

## Why You Use It

- you need to apply an authoritative mutation now
- you need a canonical write receipt that routes live, computed, and effect
  consequences
- you need to understand when staged or strategy-shaped change belongs in an
  intent path rather than a direct write path

## Stable Entry Points

Stable:

- `workspace.insert(...)`
- `workspace.update(...)`
- `workspace.delete(...)`
- `workspace.batch(...)`
- `workspace.write(...)`
- `workspace.public_mutation_api_compatibility_report()`

Vocabulary with support gate:

- `workspace.intent(...)`
- `workspace.next_effect_intent(...)`
- `write_intent(...)` inside effect declaration

Important boundary:

- direct writes are part of the stabilized public runtime facade
- intent execution is public vocabulary, but not in the stable compatibility
  support set yet
- callers must treat support admission and backend capability as authoritative
- the mutation compatibility report is the source of truth for which mutation
  surfaces are preferred, compatibility-only, or deprecated compatibility

## Core Mental Model

Use a direct write when product code already knows the mutation to perform.

Use an intent path when product code is naming a strategy-shaped change that
must pass through an admitted intent authority path.

The difference matters:

- `write(...)` mutates authoritative truth directly through the runtime's write
  authority
- `intent(...)` is a higher-level contract about strategy-shaped mutation
  execution
- `next_effect_intent(...)` consumes one staged pending write-intent unit from
  an effect, if the runtime admits that path

Do not blur those two models together.

## How It Executes

Direct write path:

1. Declare the live/computed/effect surfaces that care about the truth.
2. Execute `workspace.insert(...)`, `workspace.update(...)`,
   `workspace.delete(...)`, `workspace.batch(...)`, or the lower-level
   `workspace.write(...)` compatibility path.
3. Receive a canonical write receipt.
4. Live, computed, and effect consequences route from that write.

Direct write receipts now carry:

- mutation family
- declared collection or entity target when the surface has one
- authority lane and basis lane
- declared aspect operations, including whether each authored aspect was a
  `set` or a `clear`
- touched live/computed/effect routing evidence

Intent path:

1. Admit intent support at the runtime/backend level.
2. Execute `workspace.intent(...)` or consume staged work with
   `workspace.next_effect_intent(...)`.
3. Receive an intent receipt or typed denial.

The write path is the stable everyday path. The intent path is a support-gated
authority lane.

## Small Example

```rust
let mut workspace = runtime.workspace("tasks").unwrap();

let receipt = workspace
    .insert("Task", |task| {
        task.aspect("identity.id", "task-1")
            .aspect("title.value", "Buy milk")
    })
    .unwrap();
```

This is the smallest honest example because it shows the stable authoritative
write boundary with no extra orchestration story layered on top.

That receipt can be explained and state-snapshotted in the same public
vocabulary:

```rust
assert_eq!(receipt.mutation_family().as_str(), "insert");
assert_eq!(receipt.declared_collection(), Some("Task"));
assert_eq!(receipt.authority_lane().as_str(), "authoritative-truth");
assert_eq!(receipt.basis_lane().as_str(), "authoritative-truth");

let state = workspace.state(&receipt).unwrap();
let inspection = workspace.inspect(&receipt).unwrap();
```

Ordered multi-entity mutation uses the same aspect vocabulary:

```rust
let batch = workspace
    .batch(|ops| {
        ops.insert("Task", |task| {
            task.aspect("identity.id", "task-1")
                .aspect("title.value", "Buy milk")
        })
        .insert("Task", |task| {
            task.aspect("identity.id", "task-2")
                .aspect("title.value", "Buy bread")
        })
    })
    .unwrap();

assert_eq!(batch.write_count(), 2);
assert!(batch
    .touched_aspect_paths()
    .contains(&"title.value".to_string()));
```

Typed aspect reset stays in the same mutation surface:

```rust
let reset = workspace
    .update("task-1", |task| task.clear("description.value"))
    .unwrap();

assert_eq!(reset.mutation_family().as_str(), "update");
assert_eq!(reset.deltas()[0].aspect_paths, vec!["description.value"]);
assert_eq!(
    reset
        .declared_aspect_operations()
        .iter()
        .map(|operation| format!("{}:{}", operation.kind(), operation.aspect_path()))
        .collect::<Vec<_>>(),
    vec!["clear:description.value"]
);

let inspection = workspace.inspect(&reset).unwrap();
```

Current limit: the runtime batch surface is an ordered multi-write facade. It
does not claim cross-write atomic substrate semantics yet. The batch receipt is
the canonical routing artifact; component write receipts remain available as
operation evidence, and batch inspection exposes their families, collections,
entity identities, touched aspects, and declared `set`/`clear` operations.

## Real Example

```rust
use forge_query::facade::{
    ForgeQueryDerivedViewHandle, ForgeQueryEffectHandle, ForgeQueryIntentDeclaration,
    ForgeQueryLiveView,
};
use serde_json::{json, Value};

let mut workspace = runtime.workspace("intent-capable").unwrap();

let live: ForgeQueryLiveView<Value> = workspace
    .live_view("tasks.intent", |q| {
        q.from("Task")
            .select(["identity.id", "title.value"])
            .schema_basis("tasks-intent")
    })
    .unwrap();

let computed: ForgeQueryDerivedViewHandle<Value> = workspace
    .computed(
        "computed.intent",
        |c| {
            c.depends_on_live(&live)
                .reads(["title.value"])
                .produces(["title.summary"])
        },
        TitleListMaintainer,
    )
    .unwrap();

let effect: ForgeQueryEffectHandle<Value> = workspace
    .effect("intent.effect", |e| {
        e.when_live(&live, ["title.value"])
            .write_intent("strategy.intent.reconcile")
    })
    .unwrap();

let write_receipt = workspace
    .insert("Task", |task| {
        task.aspect("identity.id", "task-1")
            .aspect("title.value", "Direct write title")
    })
    .unwrap();

let intent_receipt = workspace.intent(ForgeQueryIntentDeclaration::strategy_commit(
    "reconcile-task-title",
    "strategy.intent.reconcile",
    "1.0",
    "intent.reconcile.input.v1",
    json!({
        "entity": "task-1",
        "title": "Intent committed title"
    }),
));

let effect_intent_receipt =
    workspace.next_effect_intent(&effect, "1.0", "effect.intent.input.v1");
```

What is authoritative:

- `workspace.insert(...)`, `workspace.update(...)`, `workspace.delete(...)`
- `workspace.write(...)` as the lower-level compatibility path
- admitted `workspace.intent(...)` only when the runtime actually supports it

What is staged:

- `write_intent(...)` on an effect stages pending work

What gets retained:

- canonical write receipts
- pending write-intent residue on effects
- intent receipts or typed denials

What you must not assume:

- that `intent(...)` is automatically stable just because the method exists
- that `next_effect_intent(...)` is meaningful if intent support is absent

## How It Relates To Other Features

- Use [Effects](./effects.md) when pending write-intent residue should be staged
  from reactive changes.
- Use [Branches and Previews](./branches-and-previews.md) when write-like work
  should stay branch-local or preview-local instead of targeting current truth.
- Use the workspace overview when you need the full retained-handle story.

Direct writes are the clean stable path. Intents are the extensible strategy
boundary around that path.

## Inspection And Debugging

- inspect write receipts when you need authoritative mutation routing details
- inspect effect handles when you need to see pending write-intent residue
- inspect intent receipts or denials when you are working in an admitted intent
  runtime

If an intent path fails, the expected outcome is a typed denial rather than a
silent fallback to direct mutation.

## Anti-Patterns

- Treating `workspace.intent(...)` as stable ordinary DX without checking
  support posture.
- Using intent language when a direct write already fully expresses the
  mutation.
- Treating `write_intent(...)` as if it mutates truth immediately.
- Falling back to hidden lower-runtime mutation plumbing when support is
  denied.

## Current Limits

- Direct authoritative writes are stable in the runtime-backed facade.
- Intent execution remains support-gated public vocabulary.
- Temporal, async/resource, and mixed-cause intent semantics remain future
  work, not current guarantees.

## Related Docs

- [Effects](./effects.md)
- [Branches and Previews](./branches-and-previews.md)
- [Workspace Overview](./workspace-overview.md)
