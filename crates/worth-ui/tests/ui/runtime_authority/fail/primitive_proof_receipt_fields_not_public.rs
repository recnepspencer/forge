use worth_ui::facade::{
    WorthUiFlowLayoutReceipt, WorthUiPrimitiveAppearanceReceipt,
    WorthUiPrimitiveConstructionGraphProof, WorthUiPrimitiveContainerReceipt,
    WorthUiPrimitiveContentReceipt, WorthUiPrimitiveEventGeometryReceipt,
    WorthUiPrimitiveInteractionReceipt, WorthUiPrimitiveMeasurementReceipt,
    WorthUiPrimitiveMotionReceipt, WorthUiPrimitiveProofReceipt,
    WorthUiStatefulAppearanceRecipeReceipt,
};

fn main() {
    let _forged = WorthUiPrimitiveProofReceipt {
        surface_id: "worth.surface.preview.primitive.proof".to_owned(),
        component_id: "worth.component.primitive_proof".to_owned(),
        container: container(),
        measurement: measurement(),
        content: content(),
        appearance: appearance(),
        appearance_state: appearance_state(),
        interaction: interaction(),
        event_geometry: event_geometry(),
        motion: motion(),
        flow_layout: flow_layout(),
        construction_graph_proof: construction_graph_proof(),
        receipt_digest: 1,
    };
}

fn container() -> WorthUiPrimitiveContainerReceipt {
    panic!("fixture only checks primitive receipt field privacy")
}

fn content() -> WorthUiPrimitiveContentReceipt {
    panic!("fixture only checks primitive receipt field privacy")
}

fn appearance() -> WorthUiPrimitiveAppearanceReceipt {
    panic!("fixture only checks primitive receipt field privacy")
}

fn measurement() -> WorthUiPrimitiveMeasurementReceipt {
    panic!("fixture only checks primitive receipt field privacy")
}

fn appearance_state() -> WorthUiStatefulAppearanceRecipeReceipt {
    panic!("fixture only checks primitive receipt field privacy")
}

fn interaction() -> WorthUiPrimitiveInteractionReceipt {
    panic!("fixture only checks primitive receipt field privacy")
}

fn event_geometry() -> WorthUiPrimitiveEventGeometryReceipt {
    panic!("fixture only checks primitive receipt field privacy")
}

fn motion() -> WorthUiPrimitiveMotionReceipt {
    panic!("fixture only checks primitive receipt field privacy")
}

fn flow_layout() -> WorthUiFlowLayoutReceipt {
    panic!("fixture only checks primitive receipt field privacy")
}

fn construction_graph_proof() -> WorthUiPrimitiveConstructionGraphProof {
    panic!("fixture only checks primitive receipt field privacy")
}
