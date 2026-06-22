use worth_kernel::workload_composition::{
    WorkloadCatalogDeclarationReceipt, WorkloadCatalogRecipe, WorkloadCatalogRecipeKind,
    WorkloadCatalogSupportPosture, WorkloadCatalogSupportReceipt,
};

fn main() {
    let _recipe = WorkloadCatalogRecipe {
        kind: WorkloadCatalogRecipeKind::Cube,
        declaration: String::from("static fixture pretending to be catalog output"),
        transform_recipe: None,
        retained_replay_recipe: None,
    };

    let _declaration = WorkloadCatalogDeclarationReceipt {
        recipe: WorkloadCatalogRecipeKind::Cube,
        declaration: String::from("static fixture pretending to be catalog output"),
        query_declaration_digest: String::from("hand written declaration digest"),
        query_envelope_digest: String::from("hand written envelope digest"),
        query_handle_digest: String::from("hand written handle digest"),
    };

    let _support = WorkloadCatalogSupportReceipt {
        recipe: WorkloadCatalogRecipeKind::Cube,
        posture: WorkloadCatalogSupportPosture::Admitted,
        query_support_digest: String::from("hand written support digest"),
        human_reason: String::from("fake support receipt"),
    };
}
