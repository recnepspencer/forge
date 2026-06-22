mod operator_harness;
mod operator_harness_support;

fn run_stack_heavy_test(test: impl FnOnce() + Send + 'static) {
    let result = std::thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(test)
        .expect("stack-heavy public workload operator test should spawn")
        .join();
    if let Err(panic_payload) = result {
        std::panic::resume_unwind(panic_payload);
    }
}
