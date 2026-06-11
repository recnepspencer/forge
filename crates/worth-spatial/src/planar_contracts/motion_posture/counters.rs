#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarMotionPostureCounters {
    motion_step_rows_inspected: usize,
    rotation_rows_inspected: usize,
    cancellation_rows_inspected: usize,
    signal_compatibility_rows_inspected: usize,
    rejected_coordinate_only_rows: usize,
    rejected_orientation_flip_rows: usize,
}

impl PlanarMotionPostureCounters {
    pub(crate) const fn certified(
        motion_step_rows_inspected: usize,
        rotation_rows_inspected: usize,
        cancellation_rows_inspected: usize,
        signal_compatibility_rows_inspected: usize,
    ) -> Self {
        Self {
            motion_step_rows_inspected,
            rotation_rows_inspected,
            cancellation_rows_inspected,
            signal_compatibility_rows_inspected,
            rejected_coordinate_only_rows: 0,
            rejected_orientation_flip_rows: 0,
        }
    }

    pub(crate) const fn rejected_coordinate_only() -> Self {
        Self {
            motion_step_rows_inspected: 0,
            rotation_rows_inspected: 0,
            cancellation_rows_inspected: 0,
            signal_compatibility_rows_inspected: 0,
            rejected_coordinate_only_rows: 1,
            rejected_orientation_flip_rows: 0,
        }
    }

    pub(crate) const fn rejected_orientation_flip() -> Self {
        Self {
            motion_step_rows_inspected: 0,
            rotation_rows_inspected: 0,
            cancellation_rows_inspected: 0,
            signal_compatibility_rows_inspected: 0,
            rejected_coordinate_only_rows: 0,
            rejected_orientation_flip_rows: 1,
        }
    }

    pub fn motion_step_rows_inspected(self) -> usize {
        self.motion_step_rows_inspected
    }

    pub fn rotation_rows_inspected(self) -> usize {
        self.rotation_rows_inspected
    }

    pub fn cancellation_rows_inspected(self) -> usize {
        self.cancellation_rows_inspected
    }

    pub fn signal_compatibility_rows_inspected(self) -> usize {
        self.signal_compatibility_rows_inspected
    }

    pub fn rejected_coordinate_only_rows(self) -> usize {
        self.rejected_coordinate_only_rows
    }

    pub fn rejected_orientation_flip_rows(self) -> usize {
        self.rejected_orientation_flip_rows
    }
}
