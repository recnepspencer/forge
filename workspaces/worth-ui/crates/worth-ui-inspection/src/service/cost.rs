#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiRuntimeServiceInspectionCost {
    owner_index_lookups: u16,
    retained_records_examined: u16,
    projected_items: u16,
    world_records_scanned: u32,
}

impl UiRuntimeServiceInspectionCost {
    pub const fn latest_record(owner_index_lookups: u16, retained_records_examined: u16) -> Self {
        Self {
            owner_index_lookups,
            retained_records_examined,
            projected_items: retained_records_examined,
            world_records_scanned: 0,
        }
    }

    pub const fn latest_record_with_projection(
        owner_index_lookups: u16,
        retained_records_examined: u16,
        projected_items: u16,
    ) -> Self {
        Self {
            owner_index_lookups,
            retained_records_examined,
            projected_items,
            world_records_scanned: 0,
        }
    }

    pub const fn owner_index_lookups(self) -> u16 {
        self.owner_index_lookups
    }

    pub const fn retained_records_examined(self) -> u16 {
        self.retained_records_examined
    }

    pub const fn projected_items(self) -> u16 {
        self.projected_items
    }

    pub const fn world_records_scanned(self) -> u32 {
        self.world_records_scanned
    }

    pub const fn is_bounded_latest_record_lookup(self) -> bool {
        self.world_records_scanned == 0 && self.retained_records_examined <= 1
    }
}

#[cfg(test)]
mod tests {
    use super::UiRuntimeServiceInspectionCost;

    #[test]
    fn latest_record_cost_names_lookup_scan_and_projection_work() {
        let cost = UiRuntimeServiceInspectionCost::latest_record_with_projection(1, 1, 17);

        assert_eq!(cost.owner_index_lookups(), 1);
        assert_eq!(cost.retained_records_examined(), 1);
        assert_eq!(cost.projected_items(), 17);
        assert_eq!(cost.world_records_scanned(), 0);
        assert!(cost.is_bounded_latest_record_lookup());
    }
}
