use worth_foundational::facade::{AspectValue, CanonicalFieldPath, FieldKey, InternedString};

use crate::domain_computation::{
    WorthQueryGraphParticipationProvider, WorthQueryGraphProviderCall,
    WorthQueryGraphProviderCheckpoint, WorthQueryGraphProviderExecution,
    WorthQueryGraphProviderFailure, WorthQueryGraphProviderStep, WorthQueryGraphProviderStepDenial,
    WorthQueryGraphProviderStepDisposition, WorthQueryGraphReadMaterial, WorthQueryGraphReadRow,
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
            step.observe_retained_bytes(1).map_err(step_failure)?;
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
            .then_some(Box::new(ConvergenceCheckpoint) as Box<dyn WorthQueryGraphProviderCheckpoint>)
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
    ) -> Result<Self::Execution, WorthQueryGraphProviderFailure> {
        Ok(CompletedGraphExecution {
            step_ordinal: 0,
            disposition: self.disposition(),
        })
    }
}

struct ConvergenceCheckpoint;

impl WorthQueryGraphProviderCheckpoint for ConvergenceCheckpoint {
    fn retained_bytes(&self) -> u64 {
        1
    }

    fn restore(
        &self,
        _call: &WorthQueryGraphProviderCall,
    ) -> Result<Box<dyn WorthQueryGraphProviderExecution>, WorthQueryGraphProviderFailure> {
        Ok(Box::new(CompletedGraphExecution {
            step_ordinal: 1,
            disposition: FixtureDisposition::YieldThenConverged,
        }))
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
