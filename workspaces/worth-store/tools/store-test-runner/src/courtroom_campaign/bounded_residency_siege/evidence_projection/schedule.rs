use serde_json::{json, Value};

use super::super::{
    oracle::BoundedResidencyCourtroomEvidence,
    schedule::{ScheduleDecision, SchedulePerturbationPlan, SourceClosureScheduleSeeds},
};
use crate::physical_work_evidence::hex;

pub(super) fn value(evidence: &BoundedResidencyCourtroomEvidence) -> Value {
    let schedule = evidence.schedule();
    let source_schedule = evidence.source_schedule();
    let rerun = evidence.run().environment().rerun();
    json!({
        "seed": schedule.seed().value(),
        "vocabulary": ScheduleDecision::VOCABULARY,
        "decisions": decisions(schedule),
        "trace_sha256": hex(&schedule.trace().digest()),
        "ci_seed_manifest": {
            "source_closure_sha256": hex(&source_schedule.source_closure_digest()),
            "lanes": lanes(source_schedule),
        },
        "replay": {
            "program": rerun.program(),
            "arguments": rerun.arguments(),
        },
    })
}

fn lanes(source_schedule: &SourceClosureScheduleSeeds) -> Vec<Value> {
    source_schedule
        .seeds()
        .iter()
        .enumerate()
        .map(|(lane, seed)| {
            let plan = SchedulePerturbationPlan::derive(*seed);
            json!({
                "lane": lane,
                "seed": seed.value(),
                "crash_seam": source_schedule
                    .crash_seam(lane)
                    .expect("canonical lane has one explicit crash seam")
                    .label(),
                "decisions": decisions(&plan),
                "trace_sha256": hex(&plan.trace().digest()),
            })
        })
        .collect()
}

fn decisions(schedule: &SchedulePerturbationPlan) -> Vec<Value> {
    schedule
        .trace()
        .decisions()
        .iter()
        .map(|decision| {
            json!({
                "family": decision.family(),
                "choice": decision.choice(),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{lanes, SourceClosureScheduleSeeds};

    #[test]
    fn ci_manifest_carries_all_sixteen_schedule_lanes() {
        let source_schedule = SourceClosureScheduleSeeds::derive([0x65; 32]).unwrap();
        let lanes = lanes(&source_schedule);
        if lanes.len() != 16 {
            panic!("MUTANT_PREDICATE:ci-schedule-manifest-truncated");
        }
        for (index, lane) in lanes.iter().enumerate() {
            assert_eq!(lane["lane"], index);
            assert!(lane["seed"].is_u64());
            assert!(lane["crash_seam"].is_string());
            assert_eq!(lane["decisions"].as_array().unwrap().len(), 5);
            assert_eq!(lane["trace_sha256"].as_str().unwrap().len(), 64);
        }
    }
}
