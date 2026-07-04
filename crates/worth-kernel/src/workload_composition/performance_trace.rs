use std::cell::Cell;
use std::sync::OnceLock;
use std::time::Instant;

thread_local! {
    static TRACE_DEPTH: Cell<usize> = const { Cell::new(0) };
}

pub(crate) fn trace_scope<T>(label: &str, action: impl FnOnce() -> T) -> T {
    if !trace_enabled() {
        return action();
    }

    TRACE_DEPTH.with(|depth| {
        let current_depth = depth.get();
        emit(current_depth, format!("start {label}"));
        depth.set(current_depth + 1);
        let start = Instant::now();
        let result = action();
        let elapsed = start.elapsed();
        depth.set(current_depth);
        emit(
            current_depth,
            format!("finish {label} ({:.3}s)", elapsed.as_secs_f64()),
        );
        result
    })
}

pub(crate) fn trace_note(message: impl Into<String>) {
    if !trace_enabled() {
        return;
    }
    TRACE_DEPTH.with(|depth| emit(depth.get(), message.into()));
}

fn trace_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var_os("WORTH_TRACE_PERFORMANCE").is_some()
            || std::env::var_os("WORTH_TRACE_PLANNER_ROUTING_TESTS").is_some()
    })
}

fn emit(depth: usize, message: String) {
    let indent = "  ".repeat(depth);
    eprintln!("[worth-perf] {indent}{message}");
}
