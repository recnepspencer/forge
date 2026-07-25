use std::path::Path;

use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    FilesystemMediaAdmission, MediaOwnedPhysicalRuntime, PhysicalRuntimeAdmission, PhysicalStore,
    RecordServingAdmissionOutcome, ServingPhysicalRuntime,
};
use worth_store_physical_backend::{FilesystemAccessPosture, MediaFaultSchedule};

pub(super) fn admit_media(
    root: &Path,
    fault_schedule: Option<MediaFaultSchedule>,
) -> Result<MediaOwnedPhysicalRuntime, String> {
    let runtime = PhysicalStore::admit(
        PhysicalRuntimeAdmission::new(root)
            .map_err(|denial| format!("courtroom root denied: {denial:?}"))?,
    )
    .map_err(|denial| format!("courtroom runtime denied: {denial:?}"))?;
    let admission =
        FilesystemMediaAdmission::production(FilesystemAccessPosture::CoordinatedServiceAccount);
    let admission = match fault_schedule {
        Some(schedule) => admission.with_fault_schedule(schedule),
        None => admission,
    };
    match runtime.try_admit_filesystem_media(admission).into_raw() {
        TransitionOutcome::Success(media) => Ok(media),
        TransitionOutcome::Denied(_) => Err("courtroom media admission was denied".to_owned()),
        TransitionOutcome::Deferred(_) => Err("courtroom media admission was deferred".to_owned()),
        TransitionOutcome::Stale(_) => Err("courtroom media admission was stale".to_owned()),
        TransitionOutcome::RebindRequired(_) => {
            Err("courtroom media admission required rebinding".to_owned())
        }
        TransitionOutcome::Failed(_) => Err("courtroom media required inspection".to_owned()),
    }
}

pub(super) fn require_serving<Denial>(
    outcome: RecordServingAdmissionOutcome<Denial>,
    operation: &str,
) -> Result<ServingPhysicalRuntime, String> {
    match outcome.into_raw() {
        TransitionOutcome::Success(serving) => Ok(serving),
        TransitionOutcome::Denied(_) => Err(format!("{operation} was denied")),
        TransitionOutcome::Deferred(_) => Err(format!("{operation} was deferred")),
        TransitionOutcome::Stale(_) => Err(format!("{operation} was stale")),
        TransitionOutcome::RebindRequired(_) => Err(format!("{operation} required rebinding")),
        TransitionOutcome::Failed(_) => Err(format!("{operation} required inspection")),
    }
}
