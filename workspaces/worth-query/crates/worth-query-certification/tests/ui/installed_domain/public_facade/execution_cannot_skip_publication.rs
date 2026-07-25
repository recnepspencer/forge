use worth_query::facade::{
    foundation::BasisOperationLane, installed::operation::WorthQueryExecutedDomainOperation,
};

fn skip<D, O, F, L: BasisOperationLane, Output>(
    executed: WorthQueryExecutedDomainOperation<D, O, F, L, Output>,
) {
    let _ = executed.settle();
}

fn main() {}
