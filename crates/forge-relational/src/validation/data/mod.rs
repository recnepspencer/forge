#![allow(unused_imports)]

mod catalog;
mod contracts;
mod execution;
mod groups;
mod results;
mod rules;

pub use catalog::{InvariantCatalog, InvariantRegistration};
pub(crate) use catalog::relation_integrity_registrations_for_plan;
pub use contracts::InvariantPlanContract;
pub use execution::{
    InvariantCheckResult, InvariantClass, InvariantExecutionPoint, InvariantFailureEffect,
    InvariantVerdict,
};
pub use groups::{InvariantCostClass, InvariantGroup, InvariantGroupSet};
pub use results::{
    InvariantAdvisory, InvariantViolation, InvariantViolationFields, RelationCardinalityBoundary,
    RelationEndpointBoundary,
};
pub use rules::{InvariantRule, RecordKindTag};
