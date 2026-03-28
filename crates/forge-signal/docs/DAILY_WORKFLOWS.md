# Daily Workflows

This guide is the normal path.

Not the research-lab path. Not the framework-author path. Just the stuff you do
all the time when building a real system.

## 1. Build a graph

You start with a `SignalGraph`.

Main pieces:

- `SignalGraph`
- `graph.node()`
- `NodeBuilder`
- `graph.set_dependencies(...)`

Use:

- `build()` for normal nodes
- `on_demand()` when a node should only run when somebody asks for it
- `partitioned_output()` when only part of the output changes at a time
- `output_identity()` when replacement versus continuity matters

Concrete example:

- product price changes
- shipping estimate depends on price and destination
- checkout summary depends on both

That is a graph.

When the same derived thing needs to exist as a durable runtime concept, define
it once with a recipe instead of rebuilding the same idea by hand:

- `runtime.define(Recipe::new(...))`
- `.keyed(...)` for stable keyed families

## 2. Mark changes

When input data changes, say so directly.

Main pieces:

- `mark_changed(...)`
- `mark_changed_with_regions(...)`
- `tx.mark_changed(...)`
- `tx.mark_changed_with_regions(...)`
- `BatchChange`
- `tx.batch_changes()`

Use:

- `mark_changed(...)` for the normal case
- `mark_changed_with_regions(...)` when only part of something changed
- `tx.batch_changes()` when many updates should move together

Example:

- one product price changes: `mark_changed(...)`
- one chunk of a geometry mesh changes: `mark_changed_with_regions(...)`
- fifty rows of source data refresh at once: `tx.batch_changes()`

## 3. Evaluate derived work

Now ask the runtime for work or results.

Main pieces:

- `runtime.target(node).read(...)`
- `runtime.targets(nodes).read_many(...)`
- `runtime.target(node).run(...)`
- `runtime.evaluate_dirty(...)`

Use:

- `target(node).read(...)` when you want one result and want the runtime to do the minimum
- `targets(nodes).read_many(...)` when you want a small set of results together
- `target(node).run(...)` when you are driving execution on purpose
- `evaluate_dirty(...)` when you want to flush pending dirty work

Example:

- page render needs one derived view model: `target(node).read(...)`
- API response needs three related computed values: `targets(nodes).read_many(...)`
- background worker wants to drain everything currently waiting: `evaluate_dirty(...)`

## 4. Use transactions when work must move together

Transactions are for serious updates.

Main pieces:

- `runtime.transaction(...)`
- `tx.mark_changed(...)`
- `tx.target(node).run(...)`
- `tx.target(node).read(...)`

Use a transaction when:

- updates and evaluation need to commit together
- failure needs a clean rollback
- replay and diagnostics should match what really happened

Example:

- user edits a document
- you update several source nodes
- you recompute indexes and summaries
- either the whole thing lands, or none of it does

## 5. Ask diagnostics why something changed

When the runtime surprises you, go here first.

Main pieces:

- `runtime.diagnostics()`
- `diagnostics.why(node)`
- `diagnostics.explain(node)`
- `diagnostics.health_now()`

Use:

- start with `why(...)`
- use `explain(...)` when you want the fuller story
- use `health_now()` when the problem is broader than one node

Example:

- "Why did this recompute?"
- "Why is this branch slower than yesterday?"
- "Why is the runtime keeping so much detail around?"

## 6. Use history when current state is not enough

Sometimes the current graph is not the whole story.

Main pieces:

- `runtime.history()`
- `.snapshot()`
- `.branches()`
- `.replay_for_branch(...)`
- `.lineage_for_node(...)`

Use history when:

- you need snapshots
- you need to inspect branches
- you are debugging long-running behavior
- you need replay or lineage, not just the latest state

Example:

- compare what happened before and after a bad deploy
- inspect a branch-specific recompute path
- replay what the runtime did during a weird batch update

Specialist merge flow:

- `runtime.merge().from(source).into(target).plan()?`
- `planned.execute()?`

## 7. Only reach for explicit executors when you mean it

Most runtime methods already choose an executor for you.

Use explicit executors only when you want deliberate control:

- `StageExecutor::Serial`
- `StageExecutor::conservative_parallel()`
- `StageExecutor::balanced_parallel()`
- `StageExecutor::aggressive_parallel()`

For the full story, read [PARALLEL_EXECUTION.md](./PARALLEL_EXECUTION.md).
