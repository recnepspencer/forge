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

## Practical rule

If you are still defining the graph, integrating the runtime, or debugging
correctness, use the default runtime path first.

Take control explicitly only when you have a real reason.
