use super::super::relevance::BridgeSliceCategory;
use super::admission::{
    LocalityAdmissionClass, LocalityMaintenanceClass, LocalityScopeAdmission,
    LocalitySemanticBasis, StreamLoweringAdmissionClass,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegionScopedSubscriptionIdentity {
    pub(in crate::live) digest: String,
    pub(in crate::live) query_digest: String,
    pub(in crate::live) locality_digest: String,
    pub(in crate::live) admission_class: LocalityAdmissionClass,
}

impl RegionScopedSubscriptionIdentity {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn locality_digest(&self) -> &str {
        &self.locality_digest
    }

    pub fn admission_class(&self) -> &LocalityAdmissionClass {
        &self.admission_class
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalityAwareRelevanceContract {
    pub(in crate::live) digest: String,
    pub(in crate::live) locality_digest: String,
    pub(in crate::live) admission_class: LocalityAdmissionClass,
    pub(in crate::live) semantic_basis: LocalitySemanticBasis,
    pub(in crate::live) scope_admission: LocalityScopeAdmission,
    pub(in crate::live) maintenance_class: LocalityMaintenanceClass,
    pub(in crate::live) stream_lowering_admission: StreamLoweringAdmissionClass,
    pub(in crate::live) expected_slice_category: BridgeSliceCategory,
}

impl LocalityAwareRelevanceContract {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn locality_digest(&self) -> &str {
        &self.locality_digest
    }

    pub fn admission_class(&self) -> &LocalityAdmissionClass {
        &self.admission_class
    }

    pub fn semantic_basis(&self) -> &LocalitySemanticBasis {
        &self.semantic_basis
    }

    pub fn scope_admission(&self) -> &LocalityScopeAdmission {
        &self.scope_admission
    }

    pub fn maintenance_class(&self) -> &LocalityMaintenanceClass {
        &self.maintenance_class
    }

    pub fn stream_lowering_admission(&self) -> &StreamLoweringAdmissionClass {
        &self.stream_lowering_admission
    }

    pub fn expected_slice_category(&self) -> &BridgeSliceCategory {
        &self.expected_slice_category
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegionSliceMatch {
    pub(in crate::live) scope: String,
    pub(in crate::live) locality_digest: String,
}

impl RegionSliceMatch {
    pub fn scope(&self) -> &str {
        &self.scope
    }

    pub fn locality_digest(&self) -> &str {
        &self.locality_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PartitionSliceMatch {
    pub(in crate::live) scope: String,
    pub(in crate::live) locality_digest: String,
}

impl PartitionSliceMatch {
    pub fn scope(&self) -> &str {
        &self.scope
    }

    pub fn locality_digest(&self) -> &str {
        &self.locality_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LocalityMatchClass {
    RegionMatch(RegionSliceMatch),
    PartitionMatch(PartitionSliceMatch),
    OffRegionSuppressed { locality_digest: String },
}

impl LocalityMatchClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RegionMatch(_) => "region_match",
            Self::PartitionMatch(_) => "partition_match",
            Self::OffRegionSuppressed { .. } => "off_region_suppressed",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LocalityWideningDecision {
    Admitted {
        matched_scope: String,
        peer_scopes: Vec<String>,
    },
    Denied {
        expected: String,
        received: Vec<String>,
    },
}

impl LocalityWideningDecision {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Admitted { .. } => "admitted",
            Self::Denied { .. } => "denied",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LocalityMatchKind {
    InRegionRegionScope,
    InRegionPartitionScope,
    InRegionRegionScopeWithPeerWidening { peer_scopes: Vec<String> },
    InRegionPartitionScopeWithPeerWidening { peer_scopes: Vec<String> },
    OffRegionSuppressed,
}
