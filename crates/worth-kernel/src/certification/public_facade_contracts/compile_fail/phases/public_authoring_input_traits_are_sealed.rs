use worth_kernel::facade::authoring::construction::{
    PrimitiveConstructionAuthoringInput, PrimitiveConstructionIntent,
    PrimitiveConstructionSpatialIntentError,
};

struct ForeignLoweringLane;

impl PrimitiveConstructionAuthoringInput for ForeignLoweringLane {
    fn lower_for_query_entry(
        self,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        Ok(PrimitiveConstructionIntent::wire_body(
            worth_kernel::facade::authoring::construction::WireBodySpec { edge_count: 4 },
        ))
    }
}

fn main() {}
