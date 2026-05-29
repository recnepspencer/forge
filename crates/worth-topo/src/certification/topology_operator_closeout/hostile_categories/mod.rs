mod hostile_category_posture;
mod hostile_category_requirements;
mod hostile_category_types;

pub use hostile_category_types::{
    MilestoneThreeHostileCertificationCategory, MilestoneThreeHostileCertificationCategoryRow,
    MilestoneThreeHostileCertificationStatus,
};

pub(in crate::certification::topology_operator_closeout) use hostile_category_posture::{
    build_hostile_certification_category_rows, ensure_hostile_certification_category_rows,
};
pub(in crate::certification::topology_operator_closeout) use hostile_category_requirements::milestone_three_expected_primitive_family_labels;




