#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedFrameDelta {
    work_class: super::UiMountWorkClass,
    initial_mounted_instances: u64,
    changed_mounted_instances: u64,
    index_entries_touched: u64,
    replaced_batch_rows: u64,
    replaced_batch_bytes: u64,
    surface_instance_pairs: u64,
    changed_binding_generations: u64,
}

impl UiMountedFrameDelta {
    pub(super) fn from_cost(report: super::UiMountCostReport) -> Self {
        Self {
            work_class: report.work_class(),
            initial_mounted_instances: report.initial_mounted_instances(),
            changed_mounted_instances: report.changed_mounted_instances(),
            index_entries_touched: report.index_entries_touched(),
            replaced_batch_rows: report.replaced_batch_rows(),
            replaced_batch_bytes: report.replaced_batch_bytes(),
            surface_instance_pairs: report.surface_instance_pairs(),
            changed_binding_generations: report.changed_binding_generations(),
        }
    }

    pub const fn work_class(self) -> super::UiMountWorkClass {
        self.work_class
    }

    pub const fn initial_mounted_instances(self) -> u64 {
        self.initial_mounted_instances
    }

    pub const fn changed_mounted_instances(self) -> u64 {
        self.changed_mounted_instances
    }

    pub const fn index_entries_touched(self) -> u64 {
        self.index_entries_touched
    }

    pub const fn replaced_batch_rows(self) -> u64 {
        self.replaced_batch_rows
    }

    pub const fn replaced_batch_bytes(self) -> u64 {
        self.replaced_batch_bytes
    }

    pub const fn surface_instance_pairs(self) -> u64 {
        self.surface_instance_pairs
    }

    pub const fn changed_binding_generations(self) -> u64 {
        self.changed_binding_generations
    }
}
