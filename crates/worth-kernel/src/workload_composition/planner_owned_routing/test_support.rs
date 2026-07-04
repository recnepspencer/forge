pub(crate) fn run_stack_heavy_planner_owned_routing_test(test: impl FnOnce() + Send + 'static) {
    let result = std::thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(|| {
            test();
        })
        .expect("planner-owned routing test should spawn on a larger stack")
        .join();
    if let Err(panic_payload) = result {
        std::panic::resume_unwind(panic_payload);
    }
}
