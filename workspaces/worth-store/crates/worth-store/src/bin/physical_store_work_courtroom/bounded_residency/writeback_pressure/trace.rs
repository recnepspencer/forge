use worth_store::physical_runtime::{
    PhysicalWorkEffectFate, PhysicalWorkRecoveryDisposition, PublishedRecordBatch,
    RecordPublicationStage,
};

pub(super) struct CandidateWritebackTrace {
    pub(super) count: u64,
    pub(super) last_operation: u64,
}

pub(super) fn candidate_writebacks(
    published: &PublishedRecordBatch,
) -> Result<CandidateWritebackTrace, String> {
    let mut count = 0_u64;
    let mut new_artifact_publications = 0_u64;
    let mut last_operation = 0_u64;
    for effect in published
        .physical_work()
        .effects()
        .iter()
        .copied()
        .filter(|effect| effect.stage() == RecordPublicationStage::CandidateDataWrite)
    {
        let settlement = effect
            .settlement()
            .ok_or_else(|| "candidate writeback omitted terminal settlement".to_owned())?;
        let physical = settlement
            .effect()
            .ok_or_else(|| "candidate writeback omitted backend effect".to_owned())?;
        if physical.work() != effect.identity()
            || settlement.recovery() != PhysicalWorkRecoveryDisposition::ContinueSettlement
        {
            return Err("candidate writeback work trace did not reconcile".to_owned());
        }
        match settlement.effect_fate() {
            PhysicalWorkEffectFate::WriteCompleted => count = count.saturating_add(1),
            PhysicalWorkEffectFate::PublicationCompleted => {
                new_artifact_publications = new_artifact_publications.saturating_add(1);
            }
            _ => return Err("candidate data work retained a non-successful fate".to_owned()),
        }
        let operation = effect.identity().operation().get();
        if operation <= last_operation {
            return Err("candidate writeback operations were not strictly ordered".to_owned());
        }
        last_operation = operation;
    }
    if count == 0 || new_artifact_publications != 1 {
        return Err(
            "ordinary extent append did not retain one creation followed by writebacks".to_owned(),
        );
    }
    Ok(CandidateWritebackTrace {
        count,
        last_operation,
    })
}
