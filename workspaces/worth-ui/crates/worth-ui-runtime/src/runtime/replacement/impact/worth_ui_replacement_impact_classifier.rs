use crate::runtime::{
    WorthUiAccessibilityImpact, WorthUiAdmittedReplacementCandidate, WorthUiCommandImpact,
    WorthUiRendererResourceImpact, WorthUiReplacementImpact,
    WorthUiReplacementImpactClassification, WorthUiReplacementImpactCounters,
    WorthUiReplacementImpactDenial, WorthUiReplacementScope, WorthUiRuntimeArtifactComparison,
    WorthUiRuntimeArtifactComparisonOutcome, WorthUiTokenThemeImpact,
    WorthUiUnsupportedReplacementImpact,
};
use crate::source::{
    WorthUiArtifactDifference, WorthUiArtifactHandle, WorthUiArtifactNodeKind,
    WorthUiArtifactSemanticDelta,
};

#[derive(Clone, Debug, Default)]
pub struct WorthUiReplacementImpactClassifier;

impl WorthUiReplacementImpactClassifier {
    pub(crate) fn classify(
        active_artifact: &crate::source::WorthUiArtifact,
        comparison: &WorthUiRuntimeArtifactComparison,
        admitted: &WorthUiAdmittedReplacementCandidate,
    ) -> Result<WorthUiReplacementImpactClassification, WorthUiReplacementImpactDenial> {
        let mut counters = WorthUiReplacementImpactCounters::default();
        counters.record_artifact_comparison_consumed();
        counters.record_impact_classification_attempted();

        reject_mismatched_active_basis(comparison, admitted, counters)?;
        reject_mismatched_comparison_candidate(comparison, admitted, counters)?;

        if comparison.outcome() == WorthUiRuntimeArtifactComparisonOutcome::EquivalentNoOp {
            return Ok(classification(
                comparison,
                WorthUiReplacementImpact::NoOp,
                WorthUiCommandImpact::Unchanged,
                WorthUiTokenThemeImpact::Unchanged,
                WorthUiAccessibilityImpact::Unchanged,
                WorthUiRendererResourceImpact::Unchanged,
                counters,
            ));
        }

        counters.record_dependency_metadata_read();
        let impact =
            classify_meaningful_difference(active_artifact, comparison, admitted, &mut counters)?;
        let command_impact = command_impact_for(&impact, comparison);
        let token_theme_impact = token_theme_impact_for(&impact, comparison);

        Ok(classification(
            comparison,
            impact,
            command_impact,
            token_theme_impact,
            WorthUiAccessibilityImpact::Unchanged,
            WorthUiRendererResourceImpact::Unchanged,
            counters,
        ))
    }
}

fn classify_meaningful_difference(
    active_artifact: &crate::source::WorthUiArtifact,
    comparison: &WorthUiRuntimeArtifactComparison,
    admitted: &WorthUiAdmittedReplacementCandidate,
    counters: &mut WorthUiReplacementImpactCounters,
) -> Result<WorthUiReplacementImpact, WorthUiReplacementImpactDenial> {
    if active_artifact.query_binding_ids()
        != admitted.artifact_bundle().artifact().query_binding_ids()
    {
        return Ok(WorthUiReplacementImpact::StructuralReplacement(
            structural_scope_from_artifacts(active_artifact, admitted, counters),
        ));
    }
    match comparison
        .artifact_equivalence()
        .first_difference()
        .expect("meaningful comparison carries first difference")
    {
        WorthUiArtifactDifference::NodeSemantics {
            module_id,
            node_kind,
            node_index,
            semantic_delta,
            ..
        } => classify_node_semantic_difference(
            module_id,
            *node_kind,
            *node_index,
            *semantic_delta,
            admitted,
            counters,
        ),
        WorthUiArtifactDifference::NodeKind { .. } => {
            Ok(WorthUiReplacementImpact::StructuralReplacement(
                structural_scope_from_artifacts(active_artifact, admitted, counters),
            ))
        }
        WorthUiArtifactDifference::ModuleNodeCount { .. }
            if super::is_query_binding_topology_only_difference(
                active_artifact,
                admitted.artifact_bundle().artifact(),
            ) =>
        {
            Ok(WorthUiReplacementImpact::StructuralReplacement(
                structural_scope_from_artifacts(active_artifact, admitted, counters),
            ))
        }
        WorthUiArtifactDifference::ModuleNodeCount { .. } => {
            Ok(WorthUiReplacementImpact::StructuralReplacement(
                structural_scope_from_artifacts(active_artifact, admitted, counters),
            ))
        }
        WorthUiArtifactDifference::ModuleCount { .. }
        | WorthUiArtifactDifference::ModuleOrder { .. } => {
            deny_broad_replacement_without_receipts(admitted, counters)
        }
    }
}

fn classify_node_semantic_difference(
    module_id: &str,
    node_kind: WorthUiArtifactNodeKind,
    node_index: usize,
    semantic_delta: WorthUiArtifactSemanticDelta,
    admitted: &WorthUiAdmittedReplacementCandidate,
    counters: &mut WorthUiReplacementImpactCounters,
) -> Result<WorthUiReplacementImpact, WorthUiReplacementImpactDenial> {
    match node_kind {
        WorthUiArtifactNodeKind::Import => {
            deny_broad_replacement_without_receipts(admitted, counters)
        }
        WorthUiArtifactNodeKind::Component | WorthUiArtifactNodeKind::Binding => {
            Ok(WorthUiReplacementImpact::LocalSubtree(
                local_scope_from_candidate(module_id, node_index, admitted, counters),
            ))
        }
        WorthUiArtifactNodeKind::Surface => classify_surface_semantic_difference(
            module_id,
            node_index,
            semantic_delta,
            admitted,
            counters,
        ),
        WorthUiArtifactNodeKind::Token => Ok(WorthUiReplacementImpact::LocalSubtree(
            local_scope_from_candidate(module_id, node_index, admitted, counters),
        )),
    }
}

fn reject_mismatched_comparison_candidate(
    comparison: &WorthUiRuntimeArtifactComparison,
    admitted: &WorthUiAdmittedReplacementCandidate,
    counters: WorthUiReplacementImpactCounters,
) -> Result<(), WorthUiReplacementImpactDenial> {
    let admitted_candidate_artifact_digest = admitted.artifact_bundle().artifact_digest().raw();
    if comparison.candidate_artifact_digest() == admitted_candidate_artifact_digest {
        Ok(())
    } else {
        Err(
            WorthUiReplacementImpactDenial::ComparisonCandidateMismatch {
                comparison_candidate_artifact_digest: comparison.candidate_artifact_digest(),
                admitted_candidate_artifact_digest,
                counters,
            },
        )
    }
}

fn reject_mismatched_active_basis(
    comparison: &WorthUiRuntimeArtifactComparison,
    admitted: &WorthUiAdmittedReplacementCandidate,
    counters: WorthUiReplacementImpactCounters,
) -> Result<(), WorthUiReplacementImpactDenial> {
    let admitted_active_artifact_digest = admitted.active_basis().artifact_digest();
    if comparison.active_artifact_digest() == admitted_active_artifact_digest {
        Ok(())
    } else {
        Err(
            WorthUiReplacementImpactDenial::ComparisonActiveBasisMismatch {
                comparison_active_artifact_digest: comparison.active_artifact_digest(),
                admitted_active_artifact_digest,
                counters,
            },
        )
    }
}

fn classify_surface_semantic_difference(
    module_id: &str,
    node_index: usize,
    semantic_delta: WorthUiArtifactSemanticDelta,
    admitted: &WorthUiAdmittedReplacementCandidate,
    counters: &mut WorthUiReplacementImpactCounters,
) -> Result<WorthUiReplacementImpact, WorthUiReplacementImpactDenial> {
    match semantic_delta {
        WorthUiArtifactSemanticDelta::SurfaceCommandSlotsChanged => {
            Ok(WorthUiReplacementImpact::LocalSubtree(
                local_scope_from_candidate(module_id, node_index, admitted, counters),
            ))
        }
        WorthUiArtifactSemanticDelta::SurfacePlacementClassChanged
        | WorthUiArtifactSemanticDelta::SurfacePlacementAndCommandSlotsChanged
        | WorthUiArtifactSemanticDelta::Other => Ok(WorthUiReplacementImpact::LaneAffecting {
            lane_impact: crate::runtime::WorthUiLaneImpactClassification::surface_semantics_changed(
            ),
            scope: local_scope_from_candidate(module_id, node_index, admitted, counters),
        }),
    }
}

fn local_scope_from_candidate(
    module_id: &str,
    node_index: usize,
    admitted: &WorthUiAdmittedReplacementCandidate,
    counters: &mut WorthUiReplacementImpactCounters,
) -> WorthUiReplacementScope {
    let graph = admitted
        .artifact_bundle()
        .dependency_metadata()
        .invalidation_basis()
        .dependency_graph();
    let handle = candidate_handle_at_node_index(admitted, module_id, node_index)
        .expect("difference node index resolves in candidate artifact");
    let impact = admitted
        .artifact_bundle()
        .dependency_metadata()
        .invalidation_basis()
        .impact_metadata()
        .impact_for_subtree(&handle);
    counters.record_impact_metadata_lookups(impact.lookup_count());
    let impacted_handles = if impact.impacted_handles().is_empty() {
        vec![handle]
    } else {
        impact.impacted_handles().to_vec()
    };
    WorthUiReplacementScope::local_subtree(
        impacted_handles,
        graph.subtree_digests().len(),
        impact.lookup_count(),
    )
}

fn structural_scope_from_artifacts(
    active_artifact: &crate::source::WorthUiArtifact,
    admitted: &WorthUiAdmittedReplacementCandidate,
    counters: &mut WorthUiReplacementImpactCounters,
) -> WorthUiReplacementScope {
    let metadata = admitted.artifact_bundle().dependency_metadata();
    let graph = metadata.invalidation_basis().dependency_graph();
    let candidate_artifact = admitted.artifact_bundle().artifact();
    let active_by_identity = active_artifact
        .identity_handles()
        .collect::<std::collections::BTreeMap<_, _>>();
    let impacted_handles = candidate_artifact
        .identity_handles()
        .filter(|(identity, candidate_handle)| {
            let Some(active_handle) = active_by_identity.get(identity) else {
                return true;
            };
            if active_handle.kind() != candidate_handle.kind() {
                return true;
            }
            let active_node = active_artifact
                .node_for_identity_basis(identity)
                .expect("active identity index resolves its node");
            let candidate_node = candidate_artifact
                .node_for_identity_basis(identity)
                .expect("candidate identity index resolves its node");
            !active_node.has_same_semantic_meaning_ignoring_location(candidate_node)
        })
        .map(|(_, candidate_handle)| candidate_handle.clone())
        .collect::<Vec<_>>();
    counters.record_impact_metadata_lookups(1);
    WorthUiReplacementScope::structural(impacted_handles, graph.subtree_digests().len(), 1)
}

fn deny_broad_replacement_without_receipts(
    admitted: &WorthUiAdmittedReplacementCandidate,
    counters: &mut WorthUiReplacementImpactCounters,
) -> Result<WorthUiReplacementImpact, WorthUiReplacementImpactDenial> {
    counters.record_broad_replacement_denial();
    let full_artifact_handle_count = admitted
        .artifact_bundle()
        .dependency_metadata()
        .invalidation_basis()
        .dependency_graph()
        .subtree_digests()
        .len();
    let scope =
        WorthUiReplacementScope::broad_without_durable_state_receipts(full_artifact_handle_count);
    Err(WorthUiReplacementImpactDenial::UnsupportedImpact {
        unsupported_impact: WorthUiUnsupportedReplacementImpact::MissingDurableStateReceipts {
            scope,
        },
        counters: *counters,
    })
}

fn candidate_handle_at_node_index(
    admitted: &WorthUiAdmittedReplacementCandidate,
    module_id: &str,
    node_index: usize,
) -> Option<WorthUiArtifactHandle> {
    let module_id =
        worth_ui_dsl::WorthUiSourceModuleId::from_relative_path(std::path::Path::new(module_id))
            .ok()?;
    admitted
        .artifact_bundle()
        .artifact()
        .module(&module_id)
        .and_then(|module| module.nodes().get(node_index))
        .map(|node| node.handle().clone())
}

fn command_impact_for(
    impact: &WorthUiReplacementImpact,
    comparison: &WorthUiRuntimeArtifactComparison,
) -> WorthUiCommandImpact {
    if surface_command_slot_difference(comparison)
        || (matches!(impact, WorthUiReplacementImpact::LocalSubtree(_))
            && node_semantic_difference_kind(comparison) == Some(WorthUiArtifactNodeKind::Binding))
    {
        WorthUiCommandImpact::BindingOnly
    } else {
        WorthUiCommandImpact::Unchanged
    }
}

fn surface_command_slot_difference(comparison: &WorthUiRuntimeArtifactComparison) -> bool {
    matches!(
        comparison.artifact_equivalence().first_difference(),
        Some(WorthUiArtifactDifference::NodeSemantics {
            semantic_delta: WorthUiArtifactSemanticDelta::SurfaceCommandSlotsChanged
                | WorthUiArtifactSemanticDelta::SurfacePlacementAndCommandSlotsChanged,
            ..
        })
    )
}

fn token_theme_impact_for(
    impact: &WorthUiReplacementImpact,
    comparison: &WorthUiRuntimeArtifactComparison,
) -> WorthUiTokenThemeImpact {
    if node_semantic_difference_kind(comparison) == Some(WorthUiArtifactNodeKind::Token)
        && matches!(impact, WorthUiReplacementImpact::LocalSubtree(_))
    {
        WorthUiTokenThemeImpact::ThemeOnly
    } else {
        WorthUiTokenThemeImpact::Unchanged
    }
}

fn node_semantic_difference_kind(
    comparison: &WorthUiRuntimeArtifactComparison,
) -> Option<WorthUiArtifactNodeKind> {
    match comparison.artifact_equivalence().first_difference() {
        Some(WorthUiArtifactDifference::NodeSemantics { node_kind, .. }) => Some(*node_kind),
        _ => None,
    }
}

fn classification(
    comparison: &WorthUiRuntimeArtifactComparison,
    impact: WorthUiReplacementImpact,
    command_impact: WorthUiCommandImpact,
    token_theme_impact: WorthUiTokenThemeImpact,
    accessibility_impact: WorthUiAccessibilityImpact,
    renderer_resource_impact: WorthUiRendererResourceImpact,
    counters: WorthUiReplacementImpactCounters,
) -> WorthUiReplacementImpactClassification {
    WorthUiReplacementImpactClassification::new(
        super::WorthUiReplacementImpactClassificationInput {
            active_artifact_digest: comparison.active_artifact_digest(),
            candidate_artifact_digest: comparison.candidate_artifact_digest(),
            impact,
            command_impact,
            token_theme_impact,
            accessibility_impact,
            renderer_resource_impact,
            counters,
        },
    )
}
