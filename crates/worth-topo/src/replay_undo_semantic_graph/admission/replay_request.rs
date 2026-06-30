use crate::derived_invalidation_execution::DerivedInvalidationExecutionReceipt;
use crate::derived_invalidation_migrated_products::{
    MaterializedGraphReadStageReceipt, TraversalViewsReadStageReceipt,
};
use crate::derived_invalidation_selected_plan::DerivedInvalidationTouchedClosure;
use crate::replay_family_catalog::{
    TopologyReplayFamilyIdentity, TopologyReplayFamilyIdentityAuthority,
};

#[derive(Clone, Copy, Debug)]
pub enum TopologyReplaySemanticGraphStageReceiptAuthority<'a> {
    TraversalViews(&'a TraversalViewsReadStageReceipt),
    MaterializedGraph(&'a MaterializedGraphReadStageReceipt),
}

impl<'a> TopologyReplaySemanticGraphStageReceiptAuthority<'a> {
    pub fn family_identity(&self) -> TopologyReplayFamilyIdentity {
        match self {
            Self::TraversalViews(_) => {
                TopologyReplayFamilyIdentityAuthority::traversal_views().identity()
            }
            Self::MaterializedGraph(_) => {
                TopologyReplayFamilyIdentityAuthority::materialized_graph().identity()
            }
        }
    }

    pub fn selected_plan_digest(&self) -> &str {
        match self {
            Self::TraversalViews(receipt) => receipt.selected_plan_digest(),
            Self::MaterializedGraph(receipt) => receipt.selected_plan_digest(),
        }
    }

    pub fn touched_closure_digest(&self) -> &str {
        match self {
            Self::TraversalViews(receipt) => receipt.touched_closure_digest(),
            Self::MaterializedGraph(receipt) => receipt.touched_closure_digest(),
        }
    }

    pub fn native_query_read_receipt_digest(&self) -> &str {
        match self {
            Self::TraversalViews(receipt) => receipt.native_query_read_receipt_digest(),
            Self::MaterializedGraph(receipt) => receipt.native_query_read_receipt_digest(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TopologyReplaySemanticGraphAdmissionRequest<'a> {
    family_identity: TopologyReplayFamilyIdentity,
    touched_closure: &'a DerivedInvalidationTouchedClosure,
    invalidation_receipt: &'a DerivedInvalidationExecutionReceipt,
    stage_receipt_authority: Option<TopologyReplaySemanticGraphStageReceiptAuthority<'a>>,
}

impl<'a> TopologyReplaySemanticGraphAdmissionRequest<'a> {
    pub fn new(
        family_identity: TopologyReplayFamilyIdentity,
        touched_closure: &'a DerivedInvalidationTouchedClosure,
        invalidation_receipt: &'a DerivedInvalidationExecutionReceipt,
        stage_receipt_authority: Option<TopologyReplaySemanticGraphStageReceiptAuthority<'a>>,
    ) -> Self {
        Self {
            family_identity,
            touched_closure,
            invalidation_receipt,
            stage_receipt_authority,
        }
    }

    pub const fn family_identity(&self) -> TopologyReplayFamilyIdentity {
        self.family_identity
    }

    pub const fn touched_closure(&self) -> &'a DerivedInvalidationTouchedClosure {
        self.touched_closure
    }

    pub const fn invalidation_receipt(&self) -> &'a DerivedInvalidationExecutionReceipt {
        self.invalidation_receipt
    }

    pub const fn stage_receipt_authority(
        &self,
    ) -> Option<TopologyReplaySemanticGraphStageReceiptAuthority<'a>> {
        self.stage_receipt_authority
    }
}
