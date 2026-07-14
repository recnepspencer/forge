#[derive(Debug, PartialEq, Eq)]
pub(super) struct BTreeDurableObservationSource {
    execution: forge_store_layout_indexes::StableBTreeLookupExecution,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct LayoutDurableObservationSource {
    btree: BTreeDurableObservationSource,
    lsm_value: forge_store_wal::BlobWalRecordIdentity,
    lsm_generation: forge_store_wal::BlobWalRecordIdentity,
    lsm_tombstone: forge_store_wal::BlobWalRecordIdentity,
    lsm_output: forge_store_wal::BlobWalRecordIdentity,
    lsm_activation: forge_store_wal::CheckpointDurablePublicationScope,
    physical_old_root: forge_store_physical_isolation::CurrentPhysicalRoot,
    physical_new_root: forge_store_physical_isolation::CurrentPhysicalRoot,
}

impl BTreeDurableObservationSource {
    pub(super) fn from_found_execution(
        execution: forge_store_layout_indexes::StableBTreeLookupExecution,
    ) -> Option<Self> {
        if matches!(
            execution.view(),
            forge_store_layout_indexes::BTreeLookupExecutionView::Found(_)
        ) {
            Some(Self { execution })
        } else {
            None
        }
    }

    const fn root(&self) -> forge_store_physical_isolation::CurrentPhysicalRoot {
        self.execution.stable_read().read_plan_release().root()
    }

    fn selected_reference(&self) -> forge_store_physical_format::PhysicalReference {
        match self.execution.view() {
            forge_store_layout_indexes::BTreeLookupExecutionView::Found(found) => {
                found.selected_reference()
            }
            _ => unreachable!("source construction requires a found owner execution"),
        }
    }
}

pub(super) fn observe_durable_artifacts(
    btree: BTreeDurableObservationSource,
) -> LayoutDurableObservationSource {
    let published = forge_store_test_support::execute_baseline_lsm_persisted_fixture();
    let replacement = published.membership_replacement();
    let physical = published.physical_compaction().publication();
    LayoutDurableObservationSource {
        btree,
        lsm_value: published.value_record().identity(),
        lsm_generation: published.generation_record().identity(),
        lsm_tombstone: published.tombstone_record().identity(),
        lsm_output: replacement.output(),
        lsm_activation: replacement.activation_scope().clone(),
        physical_old_root: physical.old_root(),
        physical_new_root: physical.new_root(),
    }
}

impl LayoutDurableObservationSource {
    pub(crate) const fn btree_root(&self) -> forge_store_physical_isolation::CurrentPhysicalRoot {
        self.btree.root()
    }

    pub(crate) fn btree_selected_reference(
        &self,
    ) -> forge_store_physical_format::PhysicalReference {
        self.btree.selected_reference()
    }

    pub(crate) const fn lsm_value(&self) -> forge_store_wal::BlobWalRecordIdentity {
        self.lsm_value
    }

    pub(crate) const fn lsm_generation(&self) -> forge_store_wal::BlobWalRecordIdentity {
        self.lsm_generation
    }

    pub(crate) const fn lsm_tombstone(&self) -> forge_store_wal::BlobWalRecordIdentity {
        self.lsm_tombstone
    }

    pub(crate) const fn lsm_output(&self) -> forge_store_wal::BlobWalRecordIdentity {
        self.lsm_output
    }

    pub(crate) const fn lsm_activation(
        &self,
    ) -> &forge_store_wal::CheckpointDurablePublicationScope {
        &self.lsm_activation
    }

    pub(crate) const fn physical_old_root(
        &self,
    ) -> forge_store_physical_isolation::CurrentPhysicalRoot {
        self.physical_old_root
    }

    pub(crate) const fn physical_new_root(
        &self,
    ) -> forge_store_physical_isolation::CurrentPhysicalRoot {
        self.physical_new_root
    }
}
