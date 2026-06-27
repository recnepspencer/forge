use std::collections::BTreeMap;

use crate::source::{
    WorthUiArtifact, WorthUiArtifactCapabilityReference,
    WorthUiArtifactCapabilityReferenceInspection, WorthUiArtifactCapabilityReferenceRole,
    WorthUiArtifactInspection, WorthUiArtifactInspectionBasis, WorthUiArtifactInspectionDiagnostic,
    WorthUiArtifactInspectionMetrics, WorthUiArtifactInspectionReport, WorthUiArtifactNode,
    WorthUiArtifactNodeInspection, WorthUiArtifactProvenanceMap, WorthUiMosaicRegionFacts,
    WorthUiQueryInspectionLink, WorthUiQueryInspectionLinkRole,
};

#[derive(Clone, Debug, Default)]
pub(crate) struct WorthUiArtifactInspectionDeriver;

impl WorthUiArtifactInspectionDeriver {
    pub(crate) fn derive(
        artifact: &WorthUiArtifact,
        basis: &WorthUiArtifactInspectionBasis,
    ) -> Result<WorthUiArtifactInspection, WorthUiArtifactInspectionReport> {
        Self::derive_with_metrics(artifact, basis).map(|(inspection, _)| inspection)
    }

    pub(crate) fn derive_with_metrics(
        artifact: &WorthUiArtifact,
        basis: &WorthUiArtifactInspectionBasis,
    ) -> Result<
        (WorthUiArtifactInspection, WorthUiArtifactInspectionMetrics),
        WorthUiArtifactInspectionReport,
    > {
        let mut metrics = WorthUiArtifactInspectionMetrics::default();
        let mut diagnostics = Vec::new();
        let mut provenance_source_origins = BTreeMap::new();
        let mut node_inspections = BTreeMap::new();
        let mut canonical_handle_order = Vec::new();

        for module_id in artifact.module_ids() {
            metrics.record_module_inspected();
            let module = artifact
                .module(module_id)
                .expect("artifact should contain every canonical module");

            for node in module.nodes() {
                metrics.record_node_inspected();
                let handle = node.handle().clone();
                canonical_handle_order.push(handle.clone());

                let Some(source_origin) = basis.source_origin(&handle).cloned() else {
                    diagnostics.push(
                        WorthUiArtifactInspectionDiagnostic::missing_artifact_source_origin(
                            handle.clone(),
                            format!("missing source origin for {:?}", handle.kind()),
                        ),
                    );
                    continue;
                };

                let capability_references = derive_capability_references(node, &mut metrics);
                let query_inspection_links = derive_query_inspection_links(node, &mut metrics);
                provenance_source_origins.insert(handle.clone(), source_origin.clone());
                node_inspections.insert(
                    handle.clone(),
                    WorthUiArtifactNodeInspection::new(
                        handle,
                        node.handle().kind(),
                        source_origin,
                        node_identity_seed(node).clone(),
                        node_durable_state_eligibility(node).clone(),
                        capability_references,
                        query_inspection_links,
                    ),
                );
            }
        }

        if !diagnostics.is_empty() {
            return Err(WorthUiArtifactInspectionReport::new(diagnostics, metrics));
        }

        let provenance_map = WorthUiArtifactProvenanceMap::new(
            provenance_source_origins,
            canonical_handle_order.clone(),
        );
        Ok((
            WorthUiArtifactInspection::new(provenance_map, node_inspections),
            metrics,
        ))
    }
}

fn derive_capability_references(
    node: &WorthUiArtifactNode,
    metrics: &mut WorthUiArtifactInspectionMetrics,
) -> Vec<WorthUiArtifactCapabilityReferenceInspection> {
    let mut references = Vec::new();
    match node {
        WorthUiArtifactNode::Import(_) => {}
        WorthUiArtifactNode::Component(node) => {
            push_reference(
                &mut references,
                metrics,
                WorthUiArtifactCapabilityReferenceRole::PrimaryComponent,
                WorthUiArtifactCapabilityReference::Component(node.component().clone()),
            );
            extend_structure_references(&mut references, metrics, node.structure());
        }
        WorthUiArtifactNode::Surface(node) => {
            push_reference(
                &mut references,
                metrics,
                WorthUiArtifactCapabilityReferenceRole::PrimarySurface,
                WorthUiArtifactCapabilityReference::Surface(node.surface().clone()),
            );
            extend_structure_references(&mut references, metrics, node.structure());
            if let Some(icon) = node.semantics().icon() {
                push_reference(
                    &mut references,
                    metrics,
                    WorthUiArtifactCapabilityReferenceRole::SurfaceIcon,
                    WorthUiArtifactCapabilityReference::Icon(icon.icon().clone()),
                );
            }
            if let Some(view_binding) = node.semantics().view_binding() {
                push_reference(
                    &mut references,
                    metrics,
                    WorthUiArtifactCapabilityReferenceRole::SurfaceViewBinding,
                    WorthUiArtifactCapabilityReference::ViewBinding(
                        view_binding.view_binding().clone(),
                    ),
                );
            }
            for command in node.semantics().command_slots() {
                push_reference(
                    &mut references,
                    metrics,
                    WorthUiArtifactCapabilityReferenceRole::SurfaceCommand,
                    WorthUiArtifactCapabilityReference::Command(command.command().clone()),
                );
                if let Some(icon) = command.semantics().icon() {
                    push_reference(
                        &mut references,
                        metrics,
                        WorthUiArtifactCapabilityReferenceRole::SurfaceCommandIcon,
                        WorthUiArtifactCapabilityReference::Icon(icon.icon().clone()),
                    );
                }
                if let Some(projection) = command.semantics().projection_eligibility() {
                    push_reference(
                        &mut references,
                        metrics,
                        WorthUiArtifactCapabilityReferenceRole::SurfaceCommandProjection,
                        WorthUiArtifactCapabilityReference::CommandProjection(
                            projection.command_projection().clone(),
                        ),
                    );
                }
            }
        }
        WorthUiArtifactNode::Binding(node) => {
            push_reference(
                &mut references,
                metrics,
                WorthUiArtifactCapabilityReferenceRole::BoundViewBinding,
                WorthUiArtifactCapabilityReference::ViewBinding(
                    node.view_binding_reference().view_binding().clone(),
                ),
            );
            extend_structure_references(&mut references, metrics, node.structure());
        }
        WorthUiArtifactNode::Token(node) => {
            push_reference(
                &mut references,
                metrics,
                WorthUiArtifactCapabilityReferenceRole::PrimaryThemeToken,
                WorthUiArtifactCapabilityReference::ThemeToken(node.theme_token().clone()),
            );
            push_reference(
                &mut references,
                metrics,
                WorthUiArtifactCapabilityReferenceRole::ThemeTokenAliasTarget,
                WorthUiArtifactCapabilityReference::ThemeToken(
                    node.semantics().resolved_target_theme_token().clone(),
                ),
            );
        }
    }
    references
}

fn derive_query_inspection_links(
    node: &WorthUiArtifactNode,
    metrics: &mut WorthUiArtifactInspectionMetrics,
) -> Vec<WorthUiQueryInspectionLink> {
    let mut links = Vec::new();
    match node {
        WorthUiArtifactNode::Binding(node) => {
            links.push(query_link_from_view_binding(
                WorthUiQueryInspectionLinkRole::BindingViewBindingQuery,
                node.view_binding_reference(),
            ));
            metrics.record_query_link();
        }
        WorthUiArtifactNode::Surface(node) => {
            if let Some(view_binding) = node.semantics().view_binding() {
                links.push(query_link_from_view_binding(
                    WorthUiQueryInspectionLinkRole::SurfaceViewBindingQuery,
                    view_binding,
                ));
                metrics.record_query_link();
            }
        }
        _ => {}
    }
    links
}

fn query_link_from_view_binding(
    role: WorthUiQueryInspectionLinkRole,
    view_binding: &crate::source::WorthUiBoundViewBindingReference,
) -> WorthUiQueryInspectionLink {
    let query_semantics = view_binding.query_semantics();
    WorthUiQueryInspectionLink::new(
        role,
        view_binding.view_binding().clone(),
        query_semantics.query_capability().clone(),
        query_semantics
            .query_composition_profile_digest()
            .to_owned(),
        query_semantics.result_shape().clone(),
        query_semantics.basis_posture().clone(),
        query_semantics.live_compatibility().clone(),
        query_semantics.denial_presentation().clone(),
    )
}

fn extend_structure_references(
    references: &mut Vec<WorthUiArtifactCapabilityReferenceInspection>,
    metrics: &mut WorthUiArtifactInspectionMetrics,
    structure: &crate::source::WorthUiMosaicStructureFacts,
) {
    for region in structure.root_regions() {
        extend_region_references(references, metrics, region);
    }
}

fn extend_region_references(
    references: &mut Vec<WorthUiArtifactCapabilityReferenceInspection>,
    metrics: &mut WorthUiArtifactInspectionMetrics,
    region: &WorthUiMosaicRegionFacts,
) {
    push_reference(
        references,
        metrics,
        WorthUiArtifactCapabilityReferenceRole::StructureRegionKind,
        WorthUiArtifactCapabilityReference::MosaicRegionKind(region.region().clone()),
    );
    if let Some((sizing, _)) = region.sizing_contract() {
        push_reference(
            references,
            metrics,
            WorthUiArtifactCapabilityReferenceRole::StructureRegionSizingContract,
            WorthUiArtifactCapabilityReference::MosaicSizingContract(sizing.clone()),
        );
    }
    if let Some((state_slot, _)) = region.state_slot() {
        push_reference(
            references,
            metrics,
            WorthUiArtifactCapabilityReferenceRole::StructureRegionStateSlot,
            WorthUiArtifactCapabilityReference::MosaicStateSlot(state_slot.clone()),
        );
    }
    for mount in region.mounts() {
        push_reference(
            references,
            metrics,
            WorthUiArtifactCapabilityReferenceRole::StructureMountSurface,
            WorthUiArtifactCapabilityReference::Surface(mount.surface().clone()),
        );
        if let Some((placement, _)) = mount.placement_policy() {
            push_reference(
                references,
                metrics,
                WorthUiArtifactCapabilityReferenceRole::StructureMountPlacementPolicy,
                WorthUiArtifactCapabilityReference::MosaicPlacementPolicy(placement.clone()),
            );
        }
        if let Some((state_slot, _)) = mount.state_slot() {
            push_reference(
                references,
                metrics,
                WorthUiArtifactCapabilityReferenceRole::StructureMountStateSlot,
                WorthUiArtifactCapabilityReference::MosaicStateSlot(state_slot.clone()),
            );
        }
    }
    for child_region in region.child_regions() {
        extend_region_references(references, metrics, child_region);
    }
}

fn push_reference(
    references: &mut Vec<WorthUiArtifactCapabilityReferenceInspection>,
    metrics: &mut WorthUiArtifactInspectionMetrics,
    role: WorthUiArtifactCapabilityReferenceRole,
    reference: WorthUiArtifactCapabilityReference,
) {
    references.push(WorthUiArtifactCapabilityReferenceInspection::new(
        role, reference,
    ));
    metrics.record_capability_reference();
}

fn node_identity_seed(node: &WorthUiArtifactNode) -> &crate::source::WorthUiArtifactIdentitySeed {
    match node {
        WorthUiArtifactNode::Import(node) => node.identity_seed(),
        WorthUiArtifactNode::Component(node) => node.identity_seed(),
        WorthUiArtifactNode::Surface(node) => node.identity_seed(),
        WorthUiArtifactNode::Binding(node) => node.identity_seed(),
        WorthUiArtifactNode::Token(node) => node.identity_seed(),
    }
}

fn node_durable_state_eligibility(
    node: &WorthUiArtifactNode,
) -> &crate::source::WorthUiDurableStateEligibility {
    match node {
        WorthUiArtifactNode::Import(node) => node.durable_state_eligibility(),
        WorthUiArtifactNode::Component(node) => node.durable_state_eligibility(),
        WorthUiArtifactNode::Surface(node) => node.durable_state_eligibility(),
        WorthUiArtifactNode::Binding(node) => node.durable_state_eligibility(),
        WorthUiArtifactNode::Token(node) => node.durable_state_eligibility(),
    }
}
