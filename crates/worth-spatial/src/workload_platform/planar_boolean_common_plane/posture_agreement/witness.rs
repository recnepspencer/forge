#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanCommonPlanePostureWitness {
    posture_identity: String,
    semantic_posture_identity: String,
    projected_workload_identity: String,
    transform_stage_identity: String,
}

impl PlanarBooleanCommonPlanePostureWitness {
    pub(crate) fn new(
        posture_identity: impl Into<String>,
        semantic_posture_identity: impl Into<String>,
        projected_workload_identity: impl Into<String>,
        transform_stage_identity: impl Into<String>,
    ) -> Self {
        Self {
            posture_identity: posture_identity.into(),
            semantic_posture_identity: semantic_posture_identity.into(),
            projected_workload_identity: projected_workload_identity.into(),
            transform_stage_identity: transform_stage_identity.into(),
        }
    }

    pub fn posture_identity(&self) -> &str {
        &self.posture_identity
    }

    pub fn semantic_posture_identity(&self) -> &str {
        &self.semantic_posture_identity
    }

    pub fn projected_workload_identity(&self) -> &str {
        &self.projected_workload_identity
    }

    pub fn transform_stage_identity(&self) -> &str {
        &self.transform_stage_identity
    }
}
