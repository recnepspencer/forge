use forge_query::facade::{
    MaterializedProjectionContract, ProjectionContractSourcePosture,
    ProjectionContractSupportPosture, ProjectionSourceFamily,
};

fn main() {
    let contract: MaterializedProjectionContract = unsafe { std::mem::zeroed() };
    let _ = contract.extract();
    let _ = (
        ProjectionSourceFamily::QueryWriteReceipt,
        ProjectionContractSourcePosture::QueryOwnedReceiptSource,
        ProjectionContractSupportPosture::Admitted,
    );
}
