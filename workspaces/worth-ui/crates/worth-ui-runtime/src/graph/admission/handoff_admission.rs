use crate::declaration::UiDeclarationGraphHandoff;
use super::runtime_basis_assignment::UiRuntimeBasisAssignments;
use crate::graph::{
    UiGraphAttachmentPosture, UiGraphCoreIndexContributionSeed, UiGraphInstantiationDenial,
    UiGraphContainmentClaim, UiGraphInstantiationLocalDenial, UiGraphInstantiationPlan,
    UiGraphMountedReceiptAuthoritySeed, UiGraphNodeInstantiationEntry, UiGraphParticipationSeed,
    UiGraphParentResolutionClaim, UiGraphTopologyLocalDenial, UiGraphTopologySeed,
    UiRepeatedInstanceBasis,
    UiRepeatedInstanceBasisDenial, UiRuntimeInstanceBasisAdmission,
};

pub(crate) fn admit_graph_handoffs(
    handoffs: &[UiDeclarationGraphHandoff],
    runtime_basis_admissions: &[UiRuntimeInstanceBasisAdmission],
) -> Result<UiGraphInstantiationPlan, UiGraphInstantiationDenial> {
    let runtime_basis_assignments =
        UiRuntimeBasisAssignments::resolve(handoffs, runtime_basis_admissions)?;

    let mut node_entries = Vec::with_capacity(handoffs.len());
    let mut local_denials = Vec::new();
    let mut seen_handoffs_by_declaration = std::collections::BTreeMap::new();

    for handoff in handoffs {
        let declaration_identity = handoff.identity().clone();
        let declaration_digest = declaration_identity.digest().raw();
        let occurrence_index = seen_handoffs_by_declaration
            .entry(declaration_digest)
            .and_modify(|count| *count += 1)
            .or_insert(0);
        let repeated_instance_basis = runtime_basis_assignments
            .basis_for(declaration_digest, *occurrence_index)
            .cloned()
            .unwrap_or_else(|| UiRepeatedInstanceBasis::declaration_keyed(declaration_identity.digest()));

        if repeated_instance_basis.denial() == Some(&UiRepeatedInstanceBasisDenial::BasisFreeRuntimeIdentityDenied)
        {
            local_denials.push(UiGraphInstantiationLocalDenial::repeated_instance_basis(
                declaration_identity,
                UiRepeatedInstanceBasisDenial::BasisFreeRuntimeIdentityDenied,
            ));
            continue;
        }

        node_entries.push(UiGraphNodeInstantiationEntry::new(
            declaration_identity,
            handoff.aspect_contract().clone(),
            repeated_instance_basis,
            UiGraphTopologySeed::new(
                handoff.role(),
                UiGraphContainmentClaim::from_declaration_intent(handoff.containment_intent()),
                parent_resolution_claim_for_handoff(handoff),
                handoff.slot_participation_intent().clone(),
                handoff.ordering_guarantee(),
            ),
            UiGraphParticipationSeed::from_attachment_and_role(
                handoff.query_binding().admitted().is_some(),
                handoff.service_usage().admitted().is_some(),
                matches!(
                    handoff.role(),
                    crate::declaration::UiDeclarationStructuralRole::DiagnosticSurface
                ),
            ),
            UiGraphAttachmentPosture::new(
                handoff.query_binding().admitted().is_some(),
                handoff.service_usage().admitted().is_some(),
            ),
            UiGraphMountedReceiptAuthoritySeed::reserved(),
            UiGraphCoreIndexContributionSeed::authoritative(),
        ));
    }

    localize_unresolved_root_topology(&mut node_entries, &mut local_denials);

    Ok(UiGraphInstantiationPlan::new(node_entries, local_denials))
}

fn parent_resolution_claim_for_handoff(
    handoff: &UiDeclarationGraphHandoff,
) -> UiGraphParentResolutionClaim {
    if handoff.containment_intent().is_root() {
        UiGraphParentResolutionClaim::RootPage
    } else {
        UiGraphParentResolutionClaim::ContainedByRootPage
    }
}

fn localize_unresolved_root_topology(
    node_entries: &mut Vec<UiGraphNodeInstantiationEntry>,
    local_denials: &mut Vec<UiGraphInstantiationLocalDenial>,
) {
    let observed_root_pages = node_entries
        .iter()
        .filter(|entry| entry.topology_seed().containment_claim().is_root_page())
        .count();

    if observed_root_pages == 1 {
        return;
    }

    let denial = UiGraphTopologyLocalDenial::RootPageCardinality { observed_root_pages };
    local_denials.extend(node_entries.drain(..).map(|entry| {
        UiGraphInstantiationLocalDenial::topology(
            entry.declaration_identity().clone(),
            denial.clone(),
        )
    }));
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use forge_query::facade::QueryExternalIdentityToken;
    use worth_ui_dsl::{
        UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
        UiDslSourceProvenance, UiDslStructuralToken,
    };

    use super::admit_graph_handoffs;
    use crate::facade::{WorthUi, WorthUiDslPackage};
    use crate::graph::{
        UiGraphInstantiationDenial, UiRepeatedInstanceBasisDenial, UiRepeatedInstanceBasisKind,
        UiRuntimeInstanceBasisAdmission,
    };
    use crate::declaration::UiDeclarationArtifact;

    #[test]
    fn runtime_data_basis_admits_only_through_internal_typed_boundary() {
        let app = WorthUi::app()
            .with_dsl_package(
                WorthUiDslPackage::named("worth-ui.runtime.graph-instantiation.runtime-basis")
                    .with_semantic_artifact_spec(control_graph_input_spec()),
            )
            .freeze();
        let handoff = artifact_from_file_provenance(&app, "app/graph_instantiation.wui", 0)
            .graph_handoff()
            .expect("control declaration should lower to graph handoff");
        let root_page_handoff = root_page_artifact(&app)
            .graph_handoff()
            .expect("bootstrap root page should lower to graph handoff");
        let runtime_basis = UiRuntimeInstanceBasisAdmission::admit_runtime_data_keyed(
            handoff.identity(),
            QueryExternalIdentityToken::new(Arc::<str>::from("row:user-7")),
        )
        .expect("internal typed runtime basis key should admit");
        let plan = admit_graph_handoffs(&[root_page_handoff, handoff], &[runtime_basis])
            .expect("internal typed runtime basis admission should admit graph instantiation");

        assert_eq!(
            plan.node_entries()[1].repeated_instance_basis().kind(),
            UiRepeatedInstanceBasisKind::RuntimeDataKeyed
        );
    }

    #[test]
    fn position_based_runtime_basis_denies_before_graph_mutation() {
        let app = WorthUi::app()
            .with_dsl_package(
                WorthUiDslPackage::named("worth-ui.runtime.graph-instantiation.position")
                    .with_semantic_artifact_spec(control_graph_input_spec()),
            )
            .freeze();
        let handoff = artifact_from_file_provenance(&app, "app/graph_instantiation.wui", 0)
            .graph_handoff()
            .expect("control declaration should lower to graph handoff");
        let denial = UiRuntimeInstanceBasisAdmission::admit_runtime_data_keyed(
            handoff.identity(),
            QueryExternalIdentityToken::new(Arc::<str>::from("position:0")),
        )
        .expect_err("position-based runtime basis must deny before graph mutation");

        assert_eq!(denial, UiRepeatedInstanceBasisDenial::PositionBasedBasis);
    }

    #[test]
    fn orphan_or_contradictory_runtime_basis_admission_denies_internal_plan_construction() {
        let app = WorthUi::app()
            .with_dsl_package(
                WorthUiDslPackage::named("worth-ui.runtime.graph-instantiation.orphan")
                    .with_semantic_artifact_spec(control_graph_input_spec()),
            )
            .freeze();
        let handoff = artifact_from_file_provenance(&app, "app/graph_instantiation.wui", 0)
            .graph_handoff()
            .expect("control declaration should lower to graph handoff");
        let unrelated_app = WorthUi::app()
            .with_dsl_package(
                WorthUiDslPackage::named("worth-ui.runtime.graph-instantiation.orphan.other")
                    .with_semantic_artifact_spec(other_control_spec()),
            )
            .freeze();
        let unrelated_handoff =
            artifact_from_file_provenance(&unrelated_app, "app/graph_instantiation_other.wui", 0)
                .graph_handoff()
                .expect("unrelated declaration should lower to graph handoff");
        let orphan_basis = UiRuntimeInstanceBasisAdmission::admit_runtime_data_keyed(
            unrelated_handoff.identity(),
            QueryExternalIdentityToken::new(Arc::<str>::from("row:other")),
        )
        .expect("typed runtime basis key should admit");

        assert!(matches!(
            admit_graph_handoffs(&[handoff.clone()], &[orphan_basis]),
            Err(UiGraphInstantiationDenial::RuntimeBasisTargetsUnknownDeclaration { .. })
        ));

        let contradictory_handoffs = vec![handoff.clone(), handoff];
        let contradictory_basis = UiRuntimeInstanceBasisAdmission::admit_runtime_data_keyed(
            contradictory_handoffs[0].identity(),
            QueryExternalIdentityToken::new(Arc::<str>::from("row:duplicate")),
        )
        .expect("typed runtime basis key should admit");

        assert!(matches!(
            admit_graph_handoffs(&contradictory_handoffs, &[contradictory_basis]),
            Err(UiGraphInstantiationDenial::ContradictoryRuntimeBasisAdmission { .. })
        ));
    }

    #[test]
    fn touch_and_measurement_posture_do_not_change_internal_graph_instantiation_outputs() {
        let baseline = WorthUi::app()
            .with_dsl_package(
                WorthUiDslPackage::named("worth-ui.runtime.graph-instantiation.invariance")
                    .with_semantic_artifact_spec(control_graph_input_without_non_graph_obligations()),
            )
            .freeze();
        let enriched = WorthUi::app()
            .with_dsl_package(
                WorthUiDslPackage::named("worth-ui.runtime.graph-instantiation.invariance")
                    .with_semantic_artifact_spec(control_graph_input_spec()),
            )
            .freeze();
        let baseline_handoff = artifact_from_file_provenance(&baseline, "app/graph_instantiation.wui", 0)
            .graph_handoff()
            .expect("baseline declaration should lower to graph handoff");
        let baseline_root_handoff = root_page_artifact(&baseline)
            .graph_handoff()
            .expect("baseline bootstrap root page should lower to graph handoff");
        let enriched_handoff = artifact_from_file_provenance(&enriched, "app/graph_instantiation.wui", 0)
            .graph_handoff()
            .expect("enriched declaration should lower to graph handoff");
        let enriched_root_handoff = root_page_artifact(&enriched)
            .graph_handoff()
            .expect("enriched bootstrap root page should lower to graph handoff");
        let baseline_plan = admit_graph_handoffs(&[baseline_root_handoff, baseline_handoff.clone()], &[])
            .expect("baseline graph handoff should admit internal graph instantiation");
        let enriched_plan = admit_graph_handoffs(&[enriched_root_handoff, enriched_handoff.clone()], &[])
            .expect("touch and measurement posture should not alter internal graph instantiation");
        let baseline_entry = baseline_plan
            .node_entries()
            .iter()
            .find(|entry| entry.declaration_identity() == baseline_handoff.identity())
            .expect("baseline control handoff should admit one graph instantiation entry");
        let enriched_entry = enriched_plan
            .node_entries()
            .iter()
            .find(|entry| entry.declaration_identity() == enriched_handoff.identity())
            .expect("enriched control handoff should admit one graph instantiation entry");

        assert_eq!(
            baseline_entry.topology_seed(),
            enriched_entry.topology_seed()
        );
        assert_eq!(
            baseline_entry.participation_seed(),
            enriched_entry.participation_seed()
        );
        assert_eq!(
            baseline_entry.attachment_posture(),
            enriched_entry.attachment_posture()
        );
        assert_eq!(
            baseline_entry.mounted_receipt_seed(),
            enriched_entry.mounted_receipt_seed()
        );
        assert_eq!(
            baseline_entry.core_index_contribution_seed(),
            enriched_entry.core_index_contribution_seed()
        );
        assert_eq!(baseline_plan.local_denials(), enriched_plan.local_denials());
    }

    fn artifact_from_file_provenance<'a>(
        app: &'a crate::facade::WorthUiApp,
        module_path: &str,
        declaration_index: usize,
    ) -> &'a UiDeclarationArtifact {
        app.declaration_artifacts()
            .iter()
            .find(|artifact| {
                let provenance = artifact.provenance().source_provenance();
                provenance.module_path() == module_path
                    && provenance.declaration_index() == declaration_index
            })
            .unwrap_or_else(|| {
                panic!(
                    "expected declaration artifact for {module_path}#{declaration_index} on freeze path"
                )
            })
    }

    fn root_page_artifact(app: &crate::facade::WorthUiApp) -> &UiDeclarationArtifact {
        app.declaration_artifacts()
            .iter()
            .find(|artifact| {
                artifact
                    .graph_handoff()
                    .map(|handoff| {
                        handoff.role() == crate::declaration::UiDeclarationStructuralRole::Page
                    })
                    .unwrap_or(false)
            })
            .expect("bootstrap root page artifact should exist")
    }

    fn control_graph_input_spec() -> UiDslSemanticArtifactSpec {
        control_graph_input_without_non_graph_obligations()
            .with_posture_token(UiDslPostureToken::new("touch:press"))
            .with_posture_token(UiDslPostureToken::new("measurement:hug-height"))
    }

    fn control_graph_input_without_non_graph_obligations() -> UiDslSemanticArtifactSpec {
        UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.inspector.save"),
            UiDslSemanticFamily::Control,
            UiDslSourceProvenance::file_authored("app/graph_instantiation.wui", 0),
        )
        .with_structural_token(UiDslStructuralToken::new("control:save"))
        .with_structural_token(UiDslStructuralToken::new("slot:footer"))
        .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
        .with_posture_token(UiDslPostureToken::new("service:portal"))
    }

    fn other_control_spec() -> UiDslSemanticArtifactSpec {
        UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.inspector.cancel"),
            UiDslSemanticFamily::Control,
            UiDslSourceProvenance::file_authored("app/graph_instantiation_other.wui", 0),
        )
        .with_structural_token(UiDslStructuralToken::new("control:cancel"))
        .with_structural_token(UiDslStructuralToken::new("slot:footer"))
    }
}
