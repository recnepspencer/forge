use worth_store::physical_runtime::{
    LifecycleGeneration, PhysicalResidencyCertification, PhysicalWorkPreEffectDenial,
    PhysicalWritebackFailureCause, ServingPhysicalRuntime,
};
use worth_store_physical_backend::ArtifactRangeWriteDurabilityRequirement;

use super::{
    coordinate, GenerationBoundarySnapshot, GenerationFenceCaseEvidence, GenerationFenceCleanup,
    GenerationFenceDenial,
};

const REPLACEMENT: [u8; 8] = [0x27, 0xc6, 0x91, 0x48, 0xa5, 0xbe, 0xd0, 0x3f];

pub(super) fn prove(
    serving: &ServingPhysicalRuntime,
    current: &PhysicalResidencyCertification,
    stale: &PhysicalResidencyCertification,
    current_generation: LifecycleGeneration,
    stale_generation: LifecycleGeneration,
) -> Result<GenerationFenceCaseEvidence, String> {
    let lease = current
        .pin_exact(coordinate())
        .map_err(|failure| format!("generation-fence writeback setup failed: {failure:?}"))?;
    let dirty = current
        .admit_dirty_frame(lease, |_, target| target.copy_from_slice(&REPLACEMENT))
        .map_err(|failure| format!("generation-fence dirty replacement failed: {failure:?}"))?;
    let before = GenerationBoundarySnapshot::capture(serving, current)?;
    let failure = match stale.prepare_writeback(
        dirty,
        ArtifactRangeWriteDurabilityRequirement::BufferedWrite,
    ) {
        Err(failure) => failure,
        Ok(_) => return Err("stale writeback consumed current dirty authority".to_owned()),
    };
    if failure.cause()
        != PhysicalWritebackFailureCause::PreEffect(PhysicalWorkPreEffectDenial::StaleGeneration)
    {
        return Err(format!(
            "stale writeback returned the wrong denial: {:?}",
            failure.cause()
        ));
    }
    let effects = before.effects_since(serving, current)?;
    failure
        .into_dirty()
        .discard()
        .map_err(|denial| format!("stale writeback dirty cleanup failed: {denial:?}"))?;
    Ok(GenerationFenceCaseEvidence {
        current_generation,
        stale_generation,
        denial: GenerationFenceDenial::StaleGeneration,
        effects,
        mutation_invocations: 0,
        cleanup: GenerationFenceCleanup::DirtyReturned,
    })
}
