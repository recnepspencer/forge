impl super::WorthUiPlanRegionStore {
    pub(crate) fn mounted_projection_rows(&self) -> Vec<(u64, u32)> {
        let mut records = Vec::with_capacity(self.region_count);
        super::identity_trie::collect_records(&self.identity_root, &mut records);
        records
            .into_iter()
            .filter_map(|record| {
                let input = record.schema.input();
                if input.owner_identity_basis().is_some() {
                    return None;
                }
                let provenance = input.mounted_projection_provenance_digest()?;
                let plan_index = u32::try_from(record.handle.stable_slot()).ok()?;
                Some((provenance, plan_index))
            })
            .collect()
    }
}
