use worth_store::physical_runtime::{
    DataDispatchedPhysicalMutation, PhysicalDataEffectSource, PhysicalWorkEffectFate,
    PhysicalWorkRecoveryDisposition,
};

pub(super) struct CandidateWritebackTrace {
    pub(super) count: u64,
    pub(super) last_operation: u64,
}

pub(super) fn candidate_writebacks(
    dispatched: &DataDispatchedPhysicalMutation,
) -> Result<CandidateWritebackTrace, String> {
    let mut count = 0_u64;
    let mut new_artifact_publications = 0_u64;
    let mut last_operation = 0_u64;
    for effect in dispatched.effects() {
        if effect.effect_identity().is_none()
            || effect.recovery() != PhysicalWorkRecoveryDisposition::ContinueSettlement
        {
            return Err("canonical data effect did not retain terminal settlement".to_owned());
        }
        match (effect.source(), effect.effect_fate()) {
            (
                PhysicalDataEffectSource::NewArtifact,
                PhysicalWorkEffectFate::PublicationCompleted,
            ) => new_artifact_publications = new_artifact_publications.saturating_add(1),
            (
                PhysicalDataEffectSource::ExistingArtifactWriteback,
                PhysicalWorkEffectFate::WriteCompleted,
            ) => count = count.saturating_add(1),
            _ => return Err("canonical data effect retained a dishonest fate".to_owned()),
        }
        let operation = effect.work_identity().operation().get();
        if operation <= last_operation {
            return Err("canonical data-effect operations were not strictly ordered".to_owned());
        }
        last_operation = operation;
    }
    if count == 0 || new_artifact_publications != 1 {
        return Err(
            "canonical extent mutation did not retain one creation followed by writebacks"
                .to_owned(),
        );
    }
    Ok(CandidateWritebackTrace {
        count,
        last_operation,
    })
}
