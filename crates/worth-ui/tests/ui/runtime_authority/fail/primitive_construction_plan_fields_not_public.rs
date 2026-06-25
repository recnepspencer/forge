use worth_ui::facade::{
    SurfaceId, WorthUiPrimitiveConstructionFamilySelection, WorthUiPrimitiveConstructionGraphPlan,
    WorthUiPrimitiveConstructionPlan, WorthUiValidatedProjectionDependencyContract,
};

fn main() {
    let _plan = WorthUiPrimitiveConstructionPlan {
        surface_id: SurfaceId::new("worth.surface.preview.primitive.proof").unwrap(),
        family_selection: WorthUiPrimitiveConstructionFamilySelection { families: Vec::new() },
        dependency_contract: dependency_contract(),
        query_graph_plan: query_graph_plan(),
    };
}

fn dependency_contract() -> WorthUiValidatedProjectionDependencyContract {
    panic!("this compile-fail test must fail before runtime")
}

fn query_graph_plan() -> WorthUiPrimitiveConstructionGraphPlan {
    panic!("this compile-fail test must fail before runtime")
}
