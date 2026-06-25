use std::collections::BTreeSet;

use crate::runtime::{
    WorthUiLiveViewControlProjectionGraphPosture, WorthUiLiveViewDeclarationReceipt,
    WorthUiRuntimeFactId, WorthUiRuntimeHost,
};

use super::declaration::{
    WorthUiLiveViewControlOptionsSource, WorthUiLiveViewControlProjectionDeclaration,
    WorthUiLiveViewControlProjectionKind,
};
use super::denial::{
    WorthUiLiveViewControlProjectionAdmissionReport, WorthUiLiveViewControlProjectionDenial,
};
use super::primitive_binding::{
    append_control_primitive_denials, control_primitives_have_denial,
    lower_control_primitive_binding,
};
use super::receipt::{
    WorthUiLiveViewControlProjectionAdmissionCounters, WorthUiLiveViewControlProjectionReceipt,
};

impl WorthUiRuntimeHost {
    pub fn admit_live_view_control_projections(
        &self,
        live_view: &WorthUiLiveViewDeclarationReceipt,
        declarations: &[WorthUiLiveViewControlProjectionDeclaration],
    ) -> Result<
        Vec<WorthUiLiveViewControlProjectionReceipt>,
        WorthUiLiveViewControlProjectionAdmissionReport,
    > {
        let denials = control_projection_denials(self, live_view, declarations);
        if !denials.is_empty() {
            return Err(WorthUiLiveViewControlProjectionAdmissionReport::denied(
                denials,
            ));
        }
        Ok(lower_live_view_control_projection_receipts(
            self,
            live_view,
            declarations,
        ))
    }

    pub fn live_view_control_projection_admission_counters(
        &self,
        declarations: &[WorthUiLiveViewControlProjectionDeclaration],
        denial_count: usize,
    ) -> WorthUiLiveViewControlProjectionAdmissionCounters {
        WorthUiLiveViewControlProjectionAdmissionCounters::new(
            declarations.len(),
            declarations
                .iter()
                .filter(|declaration| declaration.options().is_some())
                .count(),
            denial_count,
        )
    }
}

pub(crate) fn control_projection_denials(
    runtime: &WorthUiRuntimeHost,
    live_view: &WorthUiLiveViewDeclarationReceipt,
    declarations: &[WorthUiLiveViewControlProjectionDeclaration],
) -> Vec<WorthUiLiveViewControlProjectionDenial> {
    let mut denials = Vec::new();
    let mut seen = BTreeSet::new();
    for declaration in declarations {
        if invalid_identity(declaration.control_id()) {
            denials.push(WorthUiLiveViewControlProjectionDenial::InvalidControlId {
                control_id: declaration.control_id().to_owned(),
            });
        }
        if !seen.insert(declaration.control_id().to_owned()) {
            denials.push(WorthUiLiveViewControlProjectionDenial::DuplicateControlId {
                control_id: declaration.control_id().to_owned(),
            });
        }
        if live_view.binding(declaration.binding_id()).is_none() {
            denials.push(WorthUiLiveViewControlProjectionDenial::UnknownBinding {
                control_id: declaration.control_id().to_owned(),
                binding_id: declaration.binding_id().to_owned(),
            });
        }
        if !declaration.kind().is_supported() {
            denials.push(
                WorthUiLiveViewControlProjectionDenial::UnsupportedProjectionKind {
                    control_id: declaration.control_id().to_owned(),
                    projection_kind: declaration.kind().token().to_owned(),
                },
            );
        } else if let Some(component_id) = declaration.kind().component_id() {
            if runtime
                .inspect_active_component_descriptor(&component_id)
                .is_none()
            {
                denials.push(
                    WorthUiLiveViewControlProjectionDenial::UnregisteredComponent {
                        control_id: declaration.control_id().to_owned(),
                        component_id: component_id.as_str().to_owned(),
                    },
                );
            }
        }
        append_option_denials(&mut denials, declaration);
        append_control_primitive_denials(runtime, live_view, declaration, &mut denials);
    }
    denials
}

pub(crate) fn control_has_denial(
    runtime: &WorthUiRuntimeHost,
    live_view: &WorthUiLiveViewDeclarationReceipt,
    declarations: &[WorthUiLiveViewControlProjectionDeclaration],
    control_id: &str,
) -> bool {
    let mut seen = BTreeSet::new();
    declarations.iter().any(|declaration| {
        let duplicate = !seen.insert(declaration.control_id().to_owned());
        declaration.control_id() == control_id
            && (invalid_identity(declaration.control_id())
                || duplicate
                || live_view.binding(declaration.binding_id()).is_none()
                || !declaration.kind().is_supported()
                || control_component_has_denial(runtime, declaration)
                || control_options_have_denial(declaration)
                || control_primitives_have_denial(runtime, live_view, declaration))
    })
}

pub(crate) fn lower_live_view_control_projection_receipts(
    runtime: &WorthUiRuntimeHost,
    live_view: &WorthUiLiveViewDeclarationReceipt,
    declarations: &[WorthUiLiveViewControlProjectionDeclaration],
) -> Vec<WorthUiLiveViewControlProjectionReceipt> {
    declarations
        .iter()
        .map(|declaration| {
            let binding = live_view
                .binding(declaration.binding_id())
                .expect("control projection binding was admitted before lowering")
                .clone();
            let component_id = declaration
                .kind()
                .component_id()
                .expect("control projection component was admitted before lowering");
            let dependency_facts = control_projection_dependency_facts(live_view, declaration);
            let primitive_binding =
                lower_control_primitive_binding(runtime, live_view.live_view_id(), declaration);
            let graph_execution = runtime
                .graph_authority()
                .plan_live_view_control_projection_graph_operation(
                    live_view.live_view_id(),
                    declaration.control_id(),
                    dependency_facts,
                    WorthUiLiveViewControlProjectionGraphPosture::Admitted,
                )
                .into_execution_receipt();
            WorthUiLiveViewControlProjectionReceipt::new(
                live_view.live_view_id(),
                declaration,
                component_id,
                binding,
                primitive_binding.flow_layout,
                primitive_binding.appearance,
                primitive_binding.event_geometry,
                graph_execution,
            )
        })
        .collect()
}

fn append_option_denials(
    denials: &mut Vec<WorthUiLiveViewControlProjectionDenial>,
    declaration: &WorthUiLiveViewControlProjectionDeclaration,
) {
    match declaration.kind() {
        WorthUiLiveViewControlProjectionKind::Select if declaration.options().is_none() => {
            denials.push(WorthUiLiveViewControlProjectionDenial::MissingOptions {
                control_id: declaration.control_id().to_owned(),
            });
        }
        _ => {}
    }
    if let Some(options) = declaration.options() {
        match options {
            WorthUiLiveViewControlOptionsSource::Unsupported(value) => {
                denials.push(
                    WorthUiLiveViewControlProjectionDenial::UnsupportedOptionSource {
                        control_id: declaration.control_id().to_owned(),
                        option_source: value.to_owned(),
                    },
                );
            }
            WorthUiLiveViewControlOptionsSource::Static { .. } => {}
        }
    }
}

fn control_options_have_denial(declaration: &WorthUiLiveViewControlProjectionDeclaration) -> bool {
    matches!(
        declaration.kind(),
        WorthUiLiveViewControlProjectionKind::Select
    ) && declaration.options().is_none()
        || matches!(
            declaration.options(),
            Some(WorthUiLiveViewControlOptionsSource::Unsupported(_))
        )
}

fn control_component_has_denial(
    runtime: &WorthUiRuntimeHost,
    declaration: &WorthUiLiveViewControlProjectionDeclaration,
) -> bool {
    declaration
        .kind()
        .component_id()
        .is_some_and(|component_id| {
            runtime
                .inspect_active_component_descriptor(&component_id)
                .is_none()
        })
}

fn control_projection_dependency_facts(
    live_view: &WorthUiLiveViewDeclarationReceipt,
    declaration: &WorthUiLiveViewControlProjectionDeclaration,
) -> Vec<WorthUiRuntimeFactId> {
    let mut facts = vec![
        WorthUiRuntimeFactId::live_view_declaration(live_view.live_view_id()),
        WorthUiRuntimeFactId::live_view_state_binding(format!(
            "{}:{}",
            live_view.live_view_id(),
            declaration.binding_id()
        )),
        WorthUiRuntimeFactId::live_view_control_projection(format!(
            "{}:{}",
            live_view.live_view_id(),
            declaration.control_id()
        )),
        WorthUiRuntimeFactId::primitive_flow_layout(control_primitive_fact_identity(
            live_view,
            declaration,
        )),
        WorthUiRuntimeFactId::primitive_appearance_state(control_primitive_fact_identity(
            live_view,
            declaration,
        )),
        WorthUiRuntimeFactId::primitive_event_geometry(control_primitive_fact_identity(
            live_view,
            declaration,
        )),
    ];
    if let Some(options) = declaration.options() {
        facts.push(WorthUiRuntimeFactId::live_view_control_options(format!(
            "{}:{}:{}",
            live_view.live_view_id(),
            declaration.control_id(),
            options.source_id()
        )));
    }
    facts
}

fn control_primitive_fact_identity(
    live_view: &WorthUiLiveViewDeclarationReceipt,
    declaration: &WorthUiLiveViewControlProjectionDeclaration,
) -> String {
    format!("{}:{}", live_view.live_view_id(), declaration.control_id())
}

fn invalid_identity(value: &str) -> bool {
    value.trim().is_empty() || value.chars().any(char::is_whitespace)
}
