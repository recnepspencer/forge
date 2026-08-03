use super::super::generation_fencing::{
    GenerationFenceCaseEvidence, GenerationFenceCleanup, GenerationFenceDenial,
    GenerationFencingEvidence,
};

pub(super) fn emit(evidence: &GenerationFencingEvidence) {
    emit_case("BOUNDED_RESIDENCY_GENERATION_READ", evidence.read);
    emit_case("BOUNDED_RESIDENCY_GENERATION_DIRTY", evidence.dirty);
    emit_case("BOUNDED_RESIDENCY_GENERATION_WRITEBACK", evidence.writeback);
}

fn emit_case(marker: &str, case: GenerationFenceCaseEvidence) {
    let effects = case.effects;
    println!(
        "{marker} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
        case.current_generation.get(),
        case.stale_generation.get(),
        denial(case.denial),
        effects.allocation_admissions,
        effects.allocation_releases,
        effects.allocation_other,
        effects.residency_hits,
        effects.residency_faults,
        effects.source_loads,
        effects.dirty_transitions,
        effects.writeback_attempts,
        effects.work_declarations,
        effects.signal_requests,
        effects.scheduler_admissions,
        effects.media_attempts,
        case.mutation_invocations,
        cleanup(case.cleanup),
    );
}

const fn denial(denial: GenerationFenceDenial) -> &'static str {
    match denial {
        GenerationFenceDenial::StaleGeneration => "stale-generation",
        GenerationFenceDenial::StaleOrForeignFrame => "stale-or-foreign-frame",
    }
}

const fn cleanup(cleanup: GenerationFenceCleanup) -> &'static str {
    match cleanup {
        GenerationFenceCleanup::None => "none",
        GenerationFenceCleanup::LeaseReleased => "lease-released",
        GenerationFenceCleanup::DirtyReturned => "dirty-returned",
    }
}
