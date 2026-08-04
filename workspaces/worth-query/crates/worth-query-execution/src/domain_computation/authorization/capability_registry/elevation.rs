use worth_query_declaration::facade::application_capability::ApplicationCapabilityValidityTimeline;
use worth_relational::facade::identity::KindId;

pub(in crate::domain_computation::authorization) struct WorthQueryCapabilityElevationBindings {
    pub(in crate::domain_computation::authorization) elevation_kind: KindId,
    pub(in crate::domain_computation::authorization) active_path_index: usize,
    pub(in crate::domain_computation::authorization) expired_path_index: usize,
    pub(in crate::domain_computation::authorization) self_approval_path_index: usize,
    pub(in crate::domain_computation::authorization) temporal:
        WorthQueryCapabilityElevationTemporalBindings,
    pub(in crate::domain_computation::authorization) approver_conflict_requirements:
        Vec<Vec<usize>>,
}

pub(in crate::domain_computation::authorization) struct WorthQueryCapabilityElevationTemporalBindings
{
    pub(in crate::domain_computation::authorization) timeline:
        ApplicationCapabilityValidityTimeline,
    pub(in crate::domain_computation::authorization) not_before_path_index: usize,
    pub(in crate::domain_computation::authorization) not_after_path_index: usize,
    pub(in crate::domain_computation::authorization) not_before:
        worth_foundational::facade::AspectFieldLocator,
    pub(in crate::domain_computation::authorization) not_after:
        worth_foundational::facade::AspectFieldLocator,
}

impl WorthQueryCapabilityElevationBindings {
    pub(in crate::domain_computation::authorization) fn new(
        elevation_kind: KindId,
        active_path_index: usize,
        expired_path_index: usize,
        self_approval_path_index: usize,
        temporal: WorthQueryCapabilityElevationTemporalBindings,
        approver_conflict_requirements: Vec<Vec<usize>>,
    ) -> Self {
        Self {
            elevation_kind,
            active_path_index,
            expired_path_index,
            self_approval_path_index,
            temporal,
            approver_conflict_requirements,
        }
    }
}

impl WorthQueryCapabilityElevationTemporalBindings {
    pub(in crate::domain_computation::authorization) fn new(
        timeline: ApplicationCapabilityValidityTimeline,
        not_before_path_index: usize,
        not_after_path_index: usize,
        not_before: worth_foundational::facade::AspectFieldLocator,
        not_after: worth_foundational::facade::AspectFieldLocator,
    ) -> Self {
        Self {
            timeline,
            not_before_path_index,
            not_after_path_index,
            not_before,
            not_after,
        }
    }
}
