use std::collections::BTreeMap;
use std::path::Path;

use crate::runtime::candidate::rust_authored_replacement_candidate;
use crate::runtime::replacement_impact_test_support::{admitted_candidate, launch_runtime};
use crate::runtime::{
    WorthUiAccessibilityInvalidation, WorthUiAdmittedReplacementCandidate,
    WorthUiCandidateAdmission, WorthUiImpactLookupCounters, WorthUiRuntimeHost,
    WorthUiRuntimeImpactNarrowing,
};
use crate::source::{
    WorthUiArtifact, WorthUiArtifactBindingHandle, WorthUiArtifactBindingNode,
    WorthUiArtifactComponentHandle, WorthUiArtifactComponentNode, WorthUiArtifactHandle,
    WorthUiArtifactIdentitySeed, WorthUiArtifactImportHandle, WorthUiArtifactImportNode,
    WorthUiArtifactModule, WorthUiArtifactNode, WorthUiArtifactSurfaceHandle,
    WorthUiArtifactSurfaceNode, WorthUiArtifactThemeTokenHandle, WorthUiArtifactThemeTokenNode,
    WorthUiBoundSurfaceSemantics, WorthUiDurableStateEligibility, WorthUiMosaicStructureFacts,
    WorthUiSourceModuleId,
};
use crate::{capability::*, facade::WorthUiApp};

pub(super) fn identity_match_app() -> WorthUiApp {
    crate::facade::WorthUi::app()
        .register_component(component("workspace.component.dashboard"))
        .register_component(component("workspace.component.panel"))
        .register_surface(surface(
            "workspace.surface.main",
            "workspace.component.dashboard",
        ))
        .register_surface(surface(
            "workspace.surface.secondary",
            "workspace.component.dashboard",
        ))
        .freeze()
}

pub(super) fn runtime_and_narrowing(
    app: &WorthUiApp,
    active_artifact: WorthUiArtifact,
    candidate_artifact: WorthUiArtifact,
) -> (
    WorthUiRuntimeHost,
    WorthUiAdmittedReplacementCandidate,
    WorthUiRuntimeImpactNarrowing,
) {
    let runtime = launch_runtime(app, active_artifact);
    let admitted = admitted_candidate(app, &runtime, candidate_artifact);
    let narrowing = identity_narrowing_for(&runtime, &admitted);
    (runtime, admitted, narrowing)
}

pub(super) fn admitted_with_runtime(
    app: &WorthUiApp,
    active_artifact: WorthUiArtifact,
    candidate_artifact: WorthUiArtifact,
) -> (WorthUiRuntimeHost, WorthUiAdmittedReplacementCandidate) {
    let runtime = launch_runtime(app, active_artifact);
    let candidate = rust_authored_replacement_candidate(
        candidate_artifact,
        app.capabilities().digest(),
        crate::runtime::WorthUiReplacementCause::manual_refresh(900),
    )
    .expect("candidate seals");
    let admitted =
        WorthUiCandidateAdmission::for_active_basis(runtime.replacement_admission_basis())
            .admit(candidate)
            .expect("candidate admits");
    (runtime, admitted)
}

pub(super) fn artifact_from_nodes<const N: usize>(
    modules: [(&str, Vec<WorthUiArtifactNode>); N],
) -> WorthUiArtifact {
    let modules = modules
        .into_iter()
        .map(|(path, nodes)| {
            let module_id = module_id(path);
            (
                module_id.clone(),
                WorthUiArtifactModule::new(module_id.clone(), rehandle_nodes(module_id, nodes)),
            )
        })
        .collect::<Vec<_>>();
    let module_order = modules
        .iter()
        .map(|(module_id, _)| module_id.clone())
        .collect::<Vec<_>>();
    WorthUiArtifact::new(BTreeMap::from_iter(modules), module_order)
}

pub(super) fn component_node(seed: &str, index: usize) -> WorthUiArtifactNode {
    let module_id = module_id("placeholder.wui");
    WorthUiArtifactNode::Component(WorthUiArtifactComponentNode::new(
        WorthUiArtifactHandle::Component(WorthUiArtifactComponentHandle::new(module_id, index)),
        AdmittedCapability::from_checked_id(
            ComponentId::new("workspace.component.dashboard").unwrap(),
        ),
        component("workspace.component.dashboard"),
        empty_structure(),
        0,
        WorthUiArtifactIdentitySeed::authored(seed.to_owned()),
        durable_eligible(),
    ))
}

pub(super) fn component_node_with_descriptor(
    seed: &str,
    component_id: &str,
    index: usize,
) -> WorthUiArtifactNode {
    let module_id = module_id("placeholder.wui");
    WorthUiArtifactNode::Component(WorthUiArtifactComponentNode::new(
        WorthUiArtifactHandle::Component(WorthUiArtifactComponentHandle::new(module_id, index)),
        AdmittedCapability::from_checked_id(ComponentId::new(component_id).unwrap()),
        component(component_id),
        empty_structure(),
        0,
        WorthUiArtifactIdentitySeed::authored(seed.to_owned()),
        durable_eligible(),
    ))
}

pub(super) fn surface_node(seed: &str, surface_id: &str, index: usize) -> WorthUiArtifactNode {
    let module_id = module_id("placeholder.wui");
    WorthUiArtifactNode::Surface(WorthUiArtifactSurfaceNode::new(
        WorthUiArtifactHandle::Surface(WorthUiArtifactSurfaceHandle::new(module_id, index)),
        AdmittedCapability::from_checked_id(SurfaceId::new(surface_id).unwrap()),
        surface(surface_id, "workspace.component.dashboard"),
        empty_structure(),
        WorthUiBoundSurfaceSemantics::default(),
        0,
        WorthUiArtifactIdentitySeed::authored(seed.to_owned()),
        durable_eligible(),
    ))
}

fn rehandle_nodes(
    module_id: WorthUiSourceModuleId,
    nodes: Vec<WorthUiArtifactNode>,
) -> Vec<WorthUiArtifactNode> {
    nodes
        .into_iter()
        .enumerate()
        .map(|(index, node)| rehandle_node(module_id.clone(), index, node))
        .collect()
}

fn rehandle_node(
    module_id: WorthUiSourceModuleId,
    index: usize,
    node: WorthUiArtifactNode,
) -> WorthUiArtifactNode {
    match node {
        WorthUiArtifactNode::Import(node) => {
            WorthUiArtifactNode::Import(WorthUiArtifactImportNode::new(
                WorthUiArtifactHandle::Import(WorthUiArtifactImportHandle::new(module_id, index)),
                node.target().clone(),
                node.authored_provenance_digest(),
                node.identity_seed().clone(),
                node.durable_state_eligibility().clone(),
            ))
        }
        WorthUiArtifactNode::Component(node) => {
            WorthUiArtifactNode::Component(WorthUiArtifactComponentNode::new(
                WorthUiArtifactHandle::Component(WorthUiArtifactComponentHandle::new(
                    module_id, index,
                )),
                node.component().clone(),
                node.descriptor().clone(),
                node.structure().clone(),
                node.authored_provenance_digest(),
                node.identity_seed().clone(),
                node.durable_state_eligibility().clone(),
            ))
        }
        WorthUiArtifactNode::Surface(node) => {
            WorthUiArtifactNode::Surface(WorthUiArtifactSurfaceNode::new(
                WorthUiArtifactHandle::Surface(WorthUiArtifactSurfaceHandle::new(module_id, index)),
                node.surface().clone(),
                node.descriptor().clone(),
                node.structure().clone(),
                node.semantics().clone(),
                node.authored_provenance_digest(),
                node.identity_seed().clone(),
                node.durable_state_eligibility().clone(),
            ))
        }
        WorthUiArtifactNode::Binding(node) => {
            WorthUiArtifactNode::Binding(WorthUiArtifactBindingNode::new(
                WorthUiArtifactHandle::Binding(WorthUiArtifactBindingHandle::new(module_id, index)),
                node.view_binding_reference().clone(),
                node.structure().clone(),
                node.authored_provenance_digest(),
                node.identity_seed().clone(),
                node.durable_state_eligibility().clone(),
            ))
        }
        WorthUiArtifactNode::Token(node) => {
            WorthUiArtifactNode::Token(WorthUiArtifactThemeTokenNode::new(
                WorthUiArtifactHandle::Token(WorthUiArtifactThemeTokenHandle::new(
                    module_id, index,
                )),
                node.theme_token().clone(),
                node.entry().clone(),
                node.semantics().clone(),
                node.authored_provenance_digest(),
                node.identity_seed().clone(),
                node.durable_state_eligibility().clone(),
            ))
        }
    }
}

fn component(id: &str) -> ComponentDescriptor {
    ComponentDescriptor::new(
        ComponentId::new(id).unwrap(),
        ComponentPropSchema::named("workspace.props"),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
}

fn surface(id: &str, component_id: &str) -> SurfaceDescriptor {
    SurfaceDescriptor::new(
        SurfaceId::new(id).unwrap(),
        SurfaceKind::primary_content(),
        ComponentId::new(component_id).unwrap(),
        SurfacePlacementClass::primary_region(),
        SurfaceStateClass::restorable(),
    )
}

fn durable_eligible() -> WorthUiDurableStateEligibility {
    WorthUiDurableStateEligibility::Eligible {
        restorable_state_slot_count: 1,
    }
}

fn empty_structure() -> WorthUiMosaicStructureFacts {
    WorthUiMosaicStructureFacts::new(Vec::new())
}

fn identity_narrowing_for(
    runtime: &WorthUiRuntimeHost,
    admitted: &WorthUiAdmittedReplacementCandidate,
) -> WorthUiRuntimeImpactNarrowing {
    WorthUiRuntimeImpactNarrowing::new(
        runtime.replacement_admission_basis().artifact_digest(),
        admitted.artifact_bundle().artifact_digest().raw(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        WorthUiAccessibilityInvalidation::unchanged(),
        Vec::new(),
        Vec::new(),
        None,
        admitted
            .artifact_bundle()
            .dependency_metadata()
            .invalidation_basis()
            .impact_metadata()
            .full_artifact_handle_count(),
        WorthUiImpactLookupCounters::default(),
    )
}

fn module_id(path: &str) -> WorthUiSourceModuleId {
    WorthUiSourceModuleId::from_relative_path(Path::new(path)).unwrap()
}
