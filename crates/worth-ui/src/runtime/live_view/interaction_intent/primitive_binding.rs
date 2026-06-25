use crate::runtime::{
    AuthoredAppearanceStateProp, AuthoredEventGeometryProp, AuthoredFlowLayoutProp,
    WorthUiAppearanceStateAdmissionStatus, WorthUiEventGeometryAdmissionStatus,
    WorthUiFlowLayoutAdmissionStatus, WorthUiFlowLayoutReceipt,
    WorthUiPrimitiveEventGeometryReceipt, WorthUiRuntimeHost,
    WorthUiStatefulAppearanceRecipeReceipt,
};

use super::{WorthUiLiveViewInteractionIntentDeclaration, WorthUiLiveViewInteractionIntentDenial};

pub(crate) struct WorthUiLiveViewInteractionPrimitiveBinding {
    pub(crate) flow_layout: WorthUiFlowLayoutReceipt,
    pub(crate) appearance: WorthUiStatefulAppearanceRecipeReceipt,
    pub(crate) event_geometry: WorthUiPrimitiveEventGeometryReceipt,
}

pub(crate) fn lower_interaction_primitive_binding(
    runtime: &WorthUiRuntimeHost,
    live_view_id: &str,
    declaration: &WorthUiLiveViewInteractionIntentDeclaration,
) -> WorthUiLiveViewInteractionPrimitiveBinding {
    let subject_id = interaction_primitive_subject_id(live_view_id, declaration.interaction_id());
    let authored_digest = interaction_primitive_authored_digest(declaration);
    let flow_report = runtime.admit_flow_layout_props_for_subject(
        &subject_id,
        interaction_flow_props(declaration),
        authored_digest,
    );
    let appearance_report = runtime.admit_appearance_state_props_for_subject(
        &subject_id,
        interaction_appearance_props(declaration),
        authored_digest,
    );
    let event_report = runtime.admit_event_geometry_props_for_subject(
        &subject_id,
        interaction_event_props(declaration),
        authored_digest,
    );
    WorthUiLiveViewInteractionPrimitiveBinding {
        flow_layout: flow_report
            .status()
            .accepted_receipt()
            .expect("interaction primitive flow layout was admitted before lowering")
            .resolved_receipt(),
        appearance: appearance_report
            .status()
            .accepted_receipt()
            .expect("interaction primitive appearance was admitted before lowering")
            .resolved_receipt(),
        event_geometry: event_report
            .status()
            .accepted_receipt()
            .expect("interaction primitive event geometry was admitted before lowering")
            .resolved_receipt(),
    }
}

pub(crate) fn append_interaction_primitive_denials(
    runtime: &WorthUiRuntimeHost,
    live_view_id: &str,
    declaration: &WorthUiLiveViewInteractionIntentDeclaration,
    denials: &mut Vec<WorthUiLiveViewInteractionIntentDenial>,
) {
    let subject_id = interaction_primitive_subject_id(live_view_id, declaration.interaction_id());
    let authored_digest = interaction_primitive_authored_digest(declaration);
    append_flow_denials(runtime, declaration, &subject_id, authored_digest, denials);
    append_appearance_denials(runtime, declaration, &subject_id, authored_digest, denials);
    append_event_denials(runtime, declaration, &subject_id, authored_digest, denials);
}

fn append_flow_denials(
    runtime: &WorthUiRuntimeHost,
    declaration: &WorthUiLiveViewInteractionIntentDeclaration,
    subject_id: &str,
    authored_digest: u64,
    denials: &mut Vec<WorthUiLiveViewInteractionIntentDenial>,
) {
    let report = runtime.admit_flow_layout_props_for_subject(
        subject_id,
        interaction_flow_props(declaration),
        authored_digest,
    );
    let WorthUiFlowLayoutAdmissionStatus::Rejected(denial_set) = report.status() else {
        return;
    };
    denials.extend(denial_set.denials().iter().map(|denial| {
        WorthUiLiveViewInteractionIntentDenial::PrimitiveFlowLayout {
            interaction_id: declaration.interaction_id().to_owned(),
            prop_key: denial.prop_key().to_owned(),
            raw_value: denial.raw_value().to_owned(),
            expected: denial.expected_shape().to_owned(),
            denial_digest: denial.denial_digest(),
        }
    }));
}

fn append_appearance_denials(
    runtime: &WorthUiRuntimeHost,
    declaration: &WorthUiLiveViewInteractionIntentDeclaration,
    subject_id: &str,
    authored_digest: u64,
    denials: &mut Vec<WorthUiLiveViewInteractionIntentDenial>,
) {
    let report = runtime.admit_appearance_state_props_for_subject(
        subject_id,
        interaction_appearance_props(declaration),
        authored_digest,
    );
    let WorthUiAppearanceStateAdmissionStatus::Rejected(denial_set) = report.status() else {
        return;
    };
    denials.extend(denial_set.denials().iter().map(|denial| {
        WorthUiLiveViewInteractionIntentDenial::PrimitiveAppearanceState {
            interaction_id: declaration.interaction_id().to_owned(),
            prop_key: denial.prop_key().to_owned(),
            raw_value: denial.raw_value().to_owned(),
            expected: denial.expected_shape().to_owned(),
            denial_digest: denial.denial_digest(),
        }
    }));
}

fn append_event_denials(
    runtime: &WorthUiRuntimeHost,
    declaration: &WorthUiLiveViewInteractionIntentDeclaration,
    subject_id: &str,
    authored_digest: u64,
    denials: &mut Vec<WorthUiLiveViewInteractionIntentDenial>,
) {
    let report = runtime.admit_event_geometry_props_for_subject(
        subject_id,
        interaction_event_props(declaration),
        authored_digest,
    );
    let WorthUiEventGeometryAdmissionStatus::Rejected(denial_set) = report.status() else {
        return;
    };
    denials.extend(denial_set.denials().iter().map(|denial| {
        WorthUiLiveViewInteractionIntentDenial::PrimitiveEventGeometry {
            interaction_id: declaration.interaction_id().to_owned(),
            prop_key: denial.prop_key().to_owned(),
            raw_value: denial.raw_value().to_owned(),
            expected: denial.expected_shape().to_owned(),
            denial_digest: denial.denial_digest(),
        }
    }));
}

fn interaction_flow_props(
    declaration: &WorthUiLiveViewInteractionIntentDeclaration,
) -> Vec<AuthoredFlowLayoutProp> {
    declaration
        .primitive_props()
        .iter()
        .filter(|prop| prop.key().starts_with("flow_"))
        .map(|prop| AuthoredFlowLayoutProp::new(prop.key(), prop.value(), prop.source_span()))
        .collect()
}

fn interaction_appearance_props(
    declaration: &WorthUiLiveViewInteractionIntentDeclaration,
) -> Vec<AuthoredAppearanceStateProp> {
    declaration
        .primitive_props()
        .iter()
        .filter(|prop| prop.key().starts_with("appearance_"))
        .map(|prop| AuthoredAppearanceStateProp::new(prop.key(), prop.value(), prop.source_span()))
        .collect()
}

fn interaction_event_props(
    declaration: &WorthUiLiveViewInteractionIntentDeclaration,
) -> Vec<AuthoredEventGeometryProp> {
    declaration
        .primitive_props()
        .iter()
        .filter(|prop| prop.key().starts_with("event_"))
        .map(|prop| AuthoredEventGeometryProp::new(prop.key(), prop.value(), prop.source_span()))
        .collect()
}

fn interaction_primitive_subject_id(live_view_id: &str, interaction_id: &str) -> String {
    format!("{live_view_id}:{interaction_id}")
}

fn interaction_primitive_authored_digest(
    declaration: &WorthUiLiveViewInteractionIntentDeclaration,
) -> u64 {
    crate::runtime::live_view::digest::digest_parts(
        declaration
            .primitive_props()
            .iter()
            .flat_map(|prop| [prop.key().to_owned(), prop.value().to_owned()]),
    )
}
