use crate::capability::QueryDenialPresentation;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthUiQueryBindingUiRequirementsDriftFamily {
    LifecycleDeclaration,
    AsyncResultPresentation,
    RecoveryPresentation,
    InspectionRelevance,
    ProjectionConsumption,
    DenialPresentation,
}

/// UI-owned requirements adjacent to an installed Query binding.
///
/// This value describes presentation and integration needs only. It carries
/// no Query support, installation, or consumer-contract authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryBindingUiRequirements {
    lifecycle: worth_ui_query_binding::WorthUiQueryViewLifecycle,
    async_result_presentation: bool,
    recovery_presentation: bool,
    inspection_relevance: bool,
    projection_consumption: bool,
    denial_presentation: QueryDenialPresentation,
}

pub(crate) struct WorthUiQueryBindingUiRequirementsInput {
    pub lifecycle: worth_ui_query_binding::WorthUiQueryViewLifecycle,
    pub async_result_presentation: bool,
    pub recovery_presentation: bool,
    pub inspection_relevance: bool,
    pub projection_consumption: bool,
    pub denial_presentation: QueryDenialPresentation,
}

impl WorthUiQueryBindingUiRequirements {
    pub(crate) fn new(input: WorthUiQueryBindingUiRequirementsInput) -> Self {
        Self {
            lifecycle: input.lifecycle,
            async_result_presentation: input.async_result_presentation,
            recovery_presentation: input.recovery_presentation,
            inspection_relevance: input.inspection_relevance,
            projection_consumption: input.projection_consumption,
            denial_presentation: input.denial_presentation,
        }
    }

    pub fn lifecycle(&self) -> worth_ui_query_binding::WorthUiQueryViewLifecycle {
        self.lifecycle
    }

    pub fn has_async_result_state(&self) -> bool {
        self.async_result_presentation
    }

    pub fn has_recovery(&self) -> bool {
        self.recovery_presentation
    }

    pub fn has_inspection(&self) -> bool {
        self.inspection_relevance
    }

    pub fn has_projection_consumption(&self) -> bool {
        self.projection_consumption
    }

    pub fn denial_presentation(&self) -> QueryDenialPresentation {
        self.denial_presentation
    }

    pub fn canonical_identity(&self) -> u64 {
        [
            lifecycle_tag(self.lifecycle),
            u64::from(self.async_result_presentation),
            u64::from(self.recovery_presentation),
            u64::from(self.inspection_relevance),
            u64::from(self.projection_consumption),
            denial_presentation_tag(self.denial_presentation),
        ]
        .into_iter()
        .fold(0x7175_6572_7975_6972_u64, |identity, value| {
            identity.rotate_left(11).wrapping_mul(0x100_0000_01b3) ^ value
        })
    }

    pub(crate) fn drift_families_against(
        &self,
        other: &Self,
    ) -> Vec<WorthUiQueryBindingUiRequirementsDriftFamily> {
        let mut families = Vec::new();
        push_if_changed(
            &mut families,
            self.lifecycle != other.lifecycle,
            WorthUiQueryBindingUiRequirementsDriftFamily::LifecycleDeclaration,
        );
        push_if_changed(
            &mut families,
            self.async_result_presentation != other.async_result_presentation,
            WorthUiQueryBindingUiRequirementsDriftFamily::AsyncResultPresentation,
        );
        push_if_changed(
            &mut families,
            self.recovery_presentation != other.recovery_presentation,
            WorthUiQueryBindingUiRequirementsDriftFamily::RecoveryPresentation,
        );
        push_if_changed(
            &mut families,
            self.inspection_relevance != other.inspection_relevance,
            WorthUiQueryBindingUiRequirementsDriftFamily::InspectionRelevance,
        );
        push_if_changed(
            &mut families,
            self.projection_consumption != other.projection_consumption,
            WorthUiQueryBindingUiRequirementsDriftFamily::ProjectionConsumption,
        );
        push_if_changed(
            &mut families,
            self.denial_presentation != other.denial_presentation,
            WorthUiQueryBindingUiRequirementsDriftFamily::DenialPresentation,
        );
        families
    }
}

fn lifecycle_tag(lifecycle: worth_ui_query_binding::WorthUiQueryViewLifecycle) -> u64 {
    match lifecycle {
        worth_ui_query_binding::WorthUiQueryViewLifecycle::Snapshot => 1,
        worth_ui_query_binding::WorthUiQueryViewLifecycle::Live => 2,
    }
}

fn denial_presentation_tag(presentation: QueryDenialPresentation) -> u64 {
    match presentation {
        QueryDenialPresentation::Hidden => 1,
        QueryDenialPresentation::AdvisoryText => 2,
        QueryDenialPresentation::StructuredStatus => 3,
    }
}

fn push_if_changed(
    families: &mut Vec<WorthUiQueryBindingUiRequirementsDriftFamily>,
    changed: bool,
    family: WorthUiQueryBindingUiRequirementsDriftFamily,
) {
    if changed {
        families.push(family);
    }
}
