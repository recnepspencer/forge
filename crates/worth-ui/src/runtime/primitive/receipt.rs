use crate::runtime::{
    WorthUiPrimitiveProofTargetBinding, WorthUiProjectionDependencyValidationProof,
    WorthUiRuntimeFactId,
};

use super::{
    WorthUiFlowLayoutReceipt, WorthUiPrimitiveAppearanceReceipt,
    WorthUiPrimitiveConstructionGraphProof, WorthUiPrimitiveContainerReceipt,
    WorthUiPrimitiveContentReceipt, WorthUiPrimitiveDrawPlan, WorthUiPrimitiveEventGeometryReceipt,
    WorthUiPrimitiveInteractionReceipt, WorthUiPrimitiveMeasurementReceipt,
    WorthUiPrimitiveMotionReceipt, WorthUiPrimitiveObservedPostureReceipt,
    WorthUiPrimitivePaintPlan, WorthUiStatefulAppearanceRecipeReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveProofReceipt {
    surface_id: String,
    component_id: String,
    container: WorthUiPrimitiveContainerReceipt,
    measurement: WorthUiPrimitiveMeasurementReceipt,
    content: WorthUiPrimitiveContentReceipt,
    appearance: WorthUiPrimitiveAppearanceReceipt,
    appearance_state: WorthUiStatefulAppearanceRecipeReceipt,
    interaction: WorthUiPrimitiveInteractionReceipt,
    event_geometry: WorthUiPrimitiveEventGeometryReceipt,
    motion: WorthUiPrimitiveMotionReceipt,
    flow_layout: WorthUiFlowLayoutReceipt,
    target_binding: WorthUiPrimitiveProofTargetBinding,
    construction_graph_proof: WorthUiPrimitiveConstructionGraphProof,
    receipt_digest: u64,
}

impl WorthUiPrimitiveProofReceipt {
    pub(crate) fn new(
        surface_id: impl Into<String>,
        component_id: impl Into<String>,
        container: WorthUiPrimitiveContainerReceipt,
        measurement: WorthUiPrimitiveMeasurementReceipt,
        content: WorthUiPrimitiveContentReceipt,
        appearance: WorthUiPrimitiveAppearanceReceipt,
        appearance_state: WorthUiStatefulAppearanceRecipeReceipt,
        interaction: WorthUiPrimitiveInteractionReceipt,
        event_geometry: WorthUiPrimitiveEventGeometryReceipt,
        motion: WorthUiPrimitiveMotionReceipt,
        flow_layout: WorthUiFlowLayoutReceipt,
        target_binding: WorthUiPrimitiveProofTargetBinding,
        construction_graph_proof: WorthUiPrimitiveConstructionGraphProof,
        receipt_digest: u64,
    ) -> Self {
        Self {
            surface_id: surface_id.into(),
            component_id: component_id.into(),
            container,
            measurement,
            content,
            appearance,
            appearance_state,
            interaction,
            event_geometry,
            motion,
            flow_layout,
            target_binding,
            construction_graph_proof,
            receipt_digest,
        }
    }

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn component_id(&self) -> &str {
        &self.component_id
    }

    pub fn container(&self) -> &WorthUiPrimitiveContainerReceipt {
        &self.container
    }

    pub fn measurement(&self) -> &WorthUiPrimitiveMeasurementReceipt {
        &self.measurement
    }

    pub fn content(&self) -> &WorthUiPrimitiveContentReceipt {
        &self.content
    }

    pub fn appearance(&self) -> &WorthUiPrimitiveAppearanceReceipt {
        &self.appearance
    }

    pub fn appearance_state(&self) -> &WorthUiStatefulAppearanceRecipeReceipt {
        &self.appearance_state
    }

    pub fn interaction(&self) -> &WorthUiPrimitiveInteractionReceipt {
        &self.interaction
    }

    pub fn event_geometry(&self) -> &WorthUiPrimitiveEventGeometryReceipt {
        &self.event_geometry
    }

    pub fn motion(&self) -> &WorthUiPrimitiveMotionReceipt {
        &self.motion
    }

    pub fn flow_layout(&self) -> &WorthUiFlowLayoutReceipt {
        &self.flow_layout
    }

    pub fn target_binding(&self) -> &WorthUiPrimitiveProofTargetBinding {
        &self.target_binding
    }

    pub fn dependency_fact(&self) -> &WorthUiRuntimeFactId {
        self.construction_graph_proof
            .dependency_contract()
            .dependencies()
            .facts()
            .next()
            .expect("primitive proof dependency contract is non-empty")
    }

    pub fn dependency_facts(&self) -> impl Iterator<Item = &WorthUiRuntimeFactId> {
        self.construction_graph_proof
            .dependency_contract()
            .dependencies()
            .facts()
    }

    pub fn dependency_proof(&self) -> WorthUiProjectionDependencyValidationProof {
        self.construction_graph_proof
            .dependency_contract()
            .validation_proof()
    }

    pub fn construction_graph_proof(&self) -> &WorthUiPrimitiveConstructionGraphProof {
        &self.construction_graph_proof
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }

    pub fn draw_plan(
        &self,
        available_width: f32,
        available_height: f32,
    ) -> WorthUiPrimitiveDrawPlan {
        WorthUiPrimitiveDrawPlan::from_receipt(self.clone(), available_width, available_height)
    }

    pub fn paint_plan(
        &self,
        available_width: f32,
        available_height: f32,
        observed_posture: WorthUiPrimitiveObservedPostureReceipt,
    ) -> WorthUiPrimitivePaintPlan {
        WorthUiPrimitivePaintPlan::from_receipt(
            self.clone(),
            available_width,
            available_height,
            observed_posture,
        )
    }
}
