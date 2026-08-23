use worth_store_recovery_runtime::RecoveryReportOutcome;

use super::super::{c7_crash_campaign::C7CrashSeamEvidence, schedule::C7DurabilityCrashSeam};

pub(super) fn verify(case: &C7CrashSeamEvidence) -> Result<(), String> {
    let recovery = case.recovery();
    let expected_role = format!("c7:{}:c8-recovery", case.seam().label());
    if recovery.process().role() != expected_role {
        return Err(format!(
            "C8 recovery process role was `{}`, expected `{expected_role}`",
            recovery.process().role()
        ));
    }
    if recovery.process().fate().label() != "exited-success" {
        return Err("C8 recovery process did not exit successfully".to_owned());
    }
    if recovery.process().process() == case.reopen().identity().process() {
        return Err("C8 recovery reused the ordinary reopen process identity".to_owned());
    }
    let marker = recovery.marker();
    if marker.store() != case.observed().current().store() {
        return Err("C8 recovery crossed the observed Store identity".to_owned());
    }
    if recovery.report().outcome() != RecoveryReportOutcome::Recovered {
        return Err("C8 recovery evidence omitted the recovered terminal outcome".to_owned());
    }
    if case.seam() == C7DurabilityCrashSeam::AfterPhysicalDurabilityBeforeAcknowledgment
        && marker.root_generation() <= case.observed().current().generation()
    {
        return Err(
            "C8 recovery did not publish a successor generation after durable physical effect"
                .to_owned(),
        );
    }
    Ok(())
}
