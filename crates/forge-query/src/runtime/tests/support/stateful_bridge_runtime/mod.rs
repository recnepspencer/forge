use super::*;

mod authority;
mod backend;
mod state;
mod verification;
mod writes;

use std::cell::RefCell;
use std::collections::BTreeSet;
use std::rc::Rc;

use backend::StatefulBridgeRuntimeBackend;
use state::StatefulBridgeState;

pub(crate) fn stateful_bridge_task_runtime() -> ForgeQueryRuntime {
    stateful_bridge_runtime(["Task"], graph_test_support_profile())
}

pub(crate) fn stateful_bridge_task_issue_runtime() -> ForgeQueryRuntime {
    stateful_bridge_runtime(["Task", "Issue"], graph_test_support_profile())
}

pub(crate) fn stateful_bridge_grouped_task_runtime() -> ForgeQueryRuntime {
    stateful_bridge_runtime(["Task"], graph_test_support_profile())
}

pub(in crate::runtime::tests) fn stateful_bridge_task_edge_runtime() -> ForgeQueryRuntime {
    stateful_bridge_runtime(["Task", "TaskEdge"], graph_test_support_profile())
}

pub(in crate::runtime::tests) fn stateful_bridge_task_relation_runtime() -> ForgeQueryRuntime {
    stateful_bridge_runtime(["Task", "TaskRelation"], graph_test_support_profile())
}

pub(in crate::runtime::tests) fn stateful_bridge_vertex_runtime() -> ForgeQueryRuntime {
    stateful_bridge_runtime(["Vertex"], graph_test_support_profile())
}

pub(in crate::runtime::tests) fn stateful_bridge_runtime_with_collections(
    collections: &[&'static str],
) -> ForgeQueryRuntime {
    stateful_bridge_runtime_with_support(collections, graph_test_support_profile())
}

pub(in crate::runtime::tests) fn stateful_bridge_runtime_with_support(
    collections: &[&'static str],
    support_profile: ForgeQueryRuntimeSupportProfile,
) -> ForgeQueryRuntime {
    stateful_bridge_runtime(collections.iter().copied(), support_profile)
}

fn stateful_bridge_runtime(
    collections: impl IntoIterator<Item = &'static str>,
    support_profile: ForgeQueryRuntimeSupportProfile,
) -> ForgeQueryRuntime {
    let installed_collections = collections
        .into_iter()
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    let state = Rc::new(RefCell::new(StatefulBridgeState::new(
        installed_collections.clone(),
    )));
    ForgeQueryRuntime::builder()
        .backend(StatefulBridgeRuntimeBackend::new(state, support_profile))
        .build()
        .expect("stateful bridge-backed runtime should build")
}

fn graph_test_support_profile() -> ForgeQueryRuntimeSupportProfile {
    ForgeQueryRuntimeSupportProfile::bridge_backed(
        "test-subscription-activation",
        "test-preview-basis",
        "test-inspector-evidence",
    )
    .with_family_support(ForgeQueryRuntimeFamilySupport::supported(
        ForgeQueryRuntimeFacadeFamily::Write,
        [ForgeQueryAuthorityLane::AuthoritativeTruth],
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
