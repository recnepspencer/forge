use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LayoutOwnerTopology {
    families: BTreeMap<&'static str, BTreeSet<&'static str>>,
}

impl LayoutOwnerTopology {
    pub(crate) fn observe() -> Self {
        let mut topology = Self {
            families: BTreeMap::new(),
        };

        topology.extend_family(
            "btree_lookup_readiness",
            forge_store_layout_indexes::btree_lookup_readiness_cases().map(|case| case.name()),
        );
        topology.extend_family(
            "btree_lookup_execution",
            forge_store_layout_indexes::btree_lookup_execution_cases().map(|case| case.name()),
        );
        topology.extend_family(
            "degraded_scan_readiness",
            forge_store_layout_indexes::degraded_scan_readiness_cases().map(|case| case.name()),
        );
        topology.extend_family(
            "lsm_lookup",
            forge_store_layout_indexes::baseline_lsm_lookup_cases().map(|case| case.name()),
        );
        topology.extend_family(
            "lsm_lookup_readiness",
            forge_store_layout_indexes::baseline_lsm_lookup_admission_cases()
                .map(|case| case.name()),
        );
        topology.extend_family(
            "imported_blob_read_admission",
            forge_store_layout_indexes::imported_blob_read_admission_cases()
                .map(|case| case.as_str()),
        );
        topology.extend_family(
            "maintenance_admission",
            forge_store_layout_indexes::maintenance::maintenance_admission_cases()
                .map(|case| case.as_str()),
        );
        topology.extend_family(
            "migration_planning",
            forge_store_layout_indexes::evolution::migration::migration_planning_cases()
                .map(|case| case.as_str()),
        );
        topology.extend_family(
            "rollback_planning",
            forge_store_layout_indexes::evolution::migration::rollback_planning_cases()
                .map(|case| case.as_str()),
        );
        topology.extend_family(
            "corruption_classification",
            forge_store_layout_indexes::integrity::corruption_classification_cases()
                .map(|case| case.as_str()),
        );
        topology.extend_family(
            "quarantine_readmission",
            forge_store_layout_indexes::integrity::quarantine_readmission_cases()
                .map(|case| case.as_str()),
        );
        topology.extend_family(
            "offline_readmission",
            forge_store_layout_indexes::integrity::offline_readmission_cases()
                .map(|case| case.as_str()),
        );
        topology.extend_family(
            "import_readmission",
            forge_store_layout_indexes::integrity::import_readmission_cases()
                .map(|case| case.as_str()),
        );
        topology.extend_family(
            "restored_layout_materialization",
            forge_store_operations::restored_layout_materialization_cases()
                .map(|case| case.as_str()),
        );
        topology.extend_family(
            "physical_compaction",
            forge_store_physical_isolation::compaction_owner_case_inventory()
                .map(|case| case.id().name()),
        );

        topology
    }

    fn extend_family(
        &mut self,
        family: &'static str,
        cases: impl IntoIterator<Item = &'static str>,
    ) {
        let prior = self.families.insert(family, cases.into_iter().collect());
        assert!(
            prior.is_none(),
            "owner family must be aggregated exactly once"
        );
    }

    pub(crate) fn families(&self) -> &BTreeMap<&'static str, BTreeSet<&'static str>> {
        &self.families
    }
}

#[cfg(test)]
mod tests {
    use super::LayoutOwnerTopology;

    #[test]
    fn aggregation_is_a_non_authoring_union_of_owner_case_inventories() {
        let topology = LayoutOwnerTopology::observe();

        assert_eq!(topology.families().len(), 15);
        assert!(topology.families().values().all(|cases| !cases.is_empty()));

        let total_cases: usize = topology.families().values().map(|cases| cases.len()).sum();
        let unique_cases = topology
            .families()
            .values()
            .flatten()
            .copied()
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(unique_cases.len(), total_cases);
    }
}
