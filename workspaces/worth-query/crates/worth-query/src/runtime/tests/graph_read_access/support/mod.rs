use crate::runtime::{WorthQueryAspectTouch, WorthQueryRuntime, WorthQueryRuntimeSupportProfile};

pub mod graph_index_inventory;
pub mod graph_read_access;
pub mod graph_read_access_cost_model;

pub fn aspect_touch(authored_touch_text: &str) -> WorthQueryAspectTouch {
    crate::runtime::tests::support::test_aspect_touch(authored_touch_text)
}

pub mod public_bridge_runtime {
    use super::*;
    use crate::runtime::tests::support::{
        graph_test_support_profile, stateful_bridge_runtime_with_support,
    };

    pub fn public_graph_support_profile() -> WorthQueryRuntimeSupportProfile {
        graph_test_support_profile()
    }

    pub struct PublicBridgeRuntimeHarness;

    impl PublicBridgeRuntimeHarness {
        pub fn new() -> Self {
            Self
        }

        pub fn bridge_backed_runtime(&self) -> WorthQueryRuntime {
            self.bridge_backed_runtime_with_support(public_graph_support_profile())
        }

        pub fn bridge_backed_runtime_with_support(
            &self,
            support_profile: WorthQueryRuntimeSupportProfile,
        ) -> WorthQueryRuntime {
            stateful_bridge_runtime_with_support(
                &["Task", "TaskRelation", "user", "manager", "mentor"],
                support_profile,
            )
        }
    }

    pub fn workspace(name: &str) -> crate::runtime::WorthQueryWorkspace {
        PublicBridgeRuntimeHarness::new()
            .bridge_backed_runtime()
            .workspace(name)
            .expect("runtime should open workspace")
    }
}
