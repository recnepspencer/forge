use super::super::read_family_adoption::WorthGraphReadAccessPlanAdoptionSeedPairing;
use super::super::stable_digest;
use super::{query_admission_api_required, WorthGraphReadAccessPlanAdoptionAdmissionInput};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessPlanAdoptionAttemptKind {
    QueryAdmissionInspected,
    AdmittedPlanCandidate,
    RequiredOrDeniedPosture,
    MissingQueryReadFamilyArtifact,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPlanAdoptionAttempt {
    kind: WorthGraphReadAccessPlanAdoptionAttemptKind,
    source_pairing_digest: String,
    source_requirement_record_digest: String,
    read_family_identity_digest: String,
    requirement_row_digest: String,
    query_family_name: String,
    query_family_digest_seed: String,
    read_family_target: String,
    query_api_required: &'static str,
    query_admission_digest: Option<String>,
    query_requirement_set_digest: Option<String>,
    query_posture: Option<String>,
    admitted_plan_digest: Option<String>,
    denial_kind: Option<String>,
    blocker: Option<String>,
    removal_trigger: Option<String>,
    attempt_digest: String,
}

impl WorthGraphReadAccessPlanAdoptionAttempt {
    pub(in crate::graph_read_access_plan_adoption::phase_two_adoption) fn missing_query_read_family_artifact(
        input: WorthGraphReadAccessPlanAdoptionAdmissionInput<'_>,
    ) -> Self {
        debug_assert!(input.query_read_family().is_none());
        Self::new(AttemptParts {
            kind: WorthGraphReadAccessPlanAdoptionAttemptKind::MissingQueryReadFamilyArtifact,
            pairing: input.pairing(),
            query_api_required: query_admission_api_required(),
            query_admission_digest: None,
            query_requirement_set_digest: None,
            query_posture: Some("query_read_family_artifact_required".to_string()),
            admitted_plan_digest: None,
            denial_kind: Some("missing_query_read_family_artifact".to_string()),
            blocker: Some(
                "Milestone 7 closeout currently carries read-family identity projection rows, not the real ForgeQueryReadFamily artifact required by Query access-plan admission."
                    .to_string(),
            ),
            removal_trigger: Some(
                "Replace this gap when the declaration closeout preserves real ForgeQueryReadFamily artifacts for every covered read family."
                    .to_string(),
            ),
        })
    }

    fn new(parts: AttemptParts<'_>) -> Self {
        let attempt_digest = stable_digest(&[
            "worth_graph_read_access_plan_adoption_attempt_v1".to_string(),
            format!("kind:{}", parts.kind.as_str()),
            format!("pairing:{}", parts.pairing.pairing_digest()),
            format!(
                "requirement_record:{}",
                parts.pairing.source_requirement_record_digest()
            ),
            format!(
                "read_family:{}",
                parts.pairing.read_family_identity_digest()
            ),
            format!("requirement_row:{}", parts.pairing.requirement_row_digest()),
            format!("query_family:{}", parts.pairing.query_family_digest_seed()),
            format!("read_family_target:{}", parts.pairing.read_family_target()),
            format!("query_api:{}", parts.query_api_required),
            format!(
                "query_admission:{}",
                parts.query_admission_digest.as_deref().unwrap_or("none")
            ),
            format!(
                "query_requirement_set:{}",
                parts
                    .query_requirement_set_digest
                    .as_deref()
                    .unwrap_or("none")
            ),
            format!(
                "query_posture:{}",
                parts.query_posture.as_deref().unwrap_or("none")
            ),
            format!(
                "admitted_plan:{}",
                parts.admitted_plan_digest.as_deref().unwrap_or("none")
            ),
            format!("denial:{}", parts.denial_kind.as_deref().unwrap_or("none")),
        ]);

        Self {
            kind: parts.kind,
            source_pairing_digest: parts.pairing.pairing_digest().to_string(),
            source_requirement_record_digest: parts
                .pairing
                .source_requirement_record_digest()
                .to_string(),
            read_family_identity_digest: parts.pairing.read_family_identity_digest().to_string(),
            requirement_row_digest: parts.pairing.requirement_row_digest().to_string(),
            query_family_name: parts.pairing.query_family_name().to_string(),
            query_family_digest_seed: parts.pairing.query_family_digest_seed().to_string(),
            read_family_target: parts.pairing.read_family_target().to_string(),
            query_api_required: parts.query_api_required,
            query_admission_digest: parts.query_admission_digest,
            query_requirement_set_digest: parts.query_requirement_set_digest,
            query_posture: parts.query_posture,
            admitted_plan_digest: parts.admitted_plan_digest,
            denial_kind: parts.denial_kind,
            blocker: parts.blocker,
            removal_trigger: parts.removal_trigger,
            attempt_digest,
        }
    }

    #[cfg(test)]
    pub(crate) fn for_posture_matrix_test(
        kind: WorthGraphReadAccessPlanAdoptionAttemptKind,
        requirement_identity: &str,
        query_posture: &str,
        denial_kind: Option<&str>,
        blocker: Option<&str>,
        removal_trigger: Option<&str>,
    ) -> Self {
        let source_pairing_digest = format!("pairing:{requirement_identity}");
        let source_requirement_record_digest = format!("requirement_record:{requirement_identity}");
        let read_family_identity_digest = format!("read_family:{requirement_identity}");
        let requirement_row_digest = format!("requirement_row:{requirement_identity}");
        let query_family_name = format!("query_family_{requirement_identity}");
        let query_family_digest_seed = format!("query_family_seed:{requirement_identity}");
        let read_family_target = requirement_identity.to_string();
        let query_admission_digest = match kind {
            WorthGraphReadAccessPlanAdoptionAttemptKind::QueryAdmissionInspected
            | WorthGraphReadAccessPlanAdoptionAttemptKind::AdmittedPlanCandidate
            | WorthGraphReadAccessPlanAdoptionAttemptKind::RequiredOrDeniedPosture => {
                Some(format!("query_admission:{requirement_identity}"))
            }
            WorthGraphReadAccessPlanAdoptionAttemptKind::MissingQueryReadFamilyArtifact => None,
        };
        let admitted_plan_digest = match kind {
            WorthGraphReadAccessPlanAdoptionAttemptKind::AdmittedPlanCandidate => {
                Some(format!("admitted_plan:{requirement_identity}"))
            }
            WorthGraphReadAccessPlanAdoptionAttemptKind::QueryAdmissionInspected
            | WorthGraphReadAccessPlanAdoptionAttemptKind::RequiredOrDeniedPosture
            | WorthGraphReadAccessPlanAdoptionAttemptKind::MissingQueryReadFamilyArtifact => None,
        };
        let query_requirement_set_digest = query_admission_digest
            .as_ref()
            .map(|_| format!("query_requirement_set:{requirement_identity}"));
        let denial_kind = denial_kind.map(str::to_string);
        let blocker = blocker.map(str::to_string);
        let removal_trigger = removal_trigger.map(str::to_string);
        let attempt_digest = stable_digest(&[
            "worth_graph_read_access_plan_adoption_attempt_v1".to_string(),
            format!("kind:{}", kind.as_str()),
            format!("pairing:{source_pairing_digest}"),
            format!("requirement_record:{source_requirement_record_digest}"),
            format!("read_family:{read_family_identity_digest}"),
            format!("requirement_row:{requirement_row_digest}"),
            format!("query_family:{query_family_digest_seed}"),
            format!("read_family_target:{read_family_target}"),
            format!("query_api:{}", query_admission_api_required()),
            format!(
                "query_admission:{}",
                query_admission_digest.as_deref().unwrap_or("none")
            ),
            format!(
                "query_requirement_set:{}",
                query_requirement_set_digest.as_deref().unwrap_or("none")
            ),
            format!("query_posture:{query_posture}"),
            format!(
                "admitted_plan:{}",
                admitted_plan_digest.as_deref().unwrap_or("none")
            ),
            format!("denial:{}", denial_kind.as_deref().unwrap_or("none")),
        ]);

        Self {
            kind,
            source_pairing_digest,
            source_requirement_record_digest,
            read_family_identity_digest,
            requirement_row_digest,
            query_family_name,
            query_family_digest_seed,
            read_family_target,
            query_api_required: query_admission_api_required(),
            query_admission_digest,
            query_requirement_set_digest,
            query_posture: Some(query_posture.to_string()),
            admitted_plan_digest,
            denial_kind,
            blocker,
            removal_trigger,
            attempt_digest,
        }
    }

    pub const fn kind(&self) -> WorthGraphReadAccessPlanAdoptionAttemptKind {
        self.kind
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

    pub const fn query_api_required(&self) -> &'static str {
        self.query_api_required
    }

    pub fn query_admission_digest(&self) -> Option<&str> {
        self.query_admission_digest.as_deref()
    }

    pub fn query_requirement_set_digest(&self) -> Option<&str> {
        self.query_requirement_set_digest.as_deref()
    }

    pub fn query_posture(&self) -> Option<&str> {
        self.query_posture.as_deref()
    }

    pub fn admitted_plan_digest(&self) -> Option<&str> {
        self.admitted_plan_digest.as_deref()
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

    pub fn attempt_digest(&self) -> &str {
        &self.attempt_digest
    }
}

impl WorthGraphReadAccessPlanAdoptionAttemptKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QueryAdmissionInspected => "query_admission_inspected",
            Self::AdmittedPlanCandidate => "admitted_plan_candidate",
            Self::RequiredOrDeniedPosture => "required_or_denied_posture",
            Self::MissingQueryReadFamilyArtifact => "missing_query_read_family_artifact",
        }
    }
}

struct AttemptParts<'a> {
    kind: WorthGraphReadAccessPlanAdoptionAttemptKind,
    pairing: &'a WorthGraphReadAccessPlanAdoptionSeedPairing,
    query_api_required: &'static str,
    query_admission_digest: Option<String>,
    query_requirement_set_digest: Option<String>,
    query_posture: Option<String>,
    admitted_plan_digest: Option<String>,
    denial_kind: Option<String>,
    blocker: Option<String>,
    removal_trigger: Option<String>,
}
