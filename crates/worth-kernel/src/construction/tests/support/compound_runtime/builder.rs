use forge_query::facade::ForgeQueryRuntimeError;

use super::cases::compound_scenarios;
use super::row_builder::build_rows_for_lane;
use super::rows::PrimitiveConstructionCompoundRow;
use crate::construction::tests::support::compound_lowering::PrimitiveConstructionMotionLoweringError;
use crate::construction::tests::support::compound_ordering::{
    apply_compound_authoring_order_lane, PrimitiveConstructionAdversarialAuthoringOrderLane,
};

#[derive(Debug)]
pub(crate) enum PrimitiveConstructionCompoundAdversarialSiegeError {
    Motion(PrimitiveConstructionMotionLoweringError),
    QueryRuntime(ForgeQueryRuntimeError),
    InvalidSpecializedRow(String),
    NumericWitness(String),
}

impl std::fmt::Display for PrimitiveConstructionCompoundAdversarialSiegeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Motion(error) => write!(f, "{error}"),
            Self::QueryRuntime(error) => write!(f, "{error}"),
            Self::InvalidSpecializedRow(reason) => write!(f, "{reason}"),
            Self::NumericWitness(reason) => write!(f, "{reason}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionCompoundAdversarialSiegeError {}

impl From<ForgeQueryRuntimeError> for PrimitiveConstructionCompoundAdversarialSiegeError {
    fn from(error: ForgeQueryRuntimeError) -> Self {
        Self::QueryRuntime(error)
    }
}

pub(crate) type PrimitiveConstructionCompoundAdversarialLanes = Vec<(
    PrimitiveConstructionAdversarialAuthoringOrderLane,
    Vec<PrimitiveConstructionCompoundRow>,
)>;

pub(crate) fn prepare_primitive_construction_compound_adversarial_lanes() -> Result<
    PrimitiveConstructionCompoundAdversarialLanes,
    PrimitiveConstructionCompoundAdversarialSiegeError,
> {
    let scenarios = compound_scenarios();
    let mut lanes = Vec::with_capacity(
        PrimitiveConstructionAdversarialAuthoringOrderLane::all_compound().len(),
    );
    lanes.push(build_lane(
        PrimitiveConstructionAdversarialAuthoringOrderLane::Canonical,
        build_rows_for_lane(&apply_compound_authoring_order_lane(
            PrimitiveConstructionAdversarialAuthoringOrderLane::Canonical,
            &scenarios,
        ))?,
    ));
    for lane in PrimitiveConstructionAdversarialAuthoringOrderLane::all_compound()
        .into_iter()
        .filter(|lane| *lane != PrimitiveConstructionAdversarialAuthoringOrderLane::Canonical)
    {
        lanes.push(build_lane(
            lane,
            build_rows_for_lane(&apply_compound_authoring_order_lane(lane, &scenarios))?,
        ));
    }
    Ok(lanes)
}

fn build_lane(
    lane: PrimitiveConstructionAdversarialAuthoringOrderLane,
    rows: Vec<PrimitiveConstructionCompoundRow>,
) -> (
    PrimitiveConstructionAdversarialAuthoringOrderLane,
    Vec<PrimitiveConstructionCompoundRow>,
) {
    (lane, rows)
}
