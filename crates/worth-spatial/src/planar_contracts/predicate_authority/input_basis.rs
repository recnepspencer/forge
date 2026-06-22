#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PlanarPredicateCoincidencePolicy {
    AdmitCertifiedZero,
    DenyCertifiedZeroBeforeRepair,
}

impl PlanarPredicateCoincidencePolicy {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdmitCertifiedZero => "admit-certified-zero",
            Self::DenyCertifiedZeroBeforeRepair => "deny-certified-zero-before-repair",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarPredicateInputBasis {
    local_frame_identity: String,
    topology_basis_identity: String,
    movement_rotation_posture_identity: String,
    tolerance_policy_identity: String,
    projected_points: [[f64; 2]; 3],
    coincidence_policy: PlanarPredicateCoincidencePolicy,
}

impl PlanarPredicateInputBasis {
    pub fn from_projected_orient2d_points(
        local_frame_identity: impl Into<String>,
        topology_basis_identity: impl Into<String>,
        movement_rotation_posture_identity: impl Into<String>,
        tolerance_policy_identity: impl Into<String>,
        projected_points: [[f64; 2]; 3],
    ) -> Self {
        Self {
            local_frame_identity: local_frame_identity.into(),
            topology_basis_identity: topology_basis_identity.into(),
            movement_rotation_posture_identity: movement_rotation_posture_identity.into(),
            tolerance_policy_identity: tolerance_policy_identity.into(),
            projected_points,
            coincidence_policy: PlanarPredicateCoincidencePolicy::AdmitCertifiedZero,
        }
    }

    pub fn with_coincidence_policy(mut self, policy: PlanarPredicateCoincidencePolicy) -> Self {
        self.coincidence_policy = policy;
        self
    }

    pub fn local_frame_identity(&self) -> &str {
        &self.local_frame_identity
    }

    pub fn topology_basis_identity(&self) -> &str {
        &self.topology_basis_identity
    }

    pub fn movement_rotation_posture_identity(&self) -> &str {
        &self.movement_rotation_posture_identity
    }

    pub fn tolerance_policy_identity(&self) -> &str {
        &self.tolerance_policy_identity
    }

    pub fn projected_points(&self) -> [[f64; 2]; 3] {
        self.projected_points
    }

    pub fn coincidence_policy(&self) -> PlanarPredicateCoincidencePolicy {
        self.coincidence_policy
    }
}
