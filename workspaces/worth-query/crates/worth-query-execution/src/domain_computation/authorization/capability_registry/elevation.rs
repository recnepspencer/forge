use worth_relational::facade::identity::KindId;

pub(in crate::domain_computation::authorization) struct WorthQueryCapabilityElevationBindings {
    pub(in crate::domain_computation::authorization) elevation_kind: KindId,
    pub(in crate::domain_computation::authorization) required_path_index: usize,
    pub(in crate::domain_computation::authorization) self_approval_path_index: usize,
    pub(in crate::domain_computation::authorization) approver_conflict_requirements:
        Vec<Vec<usize>>,
}

impl WorthQueryCapabilityElevationBindings {
    pub(in crate::domain_computation::authorization) fn new(
        elevation_kind: KindId,
        required_path_index: usize,
        self_approval_path_index: usize,
        approver_conflict_requirements: Vec<Vec<usize>>,
    ) -> Self {
        Self {
            elevation_kind,
            required_path_index,
            self_approval_path_index,
            approver_conflict_requirements,
        }
    }
}
