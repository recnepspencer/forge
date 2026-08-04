#[derive(Debug, PartialEq, Eq)]
pub(super) struct BTreeDurableObservationSource {
    execution: worth_store_layout_indexes::StableBTreeLookupExecution,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct LayoutDurableObservationSource {
    btree: BTreeDurableObservationSource,
}

impl BTreeDurableObservationSource {
    pub(super) fn from_found_execution(
        execution: worth_store_layout_indexes::StableBTreeLookupExecution,
    ) -> Option<Self> {
        if matches!(
            execution.view(),
            worth_store_layout_indexes::BTreeLookupExecutionView::Found(_)
        ) {
            Some(Self { execution })
        } else {
            None
        }
    }

    const fn root(&self) -> worth_store_physical_isolation::CurrentPhysicalRoot {
        self.execution.stable_read().read_plan_release().root()
    }

    fn selected_reference(&self) -> worth_store_physical_format::PhysicalReference {
        match self.execution.view() {
            worth_store_layout_indexes::BTreeLookupExecutionView::Found(found) => {
                found.selected_reference()
            }
            _ => unreachable!("source construction requires a found owner execution"),
        }
    }
}

pub(super) fn observe_durable_artifacts(
    btree: BTreeDurableObservationSource,
) -> LayoutDurableObservationSource {
    LayoutDurableObservationSource { btree }
}

impl LayoutDurableObservationSource {
    pub(crate) const fn btree_root(&self) -> worth_store_physical_isolation::CurrentPhysicalRoot {
        self.btree.root()
    }

    pub(crate) fn btree_selected_reference(
        &self,
    ) -> worth_store_physical_format::PhysicalReference {
        self.btree.selected_reference()
    }
}
