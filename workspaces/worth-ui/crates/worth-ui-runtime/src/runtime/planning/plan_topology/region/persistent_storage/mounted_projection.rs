impl super::WorthUiPlanRegionStore {
    pub(crate) fn mounted_projection_plan_index(&self, provenance: u64) -> Result<Option<u32>, ()> {
        let Some(entries) = self.mounted_projection_index.get(&provenance) else {
            return Ok(None);
        };
        let mut entries = entries.iter().copied();
        let first = entries.next();
        if entries.next().is_some() {
            return Err(());
        }
        Ok(first)
    }

    pub(super) fn insert_mounted_projection_record(
        &mut self,
        record: &super::WorthUiPlanRegionRecord,
    ) {
        let Some((provenance, plan_index)) = mounted_projection_entry(record) else {
            return;
        };
        let mut entries = self
            .mounted_projection_index
            .get(&provenance)
            .cloned()
            .unwrap_or_default();
        entries.insert(plan_index);
        self.mounted_projection_index.insert(provenance, entries);
    }

    pub(super) fn remove_mounted_projection_record(
        &mut self,
        record: &super::WorthUiPlanRegionRecord,
    ) {
        let Some((provenance, plan_index)) = mounted_projection_entry(record) else {
            return;
        };
        let Some(mut entries) = self.mounted_projection_index.get(&provenance).cloned() else {
            return;
        };
        entries.remove_with_work(&plan_index);
        if entries.is_empty() {
            self.mounted_projection_index.remove(&provenance);
        } else {
            self.mounted_projection_index.insert(provenance, entries);
        }
    }
}

fn mounted_projection_entry(record: &super::WorthUiPlanRegionRecord) -> Option<(u64, u32)> {
    let input = record.schema.input();
    if input.owner_identity_basis().is_some() {
        return None;
    }
    Some((
        input.mounted_projection_provenance_digest()?,
        u32::try_from(record.handle.stable_slot()).ok()?,
    ))
}
