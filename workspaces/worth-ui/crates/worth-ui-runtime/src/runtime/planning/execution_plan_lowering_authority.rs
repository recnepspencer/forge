use crate::runtime::planning::execution_plan_input::WorthUiExecutionPlanInputPreparer;
use crate::runtime::{
    UiCommittedAllocationLoweringInput, WorthUiExecutionPlanInput, WorthUiPendingActivation,
    WorthUiPlanLoweringDenial, WorthUiPlanNodeInput, WorthUiRuntimeFrameEpoch,
};

#[derive(Debug)]
pub(crate) enum WorthUiExecutionPlanLoweringAuthorityDenial {
    CandidateGraphAuthorityMismatch,
    CandidateArtifactAuthorityMismatch,
    ForeignAllocationProjection,
    MissingQueryPosture,
    UnexpectedQueryPosture,
    QueryDefinitionNotInstalled,
    ForeignQueryInstalledAuthority,
    RegionalDelta(crate::runtime::planning::plan_topology::WorthUiPlanRegionDeltaDenial),
    PlanInput(WorthUiPlanLoweringDenial),
}

/// Move-only post-commit authority for execution-plan construction.
///
/// Allocation truth and canonical plan facts meet here for the first time.
/// Allocation planning cannot construct this type and committed allocation
/// cannot reconstruct its plan input.
#[derive(Debug)]
pub(crate) struct WorthUiExecutionPlanLoweringAuthority {
    source: WorthUiExecutionPlanLoweringSource,
    facts: WorthUiExecutionPlanLoweringFacts,
}

#[derive(Debug)]
enum WorthUiExecutionPlanLoweringSource {
    Launch(crate::runtime::planning::allocation_planning::WorthUiInitialAllocationCommit),
    Replacement {
        pending_activation: Box<WorthUiPendingActivation>,
        committed_input: Option<Box<UiCommittedAllocationLoweringInput>>,
    },
}

/// Borrow-only constituent consumed by plan-construction algorithms after the
/// owning authority has admitted the phase transition.
#[derive(Debug)]
pub(crate) struct WorthUiExecutionPlanLoweringFacts {
    candidate_application_authority:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationLoweringAuthority,
    plan_input: WorthUiExecutionPlanInput,
    identity: super::WorthUiExecutionPlanLoweringIdentity,
    allocation_identity_digest: u64,
    region_delta: Option<crate::runtime::planning::plan_topology::WorthUiPlanRegionDelta>,
    #[cfg(test)]
    committed_input: Option<UiCommittedAllocationLoweringInput>,
}

impl WorthUiExecutionPlanLoweringAuthority {
    pub(crate) fn seal(
        pending_activation: WorthUiPendingActivation,
        committed_input: UiCommittedAllocationLoweringInput,
        active_frame_epoch: WorthUiRuntimeFrameEpoch,
    ) -> Result<Self, WorthUiExecutionPlanLoweringAuthorityDenial> {
        let successor = crate::runtime::allocation_catalog_successor::UiAllocationCatalogSuccessorLoweringInput::seal(
            &pending_activation,
            committed_input
                .receipt()
                .committed_allocation()
                .allocation_identity_digest(),
        );
        Self::seal_successor(
            pending_activation,
            successor,
            active_frame_epoch,
            Some(committed_input),
        )
    }

    pub(crate) fn seal_catalog_successor(
        pending_activation: WorthUiPendingActivation,
        successor: crate::runtime::allocation_catalog_successor::UiAllocationCatalogSuccessorLoweringInput,
        active_frame_epoch: WorthUiRuntimeFrameEpoch,
    ) -> Result<Self, WorthUiExecutionPlanLoweringAuthorityDenial> {
        Self::seal_successor(pending_activation, successor, active_frame_epoch, None)
    }

    fn seal_successor(
        pending_activation: WorthUiPendingActivation,
        successor: crate::runtime::allocation_catalog_successor::UiAllocationCatalogSuccessorLoweringInput,
        active_frame_epoch: WorthUiRuntimeFrameEpoch,
        committed_input: Option<UiCommittedAllocationLoweringInput>,
    ) -> Result<Self, WorthUiExecutionPlanLoweringAuthorityDenial> {
        let candidate_application_authority =
            pending_activation.candidate_application_authority().clone();
        if candidate_application_authority.graph_authority_identity()
            != pending_activation
                .allocation_planning_projection()
                .graph_authority_identity()
        {
            return Err(
                WorthUiExecutionPlanLoweringAuthorityDenial::CandidateGraphAuthorityMismatch,
            );
        }
        if !pending_activation
            .allocation_planning_projection()
            .shares_authority_with(successor.projection())
        {
            return Err(WorthUiExecutionPlanLoweringAuthorityDenial::ForeignAllocationProjection);
        }
        let plan_input = WorthUiExecutionPlanInputPreparer::prepare(
            &pending_activation,
            active_frame_epoch,
            &[],
            candidate_application_authority.query_binding_plan(),
        )
        .map_err(WorthUiExecutionPlanLoweringAuthorityDenial::PlanInput)?;
        validate_query_constituents(&candidate_application_authority, &plan_input)?;
        let allocation_identity_digest = successor.allocation_identity_digest();
        let region_delta =
            crate::runtime::planning::plan_topology::WorthUiPlanRegionDelta::from_replacement(
                pending_activation.staged_replacement().node_plan(),
                &plan_input,
                pending_activation
                    .staged_replacement()
                    .admitted_candidate()
                    .active_basis()
                    .active_plan_digest(),
                allocation_identity_digest,
            )
            .map_err(WorthUiExecutionPlanLoweringAuthorityDenial::RegionalDelta)?;
        #[cfg(test)]
        let test_committed_input = committed_input.clone();
        Ok(Self {
            source: WorthUiExecutionPlanLoweringSource::Replacement {
                pending_activation: Box::new(pending_activation),
                committed_input: committed_input.map(Box::new),
            },
            facts: WorthUiExecutionPlanLoweringFacts {
                candidate_application_authority,
                plan_input,
                identity: super::WorthUiExecutionPlanLoweringIdentity::seal(),
                allocation_identity_digest,
                region_delta: Some(region_delta),
                #[cfg(test)]
                committed_input: test_committed_input,
            },
        })
    }

    pub(crate) fn seal_launch(
        candidate_application_authority: crate::facade::prepared_application_authority::WorthUiPreparedApplicationLoweringAuthority,
        initial_allocation_commit: crate::runtime::planning::allocation_planning::WorthUiInitialAllocationCommit,
        artifact: &crate::source::WorthUiArtifact,
        artifact_digest: crate::source::WorthUiArtifactDigest,
        frame_epoch: WorthUiRuntimeFrameEpoch,
    ) -> Result<Self, WorthUiExecutionPlanLoweringAuthorityDenial> {
        if candidate_application_authority.graph_authority_identity()
            != initial_allocation_commit
                .projection()
                .graph_authority_identity()
        {
            return Err(
                WorthUiExecutionPlanLoweringAuthorityDenial::CandidateGraphAuthorityMismatch,
            );
        }
        if !candidate_application_authority.admits_launch_artifact(artifact, artifact_digest) {
            return Err(
                WorthUiExecutionPlanLoweringAuthorityDenial::CandidateArtifactAuthorityMismatch,
            );
        }
        let plan_input = WorthUiExecutionPlanInputPreparer::prepare_launch(
            artifact,
            artifact_digest,
            frame_epoch,
            candidate_application_authority.query_binding_plan(),
        );
        validate_query_constituents(&candidate_application_authority, &plan_input)?;
        let allocation_identity_digest = initial_allocation_commit.allocation_identity_digest();
        Ok(Self {
            source: WorthUiExecutionPlanLoweringSource::Launch(initial_allocation_commit),
            facts: WorthUiExecutionPlanLoweringFacts {
                candidate_application_authority,
                plan_input,
                identity: super::WorthUiExecutionPlanLoweringIdentity::seal(),
                allocation_identity_digest,
                region_delta: None,
                #[cfg(test)]
                committed_input: None,
            },
        })
    }

    pub(crate) fn facts(&self) -> &WorthUiExecutionPlanLoweringFacts {
        &self.facts
    }

    pub(crate) fn into_replacement_parts(
        self,
    ) -> (
        WorthUiPendingActivation,
        Option<UiCommittedAllocationLoweringInput>,
        WorthUiExecutionPlanInput,
    ) {
        let WorthUiExecutionPlanLoweringSource::Replacement {
            pending_activation,
            committed_input,
        } = self.source
        else {
            panic!("launch lowering authority cannot enter replacement activation")
        };
        (
            *pending_activation,
            committed_input.map(|input| *input),
            self.facts.plan_input,
        )
    }

    pub(crate) fn finish_launch(self) {
        match self.source {
            WorthUiExecutionPlanLoweringSource::Launch(commit) => drop(commit),
            WorthUiExecutionPlanLoweringSource::Replacement { .. } => {
                panic!("replacement lowering authority cannot complete launch")
            }
        }
    }
}

impl WorthUiExecutionPlanLoweringFacts {
    pub(crate) fn candidate_application_authority(
        &self,
    ) -> &crate::facade::prepared_application_authority::WorthUiPreparedApplicationLoweringAuthority
    {
        &self.candidate_application_authority
    }

    pub(crate) fn allocation_identity_digest(&self) -> u64 {
        self.allocation_identity_digest
    }

    #[cfg(test)]
    pub(crate) fn committed_input(&self) -> &UiCommittedAllocationLoweringInput {
        self.committed_input
            .as_ref()
            .expect("replacement test facts retain committed input")
    }

    pub(crate) fn plan_input(&self) -> &WorthUiExecutionPlanInput {
        &self.plan_input
    }

    pub(crate) fn node_inputs(&self) -> &[WorthUiPlanNodeInput] {
        self.plan_input.node_inputs()
    }

    pub(crate) fn identity(&self) -> &super::WorthUiExecutionPlanLoweringIdentity {
        &self.identity
    }

    pub(crate) fn region_delta(
        &self,
    ) -> Option<&crate::runtime::planning::plan_topology::WorthUiPlanRegionDelta> {
        self.region_delta.as_ref()
    }
}

fn validate_query_constituents(
    authority: &crate::facade::prepared_application_authority::WorthUiPreparedApplicationLoweringAuthority,
    plan_input: &WorthUiExecutionPlanInput,
) -> Result<(), WorthUiExecutionPlanLoweringAuthorityDenial> {
    for node_input in plan_input.node_inputs() {
        let is_query_row =
            node_input.family() == crate::runtime::WorthUiPlanNodeInputFamily::QueryViewBinding;
        match (
            is_query_row,
            node_input.query_binding_identity(),
            node_input.query_installed_reference(),
            node_input.query_binding_posture(),
        ) {
            (true, Some(identity), Some(reference), Some(_)) => {
                if reference.definition().identity() != identity.query_view_identity()
                    || reference.definition().shape() != identity.result_shape()
                {
                    return Err(
                        WorthUiExecutionPlanLoweringAuthorityDenial::QueryDefinitionNotInstalled,
                    );
                }
                if !authority.query_binding_plan().admits_reference(reference) {
                    return Err(
                        WorthUiExecutionPlanLoweringAuthorityDenial::ForeignQueryInstalledAuthority,
                    );
                }
            }
            (true, Some(_), Some(_), None) | (true, None, None, None) => {
                return Err(WorthUiExecutionPlanLoweringAuthorityDenial::MissingQueryPosture);
            }
            (true, Some(_), None, Some(_)) | (true, Some(_), None, None) => {
                return Err(
                    WorthUiExecutionPlanLoweringAuthorityDenial::QueryDefinitionNotInstalled,
                );
            }
            (false, Some(_), _, _)
            | (false, _, Some(_), _)
            | (false, _, _, Some(_))
            | (true, None, _, Some(_))
            | (true, None, Some(_), _) => {
                return Err(WorthUiExecutionPlanLoweringAuthorityDenial::UnexpectedQueryPosture);
            }
            (false, None, None, None) => {}
        }
    }
    Ok(())
}

#[cfg(test)]
pub(crate) mod test_support {
    use super::WorthUiExecutionPlanLoweringFacts;
    use crate::runtime::{UiCommittedAllocationLoweringInput, WorthUiExecutionPlanInput};

    pub(crate) fn facts_below_authority(
        candidate_application_authority: crate::facade::prepared_application_authority::WorthUiPreparedApplicationLoweringAuthority,
        committed_input: UiCommittedAllocationLoweringInput,
        plan_input: WorthUiExecutionPlanInput,
    ) -> WorthUiExecutionPlanLoweringFacts {
        WorthUiExecutionPlanLoweringFacts {
            candidate_application_authority,
            plan_input,
            identity: crate::runtime::planning::WorthUiExecutionPlanLoweringIdentity::seal(),
            allocation_identity_digest: committed_input
                .receipt()
                .committed_allocation()
                .allocation_identity_digest(),
            region_delta: None,
            committed_input: Some(committed_input),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{validate_query_constituents, WorthUiExecutionPlanLoweringAuthorityDenial};
    use crate::runtime::{WorthUiExecutionPlanInput, WorthUiPlanNodeInputFamily};

    #[test]
    fn semantically_equal_foreign_query_installation_cannot_authorize_lowering() {
        let inputs =
            crate::runtime::tests::activation_staging_test_support::activation_staging_inputs();
        let plan_input = inputs.reconstructive_plan_input(&[]);
        let (app, runtime, pending) = inputs.into_app_runtime_and_pending();
        let authority = app.prepared_authority().lowering_authority();
        let _ = (runtime, pending);

        let foreign_domain = worth_ui_query_binding::certification::worth_ui_installed_test_domain(
            "foreign-lowering-query-domain",
        );
        let foreign_view = foreign_domain
            .live_measurement_view("workspace.view_binding.selection")
            .expect("foreign semantically equal view installs");
        let foreign_plan = worth_ui_query_binding::WorthUiQueryBindingPlan::default()
            .register_view(foreign_view)
            .expect("foreign binding plan seals");
        let query_identity = plan_input
            .node_inputs()
            .iter()
            .find(|input| input.family() == WorthUiPlanNodeInputFamily::QueryViewBinding)
            .and_then(|input| input.query_binding_identity())
            .expect("fixture carries a Query plan row");
        let foreign_reference = foreign_plan
            .resolve_definition(
                query_identity.query_view_identity(),
                query_identity.result_shape(),
            )
            .expect("foreign plan resolves the same semantic definition");
        let foreign_node_inputs = plan_input
            .node_inputs()
            .iter()
            .cloned()
            .map(|input| {
                if input.family() == WorthUiPlanNodeInputFamily::QueryViewBinding {
                    input.with_query_installed_reference_for_test(foreign_reference.clone())
                } else {
                    input
                }
            })
            .collect();
        let foreign_input = WorthUiExecutionPlanInput::new(
            plan_input.basis().clone(),
            plan_input.context().clone(),
            foreign_node_inputs,
            plan_input.counters(),
        );

        assert!(matches!(
            validate_query_constituents(&authority, &foreign_input),
            Err(WorthUiExecutionPlanLoweringAuthorityDenial::ForeignQueryInstalledAuthority)
        ));
    }
}
