#[cfg(feature = "parallel")]
pub(crate) mod admission;
pub(crate) mod dispatch;
#[cfg(feature = "parallel")]
pub(crate) mod executor_pool;
pub(crate) mod reporting;
pub(crate) mod stage;

mod eligibility;
mod read_preparation;
mod stage_data;
mod temporal;

use self::read_preparation::precompute_stage_serial;
#[cfg(feature = "parallel")]
use self::read_preparation::{build_parallel_stage_patches, precompute_stage_parallel};
pub(in crate::logic::planner) use self::stage_data::{PreparedTaskPatch, StageExecutionData};
pub(crate) use self::temporal::TemporalLoweringContext;
