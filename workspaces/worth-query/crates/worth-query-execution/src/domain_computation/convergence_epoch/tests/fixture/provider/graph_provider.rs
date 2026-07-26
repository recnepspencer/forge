use worth_foundational::facade::{AspectValue, CanonicalFieldPath, FieldKey, InternedString};

use crate::domain_computation::{
    WorthQueryCooperativeGraphProviderExecution, WorthQueryGraphParticipationProvider,
    WorthQueryGraphProviderCall, WorthQueryGraphProviderCheckpoint,
    WorthQueryGraphProviderExecution, WorthQueryGraphProviderExecutionStart,
    WorthQueryGraphProviderFailure, WorthQueryGraphProviderRestoreMemory,
    WorthQueryGraphProviderRetainedMemory, WorthQueryGraphProviderStep,
    WorthQueryGraphProviderStepDenial, WorthQueryGraphProviderStepDisposition,
    WorthQueryGraphReadMaterial, WorthQueryGraphReadRow,
};

use super::convergence_provider::ConvergentProvider;
use super::disposition::FixtureDisposition;
use super::resource_support::{
    execution_support, execution_support_with_broader_stage_queue_contract,
};

pub(in crate::domain_computation::convergence_epoch::tests::fixture) struct FixtureGraph;

pub(in crate::domain_computation::convergence_epoch::tests::fixture) struct CompletedGraphExecution
{
    step_ordinal: u8,
    disposition: FixtureDisposition,
    retained: Option<WorthQueryGraphProviderRetainedMemory>,
}

impl WorthQueryGraphProviderExecution for CompletedGraphExecution {
    fn advance(
        &mut self,
        step: &mut WorthQueryGraphProviderStep,
    ) -> Result<WorthQueryGraphProviderStepDisposition, WorthQueryGraphProviderFailure> {
        step.perform_work_unit(|| Ok(()))?;
        if let Some(width) = self.disposition.projection_width() {
            step.emit_projection_chunk(projection_rows(width))
                .map_err(step_failure)?;
        }
        if matches!(self.disposition, FixtureDisposition::YieldThenConverged)
            && self.step_ordinal == 0
        {
            self.step_ordinal = 1;
            self.retained = Some(step.retain_bytes(1).map_err(step_failure)?);
            step.record_checkpoint_available().map_err(step_failure)?;
            return Ok(WorthQueryGraphProviderStepDisposition::continue_work());
        }
        WorthQueryGraphProviderStepDisposition::complete("convergence-provider-receipt")
            .map_err(WorthQueryGraphProviderFailure::new)
    }

    fn suspend(
        &mut self,
    ) -> Result<Box<dyn WorthQueryGraphProviderCheckpoint>, WorthQueryGraphProviderFailure> {
        matches!(self.disposition, FixtureDisposition::YieldThenConverged)
            .then(|| {
                Box::new(ConvergenceCheckpoint {
                    retained: self
                        .retained
                        .take()
                        .expect("yielding convergence execution retains governed memory"),
                }) as Box<dyn WorthQueryGraphProviderCheckpoint>
            })
            .ok_or_else(|| WorthQueryGraphProviderFailure::new("checkpoint not installed"))
    }

    fn dispose(&mut self) -> Result<(), WorthQueryGraphProviderFailure> {
        Ok(())
    }
}

impl WorthQueryGraphParticipationProvider<FixtureGraph> for ConvergentProvider {
    type Execution = CompletedGraphExecution;

    fn execution_resource_support(
        &self,
    ) -> worth_query_admission::facade::resource_admission::WorthQueryExecutionResourceSupport {
        if matches!(
            self.disposition(),
            FixtureDisposition::StageQueueContractMismatch
        ) {
            return execution_support_with_broader_stage_queue_contract();
        }
        execution_support(matches!(
            self.disposition(),
            FixtureDisposition::YieldThenConverged
        ))
    }

    fn begin(
        &self,
        _call: &WorthQueryGraphProviderCall,
        start: &mut WorthQueryGraphProviderExecutionStart,
    ) -> Result<
        WorthQueryCooperativeGraphProviderExecution<Self::Execution>,
        WorthQueryGraphProviderFailure,
    > {
        let execution = CompletedGraphExecution {
            step_ordinal: 0,
            disposition: self.disposition(),
            retained: None,
        };
        start
            .admit_cooperative_execution(execution)
            .map_err(step_failure)
    }
}

struct ConvergenceCheckpoint {
    retained: WorthQueryGraphProviderRetainedMemory,
}

impl WorthQueryGraphProviderCheckpoint for ConvergenceCheckpoint {
    fn retained_bytes(&self) -> u64 {
        u64::try_from(self.retained.len()).unwrap()
    }

    fn restore(
        &self,
        _call: &WorthQueryGraphProviderCall,
        memory: &mut WorthQueryGraphProviderRestoreMemory,
    ) -> Result<
        WorthQueryCooperativeGraphProviderExecution<Box<dyn WorthQueryGraphProviderExecution>>,
        WorthQueryGraphProviderFailure,
    > {
        let execution = Box::new(CompletedGraphExecution {
            step_ordinal: 1,
            disposition: FixtureDisposition::YieldThenConverged,
            retained: Some(memory.rebind(&self.retained).map_err(step_failure)?),
        }) as Box<dyn WorthQueryGraphProviderExecution>;
        memory
            .admit_cooperative_execution(execution)
            .map_err(step_failure)
    }
}

fn step_failure(denial: WorthQueryGraphProviderStepDenial) -> WorthQueryGraphProviderFailure {
    WorthQueryGraphProviderFailure::new(denial.detail())
}

fn projection_rows(row_count: usize) -> WorthQueryGraphReadMaterial {
    let path = CanonicalFieldPath::single(FieldKey::new("state").expect("valid field key"));
    WorthQueryGraphReadMaterial::new((0..row_count).map(|index| {
        WorthQueryGraphReadRow::from_native_fields(
            format!("candidate-{index}"),
            [(
                path.clone(),
                AspectValue::String(InternedString::from(format!("state-{index}"))),
            )]
            .into_iter()
            .collect(),
        )
        .expect("fixture graph row must construct")
    }))
}
