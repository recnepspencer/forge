use forge_query::facade::{BasisFamily, BasisLifecycleSupportDiscovery, BasisSupportPosture};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryBasisPostureReference {
    family: BasisFamily,
    operation_lane: &'static str,
    posture: BasisSupportPosture,
    discovery_digest: String,
}

impl QueryBasisPostureReference {
    pub fn from_basis_support_discovery(discovery: &BasisLifecycleSupportDiscovery) -> Self {
        Self {
            family: discovery.requested_family(),
            operation_lane: discovery.requested_operation_lane(),
            posture: discovery.posture(),
            discovery_digest: discovery.discovery_digest().to_string(),
        }
    }

    pub fn posture(&self) -> BasisSupportPosture {
        self.posture
    }

    pub fn is_admitted(&self) -> bool {
        matches!(
            self.posture,
            BasisSupportPosture::Admitted | BasisSupportPosture::Advisory
        )
    }

    pub fn digest_basis(&self) -> String {
        format!(
            "{}|{}|{}|{}",
            self.family.as_str(),
            self.operation_lane,
            self.posture.as_str(),
            self.discovery_digest
        )
    }
}
