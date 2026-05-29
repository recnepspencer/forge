use schema::facade::{
    DiagnosticsInvariantGroup, GeometryInvariantGroup, InvariantGroup, LineageInvariantGroup,
    NamingInvariantGroup, TopologyInvariantGroup,
};

fn main() {
    let _ = (
        None::<InvariantGroup>,
        None::<TopologyInvariantGroup>,
        None::<GeometryInvariantGroup>,
        None::<LineageInvariantGroup>,
        None::<NamingInvariantGroup>,
        None::<DiagnosticsInvariantGroup>,
    );
}
