use worth_store::physical_runtime::{
    CertificationFrameReadFailure, CertificationFrameWorkFailure, LifecycleGeneration,
    PhysicalResidencyCertification, PhysicalWorkPreEffectDenial, ServingPhysicalRuntime,
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
    drop(
        current
            .pin_exact(coordinate())
            .map_err(|failure| format!("generation-fence hot setup failed: {failure:?}"))?,
    );
    let before = GenerationBoundarySnapshot::capture(serving, current)?;
    match stale.pin_exact(coordinate()) {
        Err(CertificationFrameReadFailure::PhysicalWork(
            CertificationFrameWorkFailure::PreEffect(PhysicalWorkPreEffectDenial::StaleGeneration),
        )) => {}
        Err(failure) => {
            return Err(format!(
                "stale residency read returned the wrong denial: {failure:?}"
            ))
        }
        Ok(_) => return Err("stale residency read consumed a current hot frame".to_owned()),
    }
    Ok(GenerationFenceCaseEvidence {
        current_generation,
        stale_generation,
        denial: GenerationFenceDenial::StaleGeneration,
        effects: before.effects_since(serving, current)?,
        mutation_invocations: 0,
        cleanup: GenerationFenceCleanup::None,
    })
}
