use std::collections::BTreeSet;
use std::sync::{Arc, OnceLock};

use worth_foundational::facade::{AspectMask, ProjectionMask};
use worth_query_declaration::facade::authoring::{
    AspectFieldSelector, AuthoredQueryBundleRequest, AuthoredResultShapeField, DetailQueryBuilder,
    DetailResultShapeBuilder, RootEntityKey,
};
use worth_query_declaration::facade::binding::QueryBindingDescriptor;
use worth_query_declaration::facade::canonicalization::canonicalize_request;
use worth_query_installation::facade as query;

use super::semantic_fixture::{contract, field_path};
use super::{
    AspectBinding, AuthoritativeAspectChangeKind, BridgeSemanticDependencyCandidate,
    BridgeSemanticLocality, MAX_ASPECTS,
};

static DEPENDENCIES: OnceLock<Vec<(String, BridgeSemanticDependencyCandidate)>> = OnceLock::new();

pub(super) fn dependency(label: &str) -> BridgeSemanticDependencyCandidate {
    installed_dependencies()
        .iter()
        .find(|(candidate_label, _)| candidate_label == label)
        .unwrap_or_else(|| panic!("missing installed Query dependency fixture `{label}`"))
        .1
        .clone()
}

fn installed_dependencies() -> &'static [(String, BridgeSemanticDependencyCandidate)] {
    DEPENDENCIES.get_or_init(|| {
        let labels = fixture_labels();
        let operation = operation_definition(&labels);
        let package = query::WorthQueryPortableDomainPackage::new(
            query::WorthQueryPortableDomainIdentity::new("worth.bridge-tests", 1, 0),
        )
        .domain_operation(operation)
        .validate()
        .expect("real correspondence Query package validates");
        let admitted = query::WorthQueryInstallationAdmissionProfile::new(
            "bridge-test-support-v1",
            "bridge-test-config-v1",
        )
        .admit(package)
        .expect("real correspondence Query package admits");
        let installation_runtime = query::WorthQueryInstallationRuntimeIdentity::fresh();
        let graph_authority = Arc::new(
            query::WorthQueryInstalledGraphParticipationAuthority::install(
                &installation_runtime,
                "model",
                "query-graph-adapter:bridge-tests",
                Arc::new(BridgeQueryGraphProviderFixture),
            )
            .expect("real installed graph participation authority"),
        );
        let index = query::WorthQueryInstalledPackageIndex::build(
            installation_runtime,
            query::WorthQueryInstallationGeneration::initial(),
            [admitted],
        )
        .expect("real correspondence Query package installs");
        let operation = index
            .domain_operation("worth.bridge-tests", "bridge-correspondence:1")
            .expect("installed correspondence operation authority");

        labels
            .into_iter()
            .map(|label| {
                let authority = operation
                    .conditional_dependency(
                        query::WorthQueryConditionalNodeLocation::operation(label.clone())
                            .expect("valid fixture node location"),
                        0,
                    )
                    .expect("exact installed conditional dependency authority");
                let entity_identity = (label != "query:partition").then(|| {
                    crate::relational_identity::RelationalBridgeRecordIdentityParts::entity(0, 1, 1)
                });
                let candidate = BridgeSemanticDependencyCandidate::from_query_authority(
                    authority,
                    Arc::clone(&graph_authority),
                    entity_identity,
                )
                .expect("installed Query dependency joins Bridge candidate");
                (label, candidate)
            })
            .collect()
    })
}

struct BridgeQueryGraphProviderFixture;

fn fixture_labels() -> Vec<String> {
    let mut labels = [
        "query:one",
        "query:first",
        "query:second",
        "query:overflow",
        "query:partition",
        "query:unregistered",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    labels.extend((0..MAX_ASPECTS).map(|slot| format!("query:{slot}")));
    labels.into_iter().collect()
}

fn operation_definition(labels: &[String]) -> query::WorthQueryPortableDomainOperationDefinition {
    let native_projection = query::WorthQueryOperationNativeProjectionContract {
        aspect_key: contract().key().clone(),
        aspect_identity: contract().identity(),
        contract_revision: contract().revision(),
        mask: AspectMask::<ProjectionMask>::new([field_path()]),
    };
    let semantics = query::WorthQueryDomainOperationSemanticClosure {
        parameters: query::WorthQueryOperationParameterContract::NotRequired,
        native_projection: native_projection.clone(),
        canonical_query: canonical_bundle(),
        collection: query::WorthQueryOperationCollectionContract::NotCollection,
        required_capabilities: Vec::new(),
        required_domains: Vec::new(),
        workflow: query::WorthQueryOperationWorkflowContract::NotRequired,
        conditional_nodes: labels.iter().map(|label| conditional_node(label)).collect(),
        graph_reads: query::WorthQueryOperationGraphReadContract::Declared {
            roles: vec![query::WorthQueryOperationGraphReadRole {
                role: "model".into(),
                participation: query::WorthQueryOperationGraphParticipation::PrimaryLogicalGraph,
                access: query::WorthQueryOperationGraphAccess::Project,
                semantic_reads: vec![native_projection],
            }],
        },
        touches: query::WorthQueryOperationTouchContract::NotRequired,
        effects: query::WorthQueryOperationEffectContract::NotRequired,
        invariants: query::WorthQueryOperationInvariantContract::NotRequired,
        replay: query::WorthQueryOperationReplayContract::ReExecutable,
        reversal: query::WorthQueryOperationReversalContract::Irreversible,
        lineage: query::WorthQueryOperationLineageContract::NotRequired,
        promotion: query::WorthQueryOperationPromotionContract::NotRequired,
        publication: query::WorthQueryOperationPublicationContract::DerivedProjection {
            projection_role: query::WorthQueryOperationProjectionRole::new("profile").unwrap(),
        },
        projection_consumption:
            query::WorthQueryOperationProjectionConsumptionContract::QueryReadAuthority,
        terminal: query::WorthQueryOperationTerminalContract {
            result_states: vec![query::WorthQueryOperationResultState::Ready],
            failure_classes: vec![query::WorthQueryOperationFailureClass::Dependency],
        },
        cost: query::WorthQueryOperationCostContract {
            lookup: query::WorthQueryOperationCostClass::Constant,
            execution: query::WorthQueryOperationCostClass::DeclaredWidth,
            result_width: query::WorthQueryOperationCostClass::DeclaredWidth,
        },
        support: query::WorthQueryOperationSupportRequirements {
            live: query::WorthQuerySupportRequirement::NotRequired,
            continuation: query::WorthQuerySupportRequirement::NotRequired,
            async_result_state: query::WorthQuerySupportRequirement::NotRequired,
            recovery: query::WorthQuerySupportRequirement::NotRequired,
            inspection: query::WorthQuerySupportRequirement::NotRequired,
            projection_consumption: query::WorthQuerySupportRequirement::NotRequired,
            dependency_impact: query::WorthQuerySupportRequirement::NotRequired,
            sharing: query::WorthQuerySupportRequirement::NotRequired,
            invalidation: query::WorthQuerySupportRequirement::NotRequired,
            collection_delivery: query::WorthQuerySupportRequirement::NotRequired,
            conditional_evaluation: query::WorthQuerySupportRequirement::NotRequired,
            conditional_comparator: query::WorthQuerySupportRequirement::NotRequired,
            conditional_trigger: query::WorthQuerySupportRequirement::NotRequired,
            conditional_temporal_or_on_demand: query::WorthQuerySupportRequirement::NotRequired,
        },
        lowering: query::WorthQueryOperationLoweringContract {
            family: "bridge-correspondence-test".into(),
            deterministic: true,
        },
    };
    query::WorthQueryDomainOperationDefinition::<(), (), ()>::new(
        query::WorthQueryDomainOperationIdentity::new("bridge-correspondence", 1),
        semantics,
    )
    .into_portable()
}

fn conditional_node(label: &str) -> query::WorthQueryPortableConditionalNodeDeclaration {
    let locality = if label == "query:partition" {
        BridgeSemanticLocality::SourcePartition(
            worth_foundational::facade::TruthPartitionRole::new("model-main").unwrap(),
        )
    } else {
        BridgeSemanticLocality::SourceRecord
    };
    let dependency = query::WorthQuerySemanticTruthDependency::new(
        query::WorthQueryConditionalGraphReadRole::new("model").unwrap(),
        contract(),
        AspectMask::new([field_path()]),
        AspectBinding::EntityField {
            field: worth_foundational::facade::FieldKey::new("profile").unwrap(),
        },
        locality,
        [AuthoritativeAspectChangeKind::FieldSet],
    )
    .unwrap();
    query::WorthQueryPortableConditionalNodeDeclaration::declare(
        label,
        query::WorthQueryConditionalNodeRole::Computed,
    )
    .dependencies([dependency.clone()])
    .outputs([query::WorthQueryConditionalNodeOutput::OperationOutput {
        projection_role: query::WorthQueryOperationProjectionRole::new("profile").unwrap(),
    }])
    .required_context([query::WorthQueryConditionalNodeContext::Snapshot])
    .evaluation(
        query::WorthQueryConditionalEvaluationCondition::aspect_filtered([dependency]).unwrap(),
        query::WorthQueryConditionalTrigger::DependencyChange,
    )
    .comparison(
        query::WorthQueryComparatorRequirement::FoundationalContractEquivalence,
        query::WorthQueryOutputEquivalenceRequirement::ExactCanonicalValue,
    )
    .artifact_policy(
        query::WorthQueryArtifactReuseEquivalence::NotReusable,
        query::WorthQueryMaintenancePosture::LazyUntilObserved,
        query::WorthQueryArtifactPosture::Ephemeral,
    )
    .output_relationship(query::WorthQueryOutputRelationship::ContributesToOperationOutput)
    .finish()
    .unwrap()
}

fn canonical_bundle() -> worth_query_declaration::facade::canonicalization::CanonicalQueryBundle {
    let selector = AspectFieldSelector::new("profile", "name").unwrap();
    let query = DetailQueryBuilder::new(RootEntityKey::new("BridgeFixture").unwrap())
        .project(selector)
        .build()
        .unwrap()
        .into_raw();
    let shape = DetailResultShapeBuilder::new()
        .field(AuthoredResultShapeField::new("profile", "name", "name").unwrap())
        .build()
        .unwrap()
        .into_raw();
    canonicalize_request(
        AuthoredQueryBundleRequest::new(query, shape, QueryBindingDescriptor::new()).unwrap(),
    )
    .unwrap()
}
