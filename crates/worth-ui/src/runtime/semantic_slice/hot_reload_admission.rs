use super::{
    WorthUiSemanticSliceDescriptor, WorthUiSemanticSliceId, WorthUiSemanticSliceInventory,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiHotReloadableSemanticSlice {
    descriptor: &'static WorthUiSemanticSliceDescriptor,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAdmittedHotReloadableSemanticSliceSet {
    slices: Vec<WorthUiHotReloadableSemanticSlice>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiSemanticHotReloadAdmissionDenial {
    UnknownSlice(WorthUiSemanticSliceId),
    CompileRequiredPlatformMeaning(WorthUiSemanticSliceId),
}

impl WorthUiAdmittedHotReloadableSemanticSliceSet {
    pub fn admit(
        inventory: &WorthUiSemanticSliceInventory,
        slice_ids: impl IntoIterator<Item = WorthUiSemanticSliceId>,
    ) -> Result<Self, WorthUiSemanticHotReloadAdmissionDenial> {
        let mut slices = Vec::new();
        for slice_id in slice_ids {
            let descriptor = inventory.slice(slice_id).ok_or(
                WorthUiSemanticHotReloadAdmissionDenial::UnknownSlice(slice_id),
            )?;
            if descriptor.meaning() == crate::runtime::WorthUiSemanticMeaningClass::PlatformMeaning
            {
                return Err(
                    WorthUiSemanticHotReloadAdmissionDenial::CompileRequiredPlatformMeaning(
                        slice_id,
                    ),
                );
            }
            slices.push(WorthUiHotReloadableSemanticSlice { descriptor });
        }
        slices.sort_by_key(|slice| slice.descriptor.id());
        slices.dedup_by_key(|slice| slice.descriptor.id());
        Ok(Self { slices })
    }

    pub fn slices(&self) -> &[WorthUiHotReloadableSemanticSlice] {
        &self.slices
    }
}

impl WorthUiHotReloadableSemanticSlice {
    pub fn descriptor(&self) -> &'static WorthUiSemanticSliceDescriptor {
        self.descriptor
    }
}
