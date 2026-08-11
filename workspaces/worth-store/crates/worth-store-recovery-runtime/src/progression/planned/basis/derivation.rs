use super::*;

mod actions;
mod closeout;
mod layout;
mod materialization;
mod pending;

pub(crate) fn derive_execution_basis(
    store: StableStoreIdentity,
    selection: &PhysicalSourceSelection,
    freshness: &StoreRecoveryBindingFreshnessSample,
    fates: &ReconciledOperationFates,
    redo: &ImmutablePhysicalRedoPlan,
    publication_source: Option<&RecoveryPublicationSourceInventory>,
    maximum_staging_bytes: u64,
    maximum_dirty_frames: u64,
) -> Result<
    (
        RecoveryStagingLayoutPlan,
        RecoveryPublicationPlan,
        RecoveryQuiescencePlan,
    ),
    ExecutionBasisDenial,
> {
    let pending = pending::admit(
        selection,
        fates,
        redo,
        maximum_staging_bytes,
        maximum_dirty_frames,
    )?;
    let materialization = materialization::collect(&pending);
    let actions = actions::derive(redo, &materialization, pending.staging_generation)?;
    let staging = layout::assemble(selection, &pending, materialization, actions)?;
    closeout::seal(
        store,
        selection,
        freshness,
        fates,
        redo,
        publication_source,
        pending,
        staging,
    )
}
