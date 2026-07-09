use worth_signal::easy::*;

fn main() {
    // Small app, small code.
    // This is the shortest path: inputs, computed values, and batched updates.
    let mut app = SignalApp::new();

    let done_tasks = app.input(2_u32);
    let total_tasks = app.input(5_u32);

    let progress_label = app.computed(move |ctx| {
        let done = ctx.get(done_tasks);
        let total = ctx.get(total_tasks);
        format!("{done} of {total} tasks done")
    });

    let remaining_tasks = app.computed(move |ctx| {
        let done = ctx.get(done_tasks);
        let total = ctx.get(total_tasks);
        total.saturating_sub(done)
    });

    assert_eq!(app.get(progress_label.clone()), "2 of 5 tasks done");
    assert_eq!(app.get(remaining_tasks.clone()), 3);

    app.batch(|graph| {
        graph.set(done_tasks, 4);
        graph.set(total_tasks, 6);
    });

    assert_eq!(app.get(progress_label), "4 of 6 tasks done");
    assert_eq!(app.get(remaining_tasks), 2);
}
