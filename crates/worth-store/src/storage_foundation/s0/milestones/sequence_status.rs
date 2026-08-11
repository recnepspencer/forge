use super::super::evidence::S0EvidenceRef;
use super::validation::{
    reject_duplicate_declarations, require_non_empty, S0MilestoneAuditRejection,
};
use std::collections::BTreeMap;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum MilestoneSpecStatus {
    Planned,
    InProgress,
    Closed,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum MilestoneCloseoutStatus {
    Missing,
    Planned,
    Closed,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum MilestoneSequenceInconsistency {
    SpecCloseoutStatusMismatch,
    ClosedWithUnclosedPrerequisite,
    MissingGatePredecessorEvidence,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum PrerequisiteWaiverRationale {
    SemanticDocumentationDrift,
    IntentionalOutOfOrder,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct MilestonePrerequisiteEdge {
    milestone_id: String,
    prerequisite_milestone_id: String,
    waiver_rationale: Option<PrerequisiteWaiverRationale>,
}

impl MilestonePrerequisiteEdge {
    pub fn new(
        milestone_id: impl Into<String>,
        prerequisite_milestone_id: impl Into<String>,
    ) -> Result<Self, S0MilestoneAuditRejection> {
        let milestone_id = require_non_empty(milestone_id)?;
        let prerequisite_milestone_id = require_non_empty(prerequisite_milestone_id)?;
        Ok(Self {
            milestone_id,
            prerequisite_milestone_id,
            waiver_rationale: None,
        })
    }

    pub fn waived(mut self, rationale: PrerequisiteWaiverRationale) -> Self {
        self.waiver_rationale = Some(rationale);
        self
    }

    pub fn milestone_id(&self) -> &str {
        &self.milestone_id
    }

    pub fn prerequisite_milestone_id(&self) -> &str {
        &self.prerequisite_milestone_id
    }

    pub fn waiver_rationale(&self) -> Option<PrerequisiteWaiverRationale> {
        self.waiver_rationale
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct MilestoneStatusDeclaration {
    milestone_id: String,
    spec_status: MilestoneSpecStatus,
    closeout_status: MilestoneCloseoutStatus,
    predecessor_evidence: Vec<S0EvidenceRef>,
}

impl MilestoneStatusDeclaration {
    pub fn new(
        milestone_id: impl Into<String>,
        spec_status: MilestoneSpecStatus,
        closeout_status: MilestoneCloseoutStatus,
        predecessor_evidence: Vec<S0EvidenceRef>,
    ) -> Result<Self, S0MilestoneAuditRejection> {
        let milestone_id = require_non_empty(milestone_id)?;
        Ok(Self {
            milestone_id,
            spec_status,
            closeout_status,
            predecessor_evidence,
        })
    }

    pub fn milestone_id(&self) -> &str {
        &self.milestone_id
    }

    pub fn spec_status(&self) -> MilestoneSpecStatus {
        self.spec_status
    }

    pub fn closeout_status(&self) -> MilestoneCloseoutStatus {
        self.closeout_status
    }

    pub fn predecessor_evidence(&self) -> &[S0EvidenceRef] {
        &self.predecessor_evidence
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct RoadmapSequenceStatusMatrix {
    declarations: Vec<MilestoneStatusDeclaration>,
    prerequisite_edges: Vec<MilestonePrerequisiteEdge>,
    inconsistencies: Vec<(String, MilestoneSequenceInconsistency)>,
}

impl RoadmapSequenceStatusMatrix {
    pub fn new(
        declarations: Vec<MilestoneStatusDeclaration>,
        prerequisite_edges: Vec<MilestonePrerequisiteEdge>,
    ) -> Result<Self, S0MilestoneAuditRejection> {
        if declarations.is_empty() {
            return Err(S0MilestoneAuditRejection::MissingMilestoneDeclaration);
        }
        reject_duplicate_declarations(&declarations)?;
        let declaration_map = declarations
            .iter()
            .map(|declaration| (declaration.milestone_id(), declaration))
            .collect::<BTreeMap<_, _>>();
        let mut inconsistencies = Vec::new();

        for declaration in &declarations {
            if declaration.spec_status() == MilestoneSpecStatus::Planned
                && declaration.closeout_status() == MilestoneCloseoutStatus::Closed
            {
                inconsistencies.push((
                    declaration.milestone_id().to_string(),
                    MilestoneSequenceInconsistency::SpecCloseoutStatusMismatch,
                ));
            }
            if declaration.closeout_status() == MilestoneCloseoutStatus::Closed
                && declaration.predecessor_evidence().is_empty()
            {
                inconsistencies.push((
                    declaration.milestone_id().to_string(),
                    MilestoneSequenceInconsistency::MissingGatePredecessorEvidence,
                ));
            }
        }

        for edge in &prerequisite_edges {
            let declaration = declaration_map
                .get(edge.milestone_id())
                .ok_or(S0MilestoneAuditRejection::UnknownMilestoneReference)?;
            let prerequisite = declaration_map
                .get(edge.prerequisite_milestone_id())
                .ok_or(S0MilestoneAuditRejection::UnknownMilestoneReference)?;
            if declaration.closeout_status() == MilestoneCloseoutStatus::Closed
                && prerequisite.closeout_status() != MilestoneCloseoutStatus::Closed
                && edge.waiver_rationale().is_none()
            {
                inconsistencies.push((
                    declaration.milestone_id().to_string(),
                    MilestoneSequenceInconsistency::ClosedWithUnclosedPrerequisite,
                ));
            }
        }

        Ok(Self {
            declarations,
            prerequisite_edges,
            inconsistencies,
        })
    }

    pub fn declarations(&self) -> &[MilestoneStatusDeclaration] {
        &self.declarations
    }

    pub fn prerequisite_edges(&self) -> &[MilestonePrerequisiteEdge] {
        &self.prerequisite_edges
    }

    pub fn inconsistencies(&self) -> &[(String, MilestoneSequenceInconsistency)] {
        &self.inconsistencies
    }

    pub fn unwaived_inconsistency_count(&self) -> u64 {
        self.inconsistencies.len() as u64
    }

    pub fn gate_readiness_witness(
        &self,
        milestone_id: &str,
    ) -> Result<RoadmapGateReadinessWitness, S0MilestoneAuditRejection> {
        let declaration = self
            .declarations
            .iter()
            .find(|declaration| declaration.milestone_id() == milestone_id)
            .ok_or(S0MilestoneAuditRejection::UnknownMilestoneReference)?;
        if self
            .inconsistencies
            .iter()
            .any(|(id, _)| id == milestone_id)
        {
            return Err(S0MilestoneAuditRejection::GateReadinessBlockedBySequenceInconsistency);
        }
        if declaration.predecessor_evidence().is_empty() {
            return Err(S0MilestoneAuditRejection::MissingGatePredecessorEvidence);
        }
        Ok(RoadmapGateReadinessWitness {
            milestone_id: declaration.milestone_id().to_string(),
            predecessor_evidence_count: declaration.predecessor_evidence().len() as u64,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct RoadmapGateReadinessWitness {
    milestone_id: String,
    predecessor_evidence_count: u64,
}

impl RoadmapGateReadinessWitness {
    pub(crate) fn new(milestone_id: String, predecessor_evidence_count: u64) -> Self {
        Self {
            milestone_id,
            predecessor_evidence_count,
        }
    }

    pub fn milestone_id(&self) -> &str {
        &self.milestone_id
    }

    pub fn predecessor_evidence_count(&self) -> u64 {
        self.predecessor_evidence_count
    }
}
