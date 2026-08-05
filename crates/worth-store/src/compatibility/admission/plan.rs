use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityBatchScope {
    family_id: ArtifactFamilyId,
    record_count: u64,
}
impl CompatibilityBatchScope {
    pub fn new(family_id: ArtifactFamilyId, record_count: u64) -> Self {
        Self {
            family_id,
            record_count,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityAdmissionPlan {
    family_id: ArtifactFamilyId,
    relation: CompatibilityRelation,
}

impl CompatibilityAdmissionPlan {
    pub fn new(family_id: ArtifactFamilyId, relation: CompatibilityRelation) -> Self {
        Self {
            family_id,
            relation,
        }
    }
}
