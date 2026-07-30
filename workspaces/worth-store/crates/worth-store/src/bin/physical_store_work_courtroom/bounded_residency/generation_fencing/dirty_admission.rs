use std::cell::Cell;

use worth_store::physical_runtime::{
    LifecycleGeneration, PhysicalDirtyTransitionFailure, PhysicalResidencyCertification,
    ServingPhysicalRuntime,
};

use super::{
    coordinate, GenerationBoundarySnapshot, GenerationFenceCaseEvidence, GenerationFenceCleanup,
    GenerationFenceDenial,
};

pub(super) fn prove(
    serving: &ServingPhysicalRuntime,
    current: &PhysicalResidencyCertification,
    stale: &PhysicalResidencyCertification,
    current_generation: LifecycleGeneration,
    stale_generation: LifecycleGeneration,
) -> Result<GenerationFenceCaseEvidence, String> {
    let lease = current
        .pin_exact(coordinate())
        .map_err(|failure| format!("generation-fence dirty setup failed: {failure:?}"))?;
    let before = GenerationBoundarySnapshot::capture(serving, current)?;
    let mutation_invocations = Cell::new(0_u64);
    match stale.admit_dirty_frame(lease, |_, _| {
        mutation_invocations.set(mutation_invocations.get().saturating_add(1));
    }) {
        Err(PhysicalDirtyTransitionFailure::StaleOrForeignFrame) => {}
        Err(failure) => {
            return Err(format!(
                "stale dirty admission returned the wrong denial: {failure:?}"
            ))
        }
        Ok(dirty) => {
            dirty
                .discard()
                .map_err(|denial| format!("stale dirty cleanup failed: {denial:?}"))?;
            return Err("stale dirty admission consumed a current lease".to_owned());
        }
    }
    Ok(GenerationFenceCaseEvidence {
        current_generation,
        stale_generation,
        denial: GenerationFenceDenial::StaleOrForeignFrame,
        effects: before.effects_since(serving, current)?,
        mutation_invocations: mutation_invocations.get(),
        cleanup: GenerationFenceCleanup::LeaseReleased,
    })
}
