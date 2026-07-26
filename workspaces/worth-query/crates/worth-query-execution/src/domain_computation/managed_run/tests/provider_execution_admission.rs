use std::sync::{Arc, Mutex};

use super::*;

struct AdmissionExecution;

impl WorthQueryGraphProviderExecution for AdmissionExecution {
    fn advance(
        &mut self,
        step: &mut WorthQueryGraphProviderStep,
    ) -> Result<WorthQueryGraphProviderStepDisposition, WorthQueryGraphProviderFailure> {
        step.perform_work_unit(|| Ok(()))?;
        WorthQueryGraphProviderStepDisposition::complete("admission-execution")
            .map_err(WorthQueryGraphProviderFailure::new)
    }

    fn dispose(&mut self) -> Result<(), WorthQueryGraphProviderFailure> {
        Ok(())
    }
}

struct ForeignAdmissionProvider {
    retained: Arc<Mutex<Option<WorthQueryCooperativeGraphProviderExecution<AdmissionExecution>>>>,
}

impl WorthQueryGraphParticipationProvider<ManagedGraph> for ForeignAdmissionProvider {
    type Execution = AdmissionExecution;

    fn execution_resource_support(
        &self,
    ) -> worth_query_admission::facade::resource_admission::WorthQueryExecutionResourceSupport {
        crate::domain_computation::provider_session::execution_resource_support(
            "foreign-execution-admission",
            8,
        )
    }

    fn begin(
        &self,
        _call: &WorthQueryGraphProviderCall,
        start: &mut WorthQueryGraphProviderExecutionStart,
    ) -> Result<
        WorthQueryCooperativeGraphProviderExecution<Self::Execution>,
        WorthQueryGraphProviderFailure,
    > {
        let mut retained = self.retained.lock().unwrap();
        if let Some(foreign) = retained.take() {
            return Ok(foreign);
        }
        *retained = Some(
            start
                .admit_cooperative_execution(AdmissionExecution)
                .map_err(step_failure)?,
        );
        Err(WorthQueryGraphProviderFailure::new(
            "retain the first execution admission for a hostile retry",
        ))
    }
}

struct MultipleAdmissionProvider;

impl WorthQueryGraphParticipationProvider<ManagedGraph> for MultipleAdmissionProvider {
    type Execution = AdmissionExecution;

    fn execution_resource_support(
        &self,
    ) -> worth_query_admission::facade::resource_admission::WorthQueryExecutionResourceSupport {
        crate::domain_computation::provider_session::execution_resource_support(
            "multiple-execution-admission",
            8,
        )
    }

    fn begin(
        &self,
        _call: &WorthQueryGraphProviderCall,
        start: &mut WorthQueryGraphProviderExecutionStart,
    ) -> Result<
        WorthQueryCooperativeGraphProviderExecution<Self::Execution>,
        WorthQueryGraphProviderFailure,
    > {
        let first = start
            .admit_cooperative_execution(AdmissionExecution)
            .map_err(step_failure)?;
        let _ignored_denial = start.admit_cooperative_execution(AdmissionExecution);
        Ok(first)
    }
}

#[test]
fn admission_from_a_prior_start_cannot_enter_a_fresh_provider_start() {
    let retained = Arc::new(Mutex::new(None));
    let (running, graph) = managed_graph_run_with_provider(
        WorthQueryOperationGraphAccess::Observe,
        ForeignAdmissionProvider {
            retained: Arc::clone(&retained),
        },
    );
    let first_failure = match running.begin_graph_execution(
        &graph,
        WorthQueryManagedGraphCallRequest::new(
            WorthQueryGraphProviderCallKind::Observe,
            "retain-first-admission",
        ),
    ) {
        Ok(_) => panic!("the hostile provider admitted after retaining the first admission"),
        Err(failure) => failure,
    };
    assert_eq!(
        first_failure.kind(),
        crate::domain_computation::WorthQueryDirectGraphExecutionStartFailureKind::ProviderStart
    );

    let foreign_failure = match first_failure.into_running().begin_graph_execution(
        &graph,
        WorthQueryManagedGraphCallRequest::new(
            WorthQueryGraphProviderCallKind::Observe,
            "return-foreign-admission",
        ),
    ) {
        Ok(_) => panic!("an admission from another start arena entered the fresh start"),
        Err(failure) => failure,
    };
    assert_eq!(
        foreign_failure.kind(),
        crate::domain_computation::WorthQueryDirectGraphExecutionStartFailureKind::
            ProviderStartContractDenied
    );
}

#[test]
fn ignored_second_execution_admission_denies_the_provider_start() {
    let (running, graph) = managed_graph_run_with_provider(
        WorthQueryOperationGraphAccess::Observe,
        MultipleAdmissionProvider,
    );
    let failure = match running.begin_graph_execution(
        &graph,
        WorthQueryManagedGraphCallRequest::new(
            WorthQueryGraphProviderCallKind::Observe,
            "multiple-admissions",
        ),
    ) {
        Ok(_) => panic!("ignoring a second-admission denial admitted the first execution"),
        Err(failure) => failure,
    };
    assert_eq!(
        failure.kind(),
        crate::domain_computation::WorthQueryDirectGraphExecutionStartFailureKind::
            ProviderStartContractDenied
    );
    assert!(
        failure.provider_execution_release().is_some(),
        "the denied execution must be explicitly released"
    );
}

fn step_failure(
    denial: crate::domain_computation::WorthQueryGraphProviderStepDenial,
) -> WorthQueryGraphProviderFailure {
    WorthQueryGraphProviderFailure::new(denial.detail())
}
