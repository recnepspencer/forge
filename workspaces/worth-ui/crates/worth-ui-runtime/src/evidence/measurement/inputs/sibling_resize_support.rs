use crate::capability::{
    CapabilitySnapshot, MosaicMeasurementAuthority, MosaicResizePermission, MosaicSizingContractId,
};
use crate::declaration::stable_text_digest;
use crate::graph::UiGraphNodeIdentity;
use crate::runtime::WorthUiAdmittedDurableResizeInput;

use crate::evidence::UiConstraintAxisScope;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMeasurementSiblingResizeSupportSource {
    MosaicSizingCapabilitySnapshot,
    RuntimeDurableResizeWitness,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMeasurementSiblingResizeSupport {
    axis_scope: UiConstraintAxisScope,
    target_graph_node_identity: UiGraphNodeIdentity,
    sizing_contract_id: Option<MosaicSizingContractId>,
    source: UiMeasurementSiblingResizeSupportSource,
    source_identity_digest: u64,
    planning_time_only: bool,
    identity_digest: u64,
}

impl UiMeasurementSiblingResizeSupport {
    pub fn from_mosaic_sizing_snapshot(
        snapshot: &CapabilitySnapshot,
        target_graph_node_identity: UiGraphNodeIdentity,
        sizing_contract_id: &MosaicSizingContractId,
    ) -> Option<Self> {
        let descriptor = snapshot.mosaic_sizing_contracts().get(sizing_contract_id)?;
        let admits_resize = descriptor.measurement_authority()
            == Some(&MosaicMeasurementAuthority::RuntimeToken)
            && descriptor.resize_permission() == Some(&MosaicResizePermission::UserResizable);
        admits_resize.then(|| Self {
            axis_scope: UiConstraintAxisScope::Both,
            target_graph_node_identity,
            sizing_contract_id: Some(sizing_contract_id.clone()),
            source: UiMeasurementSiblingResizeSupportSource::MosaicSizingCapabilitySnapshot,
            source_identity_digest: snapshot.digest().as_u64(),
            planning_time_only: false,
            identity_digest: stable_text_digest(
                "worth-ui.measurement-sibling-resize-support.mosaic-capability",
            ) ^ snapshot.digest().as_u64().rotate_left(7)
                ^ target_graph_node_identity.digest().rotate_left(13)
                ^ stable_text_digest(sizing_contract_id.as_str()).rotate_left(19),
        })
    }

    pub fn from_runtime_durable_resize_input(
        input: &WorthUiAdmittedDurableResizeInput,
        target_graph_node_identity: UiGraphNodeIdentity,
        axis_scope: UiConstraintAxisScope,
        sizing_contract_id: Option<&MosaicSizingContractId>,
    ) -> Option<Self> {
        if input.resize_permission() != &MosaicResizePermission::UserResizable {
            return None;
        }
        Some(Self {
            axis_scope,
            target_graph_node_identity,
            sizing_contract_id: sizing_contract_id.cloned(),
            source: UiMeasurementSiblingResizeSupportSource::RuntimeDurableResizeWitness,
            source_identity_digest: input.identity_digest(),
            planning_time_only: input.is_planning_time_only(),
            identity_digest: stable_text_digest(
                "worth-ui.measurement-sibling-resize-support.runtime-durable-resize",
            ) ^ input.identity_digest().rotate_left(7)
                ^ target_graph_node_identity.digest().rotate_left(13)
                ^ sizing_contract_id
                    .map(|value| stable_text_digest(value.as_str()))
                    .unwrap_or_else(|| {
                        stable_text_digest(
                            "worth-ui.measurement-sibling-resize-support.no-contract",
                        )
                    })
                    .rotate_left(19)
                ^ axis_scope_identity_digest(axis_scope).rotate_left(23),
        })
    }

    pub fn axis_scope(&self) -> UiConstraintAxisScope {
        self.axis_scope
    }

    pub fn target_graph_node_identity(&self) -> UiGraphNodeIdentity {
        self.target_graph_node_identity
    }

    pub fn sizing_contract_id(&self) -> Option<&MosaicSizingContractId> {
        self.sizing_contract_id.as_ref()
    }

    pub fn source(&self) -> UiMeasurementSiblingResizeSupportSource {
        self.source
    }

    pub fn source_identity_digest(&self) -> u64 {
        self.source_identity_digest
    }

    pub fn is_planning_time_only(&self) -> bool {
        self.planning_time_only
    }

    pub fn identity_digest(&self) -> u64 {
        self.identity_digest
    }
}

fn axis_scope_identity_digest(axis_scope: UiConstraintAxisScope) -> u64 {
    stable_text_digest(match axis_scope {
        UiConstraintAxisScope::Primary => "worth-ui.measurement-sibling-resize-support.primary",
        UiConstraintAxisScope::Cross => "worth-ui.measurement-sibling-resize-support.cross",
        UiConstraintAxisScope::Both => "worth-ui.measurement-sibling-resize-support.both",
    })
}
