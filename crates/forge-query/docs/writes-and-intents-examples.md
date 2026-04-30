# Writes And Intent Examples

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
assert_eq!(receipt.target_collection(), Some("Task"));
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

Existing-truth-targeted mutation should use the typed binding helpers instead
of raw identity reuse:

```rust
let existing_task = workspace
    .bind_existing_entity(
        ForgeQueryExistingEntityTarget::new("authority:task-1", "task-row-1")?
            .in_target_collection("Task")?,
    )?;

let task_receipt = workspace.update_existing(existing_task, |task| {
    task.aspect("title.value", "Updated title")
})?;

let existing_relation = workspace
    .bind_existing_relation(
        ForgeQueryExistingRelationTarget::new("authority:rel-7", "relation-row-7")?
            .in_target_collection("TaskRelation")?,
    )?;

let relation_receipt = workspace.delete_existing(existing_relation)?;
```

That target-first shape is the supported DX surface for admitted existing-truth
binding families. The runtime owns the canonical binding digest, declared
target, resolved target, causality, and provenance after that point.

Existing-truth assertions come in two distinct lanes:

```rust
let retained = workspace.assert_existing(existing_task.clone(), |task| {
    task.aspect("title.value", "Updated title")
})?;

let verified = workspace.verify_existing(existing_task, |task| {
    task.aspect("title.value", "Updated title")
})?;

assert_eq!(
    retained
        .existing_truth_assertion_evidence()
        .unwrap()
        .mode()
        .as_str(),
    "retained_authoritative_assertion"
);
assert_eq!(
    verified
        .existing_truth_assertion_evidence()
        .unwrap()
        .mode()
        .as_str(),
    "backend_verified_assertion"
);
```

Use `assert_existing(...)` when the caller is intentionally retaining an
authoritative assertion receipt without asking the backend to prove current
stored values. Use `verify_existing(...)` when the backend must check the
asserted aspect values now and deny typed and early on mismatch or missing
truth. Preview lanes deny both because they cannot mint authoritative
verification.

Verified existing-target mutation keeps that same target-first shape while
letting the backend prove current truth immediately before the mutation:

```rust
let update_receipt = workspace.update_existing_verified(
    existing_task.clone(),
    |verify| verify.aspect("status.value", "open"),
    |update| update.aspect("status.value", "closed"),
)?;

let delete_receipt = workspace.delete_existing_verified(
    existing_task,
    |verify| verify.aspect("status.value", "closed"),
    |delete| delete.touch("status.value"),
)?;
```

The update receipt stays an `update` mutation-family receipt. The delete
receipt stays a `delete` mutation-family receipt. Both retain backend-verified
assertion evidence so downstream code can still explain why the runtime was
willing to mutate existing truth.

Ordered batches can also bind a symbolic target once and then mutate it later
without asking downstream domains to rebuild that resolution story:

```rust
let batch = workspace
    .batch(|ops| {
        ops.insert_symbolic("draft-task", "Task", |task| {
            task.aspect("identity.id", "task-draft")
                .aspect("title.value", "Draft")
        })
        .update_symbolic(
            ForgeQuerySymbolicTargetReference::new("draft-task")
                .unwrap()
                .in_target_collection("Task")
                .unwrap(),
            |task| task.aspect("title.value", "Draft renamed"),
        )
    })
    .unwrap();

assert_eq!(
    batch.batch_mutation_evidence().symbolic_target_reference_count(),
    1
);
assert!(batch
    .batch_mutation_evidence()
    .aggregate_symbolic_target_reference_digest()
    .is_some());
assert!(batch
    .batch_mutation_evidence()
    .aggregate_naming_mutation_digest()
    .is_none());
```

Naming-aware authority evidence rides on the same mutation lane through typed
builder helpers instead of raw metadata bags:

```rust
let receipt = workspace
    .update_existing(existing_binding, |task| {
        task.naming_attach_existing_target("persistent-name:task-1", "authority:task-1")
            .aspect("title.value", "Named task renamed")
    })
    .unwrap();

let naming = receipt.naming_mutation_evidence().unwrap();
assert_eq!(naming.family().as_str(), "attach_existing_target");
assert_eq!(naming.attachment_identity(), "persistent-name:task-1");
assert_eq!(
    naming.target_authoritative_identity(),
    Some("authority:task-1")
);
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

Continuity-aware updates use the same builder surface:

```rust
let receipt = workspace
    .update_existing(existing_binding, |task| {
        task.continuity_rebind_existing_target(
            "authority:task-1",
            "authority:task-1-successor",
        )
        .aspect("status.value", "merged")
    })
    .unwrap();

let continuity = receipt.continuity_mutation_evidence().unwrap();
assert_eq!(
    continuity.prior_authoritative_identity(),
    "authority:task-1"
);
assert_eq!(
    continuity.successor_authoritative_identity(),
    Some("authority:task-1-successor")
);
assert_eq!(
    continuity.basis_binding_digest(),
    Some(existing_binding.binding_digest().as_str())
);
```

Current limit: the runtime batch surface is an ordered multi-write facade. It
does not claim cross-write atomic substrate semantics yet. The batch receipt is
the canonical routing artifact; component write receipts remain available as
operation evidence, and batch inspection exposes their families, declared and
resolved target collections/entities, touched aspects, naming evidence, and
declared `set`/`clear` operations.

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
- `workspace.delete_with(...)`
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
