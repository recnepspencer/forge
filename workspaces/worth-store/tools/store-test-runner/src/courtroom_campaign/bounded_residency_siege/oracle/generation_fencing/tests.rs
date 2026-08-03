use super::verify;
use crate::courtroom_campaign::bounded_residency_siege::protocol::{
    BoundedResidencyGenerationCleanup, BoundedResidencyGenerationDenial,
    BoundedResidencyGenerationFenceCase, BoundedResidencyGenerationFenceEffects,
    BoundedResidencyGenerationFencingObservation,
};

const WORLD_GENERATION: u64 = 9;

#[test]
fn exact_generation_fencing_evidence_is_accepted() {
    assert!(verify(accepted(), WORLD_GENERATION).is_ok());
}

#[test]
fn every_generation_fencing_field_is_independently_hostile() {
    for target in 0..3 {
        for field in 0..17 {
            let mut evidence = accepted();
            let case = match target {
                0 => &mut evidence.read,
                1 => &mut evidence.dirty,
                2 => &mut evidence.writeback,
                _ => unreachable!(),
            };
            corrupt(case, field);
            assert!(
                verify(evidence, WORLD_GENERATION).is_err(),
                "target {target} field {field} escaped the generation oracle"
            );
        }
    }
}

fn accepted() -> BoundedResidencyGenerationFencingObservation {
    let zero = BoundedResidencyGenerationFenceEffects {
        allocation_admissions: 0,
        allocation_releases: 0,
        allocation_other: 0,
        residency_hits: 0,
        residency_faults: 0,
        source_loads: 0,
        dirty_transitions: 0,
        writeback_attempts: 0,
        work_declarations: 0,
        signal_requests: 0,
        scheduler_admissions: 0,
        media_attempts: 0,
    };
    BoundedResidencyGenerationFencingObservation {
        read: BoundedResidencyGenerationFenceCase {
            current_generation: WORLD_GENERATION,
            stale_generation: WORLD_GENERATION - 1,
            denial: BoundedResidencyGenerationDenial::StaleGeneration,
            effects: zero,
            mutation_invocations: 0,
            cleanup: BoundedResidencyGenerationCleanup::None,
        },
        dirty: BoundedResidencyGenerationFenceCase {
            current_generation: WORLD_GENERATION,
            stale_generation: WORLD_GENERATION - 1,
            denial: BoundedResidencyGenerationDenial::StaleOrForeignFrame,
            effects: BoundedResidencyGenerationFenceEffects {
                allocation_releases: 2,
                ..zero
            },
            mutation_invocations: 0,
            cleanup: BoundedResidencyGenerationCleanup::LeaseReleased,
        },
        writeback: BoundedResidencyGenerationFenceCase {
            current_generation: WORLD_GENERATION,
            stale_generation: WORLD_GENERATION - 1,
            denial: BoundedResidencyGenerationDenial::StaleGeneration,
            effects: zero,
            mutation_invocations: 0,
            cleanup: BoundedResidencyGenerationCleanup::DirtyReturned,
        },
    }
}

fn corrupt(case: &mut BoundedResidencyGenerationFenceCase, field: usize) {
    match field {
        0 => case.current_generation += 1,
        1 => case.stale_generation += 1,
        2 => {
            case.denial = match case.denial {
                BoundedResidencyGenerationDenial::StaleGeneration => {
                    BoundedResidencyGenerationDenial::StaleOrForeignFrame
                }
                BoundedResidencyGenerationDenial::StaleOrForeignFrame => {
                    BoundedResidencyGenerationDenial::StaleGeneration
                }
            }
        }
        3 => case.effects.allocation_admissions += 1,
        4 => case.effects.allocation_releases += 1,
        5 => case.effects.allocation_other += 1,
        6 => case.effects.residency_hits += 1,
        7 => case.effects.residency_faults += 1,
        8 => case.effects.source_loads += 1,
        9 => case.effects.dirty_transitions += 1,
        10 => case.effects.writeback_attempts += 1,
        11 => case.effects.work_declarations += 1,
        12 => case.effects.signal_requests += 1,
        13 => case.effects.scheduler_admissions += 1,
        14 => case.effects.media_attempts += 1,
        15 => case.mutation_invocations += 1,
        16 => {
            case.cleanup = match case.cleanup {
                BoundedResidencyGenerationCleanup::None => {
                    BoundedResidencyGenerationCleanup::LeaseReleased
                }
                BoundedResidencyGenerationCleanup::LeaseReleased => {
                    BoundedResidencyGenerationCleanup::DirtyReturned
                }
                BoundedResidencyGenerationCleanup::DirtyReturned => {
                    BoundedResidencyGenerationCleanup::None
                }
            }
        }
        _ => unreachable!(),
    }
}
