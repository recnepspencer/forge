# Easy Task Board

This is the shortest path:

- two inputs
- two computed values
- one batched update

The runnable example is:

- [`../../examples/easy_task_board.rs`](../../examples/easy_task_board.rs)

## The Shape

```rust
use forge_signal::easy::*;

let mut app = SignalApp::new();

let done_tasks = app.input(2_u32);
let total_tasks = app.input(5_u32);

let progress_label = app.computed(move |ctx| {
    let done = ctx.get(done_tasks);
    let total = ctx.get(total_tasks);
    format!("{done} of {total} tasks done")
});
```

That is the whole point of `easy`.
You should be able to get useful computed state without opening the full
runtime surface on day one.

## Batched Update

```rust
# use forge_signal::easy::*;
# let mut app = SignalApp::new();
# let done_tasks = app.input(2_u32);
# let total_tasks = app.input(5_u32);
app.batch(|graph| {
    graph.set(done_tasks, 4);
    graph.set(total_tasks, 6);
});
```

The important part is not that this is small.
The important part is that this small path still belongs to the same system.

If this app grows into something with transactions, diagnostics, or history,
you are not switching engines.

Read next:

- [../GETTING_STARTED.md](../GETTING_STARTED.md)
- [../guides/running-the-runtime.md](../guides/running-the-runtime.md)
