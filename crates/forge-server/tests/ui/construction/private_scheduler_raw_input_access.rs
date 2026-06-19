use forge_server::{ForgeServerOperationScheduler, ForgeServerQueryHandoffInput};

fn impossible(scheduler: ForgeServerOperationScheduler, input: ForgeServerQueryHandoffInput) {
    let _ = scheduler.schedule_shared_read_batch([input]);
}

fn main() {}
