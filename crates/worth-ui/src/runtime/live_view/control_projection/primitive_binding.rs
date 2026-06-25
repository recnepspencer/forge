use crate::runtime::{
    AuthoredAppearanceStateProp, AuthoredEventGeometryProp, AuthoredFlowLayoutProp,
    WorthUiAppearanceStateAdmissionStatus, WorthUiEventGeometryAdmissionStatus,
    WorthUiFlowLayoutAdmissionStatus, WorthUiFlowLayoutReceipt, WorthUiLiveViewDeclarationReceipt,
    WorthUiPrimitiveEventGeometryReceipt, WorthUiRuntimeHost,
    WorthUiStatefulAppearanceRecipeReceipt,
};

use super::{WorthUiLiveViewControlProjectionDeclaration, WorthUiLiveViewControlProjectionDenial};

pub(crate) struct WorthUiLiveViewControlPrimitiveBinding {
    pub(crate) flow_layout: WorthUiFlowLayoutReceipt,
    pub(crate) appearance: WorthUiStatefulAppearanceRecipeReceipt,
    pub(crate) event_geometry: WorthUiPrimitiveEventGeometryReceipt,
}

pub(crate) fn lower_control_primitive_binding(
    runtime: &WorthUiRuntimeHost,
    live_view_id: &str,
    declaration: &WorthUiLiveViewControlProjectionDeclaration,
) -> WorthUiLiveViewControlPrimitiveBinding {
    let subject_id = control_primitive_subject_id(live_view_id, declaration.control_id());
    let authored_digest = control_primitive_authored_digest(declaration);
    let flow_report = runtime.admit_flow_layout_props_for_subject(
        &subject_id,
        control_flow_props(declaration),
        authored_digest,
    );
    let appearance_report = runtime.admit_appearance_state_props_for_subject(
        &subject_id,
        control_appearance_props(declaration),
        authored_digest,
    );
    let event_report = runtime.admit_event_geometry_props_for_subject(
        &subject_id,
        control_event_props(declaration),
        authored_digest,
    );
    WorthUiLiveViewControlPrimitiveBinding {
        flow_layout: flow_report
            .status()
            .accepted_receipt()
            .expect("control primitive flow layout was admitted before lowering")
            .resolved_receipt(),
        appearance: appearance_report
            .status()
            .accepted_receipt()
            .expect("control primitive appearance was admitted before lowering")
            .resolved_receipt(),
        event_geometry: event_report
            .status()
            .accepted_receipt()
            .expect("control primitive event geometry was admitted before lowering")
            .resolved_receipt(),
    }
}

pub(crate) fn append_control_primitive_denials(
    runtime: &WorthUiRuntimeHost,
    live_view: &WorthUiLiveViewDeclarationReceipt,
    declaration: &WorthUiLiveViewControlProjectionDeclaration,
    denials: &mut Vec<WorthUiLiveViewControlProjectionDenial>,
) {
    let subject_id =
        control_primitive_subject_id(live_view.live_view_id(), declaration.control_id());
    let authored_digest = control_primitive_authored_digest(declaration);
    append_flow_denials(runtime, declaration, &subject_id, authored_digest, denials);
    append_appearance_denials(runtime, declaration, &subject_id, authored_digest, denials);
    append_event_denials(runtime, declaration, &subject_id, authored_digest, denials);
}

pub(crate) fn control_primitives_have_denial(
    runtime: &WorthUiRuntimeHost,
    live_view: &WorthUiLiveViewDeclarationReceipt,
    declaration: &WorthUiLiveViewControlProjectionDeclaration,
) -> bool {
    let subject_id =
        control_primitive_subject_id(live_view.live_view_id(), declaration.control_id());
    let authored_digest = control_primitive_authored_digest(declaration);
    runtime
        .admit_flow_layout_props_for_subject(
            &subject_id,
            control_flow_props(declaration),
            authored_digest,
        )
        .status()
        .denial_set()
        .is_some()
        || runtime
            .admit_appearance_state_props_for_subject(
                &subject_id,
                control_appearance_props(declaration),
                authored_digest,
            )
            .status()
            .denial_set()
            .is_some()
        || runtime
            .admit_event_geometry_props_for_subject(
                &subject_id,
                control_event_props(declaration),
                authored_digest,
            )
            .status()
            .denial_set()
            .is_some()
}

fn append_flow_denials(
    runtime: &WorthUiRuntimeHost,
    declaration: &WorthUiLiveViewControlProjectionDeclaration,
    subject_id: &str,
    authored_digest: u64,
    denials: &mut Vec<WorthUiLiveViewControlProjectionDenial>,
) {
    let report = runtime.admit_flow_layout_props_for_subject(
        subject_id,
        control_flow_props(declaration),
        authored_digest,
    );
    let WorthUiFlowLayoutAdmissionStatus::Rejected(denial_set) = report.status() else {
        return;
    };
    denials.extend(denial_set.denials().iter().map(|denial| {
        WorthUiLiveViewControlProjectionDenial::PrimitiveFlowLayout {
            control_id: declaration.control_id().to_owned(),
            prop_key: denial.prop_key().to_owned(),
            raw_value: denial.raw_value().to_owned(),
            expected: denial.expected_shape().to_owned(),
            denial_digest: denial.denial_digest(),
        }
    }));
}

fn append_appearance_denials(
    runtime: &WorthUiRuntimeHost,
    declaration: &WorthUiLiveViewControlProjectionDeclaration,
    subject_id: &str,
    authored_digest: u64,
    denials: &mut Vec<WorthUiLiveViewControlProjectionDenial>,
) {
    let report = runtime.admit_appearance_state_props_for_subject(
        subject_id,
        control_appearance_props(declaration),
        authored_digest,
    );
    let WorthUiAppearanceStateAdmissionStatus::Rejected(denial_set) = report.status() else {
        return;
    };
    denials.extend(denial_set.denials().iter().map(|denial| {
        WorthUiLiveViewControlProjectionDenial::PrimitiveAppearanceState {
            control_id: declaration.control_id().to_owned(),
            prop_key: denial.prop_key().to_owned(),
            raw_value: denial.raw_value().to_owned(),
            expected: denial.expected_shape().to_owned(),
            denial_digest: denial.denial_digest(),
        }
    }));
}

fn append_event_denials(
    runtime: &WorthUiRuntimeHost,
    declaration: &WorthUiLiveViewControlProjectionDeclaration,
    subject_id: &str,
    authored_digest: u64,
    denials: &mut Vec<WorthUiLiveViewControlProjectionDenial>,
) {
    let report = runtime.admit_event_geometry_props_for_subject(
        subject_id,
        control_event_props(declaration),
        authored_digest,
    );
    let WorthUiEventGeometryAdmissionStatus::Rejected(denial_set) = report.status() else {
        return;
    };
    denials.extend(denial_set.denials().iter().map(|denial| {
        WorthUiLiveViewControlProjectionDenial::PrimitiveEventGeometry {
            control_id: declaration.control_id().to_owned(),
            prop_key: denial.prop_key().to_owned(),
            raw_value: denial.raw_value().to_owned(),
            expected: denial.expected_shape().to_owned(),
            denial_digest: denial.denial_digest(),
        }
    }));
}

fn control_flow_props(
    declaration: &WorthUiLiveViewControlProjectionDeclaration,
) -> Vec<AuthoredFlowLayoutProp> {
    declaration
        .primitive_props()
        .iter()
        .filter(|prop| prop.key().starts_with("flow_"))
        .map(|prop| AuthoredFlowLayoutProp::new(prop.key(), prop.value(), prop.source_span()))
        .collect()
}

fn control_appearance_props(
    declaration: &WorthUiLiveViewControlProjectionDeclaration,
) -> Vec<AuthoredAppearanceStateProp> {
    declaration
        .primitive_props()
        .iter()
        .filter(|prop| prop.key().starts_with("appearance_"))
        .map(|prop| AuthoredAppearanceStateProp::new(prop.key(), prop.value(), prop.source_span()))
        .collect()
}

fn control_event_props(
    declaration: &WorthUiLiveViewControlProjectionDeclaration,
) -> Vec<AuthoredEventGeometryProp> {
    declaration
        .primitive_props()
        .iter()
        .filter(|prop| prop.key().starts_with("event_"))
        .map(|prop| AuthoredEventGeometryProp::new(prop.key(), prop.value(), prop.source_span()))
        .collect()
}

fn control_primitive_subject_id(live_view_id: &str, control_id: &str) -> String {
    format!("{live_view_id}:{control_id}")
}

fn control_primitive_authored_digest(
    declaration: &WorthUiLiveViewControlProjectionDeclaration,
) -> u64 {
    crate::runtime::live_view::digest::digest_parts(
        declaration
            .primitive_props()
            .iter()
            .flat_map(|prop| [prop.key().to_owned(), prop.value().to_owned()]),
    )
}
