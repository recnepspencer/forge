use schema::facade::{
    admit_query_mutation_batch, query_mutation_support_contract, QueryMutationAdmission,
    QueryMutationAdmissionBlocker, QueryMutationSupportContract,
};

fn main() {
    let _ = (
        admit_query_mutation_batch,
        query_mutation_support_contract,
        None::<QueryMutationAdmission>,
        None::<QueryMutationAdmissionBlocker>,
        None::<QueryMutationSupportContract>,
    );
}
