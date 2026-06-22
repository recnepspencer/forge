use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

use super::{
    ForgeQueryAuthorityLane, ForgeQueryRuntimeBackendPosture, ForgeQueryRuntimeFacadeFamily,
    ForgeQueryRuntimeFamilySupportStatus, ForgeQueryRuntimeFamilyTeachingPosture,
    ForgeQueryRuntimeSupportProfile,
};
#[path = "public_api_naming.rs"]
mod naming;
#[path = "public_api_transcript.rs"]
mod transcript;

pub use naming::{ForgeQueryRuntimePublicApiNamingContract, ForgeQueryRuntimePublicApiNamingRow};
pub use transcript::ForgeQueryRuntimePublicApiTranscriptEvidence;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimePublicApiFamilyContract {
    family: ForgeQueryRuntimeFacadeFamily,
    status: ForgeQueryRuntimeFamilySupportStatus,
    teaching_posture: ForgeQueryRuntimeFamilyTeachingPosture,
    authority_lanes: Vec<ForgeQueryAuthorityLane>,
    evidence: Vec<String>,
    reason: Option<String>,
    owner_closure: String,
    extension_rule: String,
    parallel_api_forbidden: bool,
    admission_fail_closed: bool,
    contract_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryRuntimePublicApiFamilyContract {
    fn from_support_row(row: &super::ForgeQueryRuntimeFamilySupport) -> Self {
        let family = row.family();
        let status = row.status();
        let teaching_posture = row.teaching_posture();
        let authority_lanes = row.authority_lanes().to_vec();
        let evidence = row.evidence().to_vec();
        let reason = row.denial_reason().map(str::to_string);
        let owner_closure = row.owner_closure().to_string();
        let extension_rule = row.extension_rule().to_string();
        let parallel_api_forbidden = row.parallel_api_forbidden();
        let admission_fail_closed = row.admission_fail_closed();
        let contract_identity =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::RuntimePublicApiFamilyContract)
                .field_shape(ForgeQueryEvidenceTag::new("family"), family.as_str())
                .field_shape(ForgeQueryEvidenceTag::new("status"), status.as_str())
                .field_shape(
                    ForgeQueryEvidenceTag::new("teaching_posture"),
                    teaching_posture.as_str(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("owner_closure"),
                    owner_closure.clone(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("extension_rule"),
                    extension_rule.clone(),
                )
                .field_bool(
                    ForgeQueryEvidenceTag::new("parallel_api_forbidden"),
                    parallel_api_forbidden,
                )
                .field_bool(
                    ForgeQueryEvidenceTag::new("admission_fail_closed"),
                    admission_fail_closed,
                )
                .field_value_sequence(
                    ForgeQueryEvidenceTag::new("authority_lanes"),
                    authority_lanes.iter().map(|lane| lane.as_str()),
                )
                .field_value_sequence(
                    ForgeQueryEvidenceTag::new("evidence"),
                    evidence.iter().map(String::as_str),
                )
                .optional_value(ForgeQueryEvidenceTag::new("reason"), reason.as_deref())
                .seal();
        Self {
            family,
            status,
            teaching_posture,
            authority_lanes,
            evidence,
            reason,
            owner_closure,
            extension_rule,
            parallel_api_forbidden,
            admission_fail_closed,
            contract_identity,
        }
    }

    pub fn family(&self) -> ForgeQueryRuntimeFacadeFamily {
        self.family
    }

    pub fn status(&self) -> ForgeQueryRuntimeFamilySupportStatus {
        self.status
    }

    pub fn teaching_posture(&self) -> ForgeQueryRuntimeFamilyTeachingPosture {
        self.teaching_posture
    }

    pub fn ordinary_downstream_dx(&self) -> bool {
        self.teaching_posture == ForgeQueryRuntimeFamilyTeachingPosture::OrdinaryRuntimeDx
    }

    pub fn authority_lanes(&self) -> &[ForgeQueryAuthorityLane] {
        &self.authority_lanes
    }

    pub fn evidence(&self) -> &[String] {
        &self.evidence
    }

    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    pub fn owner_closure(&self) -> &str {
        &self.owner_closure
    }

    pub fn extension_rule(&self) -> &str {
        &self.extension_rule
    }

    pub fn parallel_api_forbidden(&self) -> bool {
        self.parallel_api_forbidden
    }

    pub fn admission_fail_closed(&self) -> bool {
        self.admission_fail_closed
    }

    pub fn contract_digest(&self) -> &str {
        self.contract_identity.as_str()
    }

    pub fn contract_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.contract_identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimePublicApiContract {
    backend_posture: ForgeQueryRuntimeBackendPosture,
    families: Vec<ForgeQueryRuntimePublicApiFamilyContract>,
    stable_family_count: usize,
    deferred_family_count: usize,
    unsupported_family_count: usize,
    contract_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryRuntimePublicApiContract {
    pub fn from_support_profile(profile: &ForgeQueryRuntimeSupportProfile) -> Self {
        let families: Vec<_> = profile
            .rows()
            .map(ForgeQueryRuntimePublicApiFamilyContract::from_support_row)
            .collect();
        let stable_family_count = families
            .iter()
            .filter(|family| family.status() == ForgeQueryRuntimeFamilySupportStatus::Supported)
            .count();
        let deferred_family_count = families
            .iter()
            .filter(|family| family.status() == ForgeQueryRuntimeFamilySupportStatus::DeferredDebt)
            .count();
        let unsupported_family_count = families
            .iter()
            .filter(|family| family.status() == ForgeQueryRuntimeFamilySupportStatus::Unsupported)
            .count();
        let contract_identity =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::RuntimePublicApiContract)
                .field_shape(
                    ForgeQueryEvidenceTag::new("backend_posture"),
                    profile.posture().as_str(),
                )
                .field_value_sequence(
                    ForgeQueryEvidenceTag::new("family_contract_digest"),
                    families
                        .iter()
                        .map(ForgeQueryRuntimePublicApiFamilyContract::contract_digest),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("stable_family_count"),
                    stable_family_count,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("deferred_family_count"),
                    deferred_family_count,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("unsupported_family_count"),
                    unsupported_family_count,
                )
                .seal();
        Self {
            backend_posture: profile.posture(),
            families,
            stable_family_count,
            deferred_family_count,
            unsupported_family_count,
            contract_identity,
        }
    }

    pub fn backend_posture(&self) -> ForgeQueryRuntimeBackendPosture {
        self.backend_posture
    }

    pub fn families(&self) -> &[ForgeQueryRuntimePublicApiFamilyContract] {
        &self.families
    }

    pub fn stable_family_count(&self) -> usize {
        self.stable_family_count
    }

    pub fn deferred_family_count(&self) -> usize {
        self.deferred_family_count
    }

    pub fn unsupported_family_count(&self) -> usize {
        self.unsupported_family_count
    }

    pub fn contract_digest(&self) -> &str {
        self.contract_identity.as_str()
    }

    pub fn contract_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.contract_identity
    }

    pub fn family(
        &self,
        family: ForgeQueryRuntimeFacadeFamily,
    ) -> Option<&ForgeQueryRuntimePublicApiFamilyContract> {
        self.families.iter().find(|row| row.family() == family)
    }
}
