use worth_store_authority::StoreCurrentAuthorityWitness;

pub(crate) fn publication_plan(
    authority: &StoreCurrentAuthorityWitness,
    generation: u64,
) -> worth_store_physical_isolation::CopyOnWritePublicationPlan {
    let inputs = publication_inputs(authority, generation);
    worth_store_test_support::harness::physical_isolation::publication::admitted_copy_on_write_plan(
        &inputs,
    )
}

pub(crate) fn rollback_publication_plan(
    authority: &StoreCurrentAuthorityWitness,
    prior: &worth_store_physical_isolation::PhysicalPublicationReceipt,
    generation: u64,
) -> worth_store_physical_isolation::CopyOnWritePublicationPlan {
    let store = worth_store_physical_format::PhysicalStoreIdentity::from_aspect_identity(
        authority.identity().clone(),
    );
    let rollback = worth_store_test_support::harness::physical_isolation::publication::successor_publication_inputs_for_store(
        prior,
        &store,
        generation,
    );
    worth_store_test_support::harness::physical_isolation::publication::admitted_copy_on_write_plan(
        &rollback,
    )
}

pub(super) fn current_publication_source(
    authority: &StoreCurrentAuthorityWitness,
) -> worth_store_physical_isolation::PublicationRootCandidate {
    publication_inputs(authority, 1_900).old_candidate
}

pub(super) fn publication_inputs(
    authority: &StoreCurrentAuthorityWitness,
    generation: u64,
) -> worth_store_test_support::harness::physical_isolation::publication::PublicationInputs {
    let store = worth_store_physical_format::PhysicalStoreIdentity::from_aspect_identity(
        authority.identity().clone(),
    );
    worth_store_test_support::harness::physical_isolation::publication::publication_inputs_for_store(
        &store, generation,
    )
}
