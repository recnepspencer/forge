use super::posture_row::WorthGraphReadAccessPlanAdoptionPostureKind;

pub const QUERY_ACCESS_POSTURE_MATRIX: [WorthGraphReadAccessPlanAdoptionPostureKind; 9] = [
    WorthGraphReadAccessPlanAdoptionPostureKind::InlineIndexedAdmitted,
    WorthGraphReadAccessPlanAdoptionPostureKind::BoundedEphemeralIndexAdmitted,
    WorthGraphReadAccessPlanAdoptionPostureKind::PagedStreamingAdmitted,
    WorthGraphReadAccessPlanAdoptionPostureKind::PagedStreamingRequired,
    WorthGraphReadAccessPlanAdoptionPostureKind::PersistentIndexRequired,
    WorthGraphReadAccessPlanAdoptionPostureKind::AsyncMaterializationRequired,
    WorthGraphReadAccessPlanAdoptionPostureKind::StoreBackedCapabilityRequired,
    WorthGraphReadAccessPlanAdoptionPostureKind::AccessCapabilityRegistrationRequired,
    WorthGraphReadAccessPlanAdoptionPostureKind::Denied,
];
