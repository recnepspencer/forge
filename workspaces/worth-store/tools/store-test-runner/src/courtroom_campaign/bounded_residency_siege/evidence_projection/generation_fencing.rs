use serde_json::{json, Value};

use super::super::protocol::{
    BoundedResidencyGenerationCleanup, BoundedResidencyGenerationDenial,
    BoundedResidencyGenerationFenceCase, BoundedResidencyGenerationFencingObservation,
};

pub(super) fn value(evidence: BoundedResidencyGenerationFencingObservation) -> Value {
    json!({
        "read": case(evidence.read),
        "dirty_admission": case(evidence.dirty),
        "writeback_admission": case(evidence.writeback),
    })
}

fn case(evidence: BoundedResidencyGenerationFenceCase) -> Value {
    let effects = evidence.effects;
    json!({
        "current_generation": evidence.current_generation,
        "stale_generation": evidence.stale_generation,
        "denial": denial(evidence.denial),
        "effects": {
            "allocation_admissions": effects.allocation_admissions,
            "allocation_releases": effects.allocation_releases,
            "allocation_other": effects.allocation_other,
            "residency_hits": effects.residency_hits,
            "residency_faults": effects.residency_faults,
            "source_loads": effects.source_loads,
            "dirty_transitions": effects.dirty_transitions,
            "writeback_attempts": effects.writeback_attempts,
            "work_declarations": effects.work_declarations,
            "signal_requests": effects.signal_requests,
            "scheduler_admissions": effects.scheduler_admissions,
            "media_attempts": effects.media_attempts,
        },
        "mutation_invocations": evidence.mutation_invocations,
        "cleanup": cleanup(evidence.cleanup),
    })
}

const fn denial(value: BoundedResidencyGenerationDenial) -> &'static str {
    match value {
        BoundedResidencyGenerationDenial::StaleGeneration => "stale-generation",
        BoundedResidencyGenerationDenial::StaleOrForeignFrame => "stale-or-foreign-frame",
    }
}

const fn cleanup(value: BoundedResidencyGenerationCleanup) -> &'static str {
    match value {
        BoundedResidencyGenerationCleanup::None => "none",
        BoundedResidencyGenerationCleanup::LeaseReleased => "lease-released",
        BoundedResidencyGenerationCleanup::DirtyReturned => "dirty-returned",
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::{cleanup, denial, value};
    use crate::courtroom_campaign::bounded_residency_siege::protocol::{
        BoundedResidencyGenerationCleanup, BoundedResidencyGenerationDenial,
        BoundedResidencyGenerationFenceCase, BoundedResidencyGenerationFenceEffects,
        BoundedResidencyGenerationFencingObservation,
    };

    #[test]
    fn projection_retains_every_generation_fence_case_and_field() {
        let read = case(
            19,
            BoundedResidencyGenerationDenial::StaleGeneration,
            BoundedResidencyGenerationCleanup::None,
        );
        let dirty = case(
            39,
            BoundedResidencyGenerationDenial::StaleOrForeignFrame,
            BoundedResidencyGenerationCleanup::LeaseReleased,
        );
        let writeback = case(
            59,
            BoundedResidencyGenerationDenial::StaleGeneration,
            BoundedResidencyGenerationCleanup::DirtyReturned,
        );
        let projected = value(BoundedResidencyGenerationFencingObservation {
            read,
            dirty,
            writeback,
        });
        assert_eq!(projected.as_object().unwrap().len(), 3);
        assert_case(&projected["read"], read);
        assert_case(&projected["dirty_admission"], dirty);
        assert_case(&projected["writeback_admission"], writeback);
    }

    fn case(
        current_generation: u64,
        denial: BoundedResidencyGenerationDenial,
        cleanup: BoundedResidencyGenerationCleanup,
    ) -> BoundedResidencyGenerationFenceCase {
        BoundedResidencyGenerationFenceCase {
            current_generation,
            stale_generation: current_generation - 1,
            denial,
            effects: BoundedResidencyGenerationFenceEffects {
                allocation_admissions: current_generation + 1,
                allocation_releases: current_generation + 2,
                allocation_other: current_generation + 3,
                residency_hits: current_generation + 4,
                residency_faults: current_generation + 5,
                source_loads: current_generation + 6,
                dirty_transitions: current_generation + 7,
                writeback_attempts: current_generation + 8,
                work_declarations: current_generation + 9,
                signal_requests: current_generation + 10,
                scheduler_admissions: current_generation + 11,
                media_attempts: current_generation + 12,
            },
            mutation_invocations: current_generation + 13,
            cleanup,
        }
    }

    fn assert_case(projected: &Value, expected: BoundedResidencyGenerationFenceCase) {
        let effects = expected.effects;
        assert_eq!(projected["current_generation"], expected.current_generation);
        assert_eq!(projected["stale_generation"], expected.stale_generation);
        assert_eq!(projected["denial"], denial(expected.denial));
        assert_eq!(
            projected["effects"]["allocation_admissions"],
            effects.allocation_admissions
        );
        assert_eq!(
            projected["effects"]["allocation_releases"],
            effects.allocation_releases
        );
        assert_eq!(
            projected["effects"]["allocation_other"],
            effects.allocation_other
        );
        assert_eq!(
            projected["effects"]["residency_hits"],
            effects.residency_hits
        );
        assert_eq!(
            projected["effects"]["residency_faults"],
            effects.residency_faults
        );
        assert_eq!(projected["effects"]["source_loads"], effects.source_loads);
        assert_eq!(
            projected["effects"]["dirty_transitions"],
            effects.dirty_transitions
        );
        assert_eq!(
            projected["effects"]["writeback_attempts"],
            effects.writeback_attempts
        );
        assert_eq!(
            projected["effects"]["work_declarations"],
            effects.work_declarations
        );
        assert_eq!(
            projected["effects"]["signal_requests"],
            effects.signal_requests
        );
        assert_eq!(
            projected["effects"]["scheduler_admissions"],
            effects.scheduler_admissions
        );
        assert_eq!(
            projected["effects"]["media_attempts"],
            effects.media_attempts
        );
        assert_eq!(
            projected["mutation_invocations"],
            expected.mutation_invocations
        );
        assert_eq!(projected["cleanup"], cleanup(expected.cleanup));
    }
}
