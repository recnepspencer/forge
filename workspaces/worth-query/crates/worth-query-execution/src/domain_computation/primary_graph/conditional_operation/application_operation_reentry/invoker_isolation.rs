pub(in crate::domain_computation::primary_graph::conditional_operation) fn isolate_invoker<
    Output,
>(
    invoke: impl FnOnce() -> Output,
) -> Result<Output, &'static str> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(invoke))
        .map_err(|_| "installed temporal operation invoker panicked")
}
