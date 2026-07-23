use super::authority_execution_recording::{
    blocked_before_candidate_record, WritebackAuthorityExecutionContext,
};
use super::*;

pub(super) struct PreparedWritebackAuthorityCandidate {
    candidate: BridgeValidatedWritebackCandidate,
    mapper_witness: BridgeWritebackMapperWitness,
    mapper_record: BridgeWritebackMapperRecord,
}

impl PreparedWritebackAuthorityCandidate {
    pub(super) fn candidate(&self) -> &BridgeValidatedWritebackCandidate {
        &self.candidate
    }

    pub(super) fn mapper_witness(&self) -> &BridgeWritebackMapperWitness {
        &self.mapper_witness
    }

    pub(super) fn mapper_record(&self) -> &BridgeWritebackMapperRecord {
        &self.mapper_record
    }
}

impl RuntimeBridge {
    pub(super) fn prepare_writeback_authority_candidate(
        &self,
        context: &WritebackAuthorityExecutionContext<'_>,
    ) -> Result<PreparedWritebackAuthorityCandidate, BridgeWritebackError> {
        let candidate = self
            .validate_writeback_candidate(
                context.contract(),
                context.effect(),
                context.idempotence(),
                context.loop_prevention(),
                context.strategy_coherence(),
            )
            .map_err(|error| {
                self.diagnostics
                    .record_writeback_execution(blocked_before_candidate_record(context, &error));
                error
            })?;
        let mapper_witness = BridgeWritebackMapperWitness::issue_from_effect(context.effect());
        let mapper_record = BridgeWritebackMapperRecord::new(&mapper_witness, &candidate);
        self.diagnostics
            .record_writeback_mapper(mapper_record.clone());
        Ok(PreparedWritebackAuthorityCandidate {
            candidate,
            mapper_witness,
            mapper_record,
        })
    }
}
