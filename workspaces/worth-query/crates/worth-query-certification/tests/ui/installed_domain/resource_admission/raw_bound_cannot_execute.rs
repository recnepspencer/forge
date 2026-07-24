use worth_query::facade::foundation::ObservationLaneWitness;
use worth_query_host::facade::{
    domain::{WorthQueryDirectOperation, WorthQueryExecutableDomainOperation},
    installed::WorthQueryBoundDomainOperation,
    runtime::WorthQueryWorkspace,
};

fn bypass<D: 'static, O, F: 'static>(
    bound: WorthQueryBoundDomainOperation<D, O, F, ObservationLaneWitness>,
    workspace: &mut WorthQueryWorkspace,
) where
    O: WorthQueryExecutableDomainOperation<D, F, Execution = WorthQueryDirectOperation>,
{
    let _ = bound.execute(workspace);
}

fn main() {}
