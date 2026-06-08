use serde::{Deserialize, Serialize};

use super::{
    RelationalMergeCorrespondencePosture, RelationalMergeSchemaReconciliationPosture,
    RelationalMergeTopologyIntent,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationalMergeRequestNormalizationDenial {
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
