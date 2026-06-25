use super::super::stable_identity_digest::stable_digest;
use super::gap_contract::{
    WorthGraphReadAdmissionExpectedDenial, WorthGraphReadAdmissionSuggestedPosture,
};
use super::gap_kind::WorthGraphReadAdmissionCapabilityGapKind;
use crate::graph_read_access_declarations::WorthGraphReadRequirementDerivationRecord;
use forge_query::facade::ForgeQueryGraphReadAccessAdmission;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAdmissionCapabilityGap {
    kind: WorthGraphReadAdmissionCapabilityGapKind,
    source_requirement_record_digest: String,
    query_family_anchor_digest: String,
    read_family_target: String,
    owner: &'static str,
    expected_denial: WorthGraphReadAdmissionExpectedDenial,
    suggested_posture: WorthGraphReadAdmissionSuggestedPosture,
    blocker: String,
    removal_trigger: String,
    must_not_exceed_count: usize,
    gap_digest: String,
}

impl WorthGraphReadAdmissionCapabilityGap {
    pub(crate) fn requirement_derivation_blocked(
        record: &WorthGraphReadRequirementDerivationRecord,
        blocker: impl Into<String>,
        removal_trigger: impl Into<String>,
    ) -> Self {
        Self::new(
            WorthGraphReadAdmissionCapabilityGapKind::RequirementDerivationBlocked,
            record,
            "worth_graph_read_declarations",
            WorthGraphReadAdmissionExpectedDenial::RequirementDerivationGap,
            WorthGraphReadAdmissionSuggestedPosture::RequirementDerivationMustSucceed,
            blocker,
            removal_trigger,
            1,
        )
    }

    pub(crate) fn missing_query_read_family_artifact(
        record: &WorthGraphReadRequirementDerivationRecord,
    ) -> Self {
        Self::new(
            WorthGraphReadAdmissionCapabilityGapKind::MissingQueryReadFamilyArtifact,
            record,
            "forge_query",
            WorthGraphReadAdmissionExpectedDenial::MissingQueryReadFamilyArtifact,
            WorthGraphReadAdmissionSuggestedPosture::QueryReadFamilyArtifactRequired,
            "Phase 5 cannot call Query admission because Phase 4 still carries a Query family anchor rather than a real ForgeQueryReadFamily artifact.",
            "Replace this gap when Phase 4 lowers catalog records into real ForgeQueryReadFamily artifacts.",
            1,
        )
    }

    pub(crate) fn from_query_admission_denial(
        record: &WorthGraphReadRequirementDerivationRecord,
        admission: &ForgeQueryGraphReadAccessAdmission,
    ) -> Option<Self> {
        let denial = admission.denial()?;
        let kind = WorthGraphReadAdmissionCapabilityGapKind::from_query_denial_kind(denial.kind());
        Some(Self::new(
            kind,
            record,
            "forge_query",
            WorthGraphReadAdmissionExpectedDenial::QueryAdmissionDenied(denial.kind().clone()),
            WorthGraphReadAdmissionSuggestedPosture::QueryAdmissionPosture(
                denial.suggested_posture().clone(),
            ),
            format!(
                "Query admission denied graph-read access for family {} with denial {}.",
                record.query_family_name(),
                denial.kind().as_str()
            ),
            format!(
                "Remove this gap when Query admission no longer returns {} for this read family.",
                denial.kind().as_str()
            ),
            1,
        ))
    }

    fn new(
        kind: WorthGraphReadAdmissionCapabilityGapKind,
        record: &WorthGraphReadRequirementDerivationRecord,
        owner: &'static str,
        expected_denial: WorthGraphReadAdmissionExpectedDenial,
        suggested_posture: WorthGraphReadAdmissionSuggestedPosture,
        blocker: impl Into<String>,
        removal_trigger: impl Into<String>,
        must_not_exceed_count: usize,
    ) -> Self {
        let source_requirement_record_digest = record.record_digest().to_string();
        let query_family_anchor_digest = record.query_family_digest_seed().to_string();
        let read_family_target = record.read_family_target().to_string();
        let blocker = blocker.into();
        let removal_trigger = removal_trigger.into();
        let gap_digest = stable_digest(&[
            "worth_graph_read_admission_capability_gap_v1".to_string(),
            format!("kind:{}", kind.as_str()),
            format!("requirement_record:{source_requirement_record_digest}"),
            format!("query_family_anchor:{query_family_anchor_digest}"),
            format!("read_family_target:{read_family_target}"),
            format!("owner:{owner}"),
            format!("expected_denial:{}", expected_denial.digest_part()),
            format!("suggested_posture:{}", suggested_posture.digest_part()),
            format!("blocker:{blocker}"),
            format!("removal_trigger:{removal_trigger}"),
            format!("must_not_exceed_count:{must_not_exceed_count}"),
        ]);
        Self {
            kind,
            source_requirement_record_digest,
            query_family_anchor_digest,
            read_family_target,
            owner,
            expected_denial,
            suggested_posture,
            blocker,
            removal_trigger,
            must_not_exceed_count,
            gap_digest,
        }
    }

    pub const fn kind(&self) -> WorthGraphReadAdmissionCapabilityGapKind {
        self.kind
    }

    pub fn source_requirement_record_digest(&self) -> &str {
        &self.source_requirement_record_digest
    }

    pub fn query_family_anchor_digest(&self) -> &str {
        &self.query_family_anchor_digest
    }

    pub fn read_family_target(&self) -> &str {
        &self.read_family_target
    }

    pub const fn owner(&self) -> &'static str {
        self.owner
    }

    pub fn expected_denial(&self) -> &WorthGraphReadAdmissionExpectedDenial {
        &self.expected_denial
    }

    pub fn suggested_posture(&self) -> &WorthGraphReadAdmissionSuggestedPosture {
        &self.suggested_posture
    }

    pub fn blocker(&self) -> &str {
        &self.blocker
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }

    pub const fn must_not_exceed_count(&self) -> usize {
        self.must_not_exceed_count
    }

    pub fn gap_digest(&self) -> &str {
        &self.gap_digest
    }
}
