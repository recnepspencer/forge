use super::replay_request::{
    TopologyReplaySemanticGraphAdmissionRequest, TopologyReplaySemanticGraphStageReceiptAuthority,
};
use super::stage_authority::TopologyReplaySemanticGraphPreparedStageAuthority;
use super::stage_identity::TopologyReplaySemanticGraphStageIdentity;
use crate::derived_invalidation_execution::DerivedInvalidationExecutionReceipt;
use crate::derived_invalidation_selected_plan::DerivedInvalidationTouchedClosure;
use crate::replay_family_catalog::TopologyReplayFamilyIdentity;

#[derive(Clone, Debug)]
pub struct TopologyReplaySemanticGraphPreparationRequest<'a> {
    family_identity: TopologyReplayFamilyIdentity,
    touched_closure: &'a DerivedInvalidationTouchedClosure,
    invalidation_receipt: &'a DerivedInvalidationExecutionReceipt,
    stage_receipt_authority: Option<TopologyReplaySemanticGraphStageReceiptAuthority<'a>>,
    declared_stage_identity: Option<TopologyReplaySemanticGraphStageIdentity>,
}

#[derive(Clone, Debug)]
pub struct TopologyReplaySemanticGraphPreparedRequest<'a> {
    family_identity: TopologyReplayFamilyIdentity,
    touched_closure: &'a DerivedInvalidationTouchedClosure,
    invalidation_receipt: &'a DerivedInvalidationExecutionReceipt,
    stage_authority: Option<TopologyReplaySemanticGraphPreparedStageAuthority>,
    declared_stage_identity: Option<TopologyReplaySemanticGraphStageIdentity>,
}

impl<'a> TopologyReplaySemanticGraphPreparationRequest<'a> {
    pub fn new(
        family_identity: TopologyReplayFamilyIdentity,
        touched_closure: &'a DerivedInvalidationTouchedClosure,
        invalidation_receipt: &'a DerivedInvalidationExecutionReceipt,
        stage_receipt_authority: Option<TopologyReplaySemanticGraphStageReceiptAuthority<'a>>,
        declared_stage_identity: Option<TopologyReplaySemanticGraphStageIdentity>,
    ) -> Self {
        Self {
            family_identity,
            touched_closure,
            invalidation_receipt,
            stage_receipt_authority,
            declared_stage_identity,
        }
    }
}

impl<'a> TopologyReplaySemanticGraphPreparedRequest<'a> {
    pub const fn family_identity(&self) -> TopologyReplayFamilyIdentity {
        self.family_identity
    }

    pub const fn touched_closure(&self) -> &'a DerivedInvalidationTouchedClosure {
        self.touched_closure
    }

    pub const fn invalidation_receipt(&self) -> &'a DerivedInvalidationExecutionReceipt {
        self.invalidation_receipt
    }

    pub const fn stage_authority(
        &self,
    ) -> Option<&TopologyReplaySemanticGraphPreparedStageAuthority> {
        self.stage_authority.as_ref()
    }

    pub const fn declared_stage_identity(
        &self,
    ) -> Option<&TopologyReplaySemanticGraphStageIdentity> {
        self.declared_stage_identity.as_ref()
    }
}

pub fn prepare_topology_replay_semantic_graph_request<'a>(
    request: TopologyReplaySemanticGraphPreparationRequest<'a>,
) -> TopologyReplaySemanticGraphPreparedRequest<'a> {
    TopologyReplaySemanticGraphPreparedRequest {
        family_identity: request.family_identity,
        touched_closure: request.touched_closure,
        invalidation_receipt: request.invalidation_receipt,
        stage_authority: request
            .stage_receipt_authority
            .map(TopologyReplaySemanticGraphPreparedStageAuthority::prepare),
        declared_stage_identity: request.declared_stage_identity,
    }
}

pub fn prepare_topology_replay_semantic_graph_stage_identity(
    stage_receipt_authority: TopologyReplaySemanticGraphStageReceiptAuthority<'_>,
) -> TopologyReplaySemanticGraphStageIdentity {
    TopologyReplaySemanticGraphStageIdentity::from_stage_receipt_authority(stage_receipt_authority)
}

pub(crate) fn prepare_legacy_topology_replay_semantic_graph_request<'a>(
    request: TopologyReplaySemanticGraphAdmissionRequest<'a>,
) -> TopologyReplaySemanticGraphPreparedRequest<'a> {
    let declared_stage_identity = request
        .stage_receipt_authority()
        .map(prepare_topology_replay_semantic_graph_stage_identity);
    prepare_topology_replay_semantic_graph_request(
        TopologyReplaySemanticGraphPreparationRequest::new(
            request.family_identity(),
            request.touched_closure(),
            request.invalidation_receipt(),
            request.stage_receipt_authority(),
            declared_stage_identity,
        ),
    )
}
