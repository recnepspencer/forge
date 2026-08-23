#[derive(Clone)]
pub(crate) struct UiMountedVisualRegionBasis {
    paint: crate::runtime::persistent_index::UiPersistentOrdMap<
        worth_ui_host_contract::UiMountedInstanceIdentity,
        worth_ui_host_contract::UiMountedFilledRectMechanic,
    >,
    hit_test: crate::runtime::persistent_index::UiPersistentOrdMap<
        worth_ui_host_contract::UiMountedInstanceIdentity,
        worth_ui_host_contract::UiMountedHitTestMechanic,
    >,
    binding: Option<worth_ui_host_contract::UiSurfaceBindingGeneration>,
    #[cfg(test)]
    materialized: Option<UiMaterializedVisualRegionBasis>,
}

#[cfg(test)]
#[derive(Clone)]
struct UiMaterializedVisualRegionBasis {
    paint: std::sync::Arc<[worth_ui_host_contract::UiMountedFilledRectMechanic]>,
    hit_test: std::sync::Arc<[worth_ui_host_contract::UiMountedHitTestMechanic]>,
}

impl UiMountedVisualRegionBasis {
    pub(in crate::mounting) fn from_persistent(
        paint: crate::runtime::persistent_index::UiPersistentOrdMap<
            worth_ui_host_contract::UiMountedInstanceIdentity,
            worth_ui_host_contract::UiMountedFilledRectMechanic,
        >,
        hit_test: crate::runtime::persistent_index::UiPersistentOrdMap<
            worth_ui_host_contract::UiMountedInstanceIdentity,
            worth_ui_host_contract::UiMountedHitTestMechanic,
        >,
    ) -> Self {
        Self {
            paint,
            hit_test,
            binding: None,
            #[cfg(test)]
            materialized: None,
        }
    }

    #[cfg(test)]
    pub(crate) fn new(
        paint: Box<[worth_ui_host_contract::UiMountedFilledRectMechanic]>,
        hit_test: Box<[worth_ui_host_contract::UiMountedHitTestMechanic]>,
    ) -> Self {
        Self {
            paint: Default::default(),
            hit_test: Default::default(),
            binding: None,
            materialized: Some(UiMaterializedVisualRegionBasis {
                paint: paint.into(),
                hit_test: hit_test.into(),
            }),
        }
    }

    pub(crate) fn for_binding(
        &self,
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    ) -> Self {
        Self {
            paint: self.paint.clone(),
            hit_test: self.hit_test.clone(),
            binding: Some(binding),
            #[cfg(test)]
            materialized: self.materialized.clone(),
        }
    }

    pub(crate) fn hit_test(&self) -> Box<[worth_ui_host_contract::UiMountedHitTestMechanic]> {
        #[cfg(test)]
        if let Some(materialized) = &self.materialized {
            return materialized.hit_test.iter().copied().collect();
        }
        self.hit_test
            .iter()
            .map(|(_, row)| *row)
            .filter(|row| self.binding.is_none_or(|binding| row.binding() == binding))
            .collect()
    }

    pub(crate) fn paint(&self) -> Box<[worth_ui_host_contract::UiMountedFilledRectMechanic]> {
        #[cfg(test)]
        if let Some(materialized) = &self.materialized {
            return materialized.paint.iter().copied().collect();
        }
        self.paint
            .iter()
            .map(|(_, row)| *row)
            .filter(|row| self.binding.is_none_or(|binding| row.binding() == binding))
            .collect()
    }

    pub(crate) fn retained_structural_bytes(&self) -> Option<usize> {
        #[cfg(test)]
        if let Some(materialized) = &self.materialized {
            return materialized
                .paint
                .len()
                .checked_mul(std::mem::size_of::<
                    worth_ui_host_contract::UiMountedFilledRectMechanic,
                >())?
                .checked_add(
                    materialized
                        .hit_test
                        .len()
                        .checked_mul(std::mem::size_of::<
                            worth_ui_host_contract::UiMountedHitTestMechanic,
                        >())?,
                );
        }
        self.paint
            .retained_structural_bytes()?
            .checked_add(self.hit_test.retained_structural_bytes()?)
    }
}
