use super::*;

impl ProductUnpublishedRecoveryCatalog {
    pub(crate) fn new(
        owner: RuntimeWorldOwnerIdentity,
        maximum_slots: RuntimeWorldBudgetLimit,
    ) -> Self {
        // R1b remains a shared serial-owner correction: this frozen
        // constructor cannot receive RuntimeWorldBudgets' metadata limit.
        // The lane still accounts every installed record with its real charge.
        Self {
            state: Arc::new(Mutex::new(RecoveryCatalogState {
                owner,
                maximum_slots: maximum_slots.get(),
                maximum_metadata_bytes: usize::MAX,
                reserved_slots: 0,
                abandoned_slots: 0,
                active: BTreeMap::new(),
                reserved_metadata_bytes: 0,
                updating_slots: 0,
                updating_identities: BTreeSet::new(),
                metadata_bytes: 0,
                records: BTreeMap::new(),
            })),
        }
    }

    pub(crate) fn new_with_metadata(
        owner: RuntimeWorldOwnerIdentity,
        maximum_slots: RuntimeWorldBudgetLimit,
        maximum_metadata_bytes: RuntimeWorldBudgetLimit,
    ) -> Self {
        Self {
            state: Arc::new(Mutex::new(RecoveryCatalogState {
                owner,
                maximum_slots: maximum_slots.get(),
                maximum_metadata_bytes: maximum_metadata_bytes.get(),
                reserved_slots: 0,
                abandoned_slots: 0,
                active: BTreeMap::new(),
                reserved_metadata_bytes: 0,
                updating_slots: 0,
                updating_identities: BTreeSet::new(),
                metadata_bytes: 0,
                records: BTreeMap::new(),
            })),
        }
    }
}
