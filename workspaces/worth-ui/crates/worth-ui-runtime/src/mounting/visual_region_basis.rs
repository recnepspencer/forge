#[derive(Clone)]
pub(crate) struct UiMountedVisualRegionBasis {
    paint: Box<[worth_ui_host_contract::UiMountedFilledRectMechanic]>,
    hit_test: Box<[worth_ui_host_contract::UiMountedHitTestMechanic]>,
}

impl UiMountedVisualRegionBasis {
    pub(crate) fn new(
        paint: Box<[worth_ui_host_contract::UiMountedFilledRectMechanic]>,
        hit_test: Box<[worth_ui_host_contract::UiMountedHitTestMechanic]>,
    ) -> Self {
        Self { paint, hit_test }
    }

    pub(crate) fn for_binding(
        &self,
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    ) -> Self {
        Self {
            paint: self
                .paint
                .iter()
                .copied()
                .filter(|row| row.binding() == binding)
                .collect(),
            hit_test: self
                .hit_test
                .iter()
                .copied()
                .filter(|row| row.binding() == binding)
                .collect(),
        }
    }

    pub(crate) fn paint(&self) -> &[worth_ui_host_contract::UiMountedFilledRectMechanic] {
        &self.paint
    }

    pub(crate) fn hit_test(&self) -> &[worth_ui_host_contract::UiMountedHitTestMechanic] {
        &self.hit_test
    }

    pub(crate) fn retained_structural_bytes(&self) -> Option<usize> {
        self.paint
            .len()
            .checked_mul(std::mem::size_of::<
                worth_ui_host_contract::UiMountedFilledRectMechanic,
            >())?
            .checked_add(self.hit_test.len().checked_mul(std::mem::size_of::<
                worth_ui_host_contract::UiMountedHitTestMechanic,
            >())?)
    }
}
