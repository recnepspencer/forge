use super::{WorthUiSemanticMeaningClass, WorthUiSemanticSliceId, WorthUiSemanticSliceInventory};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiSemanticCompileBoundary {
    _private: (),
}

impl WorthUiSemanticCompileBoundary {
    pub fn current() -> Self {
        Self { _private: () }
    }

    pub fn is_hot_reloadable_product_slice(&self, id: WorthUiSemanticSliceId) -> bool {
        WorthUiSemanticSliceInventory::current()
            .slice(id)
            .is_some_and(|descriptor| {
                descriptor.meaning() == WorthUiSemanticMeaningClass::ProductMeaning
            })
    }

    pub fn is_compile_required_platform_slice(&self, id: WorthUiSemanticSliceId) -> bool {
        WorthUiSemanticSliceInventory::current()
            .slice(id)
            .is_some_and(|descriptor| {
                descriptor.meaning() == WorthUiSemanticMeaningClass::PlatformMeaning
            })
    }
}
