use forge_query::facade::{
    ForgeQueryGraphReadAccessAdmissionPosture, ForgeQueryGraphReadAccessDenialKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAdmissionCapabilityGapKind {
    RequirementDerivationBlocked,
    MissingQueryReadFamilyArtifact,
    PersistentIndexRequired,
    PagedStreamingRequired,
    AsyncMaterializationRequired,
    StoreBackedCapabilityRequired,
    AccessCapabilityRegistrationRequired,
    QueryAdmissionDenied,
}

impl WorthGraphReadAdmissionCapabilityGapKind {
    pub fn from_query_denial_kind(kind: &ForgeQueryGraphReadAccessDenialKind) -> Self {
        match kind {
            ForgeQueryGraphReadAccessDenialKind::BudgetExceeded => {
                Self::AsyncMaterializationRequired
            }
            ForgeQueryGraphReadAccessDenialKind::RequiredAsyncMaterialization => {
                Self::AsyncMaterializationRequired
            }
            ForgeQueryGraphReadAccessDenialKind::RequiredAccessCapabilityRegistration => {
                Self::AccessCapabilityRegistrationRequired
            }
            ForgeQueryGraphReadAccessDenialKind::RequiredPersistentIndex => {
                Self::PersistentIndexRequired
            }
            ForgeQueryGraphReadAccessDenialKind::UnsupportedGraphIndexSupport => {
                Self::StoreBackedCapabilityRequired
            }
        }
    }

    pub fn from_required_query_posture(
        posture: &ForgeQueryGraphReadAccessAdmissionPosture,
    ) -> Option<Self> {
        match posture {
            ForgeQueryGraphReadAccessAdmissionPosture::PagedStreamingRequired => {
                Some(Self::PagedStreamingRequired)
            }
            ForgeQueryGraphReadAccessAdmissionPosture::PersistentIndexRequired => {
                Some(Self::PersistentIndexRequired)
            }
            ForgeQueryGraphReadAccessAdmissionPosture::AsyncMaterializationRequired => {
                Some(Self::AsyncMaterializationRequired)
            }
            ForgeQueryGraphReadAccessAdmissionPosture::StoreBackedCapabilityRequired => {
                Some(Self::StoreBackedCapabilityRequired)
            }
            ForgeQueryGraphReadAccessAdmissionPosture::AccessCapabilityRegistrationRequired => {
                Some(Self::AccessCapabilityRegistrationRequired)
            }
            ForgeQueryGraphReadAccessAdmissionPosture::Denied => Some(Self::QueryAdmissionDenied),
            ForgeQueryGraphReadAccessAdmissionPosture::InlineIndexed
            | ForgeQueryGraphReadAccessAdmissionPosture::BoundedEphemeralIndex
            | ForgeQueryGraphReadAccessAdmissionPosture::AdmittedPagedStreaming => None,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequirementDerivationBlocked => "requirement_derivation_blocked",
            Self::MissingQueryReadFamilyArtifact => "missing_query_read_family_artifact",
            Self::PersistentIndexRequired => "persistent_index_required",
            Self::PagedStreamingRequired => "paged_streaming_required",
            Self::AsyncMaterializationRequired => "async_materialization_required",
            Self::StoreBackedCapabilityRequired => "store_backed_capability_required",
            Self::AccessCapabilityRegistrationRequired => "access_capability_registration_required",
            Self::QueryAdmissionDenied => "query_admission_denied",
        }
    }
}
