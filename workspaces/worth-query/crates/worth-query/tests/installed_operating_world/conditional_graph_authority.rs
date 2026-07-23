use worth_foundational::facade::{
    AbsenceLaw, AspectContract, AspectContractRevision, AspectEvolutionPolicy, AspectIdentity,
    AspectKey, FieldDeclaration, FieldKey, FieldRequirement, ScalarAspectType, StructAspectShape,
};
use worth_query::facade::domain;

use super::conditional_node_contract::{dependency, dependency_for_role, node};
use super::installed_operation_fixture::{
    conditional_workflow_workspace, conditional_workspace, correspondence_bridge,
    fixture_record_identity, ConditionalModelGraph, GeometryDomain, ReadFamily, ReadVertex,
    WorkflowRead,
};

fn geometry_node_location() -> domain::WorthQueryConditionalNodeLocation {
    domain::WorthQueryConditionalNodeLocation::operation("geometry").unwrap()
}

#[test]
fn installed_operation_and_exact_graph_participation_mint_the_candidate() {
    let declared = dependency(domain::WorthQuerySemanticLocality::SourceRecord);
    let workspace = conditional_workspace(
        "installed-correspondence-candidate",
        node(
            "geometry",
            domain::WorthQueryComparatorRequirement::ExactCanonicalValue,
            domain::WorthQuerySemanticLocality::SourceRecord,
        ),
    )
    .unwrap();
    let installed_domain = workspace.domain(GeometryDomain).unwrap();
    let operating_world = workspace.observe_operating_world().unwrap();
    let operation = operating_world
        .family(ReadFamily)
        .bind(&installed_domain, ReadVertex)
        .unwrap();
    let graph = workspace
        .graph_participation(ConditionalModelGraph)
        .unwrap();
    let mut signal_graph = worth_signal::facade::SignalGraph::new();
    let signal_node = signal_graph.node().build();
    let worth_proof::TransitionOutcome::Success(signal_node) =
        signal_graph.admit_installed_node(signal_node)
    else {
        panic!("installed Signal node capability")
    };
    let target = worth_runtime_bridge::facade::BridgeSignalAspectTargetDeclaration::allocate(
        worth_runtime_bridge::facade::BridgeAspectRegistrationId::from_stable_name(
            "conditional-identity",
        ),
        worth_signal::facade::PartitionToken::new("geometry-signal"),
        signal_node,
    );
    assert!(matches!(
        operation.semantic_correspondence_registration(
            geometry_node_location(),
            0,
            &graph,
            None,
            vec![target.clone()],
        ),
        Err(denial) if denial.kind()
            == worth_runtime_bridge::facade::BridgeCorrespondenceDenialKind::InvalidPortableDependency
    ));
    let registration = operation
        .semantic_correspondence_registration(
            geometry_node_location(),
            0,
            &graph,
            Some(fixture_record_identity()),
            vec![target.clone()],
        )
        .unwrap();
    let candidate = registration.dependency();

    assert_eq!(candidate.contract(), declared.contract());
    assert_eq!(candidate.binding(), declared.binding());
    assert_eq!(candidate.declared_graph_role(), "model");
    assert!(!candidate.graph_participation_identity().is_empty());
    assert!(!candidate.graph_adapter_identity().is_empty());

    assert!(matches!(
        operation.semantic_correspondence_registration(
            geometry_node_location(),
            1,
            &graph,
            Some(fixture_record_identity()),
            vec![target],
        ),
        Err(denial) if denial.kind()
            == worth_runtime_bridge::facade::BridgeCorrespondenceDenialKind::PortableDependencyNotOwnedByOperation
    ));
}

#[test]
fn bound_query_facade_installs_correspondence_with_operation_authority() {
    let workspace = conditional_workspace(
        "installed-correspondence-owner",
        node(
            "geometry",
            domain::WorthQueryComparatorRequirement::ExactCanonicalValue,
            domain::WorthQuerySemanticLocality::SourceRecord,
        ),
    )
    .unwrap();
    let installed_domain = workspace.domain(GeometryDomain).unwrap();
    let operating_world = workspace.observe_operating_world().unwrap();
    let operation = operating_world
        .family(ReadFamily)
        .bind(&installed_domain, ReadVertex)
        .unwrap();
    let graph_participation = workspace
        .graph_participation(ConditionalModelGraph)
        .unwrap();
    let mut signal_graph = worth_signal::facade::SignalGraph::new();
    let node = signal_graph.node().build();
    let worth_proof::TransitionOutcome::Success(node_capability) =
        signal_graph.admit_installed_node(node)
    else {
        panic!("installed Signal node capability")
    };
    let aspect = worth_signal::facade::Aspect::new(0);
    let worth_proof::TransitionOutcome::Success(aspect_capability) =
        signal_graph.admit_installed_aspect(node, aspect)
    else {
        panic!("installed Signal aspect capability")
    };
    let target = worth_runtime_bridge::facade::BridgeSignalAspectTargetDeclaration::exact(
        worth_runtime_bridge::facade::BridgeAspectRegistrationId::from_stable_name(
            "conditional-identity",
        ),
        worth_signal::facade::PartitionToken::new("geometry-signal"),
        node_capability,
        aspect_capability,
    )
    .unwrap();
    let registration = operation
        .semantic_correspondence_registration(
            geometry_node_location(),
            0,
            &graph_participation,
            Some(fixture_record_identity()),
            vec![target],
        )
        .unwrap();
    let (bridge, publication_request) = correspondence_bridge(registration);
    let mut graph_binding = bridge.bind_signal_graph(&mut signal_graph).unwrap();

    let worth_proof::TransitionOutcome::Success(installed) = operation
        .install_semantic_correspondence(
            geometry_node_location(),
            0,
            &graph_participation,
            Some(fixture_record_identity()),
            &mut graph_binding,
        )
    else {
        panic!("Query should retain the installed correspondence authority")
    };
    assert_eq!(installed.installation_generation(), 1);
    assert_eq!(installed.target_count(), 1);
    assert!(!installed.graph_participation_identity().is_empty());
    let worth_proof::TransitionOutcome::Success(counters) =
        installed.deliver_authoritative_change(&mut graph_binding, publication_request)
    else {
        panic!("the real Relational publication should drive Signal invalidation")
    };
    assert_eq!(counters.truth_targets_admitted(), 1);
    assert_eq!(counters.signal_seeds_emitted(), 1);
    drop(graph_binding);
    assert_eq!(
        signal_graph.node_aspect_version(node).unwrap().get(aspect),
        1
    );
}

#[test]
fn foreign_runtime_graph_participation_denies_before_bridge_admission() {
    let first = conditional_workspace(
        "candidate-runtime-first",
        node(
            "geometry",
            domain::WorthQueryComparatorRequirement::ExactCanonicalValue,
            domain::WorthQuerySemanticLocality::SourceRecord,
        ),
    )
    .unwrap();
    let second = conditional_workspace(
        "candidate-runtime-second",
        node(
            "geometry",
            domain::WorthQueryComparatorRequirement::ExactCanonicalValue,
            domain::WorthQuerySemanticLocality::SourceRecord,
        ),
    )
    .unwrap();
    let domain = first.domain(GeometryDomain).unwrap();
    let operating_world = first.observe_operating_world().unwrap();
    let operation = operating_world
        .family(ReadFamily)
        .bind(&domain, ReadVertex)
        .unwrap();
    let foreign_graph = second.graph_participation(ConditionalModelGraph).unwrap();

    assert!(matches!(
        operation.semantic_correspondence_registration(
            geometry_node_location(),
            0,
            &foreign_graph,
            Some(fixture_record_identity()),
            Vec::new(),
        ),
        Err(denial) if denial.kind()
            == worth_runtime_bridge::facade::BridgeCorrespondenceDenialKind::GraphParticipationNotOwnedByOperation
            && denial.counters()
                == worth_runtime_bridge::facade::CorrespondenceAdmissionCounters::zero()
    ));
}

#[test]
fn dependency_outside_operation_graph_scope_fails_package_installation() {
    let dependency = dependency_for_role(
        "foreign-model",
        domain::WorthQuerySemanticLocality::SourceRecord,
    );
    let node = conditional_declaration(
        "foreign-read-role",
        domain::WorthQueryConditionalNodeRole::Computed,
        vec![dependency],
        vec![domain::WorthQueryConditionalNodeOutput::OperationOutput {
            projection_role: domain::WorthQueryOperationProjectionRole::new("vertex").unwrap(),
        }],
        domain::WorthQueryConditionalNodeContext::Basis,
        domain::WorthQueryOutputRelationship::ContributesToOperationOutput,
    )
    .unwrap();

    assert!(conditional_workspace("foreign-conditional-read-role", node).is_err());
}

#[test]
fn undeclared_operation_output_role_fails_package_installation() {
    let dependency = dependency(domain::WorthQuerySemanticLocality::SourceRecord);
    let node = conditional_declaration(
        "foreign-output-role",
        domain::WorthQueryConditionalNodeRole::Computed,
        vec![dependency],
        vec![domain::WorthQueryConditionalNodeOutput::OperationOutput {
            projection_role: domain::WorthQueryOperationProjectionRole::new("foreign").unwrap(),
        }],
        domain::WorthQueryConditionalNodeContext::Basis,
        domain::WorthQueryOutputRelationship::ContributesToOperationOutput,
    )
    .unwrap();

    assert!(conditional_workspace("foreign-conditional-output-role", node).is_err());
}

#[test]
fn derived_consequence_cannot_invent_touch_authority() {
    let dependency = dependency(domain::WorthQuerySemanticLocality::SourceRecord);
    let touch = domain::WorthQueryConditionalTouchRole::new("model", "geometry").unwrap();
    let node = conditional_declaration(
        "foreign-touch-role",
        domain::WorthQueryConditionalNodeRole::Computed,
        vec![dependency],
        vec![
            domain::WorthQueryConditionalNodeOutput::DerivedAspect {
                contract: derived_contract(),
                locality: domain::WorthQuerySemanticLocality::SourceRecord,
                consequences: vec![domain::WorthQueryConditionalConsequenceRole::Touch(touch)],
            },
            domain::WorthQueryConditionalNodeOutput::OperationOutput {
                projection_role: domain::WorthQueryOperationProjectionRole::new("vertex").unwrap(),
            },
        ],
        domain::WorthQueryConditionalNodeContext::Basis,
        domain::WorthQueryOutputRelationship::ContributesToOperationOutput,
    )
    .unwrap();

    assert!(conditional_workspace("foreign-conditional-touch-role", node).is_err());
}

#[test]
fn conditional_role_and_attachment_are_one_installed_meaning() {
    let dependency = dependency(domain::WorthQuerySemanticLocality::SourceRecord);
    let stage_node = conditional_declaration(
        "publish-when-changed",
        domain::WorthQueryConditionalNodeRole::WorkflowStage,
        vec![dependency],
        vec![
            domain::WorthQueryConditionalNodeOutput::WorkflowStageOutput {
                contract: domain::WorthQueryWorkflowValueContract::Projection,
            },
        ],
        domain::WorthQueryConditionalNodeContext::WorkflowRun,
        domain::WorthQueryOutputRelationship::IsWorkflowStageOutput,
    )
    .unwrap();

    let workspace =
        conditional_workflow_workspace("conditional-workflow-stage", stage_node.clone())
            .expect("workflow-stage conditional installs through the ordinary package path");
    let installed_domain = workspace.domain(GeometryDomain).unwrap();
    let operating_world = workspace.observe_operating_world().unwrap();
    let operation = operating_world
        .family(ReadFamily)
        .bind(&installed_domain, WorkflowRead)
        .unwrap();
    let domain::WorthQueryOperationWorkflowContract::Declared(workflow) =
        &operation.definition().semantics().workflow
    else {
        panic!("installed workflow contract")
    };
    assert_eq!(
        workflow
            .stages()
            .iter()
            .find(|stage| stage.identity() == "publish")
            .unwrap()
            .semantics()
            .conditional_nodes,
        vec![stage_node.clone()]
    );

    assert!(conditional_workspace("stage-role-at-operation", stage_node).is_err());
}

#[test]
fn output_relationship_cannot_claim_an_output_the_node_does_not_declare() {
    let dependency = dependency(domain::WorthQuerySemanticLocality::SourceRecord);
    let result = conditional_declaration(
        "dishonest-output-relationship",
        domain::WorthQueryConditionalNodeRole::Computed,
        vec![dependency],
        vec![domain::WorthQueryConditionalNodeOutput::DerivedAspect {
            contract: derived_contract(),
            locality: domain::WorthQuerySemanticLocality::SourceRecord,
            consequences: vec![domain::WorthQueryConditionalConsequenceRole::DerivedOnly],
        }],
        domain::WorthQueryConditionalNodeContext::Basis,
        domain::WorthQueryOutputRelationship::ContributesToOperationOutput,
    );

    assert_eq!(
        result.unwrap_err(),
        "conditional-node-output-relationship-missing-output"
    );
}

fn conditional_declaration(
    identity: &str,
    role: domain::WorthQueryConditionalNodeRole,
    dependencies: Vec<domain::WorthQuerySemanticTruthDependency>,
    outputs: Vec<domain::WorthQueryConditionalNodeOutput>,
    context: domain::WorthQueryConditionalNodeContext,
    relationship: domain::WorthQueryOutputRelationship,
) -> Result<domain::WorthQueryPortableConditionalNodeDeclaration, &'static str> {
    let condition =
        domain::WorthQueryConditionalEvaluationCondition::aspect_filtered(dependencies.clone())
            .unwrap();
    domain::WorthQueryPortableConditionalNodeDeclaration::declare(identity, role)
        .dependencies(dependencies)
        .outputs(outputs)
        .required_context([context])
        .evaluation(
            condition,
            domain::WorthQueryConditionalTrigger::DependencyChange,
        )
        .comparison(
            domain::WorthQueryComparatorRequirement::ExactCanonicalValue,
            domain::WorthQueryOutputEquivalenceRequirement::ExactCanonicalValue,
        )
        .artifact_policy(
            domain::WorthQueryArtifactReuseEquivalence::NotReusable,
            domain::WorthQueryMaintenancePosture::LazyUntilObserved,
            domain::WorthQueryArtifactPosture::Ephemeral,
        )
        .output_relationship(relationship)
        .finish()
}

fn derived_contract() -> AspectContract {
    let field = FieldDeclaration::new(
        FieldKey::new("derived-id").unwrap(),
        ScalarAspectType::String,
        FieldRequirement::Required,
        AbsenceLaw::Required,
        AspectEvolutionPolicy::ExplicitBreakRequired,
    )
    .unwrap();
    AspectContract::struct_aspect(
        AspectKey::new("derived-identity").unwrap(),
        AspectIdentity(0x9140_0002),
        AspectContractRevision(1),
        StructAspectShape::new([field]).unwrap(),
    )
}
