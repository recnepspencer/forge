use std::collections::{BTreeMap, BTreeSet};

use crate::domain_installation::{
    WorthQueryCollectionPatchFact, WorthQueryCollectionRowHandle, WorthQueryNativeAccessKey,
};
use crate::memory_workspace::{WorthQueryEntity, WorthQueryEntityIdentity};
use crate::runtime::WorthQueryCanonicalOrderingKey;

#[derive(Clone)]
pub(super) struct MaintenanceRow {
    pub(super) entity: WorthQueryEntity,
    pub(super) consumer_identity: WorthQueryEntityIdentity,
    pub(super) source_row_identity: String,
    pub(super) view_local_identity: String,
    pub(super) grouping_identity: Vec<String>,
}

pub(crate) struct WorthQueryCollectionMaintenanceInputs<'a> {
    pub request: crate::declarative_live::DeclarativeLiveQueryRequest,
    pub window_policy: crate::domain_installation::WorthQueryOperationWindowPolicy,
    pub continuation_posture: crate::domain_installation::WorthQueryOperationContinuationPosture,
    pub delivery_supported: bool,
    pub entities: Vec<WorthQueryEntity>,
    pub handles: &'a [WorthQueryCollectionRowHandle],
    pub native_keys: Vec<WorthQueryNativeAccessKey>,
    pub grouping_fields: Vec<worth_query_installation::facade::WorthQueryOperationCollectionField>,
}

pub(crate) struct WorthQueryCollectionIndexDelta {
    pub(super) removals: BTreeSet<WorthQueryCanonicalOrderingKey>,
    pub(super) upserts: BTreeMap<WorthQueryCanonicalOrderingKey, MaintenanceRow>,
    pub(super) group_transitions: Vec<WorthQueryCollectionGroupTransition>,
}

#[derive(Clone)]
pub(crate) struct WorthQueryCollectionGroupTransition {
    pub(crate) entity: WorthQueryEntityIdentity,
    pub(crate) from: Option<Vec<String>>,
    pub(crate) to: Option<Vec<String>>,
}

pub(crate) struct WorthQueryCollectionIndexPreview {
    pub(crate) rows: Vec<WorthQueryCollectionRowHandle>,
    pub(crate) consumer_affected: BTreeSet<WorthQueryEntityIdentity>,
    pub(crate) has_more: bool,
    pub(crate) facts: Vec<WorthQueryCollectionPatchFact>,
    pub(crate) delta: WorthQueryCollectionIndexDelta,
}
