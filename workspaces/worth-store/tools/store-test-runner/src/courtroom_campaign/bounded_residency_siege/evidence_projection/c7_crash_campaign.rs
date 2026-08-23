use serde_json::{json, Value};
use worth_store::physical_runtime::PhysicalWorkProcessEvidence;

use super::super::c7_crash_campaign::C7CrashCampaignEvidence;
use crate::physical_work_evidence::process_value;

pub(super) fn value(campaign: &C7CrashCampaignEvidence) -> Value {
    json!({
        "selected_boundary_set": campaign.cases().iter().map(|case| case.seam().label()).collect::<Vec<_>>(),
        "processes": process_values(campaign.processes()),
        "cases": campaign.cases().iter().map(|case| {
            let baseline = case.baseline().current();
            let observed = case.observed().current();
            let reopen = case.reopen();
            let rerun = case.rerun();
            json!({
                "crash_seam": case.seam().label(),
                "schedule_choice": case.checkpoint_order().encoded(),
                "checkpoint": case.checkpoint(),
                "timing": case.timing(),
                "baseline": {
                    "store": crate::physical_work_evidence::hex(&baseline.store()),
                    "generation": baseline.generation(),
                    "records": baseline.records(),
                    "payload_bytes": baseline.payload_bytes(),
                    "recovery_obligations": case.baseline().recovery_obligations(),
                    "artifact_manifest": case.baseline().artifacts().iter().map(super::artifact).collect::<Vec<_>>(),
                },
                "observed": {
                    "store": crate::physical_work_evidence::hex(&observed.store()),
                    "generation": observed.generation(),
                    "records": observed.records(),
                    "payload_bytes": observed.payload_bytes(),
                    "recovery_obligations": case.observed().recovery_obligations(),
                    "artifact_manifest": case.observed().artifacts().iter().map(super::artifact).collect::<Vec<_>>(),
                },
                "reopen": {
                    "generation": reopen.identity().generation(),
                    "records": reopen.identity().records(),
                    "inspection_required": reopen.posture().inspection_required(),
                    "recovery_obligations": reopen.posture().recovery_obligations(),
                },
                "c8_recovery": super::c8_recovery::value(case.recovery()),
                "exact_rerun": {
                    "program": rerun.program(),
                    "arguments": rerun.arguments(),
                },
            })
        }).collect::<Vec<_>>(),
    })
}

fn process_values(processes: &[PhysicalWorkProcessEvidence]) -> Vec<Value> {
    processes.iter().map(process_value).collect()
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use worth_store::physical_runtime::PhysicalWorkProcessEvidence;

    use super::process_values;

    #[test]
    fn process_projection_preserves_each_runtime_identity_and_role() {
        let processes = [
            PhysicalWorkProcessEvidence::exited_success(
                "c7:before-wal-append:seed-producer",
                NonZeroU32::new(41).unwrap(),
            )
            .unwrap(),
            PhysicalWorkProcessEvidence::exited_success(
                "c7:before-wal-append:baseline-observer",
                NonZeroU32::new(42).unwrap(),
            )
            .unwrap(),
        ];

        let projected = process_values(&processes);

        assert_eq!(projected.len(), 2);
        assert_eq!(projected[0]["id"], 41);
        assert_eq!(projected[1]["role"], processes[1].role());
    }
}
