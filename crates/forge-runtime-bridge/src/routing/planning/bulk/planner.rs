use std::sync::Arc;

use crate::error::{BridgeRouteError, BridgeRouteErrorKind};
use crate::facade::{BridgeDiagnosticsTier, RuntimeBridge};
use crate::mapping::{CoarseRoutingMode, FrozenMappingRegistry, MappingSelector};
use crate::routing::canonicalization::digest_string;
use crate::routing::planning::BridgePlannedRoute;

use super::types::*;

mod admission;
mod labels;
mod packet_reduction;
mod support;
mod workload_pipeline;

pub(crate) use labels::{
    bulk_decision_kind_label, parallel_admission_class_label, parallel_admission_reason_label,
    parallel_legality_class_label, parallel_legality_reason_label,
    parallel_profitability_class_label, parallel_profitability_reason_label,
    planning_failure_kind_label, preparation_mode_label,
};
pub(crate) use workload_pipeline::plan_bulk_workload;
