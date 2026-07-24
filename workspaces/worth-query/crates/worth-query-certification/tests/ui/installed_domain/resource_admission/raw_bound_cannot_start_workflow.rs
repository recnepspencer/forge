use worth_query::facade::foundation::ObservationLaneWitness;
use worth_query_host::facade::{
    domain::{WorthQueryExecutableDomainOperation, WorthQueryWorkflowOperation},
    installed::WorthQueryBoundDomainOperation,
    runtime::WorthQueryWorkspace,
};

fn bypass<D: 'static, O: 'static, F: 'static>(
    bound: WorthQueryBoundDomainOperation<D, O, F, ObservationLaneWitness>,
    workspace: &mut WorthQueryWorkspace,
) where
    O: WorthQueryExecutableDomainOperation<D, F, Execution = WorthQueryWorkflowOperation>,
{
    let _ = bound.start_workflow(workspace);
}

fn main() {}
