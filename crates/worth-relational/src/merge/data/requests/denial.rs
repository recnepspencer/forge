use serde::{Deserialize, Serialize};

use super::{
    RelationalMergeCorrespondencePosture, RelationalMergeRequestBindingDenial,
    RelationalMergeSchemaReconciliationPosture, RelationalMergeTopologyIntent,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationalMergeRequestNormalizationDenial {
    OwnerBinding(RelationalMergeRequestBindingDenial),
    UnsupportedCorrespondencePosture {
        posture: RelationalMergeCorrespondencePosture,
    },
    UnsupportedSchemaReconciliationPosture {
        posture: RelationalMergeSchemaReconciliationPosture,
    },
    UnsupportedTopologyIntent {
        intent: RelationalMergeTopologyIntent,
    },
}
