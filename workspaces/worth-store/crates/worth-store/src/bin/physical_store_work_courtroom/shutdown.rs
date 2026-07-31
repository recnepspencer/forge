use worth_store::physical_runtime::{PhysicalRecordOpen, PhysicalStoreClosePhase};

use super::arguments::ShutdownInvocation;

pub(super) fn run(invocation: ShutdownInvocation) -> Result<(), String> {
    let _configuration =
        super::configuration::CourtroomConfiguration::read(&invocation.configuration)?;
    let (format, _, access) = super::configuration::record_configuration();
    let media = super::admission::admit_media(&invocation.root, None)?;
    let durability = super::admission::admit_durability(&media)?;
    let serving = super::admission::require_serving(
        media.open_record_store(PhysicalRecordOpen::new(format, access, durability)),
        "record-store shutdown open",
    )?;
    let plan = serving.close_plan();
    let gate = plan.certification_pause_at(PhysicalStoreClosePhase::SignalDisposed);
    super::checkpoint::watch_close("during-shutdown", gate);
    let _closed = plan.execute();
    super::checkpoint::park_until_parent_kills();
}
