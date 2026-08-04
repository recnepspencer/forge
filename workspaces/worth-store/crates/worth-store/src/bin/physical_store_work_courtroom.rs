#[path = "physical_store_work_courtroom/admission.rs"]
mod admission;
#[path = "physical_store_work_courtroom/arguments.rs"]
mod arguments;
#[path = "physical_store_work_courtroom/bounded_residency/mod.rs"]
mod bounded_residency;
#[path = "physical_store_work_courtroom/c7_crash.rs"]
mod c7_crash;
#[path = "physical_store_work_courtroom/checkpoint.rs"]
mod checkpoint;
#[path = "physical_store_work_courtroom/configuration.rs"]
mod configuration;
#[path = "physical_store_work_courtroom/exact_write.rs"]
mod exact_write;
#[path = "physical_store_work_courtroom/filesystem_profile.rs"]
mod filesystem_profile;
#[path = "physical_store_work_courtroom/process_allocation.rs"]
mod process_allocation;
#[path = "physical_store_work_courtroom/reopen.rs"]
mod reopen;
#[path = "physical_store_work_courtroom/shutdown.rs"]
mod shutdown;
#[path = "physical_store_work_courtroom/write.rs"]
mod write;

#[global_allocator]
static PROCESS_ALLOCATOR: tracking_allocator::Allocator<std::alloc::System> =
    tracking_allocator::Allocator::system();

fn main() {
    if let Err(failure) = run() {
        eprintln!("C5_1_COURTROOM_DENIED {failure}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), String> {
    match arguments::CourtroomInvocation::parse(std::env::args_os().skip(1))? {
        arguments::CourtroomInvocation::Write(invocation) => write::run(invocation),
        arguments::CourtroomInvocation::Reopen(invocation) => reopen::run(invocation),
        arguments::CourtroomInvocation::Shutdown(invocation) => shutdown::run(invocation),
        arguments::CourtroomInvocation::BoundedResidencyProducer(invocation) => {
            bounded_residency::produce(invocation)
        }
        arguments::CourtroomInvocation::BoundedResidencyServing(invocation) => {
            bounded_residency::serve(invocation)
        }
        arguments::CourtroomInvocation::C7Crash(invocation) => c7_crash::run(invocation),
    }
}
