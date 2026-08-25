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
    semantic_text: crate::mounting::projection::UiMountedSemanticMechanicSource,
    binding: Option<worth_ui_host_contract::UiSurfaceBindingGeneration>,
    receipts: Option<super::UiMountedNodeReceiptBasis>,
    #[cfg(test)]
    materialized: Option<UiMaterializedVisualRegionBasis>,
}

#[cfg(test)]
#[derive(Clone)]
struct UiMaterializedVisualRegionBasis {
    paint: std::sync::Arc<[worth_ui_host_contract::UiMountedFilledRectMechanic]>,
    hit_test: std::sync::Arc<[worth_ui_host_contract::UiMountedHitTestMechanic]>,
    unsupported_paint: std::sync::Arc<[UiMountedUnsupportedPaintBasis]>,
}

#[derive(Clone, Copy)]
pub(crate) struct UiMountedUnsupportedPaintBasis {
    node_receipt: worth_ui_host_contract::UiMountedNodeReceiptIdentity,
    bounds: worth_ui_host_contract::UiMountedCanonicalBox,
    clip: worth_ui_host_contract::UiMountedCanonicalBox,
    semantic_order: u32,
    source_digest: u64,
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
        semantic_text: crate::mounting::projection::UiMountedSemanticMechanicSource,
    ) -> Self {
        Self {
            paint,
            hit_test,
            semantic_text,
            binding: None,
            receipts: None,
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
            semantic_text: Default::default(),
            binding: None,
            receipts: None,
            materialized: Some(UiMaterializedVisualRegionBasis {
                paint: paint.into(),
                hit_test: hit_test.into(),
                unsupported_paint: std::sync::Arc::from([]),
            }),
        }
    }

    pub(crate) fn for_binding(
        &self,
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
        receipts: super::UiMountedNodeReceiptBasis,
    ) -> Self {
        Self {
            paint: self.paint.clone(),
            hit_test: self.hit_test.clone(),
            semantic_text: self.semantic_text.clone(),
            binding: Some(binding),
            receipts: Some(receipts),
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
            .map(|row| {
                let Some(receipts) = self.receipts.as_ref() else {
                    return row;
                };
                crate::mounting::projection::reattribute_hit_test(row, receipts.frame(), receipts)
                    .expect("retained hit rows belong to the presented receipt basis")
            })
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
            .map(|row| {
                let Some(receipts) = self.receipts.as_ref() else {
                    return row;
                };
                crate::mounting::projection::reattribute_filled_rect(
                    row,
                    receipts.frame(),
                    receipts,
                )
                .expect("retained paint rows belong to the presented receipt basis")
            })
            .collect()
    }

    pub(crate) fn unsupported_paint(&self) -> Box<[UiMountedUnsupportedPaintBasis]> {
        #[cfg(test)]
        if let Some(materialized) = &self.materialized {
            return materialized.unsupported_paint.iter().copied().collect();
        }
        self.semantic_text
            .visual_mechanics()
            .filter(|row| self.binding.is_none_or(|binding| row.binding() == binding))
            .map(|row| UiMountedUnsupportedPaintBasis {
                node_receipt: self
                    .receipts
                    .as_ref()
                    .and_then(|receipts| receipts.receipt_for(row.mounted_instance()))
                    .unwrap_or_else(|| row.node_receipt()),
                bounds: row.bounds(),
                clip: row.clip_bounds(),
                semantic_order: row.layer_semantic_order(),
                source_digest: row.semantic_digest(),
            })
            .collect()
    }

    #[cfg(test)]
    pub(crate) fn with_unsupported_paint(
        mut self,
        unsupported_paint: Box<[UiMountedUnsupportedPaintBasis]>,
    ) -> Self {
        self.materialized
            .as_mut()
            .expect("test visual region basis is materialized")
            .unsupported_paint = unsupported_paint.into();
        self
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
            .checked_add(self.hit_test.retained_structural_bytes()?)?
            .checked_add(self.semantic_text.retained_structural_bytes()?)
    }
}

impl UiMountedUnsupportedPaintBasis {
    #[cfg(test)]
    pub(crate) const fn new(
        node_receipt: worth_ui_host_contract::UiMountedNodeReceiptIdentity,
        bounds: worth_ui_host_contract::UiMountedCanonicalBox,
        clip: worth_ui_host_contract::UiMountedCanonicalBox,
        semantic_order: u32,
        source_digest: u64,
    ) -> Self {
        Self {
            node_receipt,
            bounds,
            clip,
            semantic_order,
            source_digest,
        }
    }

    pub(crate) const fn node_receipt(self) -> worth_ui_host_contract::UiMountedNodeReceiptIdentity {
        self.node_receipt
    }

    pub(crate) const fn bounds(self) -> worth_ui_host_contract::UiMountedCanonicalBox {
        self.bounds
    }

    pub(crate) const fn clip(self) -> worth_ui_host_contract::UiMountedCanonicalBox {
        self.clip
    }

    pub(crate) const fn semantic_order(self) -> u32 {
        self.semantic_order
    }

    pub(crate) const fn source_digest(self) -> u64 {
        self.source_digest
    }
}
