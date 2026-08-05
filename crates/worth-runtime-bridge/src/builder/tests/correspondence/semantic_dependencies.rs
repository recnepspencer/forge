use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};

use worth_foundational::facade::{AspectMask, ProjectionMask};
use worth_signal::facade::{SignalConditionalArtifactReuse, SignalConditionalVersionComparator};

use super::semantic_fixture::{contract, field_path};
use super::{
    AspectBinding, AuthoritativeAspectChangeKind, BridgeSemanticDependencyCandidate,
    BridgeSemanticLocality, MAX_ASPECTS,
};
use crate::facade::{
    BridgeConditionalCondition, BridgeConditionalContract, BridgeConditionalContractParts,
    BridgeSemanticDependencyCandidateParts,
};

static NEXT_INSTALLATION: AtomicU64 = AtomicU64::new(2);
static DEPENDENCIES: OnceLock<BTreeMap<String, BridgeSemanticDependencyCandidate>> =
    OnceLock::new();

pub(super) fn dependency(label: &str) -> BridgeSemanticDependencyCandidate {
    installed_dependencies()
        .get(label)
        .unwrap_or_else(|| panic!("missing semantic dependency fixture `{label}`"))
        .clone()
}

pub(super) fn freshly_installed_dependency(label: &str) -> BridgeSemanticDependencyCandidate {
    build_dependency(label, NEXT_INSTALLATION.fetch_add(1, Ordering::Relaxed))
}

fn installed_dependencies() -> &'static BTreeMap<String, BridgeSemanticDependencyCandidate> {
    DEPENDENCIES.get_or_init(|| {
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
        .collect::<Vec<_>>();
        labels.extend((0..MAX_ASPECTS).map(|slot| format!("query:{slot}")));
        labels
            .into_iter()
            .map(|label| (label.clone(), build_dependency(&label, 1)))
            .collect()
    })
}

fn build_dependency(label: &str, installation: u64) -> BridgeSemanticDependencyCandidate {
    let locality = if label == "query:partition" {
        BridgeSemanticLocality::SourcePartition(
            worth_foundational::facade::TruthPartitionRole::new("model-main").unwrap(),
        )
    } else {
        BridgeSemanticLocality::SourceRecord
    };
    BridgeSemanticDependencyCandidate::admit(BridgeSemanticDependencyCandidateParts {
        source_installation_identity: Arc::from(format!("bridge-test-installation:{installation}")),
        source_basis: Arc::from("bridge-correspondence:1"),
        source_runtime_authority: installation,
        source_installation_generation: installation,
        source_authority_binding_identity: Arc::from(format!(
            "bridge-test-binding:{installation}:{label}"
        )),
        source_stage_identity: None,
        source_node_identity: Arc::from(label),
        dependency_ordinal: 0,
        declared_graph_role: Arc::from("model"),
        graph_participation_identity: Arc::from(format!(
            "bridge-test-graph-authority:{installation}"
        )),
        graph_adapter_identity: Arc::from("bridge-test-graph-adapter"),
        source_record_identity: (label != "query:partition").then(|| {
            crate::relational_identity::RelationalBridgeRecordIdentityParts::entity(0, 1, 1)
        }),
        contract: contract(),
        projection_mask: AspectMask::<ProjectionMask>::new([field_path()]),
        binding: AspectBinding::EntityField {
            field: worth_foundational::facade::FieldKey::new("profile").unwrap(),
        },
        locality,
        relevant_changes: vec![AuthoritativeAspectChangeKind::FieldSet],
    })
    .expect("neutral semantic dependency fixture is valid")
}

pub(super) fn conditional_contract(label: &str) -> BridgeConditionalContract {
    build_conditional_contract(label, BridgeConditionalCondition::AspectFiltered, false)
}

pub(super) fn always_eligible_contract(label: &str) -> BridgeConditionalContract {
    build_conditional_contract(label, BridgeConditionalCondition::Always, false)
}

pub(super) fn on_demand_contract(label: &str) -> BridgeConditionalContract {
    build_conditional_contract(label, BridgeConditionalCondition::OnDemand, false)
}

pub(super) fn temporal_contract(label: &str) -> BridgeConditionalContract {
    build_conditional_contract(label, BridgeConditionalCondition::TemporalWake, false)
}

pub(super) fn registered_comparator_contract(label: &str) -> BridgeConditionalContract {
    build_conditional_contract(label, BridgeConditionalCondition::AspectFiltered, true)
}

pub(super) fn runtime_predicate_contract(label: &str) -> BridgeConditionalContract {
    build_conditional_contract(label, BridgeConditionalCondition::RuntimePredicate, false)
}

fn build_conditional_contract(
    label: &str,
    condition: BridgeConditionalCondition,
    runtime_comparator: bool,
) -> BridgeConditionalContract {
    let condition_dependency_ordinals = matches!(
        condition,
        BridgeConditionalCondition::AspectFiltered | BridgeConditionalCondition::DeltaThreshold(_)
    )
    .then_some(0)
    .into_iter()
    .collect();
    BridgeConditionalContract::new(BridgeConditionalContractParts {
        identity: Arc::from(label),
        dependency_count: 1,
        condition_dependency_ordinals,
        condition,
        dependency_comparator: if runtime_comparator {
            SignalConditionalVersionComparator::RuntimeResolved
        } else {
            SignalConditionalVersionComparator::Exact
        },
        output_comparator: SignalConditionalVersionComparator::Exact,
        artifact_reuse: SignalConditionalArtifactReuse::NotReusable,
    })
}
