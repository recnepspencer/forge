use forge_query::facade::{
    ProjectionConsumptionAuthoringSurface, ProjectionConsumptionDeclarationBuilder,
};

fn main() {
    let surface = ProjectionConsumptionAuthoringSurface {
        source: loop {},
        binding: loop {},
    };

    let _builder = ProjectionConsumptionDeclarationBuilder {
        surface,
        requested: loop {},
    };
}
