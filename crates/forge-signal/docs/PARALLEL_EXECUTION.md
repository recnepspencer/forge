# Parallel Execution

Parallelism in `forge-signal` is mostly automatic until you decide otherwise.

## The short answer

If you use the normal runtime path:

- `runtime.target(node).read(...)`
- `runtime.targets(nodes).read_many(...)`
- `runtime.target(node).run(...)`
- `runtime.evaluate_dirty(...)`
- `runtime.execute_prepared_plan(...)`

the runtime chooses an executor automatically from its derived evaluation
strategy.

So for everyday use, you usually do not need to touch parallel settings at all.

If you want explicit control, use the `*_with_executor(...)` methods.

## Automatic behavior

`SignalRuntime` derives an evaluation strategy and maps that strategy to a
`StageExecutor`.

That means normal runtime calls already do a decent job without manual tuning.

Important caveat:

- graph-level `SignalGraph` execution APIs are still explicit
- they do not automatically choose a parallel executor for you

So the automatic story is mainly the runtime story, not the raw graph story.

## Explicit control

Use these when you actually want to shape execution:

- `StageExecutor::Serial`
- `StageExecutor::conservative_parallel()`
- `StageExecutor::balanced_parallel()`
- `StageExecutor::aggressive_parallel()`
- `StageExecutor::parallel(min_stage_width)`
- `StageExecutor::full_parallel(min_stage_width)`

And, when you need detailed tuning:

- `ParallelExecutionPolicy`

## Which one should I use?

If you are choosing on purpose:

- `Serial` when you want the simplest, most predictable execution path
- `conservative_parallel()` for request-driven or observability-heavy workloads
- `balanced_parallel()` for general production use
- `aggressive_parallel()` for heavier compute or deliberate stress

Concrete examples:

- local debugging and "just make it obvious": `Serial`
- normal app backend with decent parallel work: `balanced_parallel()`
- big compute-heavy derived jobs: `aggressive_parallel()`

## When not to care

If you are still:

- defining the graph
- getting the runtime integrated
- debugging semantics

then do not burn time tuning executors yet.

Use the default runtime path first, then take control explicitly only when you
have a real reason.
