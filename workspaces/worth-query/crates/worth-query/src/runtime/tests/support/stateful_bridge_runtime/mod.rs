use super::*;

mod backend;
mod merge;
mod projection_paths;
mod state;
mod verification;
mod writes;

use std::cell::RefCell;
use std::collections::BTreeSet;
use std::rc::Rc;

use backend::StatefulBridgeRuntimeBackend;
use state::StatefulBridgeState;
use worth_relational::facade::history::BranchId;

type SharedState = Rc<RefCell<StatefulBridgeState>>;

pub(crate) struct StatefulBridgeMergeProbe {
    state: SharedState,
}

impl StatefulBridgeMergeProbe {
    pub(crate) fn main_entity_count(&self) -> usize {
        let mut state = self.state.borrow_mut();
        let runtime = state
            .relational_runtime
            .as_mut()
            .expect("merge probe requires its Relational owner");
        let snapshot =
            crate::harness::fixtures::effect_authorities::exact_branch_snapshot(runtime, "main");
        let read = runtime
            .read_truth()
            .read_snapshot(&snapshot)
            .expect("merge probe reads the exact target branch snapshot");
        read.entities().len()
    }
}

pub(in crate::runtime::tests) fn custom_backend_without_primary_graph_transfer_builder(
) -> WorthQueryRuntimeBuilder {
    let state = Rc::new(RefCell::new(StatefulBridgeState::new(
        ["Task".to_string()].into_iter().collect(),
    )));
    WorthQueryRuntime::builder()
        .backend(StatefulBridgeRuntimeBackend::new(
            state,
            graph_test_support_profile(),
        ))
        .aspect_contracts(stateful_bridge_aspect_contracts())
        .expect("stateful bridge aspect contracts should admit")
}

pub(crate) fn stateful_bridge_task_runtime() -> WorthQueryRuntime {
    stateful_bridge_runtime_via_custom_backend(["Task"], graph_test_support_profile())
}

pub(in crate::runtime::tests) fn stateful_bridge_task_runtime_with_domain<D>(
    package: crate::domain_installation::WorthQueryDomainPackage<D>,
) -> WorthQueryRuntime
where
    D: crate::application::WorthQueryDomainEntryMarker + 'static,
{
    let state = Rc::new(RefCell::new(StatefulBridgeState::new(
        ["Task".to_string()].into_iter().collect(),
    )));
    WorthQueryRuntime::builder()
        .backend(StatefulBridgeRuntimeBackend::new(
            state,
            graph_test_support_profile(),
        ))
        .aspect_contracts(stateful_bridge_aspect_contracts())
        .expect("stateful bridge aspect contracts should admit")
        .domain_package(package)
        .expect("test domain package should admit")
        .build()
        .expect("stateful bridge runtime with domain should build")
}

pub(crate) fn stateful_bridge_task_runtime_with_merge() -> WorthQueryRuntime {
    stateful_bridge_task_runtime_with_merge_posture(false).0
}

pub(crate) fn stateful_bridge_task_runtime_with_merge_durable_fault(
) -> (WorthQueryRuntime, StatefulBridgeMergeProbe) {
    stateful_bridge_task_runtime_with_merge_posture(true)
}

fn stateful_bridge_task_runtime_with_merge_posture(
    fail_next_durable_append: bool,
) -> (WorthQueryRuntime, StatefulBridgeMergeProbe) {
    let mut relational =
        crate::harness::fixtures::effect_authorities::relational_runtime_with_intent_strategy();
    crate::harness::fixtures::effect_authorities::create_entity(
        &mut relational,
        "main",
        BranchId("main".to_string()),
    );
    let (_, basis) = relational
        .observe_fork_source(&BranchId("main".to_string()))
        .expect("main branch should expose an exact fork source");
    relational
        .fork_branch(BranchId("candidate".to_string()), basis)
        .expect("candidate branch should be created");
    crate::harness::fixtures::effect_authorities::create_entity(
        &mut relational,
        "candidate-only",
        BranchId("candidate".to_string()),
    );
    if fail_next_durable_append {
        relational.fail_next_durable_append_for_test();
    }
    let installed_collections = ["Task".to_string()].into_iter().collect();
    let state = Rc::new(RefCell::new(
        StatefulBridgeState::new(installed_collections).with_relational_runtime(relational),
    ));
    let probe = StatefulBridgeMergeProbe {
        state: Rc::clone(&state),
    };
    let runtime = WorthQueryRuntime::builder()
        .backend(StatefulBridgeRuntimeBackend::new(
            state,
            graph_test_support_profile(),
        ))
        .aspect_contracts(stateful_bridge_aspect_contracts())
        .expect("stateful bridge aspect contracts should admit")
        .build()
        .expect("stateful bridge runtime with merge should build");
    (runtime, probe)
}

pub(crate) fn stateful_bridge_task_runtime_without_writeback() -> WorthQueryRuntime {
    let installed_collections = ["Task".to_string()].into_iter().collect::<BTreeSet<_>>();
    let state = Rc::new(RefCell::new(StatefulBridgeState::with_bridge(
        installed_collections,
        super::test_bridge_without_writeback_authority(),
    )));
    WorthQueryRuntime::builder()
        .backend(StatefulBridgeRuntimeBackend::new(
            state,
            graph_test_support_profile(),
        ))
        .aspect_contracts(stateful_bridge_aspect_contracts())
        .expect("stateful bridge aspect contracts should admit")
        .build()
        .expect("stateful bridge runtime without writeback should build")
}

pub(crate) fn stateful_bridge_task_issue_runtime() -> WorthQueryRuntime {
    stateful_bridge_runtime_via_custom_backend(["Task", "Issue"], graph_test_support_profile())
}

pub(crate) fn stateful_bridge_grouped_task_runtime() -> WorthQueryRuntime {
    stateful_bridge_runtime_via_custom_backend(["Task"], graph_test_support_profile())
}

pub(in crate::runtime::tests) fn stateful_bridge_task_edge_runtime() -> WorthQueryRuntime {
    stateful_bridge_runtime_via_custom_backend(["Task", "TaskEdge"], graph_test_support_profile())
}

pub(in crate::runtime::tests) fn stateful_bridge_task_relation_runtime() -> WorthQueryRuntime {
    stateful_bridge_runtime_via_custom_backend(
        ["Task", "TaskRelation"],
        graph_test_support_profile(),
    )
}

pub(in crate::runtime::tests) fn stateful_bridge_vertex_runtime() -> WorthQueryRuntime {
    stateful_bridge_runtime_via_custom_backend(["Vertex"], graph_test_support_profile())
}

pub(in crate::runtime::tests) fn stateful_bridge_runtime_with_collections(
    collections: &[&'static str],
) -> WorthQueryRuntime {
    stateful_bridge_runtime_with_support(collections, graph_test_support_profile())
}

pub(in crate::runtime::tests) fn stateful_bridge_runtime_with_support(
    collections: &[&'static str],
    support_profile: WorthQueryRuntimeSupportProfile,
) -> WorthQueryRuntime {
    stateful_bridge_runtime_via_custom_backend(collections.iter().copied(), support_profile)
}

fn stateful_bridge_runtime_via_custom_backend(
    collections: impl IntoIterator<Item = &'static str>,
    support_profile: WorthQueryRuntimeSupportProfile,
) -> WorthQueryRuntime {
    let installed_collections = collections
        .into_iter()
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    let state = Rc::new(RefCell::new(StatefulBridgeState::new(
        installed_collections.clone(),
    )));
    WorthQueryRuntime::builder()
        .backend(StatefulBridgeRuntimeBackend::new(state, support_profile))
        .aspect_contracts(stateful_bridge_aspect_contracts())
        .expect("stateful bridge aspect contracts should admit")
        .build()
        .expect("stateful bridge-backed runtime should build")
}

pub(in crate::runtime::tests) fn graph_test_support_profile() -> WorthQueryRuntimeSupportProfile {
    WorthQueryRuntimeSupportProfile::bridge_backed(
        "test-subscription-activation",
        "test-preview-basis",
        "test-inspector-evidence",
    )
    .with_family_support(WorthQueryRuntimeFamilySupport::supported(
        WorthQueryRuntimeFacadeFamily::Write,
        [WorthQueryAuthorityLane::AuthoritativeTruth],
        [],
        ["test-write-authority"],
    ))
    .with_bridge_backed_verification_support(
        "probe_existing",
        "direct_entity_identity",
        true,
        true,
        None,
    )
    .with_bridge_backed_verification_support(
        "probe_existing",
        "direct_relation_identity",
        true,
        true,
        None,
    )
}
