use worth_kernel::workload_composition::{
    BuiltBooleanOperandPairRecipe, WorkloadCatalogRecipeKind,
};

fn main() {
    let _ = BuiltBooleanOperandPairRecipe {
        recipe: WorkloadCatalogRecipeKind::BooleanCleanPlanarBodyPair,
        declaration: panic!("declaration is private"),
        support: panic!("support is private"),
        operand_pair_identity: String::from("forged operand pair"),
        left: panic!("left operand is private"),
        right: panic!("right operand is private"),
    };
}
