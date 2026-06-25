use crate::runtime::WorthUiQueryGraphExecutionReceipt;

use super::operation_family::WorthUiUserIntentOperationFamily;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiUserIntentTargetDenial {
    MissingSlot {
        page_name: String,
        slot_name: String,
        operation_family: WorthUiUserIntentOperationFamily,
        graph_execution: WorthUiQueryGraphExecutionReceipt,
    },
    MissingSurface {
        slot_name: String,
        surface_id: String,
        operation_family: WorthUiUserIntentOperationFamily,
        graph_execution: WorthUiQueryGraphExecutionReceipt,
    },
    InvalidSurfaceId {
        slot_name: String,
        surface_id: String,
        operation_family: WorthUiUserIntentOperationFamily,
        graph_execution: WorthUiQueryGraphExecutionReceipt,
    },
    InvalidComponentId {
        slot_name: String,
        surface_id: String,
        component_id: String,
        operation_family: WorthUiUserIntentOperationFamily,
        graph_execution: WorthUiQueryGraphExecutionReceipt,
    },
}

impl WorthUiUserIntentTargetDenial {
    pub fn query_graph_execution(&self) -> &WorthUiQueryGraphExecutionReceipt {
        match self {
            Self::MissingSlot {
                graph_execution, ..
            }
            | Self::MissingSurface {
                graph_execution, ..
            }
            | Self::InvalidSurfaceId {
                graph_execution, ..
            }
            | Self::InvalidComponentId {
                graph_execution, ..
            } => graph_execution,
        }
    }
}
