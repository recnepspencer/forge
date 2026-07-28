use std::collections::BTreeSet;
use std::sync::{Arc, OnceLock};

use worth_foundational::facade::{AspectMask, ProjectionMask};
use worth_query_installation::facade as query;

use super::semantic_fixture::{contract, field_path};
use super::{
    AspectBinding, AuthoritativeAspectChangeKind, BridgeSemanticDependencyCandidate,
    BridgeSemanticLocality, MAX_ASPECTS,
};

mod canonical_bundle;

use canonical_bundle::canonical_bundle;

static DEPENDENCIES: OnceLock<Vec<(String, BridgeSemanticDependencyCandidate)>> = OnceLock::new();

pub(super) fn dependency(label: &str) -> BridgeSemanticDependencyCandidate {
    installed_dependencies()
        .iter()
        .find(|(candidate_label, _)| candidate_label == label)
        .unwrap_or_else(|| panic!("missing installed Query dependency fixture `{label}`"))
        .1
        .clone()
}

pub(super) fn freshly_installed_dependency(label: &str) -> BridgeSemanticDependencyCandidate {
    build_installed_dependencies()
        .into_iter()
        .find(|(candidate_label, _)| candidate_label == label)
        .unwrap_or_else(|| panic!("missing fresh installed Query dependency fixture `{label}`"))
        .1
}

fn installed_dependencies() -> &'static [(String, BridgeSemanticDependencyCandidate)] {
    DEPENDENCIES.get_or_init(build_installed_dependencies)
}

fn build_installed_dependencies() -> Vec<(String, BridgeSemanticDependencyCandidate)> {
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
            false,
            Option::<String>::None,
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
    let native_projection = query::WorthQueryOperationNativeProjectionContract::new(
        contract(),
        AspectMask::<ProjectionMask>::new([field_path()]),
    )
    .expect("the fixture projection mask is admitted by its native contract");
    let semantics = query::WorthQueryDomainOperationSemanticClosure {
        parameters: query::WorthQueryOperationParameterContract::NotRequired,
        native_projection: native_projection.clone(),
        canonical_query: canonical_bundle(),
        collection: query::WorthQueryOperationCollectionContract::NotCollection,
        required_capabilities: Vec::new(),
        required_domains: Vec::new(),
        workflow: query::WorthQueryOperationWorkflowContract::NotRequired,
        evidence: query::WorthQueryDomainEvidenceContract::NotRequired,
        conditional_nodes: labels.iter().map(|label| conditional_node(label)).collect(),
        graph_reads: query::WorthQueryOperationGraphReadContract::Declared {
            roles: vec![query::WorthQueryOperationGraphReadRole {
                role: "model".into(),
                participation: query::WorthQueryOperationGraphParticipation::PrimaryLogicalGraph,
                access: query::WorthQueryOperationGraphAccess::Project,
                semantic_reads: vec![native_projection],
            }],
        },
        decision_facts: query::WorthQueryOperationDecisionFactContract::NotRequired,
        touches: query::WorthQueryOperationTouchContract::NotRequired,
        effects: query::WorthQueryOperationEffectContract::NotRequired,
        invariants: query::WorthQueryOperationInvariantContract::NotRequired,
        invariant_execution: query::WorthQueryInvariantExecutionContract::NotRequired,
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
        resources: execution_resource_contract(),
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

fn execution_resource_contract() -> query::WorthQueryOperationExecutionResourceContract {
    query::WorthQueryExecutionResourceContract::declared([
        query::WorthQueryExecutionStrategyContract::new(
            query::WorthQueryExecutionStrategyName::new("bridge-correspondence-bounded").unwrap(),
            query::WorthQueryExecutionResourceEnvelope::bounded(
                1_000_000,
                1_000_000,
                worth_query_declaration::facade::domain_computation::WorthQueryExecutionMode::Synchronous,
                worth_query_declaration::facade::domain_computation::WorthQueryCancellationSafePointFamily::new(
                    "bridge-correspondence-boundary",
                )
                .unwrap(),
            ),
            query::WorthQueryExecutionProviderRequirements::new(
                query::WorthQueryExecutionProviderFamily::new(
                    "bridge-correspondence-provider",
                )
                .unwrap(),
                query::WorthQueryExecutionAccessProductFamily::new(
                    "bridge-correspondence-access",
                )
                .unwrap(),
                query::WorthQueryExecutionAllocatorFamily::new(
                    "bridge-correspondence-arena",
                )
                .unwrap(),
            ),
        ),
    ])
    .expect("bridge correspondence fixture resource contract should be bounded")
}

pub(super) fn conditional_node(label: &str) -> query::WorthQueryPortableConditionalNodeDeclaration {
    conditional_node_with_posture(label, FixtureConditionalPosture::Default)
}

pub(super) fn conditional_node_always_eligible(
    label: &str,
) -> query::WorthQueryPortableConditionalNodeDeclaration {
    conditional_node_with_posture(label, FixtureConditionalPosture::AlwaysEligible)
}

pub(super) fn conditional_node_on_demand(
    label: &str,
) -> query::WorthQueryPortableConditionalNodeDeclaration {
    conditional_node_with_posture(label, FixtureConditionalPosture::OnDemand)
}

pub(super) fn conditional_node_temporal(
    label: &str,
) -> query::WorthQueryPortableConditionalNodeDeclaration {
    conditional_node_with_posture(label, FixtureConditionalPosture::Temporal)
}

pub(super) fn conditional_node_registered_comparator(
    label: &str,
) -> query::WorthQueryPortableConditionalNodeDeclaration {
    conditional_node_with_posture(label, FixtureConditionalPosture::RegisteredComparator)
}

pub(super) fn conditional_node_domain_condition(
    label: &str,
) -> query::WorthQueryPortableConditionalNodeDeclaration {
    conditional_node_with_posture(label, FixtureConditionalPosture::DomainCondition)
}

enum FixtureConditionalPosture {
    Default,
    AlwaysEligible,
    OnDemand,
    Temporal,
    RegisteredComparator,
    DomainCondition,
}

struct FixtureTrigger;
impl query::WorthQueryOnDemandTriggerFamily for FixtureTrigger {
    const PORTABLE_IDENTITY: &'static str = "worth.bridge-tests.trigger";
}

struct FixtureComparator;
impl query::WorthQueryComparatorFamily for FixtureComparator {
    const PORTABLE_IDENTITY: &'static str = "worth.bridge-tests.comparator";
}

struct FixtureCondition;
impl query::WorthQueryDomainConditionFamily for FixtureCondition {
    const PORTABLE_IDENTITY: &'static str = "worth.bridge-tests.condition";
}

fn conditional_node_with_posture(
    label: &str,
    posture: FixtureConditionalPosture,
) -> query::WorthQueryPortableConditionalNodeDeclaration {
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
    let filtered = || {
        query::WorthQueryConditionalEvaluationCondition::aspect_filtered([dependency.clone()])
            .unwrap()
    };
    let (condition, trigger, maintenance, dependency_comparator) =
        match posture {
            FixtureConditionalPosture::Default => (
                filtered(),
                query::WorthQueryConditionalTrigger::DependencyChange,
                query::WorthQueryMaintenancePosture::LazyUntilObserved,
                query::WorthQueryComparatorRequirement::FoundationalContractEquivalence,
            ),
            FixtureConditionalPosture::AlwaysEligible => (
                query::WorthQueryConditionalEvaluationCondition::always_eligible(),
                query::WorthQueryConditionalTrigger::DependencyChange,
                query::WorthQueryMaintenancePosture::LazyUntilObserved,
                query::WorthQueryComparatorRequirement::FoundationalContractEquivalence,
            ),
            FixtureConditionalPosture::OnDemand => (
                query::WorthQueryConditionalEvaluationCondition::on_demand(),
                query::WorthQueryConditionalTrigger::on_demand::<FixtureTrigger>(),
                query::WorthQueryMaintenancePosture::OnDemandOnly,
                query::WorthQueryComparatorRequirement::FoundationalContractEquivalence,
            ),
            FixtureConditionalPosture::Temporal => (
                query::WorthQueryConditionalEvaluationCondition::temporal(
                    query::WorthQueryTemporalCondition::IntervalNanoseconds(1),
                ),
                query::WorthQueryConditionalTrigger::Temporal(
                    query::WorthQueryTemporalWake::MonotonicClock,
                ),
                query::WorthQueryMaintenancePosture::Temporal,
                query::WorthQueryComparatorRequirement::FoundationalContractEquivalence,
            ),
            FixtureConditionalPosture::RegisteredComparator => (
                filtered(),
                query::WorthQueryConditionalTrigger::DependencyChange,
                query::WorthQueryMaintenancePosture::LazyUntilObserved,
                query::WorthQueryComparatorRequirement::registered::<FixtureComparator>(),
            ),
            FixtureConditionalPosture::DomainCondition => {
                (
                    query::WorthQueryConditionalEvaluationCondition::domain_specific::<
                        FixtureCondition,
                    >([])
                    .unwrap(),
                    query::WorthQueryConditionalTrigger::DependencyChange,
                    query::WorthQueryMaintenancePosture::LazyUntilObserved,
                    query::WorthQueryComparatorRequirement::FoundationalContractEquivalence,
                )
            }
        };
    query::WorthQueryPortableConditionalNodeDeclaration::declare(
        label,
        query::WorthQueryConditionalNodeRole::Computed,
    )
    .dependencies([dependency.clone()])
    .outputs([query::WorthQueryConditionalNodeOutput::OperationOutput {
        projection_role: query::WorthQueryOperationProjectionRole::new("profile").unwrap(),
    }])
    .required_context([query::WorthQueryConditionalNodeContext::Snapshot])
    .evaluation(condition, trigger)
    .comparison(
        dependency_comparator,
        query::WorthQueryOutputEquivalenceRequirement::ExactCanonicalValue,
    )
    .artifact_policy(
        query::WorthQueryArtifactReuseEquivalence::NotReusable,
        maintenance,
        query::WorthQueryArtifactPosture::Ephemeral,
    )
    .output_relationship(query::WorthQueryOutputRelationship::ContributesToOperationOutput)
    .finish()
    .unwrap()
}
