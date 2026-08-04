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
    pub(in crate::domain_computation::authorization) lifecycle:
        WorthQueryCapabilityElevationLifecycleBindings,
}

pub(in crate::domain_computation::authorization) struct WorthQueryCapabilityElevationLifecycleBindings
{
    pub(in crate::domain_computation::authorization) review_kind: KindId,
    pub(in crate::domain_computation::authorization) identity:
        worth_foundational::facade::AspectFieldLocator,
    pub(in crate::domain_computation::authorization) reason:
        worth_foundational::facade::AspectFieldLocator,
    pub(in crate::domain_computation::authorization) status:
        worth_foundational::facade::AspectFieldLocator,
    pub(in crate::domain_computation::authorization) review_identity:
        worth_foundational::facade::AspectFieldLocator,
    pub(in crate::domain_computation::authorization) review_status:
        worth_foundational::facade::AspectFieldLocator,
    pub(in crate::domain_computation::authorization) requester_relation: KindId,
    pub(in crate::domain_computation::authorization) approver_relation: KindId,
    pub(in crate::domain_computation::authorization) grant_relation: KindId,
    pub(in crate::domain_computation::authorization) review_relation: KindId,
    pub(in crate::domain_computation::authorization) reviewer_relation: KindId,
    pub(in crate::domain_computation::authorization) requested:
        worth_foundational::facade::AspectValue,
    pub(in crate::domain_computation::authorization) approved:
        worth_foundational::facade::AspectValue,
    pub(in crate::domain_computation::authorization) expired:
        worth_foundational::facade::AspectValue,
    pub(in crate::domain_computation::authorization) revoked:
        worth_foundational::facade::AspectValue,
    pub(in crate::domain_computation::authorization) review_required:
        worth_foundational::facade::AspectValue,
    pub(in crate::domain_computation::authorization) review_completed:
        worth_foundational::facade::AspectValue,
    pub(in crate::domain_computation::authorization) maximum_duration: std::time::Duration,
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
        lifecycle: WorthQueryCapabilityElevationLifecycleBindings,
    ) -> Self {
        Self {
            elevation_kind,
            active_path_index,
            expired_path_index,
            self_approval_path_index,
            temporal,
            approver_conflict_requirements,
            lifecycle,
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
