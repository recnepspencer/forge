use super::super::query_admission::{
    WorthGraphReadAccessPlanAdoptionAttempt, WorthGraphReadAccessPlanAdoptionAttemptKind,
};
use super::super::stable_digest;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessPlanAdoptionPostureKind {
    AdmittedPlanCandidate,
    InlineIndexedAdmitted,
    BoundedEphemeralIndexAdmitted,
    PagedStreamingAdmitted,
    RequiredSupportPosture,
    PagedStreamingRequired,
    PersistentIndexRequired,
    AsyncMaterializationRequired,
    StoreBackedCapabilityRequired,
    AccessCapabilityRegistrationRequired,
    Denied,
    CarriedCapabilityGap,
    MissingQueryReadFamilyArtifact,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPlanAdoptionPostureRow {
    posture_kind: WorthGraphReadAccessPlanAdoptionPostureKind,
    source_attempt_digest: String,
    source_pairing_digest: String,
    source_requirement_record_digest: String,
    read_family_identity_digest: String,
    requirement_row_digest: String,
    query_family_name: String,
    query_family_digest_seed: String,
    read_family_target: String,
    query_posture: String,
    denial_kind: Option<String>,
    blocker: Option<String>,
    removal_trigger: Option<String>,
    row_digest: String,
}

impl WorthGraphReadAccessPlanAdoptionPostureRow {
    pub(crate) fn from_adoption_attempt(attempt: &WorthGraphReadAccessPlanAdoptionAttempt) -> Self {
        let posture_kind = posture_kind_for_attempt(attempt);
        let query_posture = attempt
            .query_posture()
            .unwrap_or(posture_kind.as_str())
            .to_string();
        let row_digest = stable_digest(&[
            "worth_graph_read_access_plan_adoption_posture_row_v1".to_string(),
            format!("kind:{}", posture_kind.as_str()),
            format!("attempt:{}", attempt.attempt_digest()),
            format!("pairing:{}", attempt.source_pairing_digest()),
            format!(
                "requirement_record:{}",
                attempt.source_requirement_record_digest()
            ),
            format!("read_family:{}", attempt.read_family_identity_digest()),
            format!("requirement_row:{}", attempt.requirement_row_digest()),
            format!("query_family:{}", attempt.query_family_digest_seed()),
            format!("read_family_target:{}", attempt.read_family_target()),
            format!("query_posture:{query_posture}"),
            format!("denial:{}", attempt.denial_kind().unwrap_or("none")),
        ]);

        Self {
            posture_kind,
            source_attempt_digest: attempt.attempt_digest().to_string(),
            source_pairing_digest: attempt.source_pairing_digest().to_string(),
            source_requirement_record_digest: attempt
                .source_requirement_record_digest()
                .to_string(),
            read_family_identity_digest: attempt.read_family_identity_digest().to_string(),
            requirement_row_digest: attempt.requirement_row_digest().to_string(),
            query_family_name: attempt.query_family_name().to_string(),
            query_family_digest_seed: attempt.query_family_digest_seed().to_string(),
            read_family_target: attempt.read_family_target().to_string(),
            query_posture,
            denial_kind: attempt.denial_kind().map(str::to_string),
            blocker: attempt.blocker().map(str::to_string),
            removal_trigger: attempt.removal_trigger().map(str::to_string),
            row_digest,
        }
    }

    pub const fn posture_kind(&self) -> WorthGraphReadAccessPlanAdoptionPostureKind {
        self.posture_kind
    }

    pub fn source_attempt_digest(&self) -> &str {
        &self.source_attempt_digest
    }

    pub fn source_pairing_digest(&self) -> &str {
        &self.source_pairing_digest
    }

    pub fn source_requirement_record_digest(&self) -> &str {
        &self.source_requirement_record_digest
    }

    pub fn read_family_identity_digest(&self) -> &str {
        &self.read_family_identity_digest
    }

    pub fn requirement_row_digest(&self) -> &str {
        &self.requirement_row_digest
    }

    pub fn query_family_name(&self) -> &str {
        &self.query_family_name
    }

    pub fn query_family_digest_seed(&self) -> &str {
        &self.query_family_digest_seed
    }

    pub fn read_family_target(&self) -> &str {
        &self.read_family_target
    }

    pub fn query_posture(&self) -> &str {
        &self.query_posture
    }

    pub fn denial_kind(&self) -> Option<&str> {
        self.denial_kind.as_deref()
    }

    pub fn blocker(&self) -> Option<&str> {
        self.blocker.as_deref()
    }

    pub fn removal_trigger(&self) -> Option<&str> {
        self.removal_trigger.as_deref()
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }

    pub const fn claims_access_plan_consumption(&self) -> bool {
        false
    }

    pub const fn claims_graph_read_execution(&self) -> bool {
        false
    }
}

impl WorthGraphReadAccessPlanAdoptionPostureKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdmittedPlanCandidate => "admitted_plan_candidate",
            Self::InlineIndexedAdmitted => "inline_indexed",
            Self::BoundedEphemeralIndexAdmitted => "bounded_ephemeral_index",
            Self::PagedStreamingAdmitted => "admitted_paged_streaming",
            Self::RequiredSupportPosture => "required_support_posture",
            Self::PagedStreamingRequired => "paged_streaming_required",
            Self::PersistentIndexRequired => "persistent_index_required",
            Self::AsyncMaterializationRequired => "async_materialization_required",
            Self::StoreBackedCapabilityRequired => "store_backed_capability_required",
            Self::AccessCapabilityRegistrationRequired => "access_capability_registration_required",
            Self::Denied => "denied",
            Self::CarriedCapabilityGap => "carried_capability_gap",
            Self::MissingQueryReadFamilyArtifact => "missing_query_read_family_artifact",
        }
    }

    pub const fn is_required_or_denied(self) -> bool {
        match self {
            Self::AdmittedPlanCandidate
            | Self::InlineIndexedAdmitted
            | Self::BoundedEphemeralIndexAdmitted
            | Self::PagedStreamingAdmitted => false,
            Self::RequiredSupportPosture
            | Self::PagedStreamingRequired
            | Self::PersistentIndexRequired
            | Self::AsyncMaterializationRequired
            | Self::StoreBackedCapabilityRequired
            | Self::AccessCapabilityRegistrationRequired
            | Self::Denied
            | Self::CarriedCapabilityGap
            | Self::MissingQueryReadFamilyArtifact => true,
        }
    }
}

fn posture_kind_for_attempt(
    attempt: &WorthGraphReadAccessPlanAdoptionAttempt,
) -> WorthGraphReadAccessPlanAdoptionPostureKind {
    match attempt.kind() {
        WorthGraphReadAccessPlanAdoptionAttemptKind::AdmittedPlanCandidate => {
            WorthGraphReadAccessPlanAdoptionPostureKind::AdmittedPlanCandidate
        }
        WorthGraphReadAccessPlanAdoptionAttemptKind::QueryAdmissionInspected
        | WorthGraphReadAccessPlanAdoptionAttemptKind::RequiredOrDeniedPosture => {
            WorthGraphReadAccessPlanAdoptionPostureKind::RequiredSupportPosture
        }
        WorthGraphReadAccessPlanAdoptionAttemptKind::MissingQueryReadFamilyArtifact => {
            WorthGraphReadAccessPlanAdoptionPostureKind::MissingQueryReadFamilyArtifact
        }
    }
}
