use crate::runtime::{
    AuthoredAppearanceStateProp, AuthoredFlowLayoutProp, WorthUiAppearanceStateAdmissionStatus,
    WorthUiFlowLayoutAdmissionStatus, WorthUiFlowLayoutReceipt,
    WorthUiLiveViewProjectionAdmissionDenial, WorthUiRuntimeHost,
    WorthUiStatefulAppearanceRecipeReceipt,
};

use super::super::WorthUiAuthoredLiveViewDeclaration;

pub(crate) struct WorthUiLiveViewPrimitiveBinding {
    pub(crate) flow_layout: WorthUiFlowLayoutReceipt,
    pub(crate) appearance: WorthUiStatefulAppearanceRecipeReceipt,
}

pub(crate) fn lower_live_view_primitive_binding(
    runtime: &WorthUiRuntimeHost,
    authored: &WorthUiAuthoredLiveViewDeclaration,
) -> WorthUiLiveViewPrimitiveBinding {
    let subject_id = live_view_subject_id(authored.live_view_id());
    let authored_digest = live_view_primitive_authored_digest(authored);
    let flow_report = runtime.admit_flow_layout_props_for_subject(
        &subject_id,
        live_view_flow_props(authored),
        authored_digest,
    );
    let appearance_report = runtime.admit_appearance_state_props_for_subject(
        &subject_id,
        live_view_appearance_props(authored),
        authored_digest,
    );
    WorthUiLiveViewPrimitiveBinding {
        flow_layout: flow_report
            .status()
            .accepted_receipt()
            .expect("live-view primitive flow layout was admitted before lowering")
            .resolved_receipt(),
        appearance: appearance_report
            .status()
            .accepted_receipt()
            .expect("live-view primitive appearance was admitted before lowering")
            .resolved_receipt(),
    }
}

pub(crate) fn lower_live_view_default_primitive_binding(
    runtime: &WorthUiRuntimeHost,
    live_view_id: &str,
) -> WorthUiLiveViewPrimitiveBinding {
    let subject_id = live_view_subject_id(live_view_id);
    let authored_digest =
        crate::runtime::live_view::digest::digest_parts(std::iter::empty::<&str>());
    let flow_report =
        runtime.admit_flow_layout_props_for_subject(&subject_id, Vec::new(), authored_digest);
    let appearance_report =
        runtime.admit_appearance_state_props_for_subject(&subject_id, Vec::new(), authored_digest);
    WorthUiLiveViewPrimitiveBinding {
        flow_layout: flow_report
            .status()
            .accepted_receipt()
            .expect("default live-view primitive flow layout must admit")
            .resolved_receipt(),
        appearance: appearance_report
            .status()
            .accepted_receipt()
            .expect("default live-view primitive appearance must admit")
            .resolved_receipt(),
    }
}

pub(crate) fn append_live_view_primitive_denials(
    runtime: &WorthUiRuntimeHost,
    authored: &WorthUiAuthoredLiveViewDeclaration,
    denials: &mut Vec<WorthUiLiveViewProjectionAdmissionDenial>,
) {
    let subject_id = live_view_subject_id(authored.live_view_id());
    let authored_digest = live_view_primitive_authored_digest(authored);
    append_flow_denials(runtime, authored, &subject_id, authored_digest, denials);
    append_appearance_denials(runtime, authored, &subject_id, authored_digest, denials);
}

fn append_flow_denials(
    runtime: &WorthUiRuntimeHost,
    authored: &WorthUiAuthoredLiveViewDeclaration,
    subject_id: &str,
    authored_digest: u64,
    denials: &mut Vec<WorthUiLiveViewProjectionAdmissionDenial>,
) {
    let report = runtime.admit_flow_layout_props_for_subject(
        subject_id,
        live_view_flow_props(authored),
        authored_digest,
    );
    let WorthUiFlowLayoutAdmissionStatus::Rejected(denial_set) = report.status() else {
        return;
    };
    denials.extend(denial_set.denials().iter().map(|denial| {
        WorthUiLiveViewProjectionAdmissionDenial::PrimitiveFlowLayout {
            live_view_id: authored.live_view_id().to_owned(),
            prop_key: denial.prop_key().to_owned(),
            raw_value: denial.raw_value().to_owned(),
            expected: denial.expected_shape().to_owned(),
            denial_digest: denial.denial_digest(),
        }
    }));
}

fn append_appearance_denials(
    runtime: &WorthUiRuntimeHost,
    authored: &WorthUiAuthoredLiveViewDeclaration,
    subject_id: &str,
    authored_digest: u64,
    denials: &mut Vec<WorthUiLiveViewProjectionAdmissionDenial>,
) {
    let report = runtime.admit_appearance_state_props_for_subject(
        subject_id,
        live_view_appearance_props(authored),
        authored_digest,
    );
    let WorthUiAppearanceStateAdmissionStatus::Rejected(denial_set) = report.status() else {
        return;
    };
    denials.extend(denial_set.denials().iter().map(|denial| {
        WorthUiLiveViewProjectionAdmissionDenial::PrimitiveAppearanceState {
            live_view_id: authored.live_view_id().to_owned(),
            prop_key: denial.prop_key().to_owned(),
            raw_value: denial.raw_value().to_owned(),
            expected: denial.expected_shape().to_owned(),
            denial_digest: denial.denial_digest(),
        }
    }));
}

fn live_view_flow_props(
    authored: &WorthUiAuthoredLiveViewDeclaration,
) -> Vec<AuthoredFlowLayoutProp> {
    authored
        .primitive_props()
        .iter()
        .filter(|prop| prop.key().starts_with("flow_"))
        .map(|prop| AuthoredFlowLayoutProp::new(prop.key(), prop.value(), prop.source_span()))
        .collect()
}

fn live_view_appearance_props(
    authored: &WorthUiAuthoredLiveViewDeclaration,
) -> Vec<AuthoredAppearanceStateProp> {
    authored
        .primitive_props()
        .iter()
        .filter(|prop| prop.key().starts_with("appearance_"))
        .map(|prop| AuthoredAppearanceStateProp::new(prop.key(), prop.value(), prop.source_span()))
        .collect()
}

fn live_view_subject_id(live_view_id: &str) -> String {
    format!("{live_view_id}:mounted_view")
}

fn live_view_primitive_authored_digest(authored: &WorthUiAuthoredLiveViewDeclaration) -> u64 {
    crate::runtime::live_view::digest::digest_parts(
        authored
            .primitive_props()
            .iter()
            .flat_map(|prop| [prop.key().to_owned(), prop.value().to_owned()]),
    )
}
