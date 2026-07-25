use crate::runtime::planning::execution_plan_input::WorthUiExecutionPlanInputPreparer;
use crate::runtime::{
    UiCommittedAllocationLoweringInput, WorthUiExecutionPlanInput, WorthUiRuntimeFrameEpoch,
};

use super::{
    validate_query_constituents, WorthUiExecutionPlanLoweringAuthority,
    WorthUiExecutionPlanLoweringAuthorityDenial, WorthUiExecutionPlanLoweringFacts,
    WorthUiExecutionPlanLoweringSource,
};

impl WorthUiExecutionPlanLoweringAuthority {
    pub(crate) fn seal_mounted(
        basis: crate::runtime::WorthUiMountedAllocationActivationBasis,
        committed_input: UiCommittedAllocationLoweringInput,
        artifact: &crate::source::WorthUiArtifact,
        artifact_digest: crate::source::WorthUiArtifactDigest,
        active_frame_epoch: WorthUiRuntimeFrameEpoch,
        active_plan_digest: u64,
    ) -> Result<Self, WorthUiExecutionPlanLoweringAuthorityDenial> {
        let candidate_application_authority = basis.candidate_application_authority().clone();
        if candidate_application_authority.graph_authority_identity()
            != basis.projection().graph_authority_identity()
        {
            return Err(
                WorthUiExecutionPlanLoweringAuthorityDenial::CandidateGraphAuthorityMismatch,
            );
        }
        if basis.projection().frame_epoch() != active_frame_epoch
            || basis.projection().candidate_artifact_digest() != artifact_digest.raw()
        {
            return Err(WorthUiExecutionPlanLoweringAuthorityDenial::ForeignAllocationProjection);
        }
        let committed_projection = committed_input
            .receipt()
            .committed_allocation()
            .planning_projection();
        if !basis
            .projection()
            .shares_authority_with(committed_projection)
        {
            return Err(WorthUiExecutionPlanLoweringAuthorityDenial::ForeignAllocationProjection);
        }
        if !candidate_application_authority.admits_launch_artifact(artifact, artifact_digest) {
            return Err(
                WorthUiExecutionPlanLoweringAuthorityDenial::CandidateArtifactAuthorityMismatch,
            );
        }
        let plan_input = WorthUiExecutionPlanInputPreparer::prepare_launch(
            artifact,
            artifact_digest,
            active_frame_epoch,
            candidate_application_authority.query_binding_plan(),
        );
        validate_query_constituents(&candidate_application_authority, &plan_input)?;
        let allocation_identity_digest = committed_input
            .receipt()
            .committed_allocation()
            .allocation_identity_digest();
        let region_delta =
            crate::runtime::planning::plan_topology::WorthUiPlanRegionDelta::from_mounted(
                &plan_input,
                artifact_digest.raw(),
                active_plan_digest,
                allocation_identity_digest,
            )
            .map_err(WorthUiExecutionPlanLoweringAuthorityDenial::RegionalDelta)?;
        Ok(Self {
            source: WorthUiExecutionPlanLoweringSource::Mounted {
                basis: Box::new(basis),
                committed_input: Box::new(committed_input),
            },
            facts: WorthUiExecutionPlanLoweringFacts {
                candidate_application_authority,
                plan_input,
                identity: super::super::WorthUiExecutionPlanLoweringIdentity::seal(),
                allocation_identity_digest,
                region_delta: Some(region_delta),
                #[cfg(test)]
                committed_input: None,
            },
        })
    }

    pub(crate) fn into_mounted_parts(
        self,
    ) -> (
        crate::runtime::WorthUiMountedAllocationActivationBasis,
        UiCommittedAllocationLoweringInput,
        WorthUiExecutionPlanInput,
    ) {
        let WorthUiExecutionPlanLoweringSource::Mounted {
            basis,
            committed_input,
        } = self.source
        else {
            panic!("non-mounted lowering authority cannot enter mounted activation")
        };
        (*basis, *committed_input, self.facts.plan_input)
    }
}
