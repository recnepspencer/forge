use super::{
    ForgeQueryGraphReadAccessAdmissionPosture, ForgeQueryGraphReadRequiredCapabilityOwner,
};
use crate::identity::hash_parts;
use crate::runtime::{
    ForgeQueryGraphIndexInventoryMatchReport, ForgeQueryGraphReadAccessRequirementKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadAccessCase {
    requirement_kind: ForgeQueryGraphReadAccessRequirementKind,
    inline_posture: ForgeQueryGraphReadAccessAdmissionPosture,
    required_capability_owner: ForgeQueryGraphReadRequiredCapabilityOwner,
}

impl ForgeQueryGraphReadAccessCase {
    pub fn requirement_kind(&self) -> &ForgeQueryGraphReadAccessRequirementKind {
        &self.requirement_kind
    }

    pub fn inline_posture(&self) -> &ForgeQueryGraphReadAccessAdmissionPosture {
        &self.inline_posture
    }

    pub fn required_capability_owner(&self) -> &ForgeQueryGraphReadRequiredCapabilityOwner {
        &self.required_capability_owner
    }

    pub(crate) fn for_requirement_kind(
        requirement_kind: ForgeQueryGraphReadAccessRequirementKind,
    ) -> Self {
        let (inline_posture, required_capability_owner) =
            access_case_posture_and_owner(&requirement_kind);
        Self {
            requirement_kind,
            inline_posture,
            required_capability_owner,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "case:{}:{}:{}",
            self.requirement_kind.as_str(),
            self.inline_posture.as_str(),
            self.required_capability_owner.as_str()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadAccessCaseRegistry {
    digest: String,
    cases: Vec<ForgeQueryGraphReadAccessCase>,
}

impl ForgeQueryGraphReadAccessCaseRegistry {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn cases(&self) -> &[ForgeQueryGraphReadAccessCase] {
        &self.cases
    }

    pub fn requirement_kinds(&self) -> Vec<ForgeQueryGraphReadAccessRequirementKind> {
        self.cases
            .iter()
            .map(|case| case.requirement_kind().clone())
            .collect()
    }

    pub fn case_for_requirement_kind(
        &self,
        requirement_kind: &ForgeQueryGraphReadAccessRequirementKind,
    ) -> Option<&ForgeQueryGraphReadAccessCase> {
        self.cases
            .iter()
            .find(|case| case.requirement_kind() == requirement_kind)
    }

    pub(crate) fn exhaustive() -> Self {
        Self::from_cases(
            ForgeQueryGraphReadAccessRequirementKind::all()
                .iter()
                .cloned()
                .map(ForgeQueryGraphReadAccessCase::for_requirement_kind)
                .collect(),
        )
    }

    pub(crate) fn from_cases(cases: Vec<ForgeQueryGraphReadAccessCase>) -> Self {
        let digest = hash_parts(
            &std::iter::once("forge_query_graph_read_access_case_registry_v1".to_string())
                .chain(cases.iter().map(ForgeQueryGraphReadAccessCase::digest_part))
                .collect::<Vec<_>>(),
        );
        Self { digest, cases }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadAccessInventoryMatch {
    requirement_kind: ForgeQueryGraphReadAccessRequirementKind,
    required_capability_owner: ForgeQueryGraphReadRequiredCapabilityOwner,
    resolved_posture: ForgeQueryGraphReadAccessAdmissionPosture,
}

impl ForgeQueryGraphReadAccessInventoryMatch {
    pub fn requirement_kind(&self) -> &ForgeQueryGraphReadAccessRequirementKind {
        &self.requirement_kind
    }

    pub fn required_capability_owner(&self) -> &ForgeQueryGraphReadRequiredCapabilityOwner {
        &self.required_capability_owner
    }

    pub fn resolved_posture(&self) -> &ForgeQueryGraphReadAccessAdmissionPosture {
        &self.resolved_posture
    }

    pub(crate) fn from_graph_index_match_report(
        report: &ForgeQueryGraphIndexInventoryMatchReport,
    ) -> Vec<Self> {
        report
            .matches()
            .iter()
            .map(|row| Self {
                requirement_kind: row.requirement_kind().clone(),
                required_capability_owner: row.required_capability_owner().clone(),
                resolved_posture: row.resolved_admission_posture().clone(),
            })
            .collect()
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "inventory_match:{}:{}:{}",
            self.requirement_kind.as_str(),
            self.required_capability_owner.as_str(),
            self.resolved_posture.as_str()
        )
    }
}

pub(crate) fn inventory_match_digest(
    inventory_matches: &[ForgeQueryGraphReadAccessInventoryMatch],
) -> String {
    hash_parts(
        &std::iter::once("forge_query_graph_read_access_inventory_match_v1".to_string())
            .chain(
                inventory_matches
                    .iter()
                    .map(ForgeQueryGraphReadAccessInventoryMatch::digest_part),
            )
            .collect::<Vec<_>>(),
    )
}

fn access_case_posture_and_owner(
    requirement_kind: &ForgeQueryGraphReadAccessRequirementKind,
) -> (
    ForgeQueryGraphReadAccessAdmissionPosture,
    ForgeQueryGraphReadRequiredCapabilityOwner,
) {
    match requirement_kind {
        ForgeQueryGraphReadAccessRequirementKind::DirectionalAdjacency
        | ForgeQueryGraphReadAccessRequirementKind::ReverseAdjacency
        | ForgeQueryGraphReadAccessRequirementKind::TraversalWorkset
        | ForgeQueryGraphReadAccessRequirementKind::VisitedSet
        | ForgeQueryGraphReadAccessRequirementKind::DedupSet
        | ForgeQueryGraphReadAccessRequirementKind::PredicateSupport
        | ForgeQueryGraphReadAccessRequirementKind::OrderingSupport
        | ForgeQueryGraphReadAccessRequirementKind::ProofSupport
        | ForgeQueryGraphReadAccessRequirementKind::ResultBuffer
        | ForgeQueryGraphReadAccessRequirementKind::MaterializationLifecycle => (
            ForgeQueryGraphReadAccessAdmissionPosture::InlineIndexed,
            ForgeQueryGraphReadRequiredCapabilityOwner::QueryRuntime,
        ),
        ForgeQueryGraphReadAccessRequirementKind::LiveMaintenanceSupport
        | ForgeQueryGraphReadAccessRequirementKind::DomainOperationCapabilityRegistration => (
            ForgeQueryGraphReadAccessAdmissionPosture::AccessCapabilityRegistrationRequired,
            ForgeQueryGraphReadRequiredCapabilityOwner::DomainRegistration,
        ),
    }
}
