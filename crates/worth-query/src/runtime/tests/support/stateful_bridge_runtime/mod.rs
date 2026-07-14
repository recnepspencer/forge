use super::*;

mod backend;
mod merge;
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
        .domain_package(package)
        .expect("test domain package should admit")
        .build()
        .expect("stateful bridge runtime with domain should build")
}

pub(crate) fn stateful_bridge_task_runtime_with_merge() -> WorthQueryRuntime {
    let mut relational =
        crate::harness::fixtures::effect_authorities::relational_runtime_with_intent_strategy();
    crate::harness::fixtures::effect_authorities::create_entity(
        &mut relational,
        "main",
        BranchId("main".to_string()),
    );
    relational
        .history_authority()
        .create_branch(
            BranchId("candidate".to_string()),
            &BranchId("main".to_string()),
        )
        .expect("candidate branch should be created");
    crate::harness::fixtures::effect_authorities::create_entity(
        &mut relational,
        "candidate-only",
        BranchId("candidate".to_string()),
    );
    let installed_collections = ["Task".to_string()].into_iter().collect();
    let state = Rc::new(RefCell::new(
        StatefulBridgeState::new(installed_collections).with_relational_runtime(relational),
    ));
    WorthQueryRuntime::builder()
        .backend(StatefulBridgeRuntimeBackend::new(
            state,
            graph_test_support_profile(),
        ))
        .build()
        .expect("stateful bridge runtime with merge should build")
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
        .build()
        .expect("stateful bridge-backed runtime should build")
}

fn graph_test_support_profile() -> WorthQueryRuntimeSupportProfile {
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
