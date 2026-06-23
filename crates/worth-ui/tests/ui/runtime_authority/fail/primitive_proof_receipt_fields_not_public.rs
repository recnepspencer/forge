use worth_ui::facade::{
    WorthUiPrimitiveAppearanceReceipt, WorthUiPrimitiveContainerReceipt,
    WorthUiPrimitiveContentReceipt, WorthUiPrimitiveProofReceipt,
    WorthUiValidatedProjectionDependencyContract,
};

fn main() {
    let _forged = WorthUiPrimitiveProofReceipt {
        surface_id: "worth.surface.preview.primitive.proof".to_owned(),
        component_id: "worth.component.primitive_proof".to_owned(),
        container: container(),
        content: content(),
        appearance: appearance(),
        dependency_contract: dependency_contract(),
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

fn dependency_contract() -> WorthUiValidatedProjectionDependencyContract {
    panic!("fixture only checks primitive receipt field privacy")
}
