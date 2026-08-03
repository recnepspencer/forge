use std::time::Duration;

use serde::Serialize;

const CASE_SECONDARY_HANG_GUARD_MS: u64 = 180_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum C7CaseStage {
    WorldConstruction,
    SeedProducer,
    BaselineObserver,
    ServingWriter,
    PostInterruptionObserver,
    FreshReopener,
    EvidenceBinding,
}

impl C7CaseStage {
    const fn label(self) -> &'static str {
        match self {
            Self::WorldConstruction => "world-construction",
            Self::SeedProducer => "seed-producer",
            Self::BaselineObserver => "baseline-observer",
            Self::ServingWriter => "serving-writer",
            Self::PostInterruptionObserver => "post-interruption-observer",
            Self::FreshReopener => "fresh-reopener",
            Self::EvidenceBinding => "evidence-binding",
        }
    }
}

pub(super) struct C7CaseStageDurations {
    pub(super) world_construction: Duration,
    pub(super) seed_producer: Duration,
    pub(super) baseline_observer: Duration,
    pub(super) serving_writer: Duration,
    pub(super) post_interruption_observer: Duration,
    pub(super) fresh_reopener: Duration,
    pub(super) evidence_binding: Duration,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct C7CaseTiming {
    total_elapsed_ms: u64,
    stages: [C7TimedCaseStage; 7],
}

#[derive(Debug, Clone, Copy, Serialize)]
struct C7TimedCaseStage {
    name: &'static str,
    elapsed_ms: u64,
}

impl C7CaseTiming {
    pub(super) fn bind(
        durations: C7CaseStageDurations,
        total_elapsed: Duration,
    ) -> Result<Self, String> {
        let stages = [
            timed(C7CaseStage::WorldConstruction, durations.world_construction),
            timed(C7CaseStage::SeedProducer, durations.seed_producer),
            timed(C7CaseStage::BaselineObserver, durations.baseline_observer),
            timed(C7CaseStage::ServingWriter, durations.serving_writer),
            timed(
                C7CaseStage::PostInterruptionObserver,
                durations.post_interruption_observer,
            ),
            timed(C7CaseStage::FreshReopener, durations.fresh_reopener),
            timed(C7CaseStage::EvidenceBinding, durations.evidence_binding),
        ];
        let total_elapsed_ms = elapsed_ms(total_elapsed);
        let measured_stage_ms = stages.iter().try_fold(0_u64, |total, stage| {
            total
                .checked_add(stage.elapsed_ms)
                .ok_or_else(|| "Courtroom C per-case timing stage total overflowed u64".to_owned())
        })?;
        if measured_stage_ms > total_elapsed_ms {
            return Err(format!(
                "Courtroom C per-case stages measured {measured_stage_ms}ms inside a \
                 {total_elapsed_ms}ms total"
            ));
        }
        if total_elapsed_ms > CASE_SECONDARY_HANG_GUARD_MS {
            return Err(hang_guard_denial(total_elapsed_ms, &stages));
        }
        Ok(Self {
            total_elapsed_ms,
            stages,
        })
    }

    #[cfg(test)]
    const fn total_elapsed_ms(&self) -> u64 {
        self.total_elapsed_ms
    }

    #[cfg(test)]
    fn stage_names(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.stages.iter().map(|stage| stage.name)
    }
}

fn timed(stage: C7CaseStage, elapsed: Duration) -> C7TimedCaseStage {
    C7TimedCaseStage {
        name: stage.label(),
        elapsed_ms: elapsed_ms(elapsed),
    }
}

fn elapsed_ms(elapsed: Duration) -> u64 {
    elapsed.as_millis().try_into().unwrap_or(u64::MAX)
}

fn hang_guard_denial(total_elapsed_ms: u64, stages: &[C7TimedCaseStage; 7]) -> String {
    let slowest = stages
        .iter()
        .max_by_key(|stage| stage.elapsed_ms)
        .expect("a C7 case always measures seven stages");
    let stage_summary = stages
        .iter()
        .map(|stage| format!("{}={}ms", stage.name, stage.elapsed_ms))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "Courtroom C case took {total_elapsed_ms}ms; secondary hang guard is \
         {CASE_SECONDARY_HANG_GUARD_MS}ms; slowest stage {}={}ms; stages [{stage_summary}]",
        slowest.name, slowest.elapsed_ms
    )
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::time::Duration;

    use super::{C7CaseStageDurations, C7CaseTiming};

    #[test]
    fn complete_case_timing_has_one_ordered_entry_per_stage() {
        let timing = C7CaseTiming::bind(
            stage_durations(Duration::from_millis(1)),
            Duration::from_millis(20),
        )
        .unwrap();
        let names = timing.stage_names().collect::<Vec<_>>();
        assert_eq!(names.len(), 7);
        assert_eq!(names.iter().copied().collect::<BTreeSet<_>>().len(), 7);
        assert_eq!(
            names,
            [
                "world-construction",
                "seed-producer",
                "baseline-observer",
                "serving-writer",
                "post-interruption-observer",
                "fresh-reopener",
                "evidence-binding",
            ]
        );
    }

    #[test]
    fn case_timing_serializes_exact_stage_and_total_evidence() {
        let timing = C7CaseTiming::bind(
            stage_durations(Duration::from_millis(1)),
            Duration::from_millis(20),
        )
        .unwrap();
        let encoded = serde_json::to_value(&timing).unwrap();
        assert_eq!(encoded["total_elapsed_ms"], 20);
        assert_eq!(encoded["stages"].as_array().unwrap().len(), 7);
        assert!(encoded["stages"].as_array().unwrap().iter().all(|stage| {
            stage["name"].as_str().is_some() && stage["elapsed_ms"].as_u64().is_some()
        }));
    }

    #[test]
    fn secondary_hang_guard_retains_canonical_multi_case_headroom() {
        let durations = stage_durations(Duration::from_millis(11_000));
        let timing = C7CaseTiming::bind(durations, Duration::from_millis(90_000));
        if timing.is_err() {
            panic!("MUTANT_PREDICATE:c7-case-wall-headroom-regressed");
        }
        assert_eq!(timing.unwrap().total_elapsed_ms(), 90_000);
    }

    #[test]
    fn secondary_hang_guard_retains_observed_environment_headroom() {
        let durations = stage_durations(Duration::from_millis(15_000));
        let timing = C7CaseTiming::bind(durations, Duration::from_millis(125_000));
        if timing.is_err() {
            panic!("MUTANT_PREDICATE:c7-case-observed-wall-headroom-regressed");
        }
        assert_eq!(timing.unwrap().total_elapsed_ms(), 125_000);
    }

    #[test]
    fn impossible_total_and_excessive_secondary_wall_are_rejected() {
        assert!(C7CaseTiming::bind(
            stage_durations(Duration::from_millis(2)),
            Duration::from_millis(10),
        )
        .unwrap_err()
        .contains("inside a 10ms total"));
        assert!(C7CaseTiming::bind(
            stage_durations(Duration::from_millis(1)),
            Duration::from_millis(180_001),
        )
        .unwrap_err()
        .contains("secondary hang guard"));
    }

    #[test]
    fn hang_guard_denial_localizes_slowest_and_every_stage() {
        let denial =
            C7CaseTiming::bind(diagnostic_stage_durations(), Duration::from_millis(180_001))
                .unwrap_err();
        let complete = denial.contains("slowest stage evidence-binding=7000ms")
            && [
                "world-construction=1000ms",
                "seed-producer=2000ms",
                "baseline-observer=3000ms",
                "serving-writer=4000ms",
                "post-interruption-observer=5000ms",
                "fresh-reopener=6000ms",
                "evidence-binding=7000ms",
            ]
            .into_iter()
            .all(|stage| denial.contains(stage));
        if !complete {
            panic!("MUTANT_PREDICATE:c7-case-wall-denial-unlocalized");
        }
    }

    fn stage_durations(each: Duration) -> C7CaseStageDurations {
        C7CaseStageDurations {
            world_construction: each,
            seed_producer: each,
            baseline_observer: each,
            serving_writer: each,
            post_interruption_observer: each,
            fresh_reopener: each,
            evidence_binding: each,
        }
    }

    fn diagnostic_stage_durations() -> C7CaseStageDurations {
        C7CaseStageDurations {
            world_construction: Duration::from_secs(1),
            seed_producer: Duration::from_secs(2),
            baseline_observer: Duration::from_secs(3),
            serving_writer: Duration::from_secs(4),
            post_interruption_observer: Duration::from_secs(5),
            fresh_reopener: Duration::from_secs(6),
            evidence_binding: Duration::from_secs(7),
        }
    }
}
