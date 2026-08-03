use serde_json::{json, Value};

use super::super::{
    oracle::BoundedResidencyCourtroomEvidence,
    schedule::{RevisionScheduleSeeds, ScheduleDecision, SchedulePerturbationPlan},
};
use crate::physical_work_evidence::hex;

pub(super) fn value(evidence: &BoundedResidencyCourtroomEvidence) -> Value {
    let schedule = evidence.schedule();
    let revision = evidence.revision_schedule();
    let rerun = evidence.run().environment().rerun();
    json!({
        "seed": schedule.seed().value(),
        "vocabulary": ScheduleDecision::VOCABULARY,
        "decisions": decisions(schedule),
        "trace_sha256": hex(&schedule.trace().digest()),
        "ci_seed_manifest": {
            "revision": revision.revision(),
            "lanes": lanes(revision),
        },
        "replay": {
            "program": rerun.program(),
            "arguments": rerun.arguments(),
        },
    })
}

fn lanes(revision: &RevisionScheduleSeeds) -> Vec<Value> {
    revision
        .seeds()
        .iter()
        .enumerate()
        .map(|(lane, seed)| {
            let plan = SchedulePerturbationPlan::derive(*seed);
            json!({
                "lane": lane,
                "seed": seed.value(),
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
    use super::{lanes, RevisionScheduleSeeds};

    #[test]
    fn ci_manifest_carries_all_sixteen_schedule_lanes() {
        let revision =
            RevisionScheduleSeeds::derive("659c5775de9637a2f8a7c1c7d7205d0851ada683").unwrap();
        let lanes = lanes(&revision);
        if lanes.len() != 16 {
            panic!("MUTANT_PREDICATE:ci-schedule-manifest-truncated");
        }
        for (index, lane) in lanes.iter().enumerate() {
            assert_eq!(lane["lane"], index);
            assert!(lane["seed"].is_u64());
            assert_eq!(lane["decisions"].as_array().unwrap().len(), 4);
            assert_eq!(lane["trace_sha256"].as_str().unwrap().len(), 64);
        }
    }
}
