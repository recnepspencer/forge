use std::collections::BTreeSet;

use crate::workload_platform::projection_fact_parity::{
    projection_fact_parity_denial_evidence_identity, ProjectionFactParityDenialKind,
    ProjectionFactParityLane, ProjectionFactParityReceipt,
};
use crate::workload_platform::user_response::{
    WorthUserOutcome, WorthUserOutcomeCauseKind, WorthUserOutcomeKind,
};

use super::{
    denial::{ProjectionPolicyChoiceDenial, ProjectionPolicyChoiceDenialKind},
    lane_choice::ProjectionPolicyLaneChoice,
    receipt::ProjectionPolicyChoiceReceipt,
};

pub struct ProjectionPolicyChoiceMatrix<'a> {
    parity: &'a ProjectionFactParityReceipt,
    outcomes: Vec<(ProjectionFactParityLane, &'a WorthUserOutcome)>,
}

impl<'a> ProjectionPolicyChoiceMatrix<'a> {
    pub fn from_parity_receipt(parity: &'a ProjectionFactParityReceipt) -> Self {
        Self {
            parity,
            outcomes: Vec::new(),
        }
    }

    pub fn with_policy_required_outcome(
        mut self,
        lane: ProjectionFactParityLane,
        outcome: &'a WorthUserOutcome,
    ) -> Self {
        self.outcomes.push((lane, outcome));
        self
    }

    pub fn compile(self) -> Result<ProjectionPolicyChoiceReceipt, ProjectionPolicyChoiceDenial> {
        if self.outcomes.is_empty() {
            return Err(ProjectionPolicyChoiceDenial::new(
                ProjectionPolicyChoiceDenialKind::EmptyPolicyMatrix,
                None,
                "Projection policy choice matrix must include at least one policy-required lane.",
            ));
        }
        let mut lane_choices = Vec::with_capacity(self.outcomes.len());
        let mut seen_lanes = BTreeSet::new();
        for (lane, outcome) in self.outcomes {
            if !seen_lanes.insert(lane) {
                return Err(ProjectionPolicyChoiceDenial::new(
                    ProjectionPolicyChoiceDenialKind::DuplicatePolicyLane,
                    Some(lane),
                    format!(
                        "Projection policy choice matrix cannot repeat the {}.",
                        lane.human_name()
                    ),
                ));
            }
            validate_policy_outcome_for_lane(lane, self.parity, outcome)?;
            let lane_evidence = self.parity.evidence_for_lane(lane).ok_or_else(|| {
                ProjectionPolicyChoiceDenial::new(
                    ProjectionPolicyChoiceDenialKind::MissingPolicyLane,
                    Some(lane),
                    format!(
                        "Projection policy choice requires parity evidence for the {}.",
                        lane.human_name()
                    ),
                )
            })?;
            if outcome.choices().is_empty() {
                return Err(ProjectionPolicyChoiceDenial::new(
                    ProjectionPolicyChoiceDenialKind::MissingUserChoices,
                    Some(lane),
                    format!(
                        "Projection policy choice for the {} must expose user choices.",
                        lane.human_name()
                    ),
                ));
            }
            lane_choices.push(ProjectionPolicyLaneChoice::new(
                lane,
                outcome.choices().to_vec(),
                lane_evidence.parity_basis_identity(),
                outcome.evidence().digest(),
                outcome.evidence().source_identity(),
            ));
        }
        Ok(ProjectionPolicyChoiceReceipt::new(
            lane_choices,
            self.parity.workload_basis_identity(),
        ))
    }
}

fn validate_policy_outcome_for_lane(
    lane: ProjectionFactParityLane,
    parity: &ProjectionFactParityReceipt,
    outcome: &WorthUserOutcome,
) -> Result<(), ProjectionPolicyChoiceDenial> {
    if outcome.kind() != WorthUserOutcomeKind::PolicyRequired
        || outcome.cause().map(|cause| cause.kind())
            != Some(WorthUserOutcomeCauseKind::PolicyRequired)
    {
        return Err(ProjectionPolicyChoiceDenial::new(
            ProjectionPolicyChoiceDenialKind::OutcomeWasNotPolicyRequired,
            None,
            "Projection policy choice must start from a policy-required user outcome.",
        ));
    }
    if !outcome
        .human_response()
        .summary()
        .contains(lane.human_name())
    {
        return Err(ProjectionPolicyChoiceDenial::new(
            ProjectionPolicyChoiceDenialKind::MissingPolicyLane,
            Some(lane),
            format!(
                "Projection policy choice response must name the {}.",
                lane.human_name()
            ),
        ));
    }
    let expected_source_identity = expected_policy_required_source_identity(
        lane,
        parity.workload_basis_identity(),
        outcome.human_response().summary(),
    );
    if outcome.evidence().source_identity() != expected_source_identity {
        return Err(ProjectionPolicyChoiceDenial::new(
            ProjectionPolicyChoiceDenialKind::MismatchedPolicyOutcomeEvidence,
            Some(lane),
            format!(
                "Projection policy choice for the {} must consume policy evidence from the same parity basis.",
                lane.human_name()
            ),
        ));
    }
    Ok(())
}

fn expected_policy_required_source_identity(
    lane: ProjectionFactParityLane,
    workload_basis_identity: &str,
    human_reason: &str,
) -> String {
    projection_fact_parity_denial_evidence_identity(
        ProjectionFactParityDenialKind::PolicyRequired,
        Some(lane),
        workload_basis_identity,
        human_reason,
    )
}
