# Writes and Intent Boundaries

## What This Feature Is

This is the authority boundary for changing truth or staging future change.
`workspace.insert(...)`, `workspace.update(...)`, `workspace.update_existing(...)`,
`workspace.delete(...)`, `workspace.delete_with(...)`, `workspace.delete_existing(...)`,
and `workspace.batch(...)` are the preferred
direct mutation paths.
`workspace.write(...)` remains a stable lower-level seam. Intent surfaces now
include real covered families, but callers still need to distinguish
between:

- stable direct mutation surfaces
- covered intent families with shared admission and handoff behavior
- broader intent-shaped vocabulary that is still support-gated or deferred

On primary backends, a multi-command `workspace.batch(...)` is one backend
commit boundary, not a nice-looking wrapper around separate per-command
commits. That atomicity rule is what makes invariant-complete graph closures
and verified existing-target batches trustworthy instead of only “correct if
nobody looks in the middle.”

The important posture is simple: ordinary runtime code should not need
`workspace.write(...)` or `ForgeQueryWriteCommand::*` unless it is
intentionally working at the lower mutation seam.

On admitted families, `workspace.bind_existing_relation(...)` plus
`workspace.update_existing(...)` is also the identity-preserving relation
rewrite surface. The resulting receipt stays an `update` receipt and keeps the
existing-truth relation binding intact instead of treating the rewrite as a
delete-plus-recreate disguise.

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
- `workspace.compose_graph(...)`
- `workspace.bind_existing_entity(...)`
- `workspace.bind_existing_relation(...)`
- `workspace.update_existing(...)`
- `workspace.assert_existing(...)`
- `workspace.delete(...)`
- `workspace.delete_with(...)`
- `workspace.delete_existing(...)`
- `workspace.batch(...)`
- `workspace.write(...)`
- `workspace.public_mutation_surface_report()`

Covered through the shared intent lattice:

- `workspace.verify_existing(...)`
- `workspace.probe_existing(...)`
- `workspace.update_existing_verified(...)`
- `workspace.delete_existing_verified(...)`
- `runtime.write_intent(...)`
- `workspace.write_intent(...)`
- `runtime.write_batch_intent(...)`
- `workspace.write_batch_intent(...)`
- `runtime.next_effect_write_intent(...)`

Still support-gated or deferred beyond the covered families:

- broader intent-shaped vocabulary outside the named covered surfaces
- future temporal, async/resource, and durable restart intent families

Important boundary:

- direct writes are part of the stabilized public runtime facade
- graph-shaped same-batch authoring belongs on `workspace.compose_graph(...)`,
  not on caller-owned `workspace.batch(...)` string choreography
- backend-verified existing-truth lanes are public and typed, but callers must
  read the bridge-backed verification support rows before teaching them as
  ordinary bridge-backed production flows
- covered intent execution is real, but it is not the same thing as blanket
  stable facade-family intent support
- callers must treat support admission and backend capability as authoritative
- the mutation surface report is the source of truth for which mutation
  surfaces are preferred, lower-level, or support-gated

## Core Mental Model

Use a direct write when product code already knows the mutation to perform.

Use an intent path when product code is naming strategy-shaped or runtime-gated
change that must pass through the shared admitted intent path.

The difference matters:

- `write(...)` mutates authoritative truth directly through the runtime's write
  authority
- `write_intent(...)` and `write_batch_intent(...)` are covered mutation
  families that execute through the shared intent lattice
- `next_effect_write_intent(...)` consumes one staged pending write-intent unit
  from an effect, if the runtime admits that path

Do not blur those two models together.

## How It Executes

Direct write path:

1. Declare the live/computed/effect surfaces that care about the truth.
2. Execute `workspace.insert(...)`, `workspace.update(...)`,
   `workspace.compose_graph(...)`,
   `workspace.bind_existing_entity(...)`,
   `workspace.bind_existing_relation(...)`,
   `workspace.update_existing(...)`, `workspace.assert_existing(...)`,
   `workspace.verify_existing(...)`, `workspace.update_existing_verified(...)`,
   `workspace.delete(...)`, `workspace.delete_existing(...)`,
   `workspace.delete_existing_verified(...)`, `workspace.batch(...)`, or the
   lower-level `workspace.write(...)` path.
3. Receive a canonical write receipt.
4. Live, computed, and effect consequences route from that write.

Direct write receipts now carry:

- mutation family
- structured target evidence with distinct declared and resolved target views
- existing-truth binding evidence when the mutation targeted admitted
  authoritative preexisting truth
- existing-truth assertion evidence when the mutation declared or backend-verified
  authoritative truth without mutating stored values
- verified assumption-set evidence on backend-verified existing-truth lanes,
  including assumption snapshot token, assumption snapshot digest, verified
  precondition digest, and verification read-set breadth
- canonical existing-truth binding digests so batch/session consumers can
  preserve one explicit binding story instead of re-summarizing component
  identities themselves
- declared collection or entity target when the surface has one
- resolved target collection and entity identity when the runtime can prove them
- authority lane and basis lane
- declared aspect operations, including whether each authored aspect was a
  `set` or a `clear`
- authoritative causality evidence when the write crossed the bridge-backed
  authority lane
- authoritative provenance evidence when the write crossed the bridge-backed
  authority lane
- aggregate batch mutation evidence when the write is part of an ordered batch
  or authoritative import session
- aggregate existing-truth and symbolic-reference digests when the batch mixes
  preexisting authoritative targets and same-batch declarations
- graph composition program, breadth, symbolic-resolution map, and composition
  evidence when the batch came from `workspace.compose_graph(...)`
- aggregate naming digests when the batch mixes attachment, rebinding, or
  removal outcomes and later consumers need one stable session explanation
- continuity-aware authority evidence when an admitted update-existing mutation
  carries authoritative predecessor and successor meaning through the bridge
- same-batch symbolic target reference evidence on batch components when an
  ordered batch intentionally mutates truth created earlier in that same batch
- touched live/computed/effect routing evidence

That means downstream domains can ask one receipt:

- what class of thing did I declare?
- what class of thing did the runtime actually resolve?
- which authoritative identity intentionally selected the preexisting target?
- what canonical binding artifact proves that existing-target selection?
- what causality chain did authority execution follow?
- what provenance bundle explains the resulting authoritative artifact?

without rebuilding that explanation from raw deltas or lower-runtime logs.

Continuity-aware authority evidence now has one admitted family:

- `continuity_rebind_existing_target(...)` on an update-existing mutation
- `continuity_split_successors(...)` on an update-existing mutation when one
  authoritative predecessor continues as multiple authoritative successors

  That path is intentionally narrow right now. It preserves prior authoritative
  identity, successor authoritative identities, existing-truth binding basis
  digest, resolved target identity, lineage digest, and continuity-resolution
  digest on the resulting receipt and on the aggregate batch/session evidence
  when the write crossed the bridge-backed authority lane.

Preview lanes do not synthesize continuity evidence from authored intent alone.
If continuity intent appears in preview-local execution, the runtime denies it
typed and early with `requires_authoritative_lane` instead of pretending the
preview carried authoritative lineage truth.

If continuity intent appears on a non-update mutation family, or if an
update-shaped continuity mutation does not carry an existing-truth binding, the
runtime denies it typed and early instead of quietly flattening it into generic
metadata.

When a delete would otherwise lose important touched-aspect meaning, the same
surface can retain that meaning explicitly:

```rust
let receipt = workspace
    .delete_with("task-1", |delete| {
        delete
            .target_collection("Task")
            .touches(["title.value", "status.state"])
            .metadata("author", "worth-topo")
    })
    .unwrap();
```

That is the honest path for domains that need delete routing and inspection to
preserve more than "some entity disappeared." The declared target collection is
especially useful in preview or offline-shaped paths where the runtime cannot
re-derive target class from an authoritative commit result.

Intent path:

1. Author the covered mutation or effect-triggered intent.
2. Let Query resolve admission posture through the shared lattice.
3. If admitted, execute only through the sealed handoff for that family.
4. Receive a receipt or typed denial or failure that still carries the
   decision trace and provenance chain.

The write path is still the stable everyday path. The intent path is the
shared admitted boundary for the covered mutation and effect families that
truly need it.

## Examples

Small authoritative write:

```rust
let receipt = workspace
    .insert("Task", |task| {
        task.aspect("identity.id", "task-1")
            .aspect("title.value", "Buy milk")
    })
    .unwrap();

assert_eq!(receipt.mutation_family().as_str(), "insert");
assert_eq!(receipt.declared_collection(), Some("Task"));
assert_eq!(receipt.target_collection(), Some("Task"));
```

Ordered batch mutation:

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
```

For the existing-target mutation family, including probing, retained
assertion, backend-verified update, and backend-verified delete, see
[Existing Truth](../capabilities/existing-truth.md).

For the shared admitted handoff model and proof-chain story behind
`runtime.write_intent(...)`, `workspace.write_intent(...)`,
`runtime.write_batch_intent(...)`, `workspace.write_batch_intent(...)`, and
`runtime.next_effect_write_intent(...)`, see
[Intent Admission](intent-admission.md).

## How It Relates To Other Features

- Use [Effects](effects.md) when pending write-intent residue should be staged
  from reactive changes.
- Use [Branches and Previews](../foundations/branches-and-previews.md) when write-like work
  should stay branch-local or preview-local instead of targeting current truth.
- Use the workspace overview when you need the full retained-handle story.

Direct writes are the clean stable path. Covered intent families are the shared
admitted boundary around the write and effect lanes that genuinely need proof,
handoff, and provenance.

Good to know:

- if a downstream caller needs typed target, provenance, membership, or
  continuity facts from a write receipt, use
  [Projection Consumption](../capabilities/projection-consumption.md) instead of rebuilding
  that meaning from receipt payloads or lower-runtime evidence

For the support-row reading pattern around backend-verified lanes, see:

- [Graph Composition Authoring](../authoring/graph-composition-authoring.md)
- [Existing Truth](../capabilities/existing-truth.md)

## Inspection And Debugging

- inspect write receipts when you need authoritative mutation routing details
- use [Projection Consumption](../capabilities/projection-consumption.md) when those write
  receipts need to become typed consumed facts with their own receipt and
  envelope
- inspect effect handles when you need to see pending write-intent residue
- inspect intent receipts or denials when you are working in an admitted intent
  runtime

If an intent path fails, the expected outcome is a typed denial rather than a
silent fallback to direct mutation.

## Anti-Patterns

- Treating all intent-shaped vocabulary as equally stable ordinary DX.
- Using intent language when a direct write already fully expresses the
  mutation.
- Treating covered intent families as parallel execution systems instead of
  thin admitted wrappers around named runtime seams.
- Falling back to hidden lower-runtime mutation plumbing when support is
  denied.

## Current Limits

- Direct authoritative writes are stable in the runtime-backed facade.
- Covered intent execution is real for the named mutation and effect
  families, but broader future intent families remain deferred.
- Temporal, async/resource, and mixed-cause intent semantics remain future
  work, not current guarantees.

## Related Docs

- [Effects](effects.md)
- [Branches and Previews](../foundations/branches-and-previews.md)
- [Workspace Overview](../foundations/workspace-overview.md)
- [Projection Consumption](../capabilities/projection-consumption.md)
- [Intent Admission](intent-admission.md)



