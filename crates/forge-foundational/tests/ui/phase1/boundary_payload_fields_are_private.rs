use forge_foundational::FoundationalBoundaryDeclaration;

fn main() {
    let _ = FoundationalBoundaryDeclaration {
        crate_name: "forge-foundational",
        standardizes: "whatever a caller claims",
        does_not_standardize: "whatever a caller claims",
    };
}
