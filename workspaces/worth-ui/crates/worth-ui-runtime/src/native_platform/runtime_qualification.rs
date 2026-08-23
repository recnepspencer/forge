#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativeRuntimeDerivedStateLossClass {
    MountedLayouts,
    RasterCache,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiNativeRuntimeQualificationPlan {
    completed_presentation_ordinal: u64,
    class: UiNativeRuntimeDerivedStateLossClass,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativeRuntimeQualificationPlanDenial {
    ZeroPresentationOrdinal,
}

impl UiNativeRuntimeQualificationPlan {
    pub fn derived_state_loss_after_completed_presentation(
        completed_presentation_ordinal: u64,
        class: UiNativeRuntimeDerivedStateLossClass,
    ) -> Result<Self, UiNativeRuntimeQualificationPlanDenial> {
        if completed_presentation_ordinal == 0 {
            return Err(UiNativeRuntimeQualificationPlanDenial::ZeroPresentationOrdinal);
        }
        Ok(Self {
            completed_presentation_ordinal,
            class,
        })
    }

    pub(crate) const fn completed_presentation_ordinal(self) -> u64 {
        self.completed_presentation_ordinal
    }

    pub(crate) const fn class(self) -> UiNativeRuntimeDerivedStateLossClass {
        self.class
    }
}
